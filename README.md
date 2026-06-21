# better-auth-toasty

An MIT-licensed [Toasty](https://github.com/tokio-rs/toasty) database adapter for the [better-auth-rs](https://github.com/better-auth-rs/better-auth-rs) framework.

## Features

- Implements `better-auth-rs` adapter operation traits with a conversion layer between ORM models and auth types
- **Fully implemented:** `UserOps`, `SessionOps`, `AccountOps`, `VerificationOps`
- **Stub implementations (todo!):** `OrganizationOps`, `MemberOps`, `InvitationOps`, `TwoFactorOps`, `ApiKeyOps`, `PasskeyOps`
- Supports **SQLite** (file-backed or in-memory) and **Turso** (SQLite-compatible, including cloud embedded replicas)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
better-auth-toasty = { git = "https://github.com/benluo/better-auth-toasty" }
better-auth = { version = "0.10", features = ["rustls"] }
toasty = "0.7"
tokio = { version = "1", features = ["full"] }
```

## Usage

### Quick Start (SQLite)

```rust
use better_auth::{AuthConfig, BetterAuth};
use better_auth_toasty::ToastyAdapter;
use toasty::Db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Toasty with the adapter's models
    let db = Db::builder()
        .models(toasty::models!(better_auth_toasty::models::*))
        .connect("sqlite:./auth.db")
        .await?;

    // 2. Push the schema to create tables
    db.push_schema().await?;

    // 3. Wrap the Db in the ToastyAdapter
    let adapter = ToastyAdapter::new(db);

    // 4. Inject it into BetterAuth
    let auth = BetterAuth::new(AuthConfig::new("your-secret-key"))
        .database(adapter)
        .build()
        .await?;

    Ok(())
}
```

### Using Turso (Local or Cloud)

You can also use [Turso](https://turso.tech) for a cloud-hosted SQLite-compatible database with edge replication.

#### Local Turso file

Replace the connection string with a `turso:` prefix:

```rust
let db = Db::builder()
    .models(toasty::models!(better_auth_toasty::models::*))
    .connect("turso:./auth.db")
    .await?;
```

#### Turso Cloud with embedded replica

1. **Create a Turso cloud database:**

    ```bash
    curl -sSfL https://get.tur.so/install.sh | bash
    turso auth login
    turso db create my-auth-db
    turso db show my-auth-db --url
    turso db tokens create my-auth-db
    ```

2. **Set environment variables:**

    ```bash
    export TURSO_DATABASE_URL="libsql://my-auth-db-your-org.turso.io"
    export TURSO_AUTH_TOKEN="your-auth-token"
    ```

3. **Configure your application:**

    ```rust
    let db = Db::builder()
        .models(toasty::models!(better_auth_toasty::models::*))
        .connect("turso:./auth.db")
        .await?;

    db.push_schema().await?;

    let adapter = ToastyAdapter::new(db);

    let auth = BetterAuth::new(
        AuthConfig::new(
            std::env::var("AUTH_SECRET")
                .expect("AUTH_SECRET must be set"),
        ),
    )
    .database(adapter)
    .build()
    .await?;
    ```

> **Note:** Toasty's Turso driver supports local file-backed and in-memory databases. For programmatic cloud sync (push/pull), use the [`turso`](https://docs.turso.tech/sdk/rust/quickstart) crate alongside this adapter.

### Connection Strings

| Backend | Connection String | Description |
|---------|------------------|-------------|
| SQLite (file) | `"sqlite:./auth.db"` | Local SQLite file |
| Turso (file) | `"turso:./auth.db"` | Local Turso file (SQLite-compatible) |
| Turso (in-memory) | `"turso::memory:"` | In-memory Turso database |

## Axum Example

A full Axum web server example is available in [`examples/axum_server_toasty.rs`](examples/axum_server_toasty.rs). It demonstrates:

- Email/password sign-up and sign-in
- Session management (get, list, sign-out)
- Password management (forget, reset, change)
- Account management (update profile, delete account)
- Organization creation and listing
- Protected routes using `CurrentSession<ToastyAdapter>` extractor
- Public routes using `OptionalSession<ToastyAdapter>` extractor
- CORS via `tower-http`

Run it with:

```bash
cargo run --example axum_server_toasty
```

## Testing

The project includes integration tests for both SQLite and Turso backends:

```bash
# Run all tests
cargo test

# Run SQLite tests only
cargo test --test sqlite_tests

# Run Turso tests only
cargo test --test turso_tests
```

## Architecture

```
┌─────────────────────────────────────────────┐
│              better-auth-rs                  │
│   (UserOps, SessionOps, AccountOps, ...)    │
└──────────────────┬──────────────────────────┘
                   │  framework types (User, Session, ...)
                   ▼
┌─────────────────────────────────────────────┐
│           better-auth-toasty                 │
│   conversions.rs  │  ops/*.rs               │
└──────────────────┬──────────────────────────┘
                   │  ORM models (User, Session, ...)
                   ▼
┌─────────────────────────────────────────────┐
│           Toasty ORM                         │
│   SQLite / Turso (toasty-driver-turso)      │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│   SQLite file / Turso local or cloud        │
└─────────────────────────────────────────────┘
```

## License

Licensed under [MIT License](https://github.com/benluo/better-auth-toasty/blob/main/LICENSE)
