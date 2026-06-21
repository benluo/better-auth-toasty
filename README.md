# better-auth-toasty

An MIT-licensed [Toasty](https://github.com/tokio-rs/toasty) database adapter for the [better-auth-rs](https://github.com/better-auth-rs/better-auth-rs) framework.

## Features

- Implements all 10 `better-auth-rs` adapter operation traits (`UserOps`, `SessionOps`, `AccountOps`, `VerificationOps`, `OrganizationOps`, `MemberOps`, `InvitationOps`, `TwoFactorOps`, `ApiKeyOps`, `PasskeyOps`)
- Uses framework built-in types as associated types with a conversion layer between ORM models and auth types
- Supports Turso (SQLite-compatible) via the `toasty-driver-turso` backend

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
better-auth-toasty = "0.1"
better-auth = { version = "0.10", features = ["rustls"] }
toasty = "0.7"
tokio = { version = "1", features = ["full"] }
```

## Usage

### Quick Start (Local Turso)

```rust
use better_auth::{AuthConfig, BetterAuth};
use better_auth_toasty::ToastyAdapter;
use toasty::Db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Toasty with the adapter's models
    let db = Db::builder()
        .models(toasty::models!(better_auth_toasty::models::*))
        .connect("turso:./auth.db")
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

### Turso Cloud Database

[Turso](https://turso.tech) provides a cloud-hosted SQLite-compatible database with edge replication. You can use it with `better-auth-toasty` via an [embedded replica](https://docs.turso.tech/features/embedded-replicas/introduction) that syncs to your local disk.

#### 1. Create a Turso cloud database

```bash
# Install the Turso CLI
curl -sSfL https://get.tur.so/install.sh | bash

# Log in and create a database
turso auth login
turso db create my-auth-db

# Get the database URL and auth token
turso db show my-auth-db --url
turso db tokens create my-auth-db
```

#### 2. Set environment variables

```bash
export TURSO_DATABASE_URL="libsql://my-auth-db-your-org.turso.io"
export TURSO_AUTH_TOKEN="your-auth-token"
```

#### 3. Configure your application

```rust
use better_auth::{AuthConfig, BetterAuth};
use better_auth_toasty::ToastyAdapter;
use toasty::Db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The embedded replica syncs from Turso cloud to a local file.
    // Toasty connects to the local file via the turso driver.
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

    // --- Example: sign up a user ---
    // let result = auth.sign_up_email("alice@example.com", "password123").await?;

    // --- Example: sign in ---
    // let session = auth.sign_in_email("alice@example.com", "password123").await?;

    // --- Example: get session from token ---
    // let user = auth.get_session("session_token_here").await?;

    Ok(())
}
```

#### 4. Sync the embedded replica

Run the Turso CLI to push/pull data between the local replica and the cloud database:

```bash
# Push local changes to cloud
turso db shell my-auth-db < /dev/null

# Or use the embedded replica sync in your app via the turso crate directly
# for programmatic push/pull operations.
```

> **Note:** Toasty's Turso driver currently supports local file-backed and in-memory databases. For programmatic cloud sync (push/pull), use the [`turso`](https://docs.turso.tech/sdk/rust/quickstart) crate alongside this adapter.

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
│   toasty-driver-turso (SQLite-compatible)   │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│       Turso (local file / embedded replica)  │
└─────────────────────────────────────────────┘
```

## License

Licensed under [MIT License](https://github.com/benluo/better-auth-toasty/blob/main/LICENSE)
