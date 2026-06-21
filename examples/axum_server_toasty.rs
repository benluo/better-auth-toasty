use axum::{Json, Router, response::IntoResponse, routing::get};
use better_auth::handlers::{AxumIntegration, CurrentSession, OptionalSession};
use better_auth::plugins::{
    AccountManagementPlugin, EmailPasswordPlugin, OrganizationPlugin, PasswordManagementPlugin,
    SessionManagementPlugin,
};
use better_auth::{AuthBuilder, AuthConfig, AuthUser, BetterAuth};
use better_auth_toasty::ToastyAdapter;
use std::sync::Arc;
use tokio::net::TcpListener;
use toasty::Db;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();

    println!("Starting Better Auth Axum Server with Toasty + Turso");

    // Get database path from environment or use default
    let db_path = std::env::var("TURSO_DB_PATH").unwrap_or_else(|_| "./auth.db".to_string());
    let db_url = format!("turso:{}", db_path);

    println!("Connecting to database: {}", db_url);

    // Initialize Toasty with Turso driver
    let db = Db::builder()
        .models(toasty::models!(
            better_auth_toasty::models::User,
            better_auth_toasty::models::Session,
            better_auth_toasty::models::Account,
            better_auth_toasty::models::Verification,
        ))
        .connect(&db_url)
        .await?;

    // Push schema to create tables
    db.push_schema().await?;
    println!("Database schema initialized");

    // Wrap in ToastyAdapter
    let adapter = ToastyAdapter::new(db);

    // Create configuration
    let secret = std::env::var("AUTH_SECRET")
        .unwrap_or_else(|_| "your-very-secure-secret-key-at-least-32-chars-long".to_string());

    let config = AuthConfig::new(&secret)
        .base_url("http://localhost:8080")
        .password_min_length(6);

    println!("Configuration created");

    // Build the authentication system
    let auth = Arc::new(
        AuthBuilder::new(config)
            .database(adapter)
            .plugin(EmailPasswordPlugin::new().enable_signup(true))
            .plugin(SessionManagementPlugin::new())
            .plugin(PasswordManagementPlugin::new())
            .plugin(AccountManagementPlugin::new())
            .plugin(OrganizationPlugin::new())
            .build()
            .await?,
    );

    println!("BetterAuth instance created");
    println!("Registered plugins: {:?}", auth.plugin_names());

    // Create the main application router
    let app = create_app_router(auth).await;

    println!("Starting server on http://localhost:8080");
    println!("Available endpoints:");
    println!("  Authentication:");
    println!("    POST /auth/sign-up/email - Sign up with email/password");
    println!("    POST /auth/sign-in/email - Sign in with email/password");
    println!("  Session Management:");
    println!("    GET /auth/get-session - Get current session info");
    println!("    POST /auth/sign-out - Sign out current session");
    println!("    GET /auth/list-sessions - List all user sessions");
    println!("  Password Management:");
    println!("    POST /auth/forget-password - Request password reset");
    println!("    POST /auth/reset-password - Reset password with token");
    println!("    POST /auth/change-password - Change password (auth)");
    println!("  User Management:");
    println!("    POST /auth/update-user - Update user profile (auth)");
    println!("    POST /auth/delete-user - Delete user account (auth)");
    println!("  Organization:");
    println!("    POST /auth/organization/create - Create organization (auth)");
    println!("    GET /auth/organization/list - List organizations (auth)");
    println!("  Other:");
    println!("    GET /auth/ok - Health check");
    println!("    GET /api/profile - Protected API route");
    println!("    GET /api/public - Public API route");

    // Start the server
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_app_router(auth: Arc<BetterAuth<ToastyAdapter>>) -> Router {
    let auth_router = auth.clone().axum_router();

    Router::new()
        .route("/api/profile", get(get_user_profile))
        .route("/api/protected", get(protected_route))
        .route("/api/public", get(public_route))
        .nest("/auth", auth_router)
        .layer(CorsLayer::permissive())
        .with_state(auth)
}

// Protected route - uses CurrentSession extractor (returns 401 automatically)
async fn get_user_profile(session: CurrentSession<ToastyAdapter>) -> impl IntoResponse {
    Json(serde_json::json!({
        "id": session.user.id(),
        "email": session.user.email(),
        "name": session.user.name(),
        "created_at": session.user.created_at().to_rfc3339(),
    }))
}

async fn protected_route(session: CurrentSession<ToastyAdapter>) -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "This is a protected route",
        "user_id": session.user.id(),
    }))
}

// Public route - uses OptionalSession to optionally show user info
async fn public_route(session: OptionalSession<ToastyAdapter>) -> impl IntoResponse {
    let user_info = session.0.map(|s| {
        serde_json::json!({
            "id": s.user.id(),
            "email": s.user.email(),
        })
    });

    Json(serde_json::json!({
        "message": "This is a public route",
        "user": user_info,
    }))
}
