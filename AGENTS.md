# better-auth-toasty

A Toasty ORM database adapter for the [better-auth-rs](https://github.com/better-auth-rs/better-auth-rs) authentication framework. Bridges better-auth-rs adapter traits to Toasty-supported databases (SQLite, Turso).

## Tech Stack

- **Language:** Rust (edition 2024)
- **ORM:** Toasty 0.7
- **Auth Framework:** better-auth 0.10 / better-auth-core 0.10
- **Async Runtime:** Tokio
- **Web Framework (examples):** Axum 0.8, tower-http 0.6
- **Serialization:** serde / serde_json
- **Databases:** SQLite (file/in-memory), Turso (file/in-memory/cloud embedded replica)

## Build Commands

- Build: `cargo build`
- Test all: `cargo test`
- Test SQLite only: `cargo test --test sqlite_tests`
- Test Turso only: `cargo test --test turso_tests`
- Clippy: `cargo clippy --all-targets --all-features`
- Format: `cargo fmt`
- Format check: `cargo fmt --check`
- Run example: `cargo run --example axum_server_toasty`
- Docs: `cargo doc --no-deps`

## Project Structure

```
src/
  lib.rs              -- Crate root, re-exports ToastyAdapter
  adapter.rs          -- ToastyAdapter struct (wraps toasty::Db)
  models.rs           -- Toasty ORM models: User, Session, Account, Verification
  conversions.rs      -- Bidirectional conversion: ORM models <-> better-auth-core types
  ops/
    mod.rs            -- Module declarations
    user.rs           -- UserOps impl (CRUD, 2 methods still todo!)
    session.rs        -- SessionOps impl (fully implemented)
    account.rs        -- AccountOps impl (fully implemented)
    verification.rs   -- VerificationOps impl (fully implemented)
    stubs.rs          -- Stub todo!() for 6 traits: OrganizationOps, MemberOps,
                         InvitationOps, TwoFactorOps, ApiKeyOps, PasskeyOps
examples/
  axum_server_toasty.rs  -- Full Axum server with 5 plugins
tests/
  common/mod.rs       -- Shared setup helper (setup_adapter)
  sqlite_tests.rs     -- 16 integration tests against SQLite
  turso_tests.rs      -- 12 integration tests against Turso (in-memory)
```

## Implementation Status

| Trait | Status | Methods |
|---|---|---|
| UserOps | Mostly done | 5/7 (get_user_by_username, list_users are todo!) |
| SessionOps | Done | 8/8 |
| AccountOps | Done | 5/5 |
| VerificationOps | Done | 6/6 |
| OrganizationOps | Stub | 0/6 |
| MemberOps | Stub | 0/8 |
| InvitationOps | Stub | 0/6 |
| TwoFactorOps | Stub | 0/4 |
| ApiKeyOps | Stub | 0/7 |
| PasskeyOps | Stub | 0/7 |

## Coding Conventions

- All timestamps stored as `i64` milliseconds (Unix epoch); converted via `conversions::ts()` / `conversions::millis()` / `conversions::now_millis()`
- IDs are UUIDv4 strings, auto-generated when not provided by the framework
- Session tokens follow the format `session_<uuid-without-dashes>`
- User metadata is stored as a JSON string (`Option<String>`), serialized via `serde_json`
- Each ops file defines its own `db_err()` helper to map Toasty errors to `AuthError::Database(DatabaseError::Query(...))`
- Use `async-trait` for all trait implementations
- Use `toasty::create!()` macro for inserts, `toasty::query!()` macro for filtered queries, `Model::get_by_id()` / `Model::get_by_*()` for lookups
- Use `.update().field(value).exec()` builder pattern for updates
- Clone `self.db` at the start of each async method (`let mut db = self.db.clone();`)
- Rust edition 2024; `rustfmt.toml` sets `max_width = 100`
- No `unsafe` code

## Key Patterns

- **Create flow:** Convert framework `Create*` type to ORM model via `conversions::create_*_to_model()`, insert with `toasty::create!()`, then re-fetch with `get_by_id()` to return the stored row.
- **Read flow:** Fetch ORM model via `Model::get_by_*()` or `toasty::query!()`, convert to framework type via `From<Model> for FrameworkType`.
- **Update flow:** Fetch model, use `.update()` builder with conditional field setters, call `.exec()`, then re-fetch.
- **Delete flow:** Fetch model, call `.delete().exec()`. Silently succeed if not found (for idempotent deletes).
- **Error mapping:** Toasty errors are converted via `.map_err(db_err)` where `db_err` wraps them in `AuthError::Database(DatabaseError::Query(...))`. Auth-specific errors like `AuthError::UserNotFound` or `AuthError::SessionNotFound` are used where semantically appropriate.

## Warnings

- The `username` field on `User` model has no `#[unique]` or `#[index]` annotation, which is why `get_user_by_username` is still `todo!()`.
- The Axum example registers `OrganizationPlugin` but `OrganizationOps` is a stub -- calling organization endpoints will panic.
- Do not write raw SQL; use Toasty's query DSL (`toasty::query!()`, `toasty::create!()`, model methods).
- The `toasty::Db` instance must be initialized with `toasty::models!(better_auth_toasty::models::*)` and `db.push_schema()` must be called before any operations.
