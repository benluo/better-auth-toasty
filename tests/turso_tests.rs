mod common;

use better_auth_core::adapters::{AccountOps, SessionOps, UserOps, VerificationOps};
use better_auth_core::types::{CreateAccount, CreateSession, CreateUser, CreateVerification, UpdateUser};
use better_auth_core::{AuthAccount, AuthSession, AuthUser};
use chrono::{Duration, Utc};
use uuid::Uuid;

use better_auth_toasty::ToastyAdapter;

async fn setup_turso() -> ToastyAdapter {
    let url = std::env::var("TURSO_DATABASE_URL")
        .unwrap_or_else(|_| "turso::memory:".to_string());
    common::setup_adapter(&url).await
}

// ── User Tests (Turso) ──────────────────────────────────────────────────────

#[tokio::test]
async fn test_turso_create_user() {
    let adapter = setup_turso().await;
    let create = CreateUser {
        id: Some(Uuid::new_v4().to_string()),
        email: Some("turso-alice@example.com".to_string()),
        name: Some("Turso Alice".to_string()),
        image: None,
        email_verified: Some(true),
        password: None,
        username: Some("turso_alice".to_string()),
        display_username: None,
        role: Some("user".to_string()),
        metadata: None,
    };
    let user = adapter.create_user(create).await.unwrap();
    assert_eq!(user.email().unwrap(), "turso-alice@example.com");
    assert_eq!(user.name().unwrap(), "Turso Alice");
    assert!(user.email_verified());
}

#[tokio::test]
async fn test_turso_get_user_by_id() {
    let adapter = setup_turso().await;
    let id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(id.clone()),
            email: Some("turso-bob@example.com".to_string()),
            name: Some("Turso Bob".to_string()),
            image: None,
            email_verified: Some(false),
            password: None,
            username: None,
            display_username: None,
            role: None,
            metadata: None,
        })
        .await
        .unwrap();

    let found = adapter.get_user_by_id(&id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().email().unwrap(), "turso-bob@example.com");
}

#[tokio::test]
async fn test_turso_get_user_by_email() {
    let adapter = setup_turso().await;
    adapter
        .create_user(CreateUser {
            id: Some(Uuid::new_v4().to_string()),
            email: Some("turso-carol@example.com".to_string()),
            name: Some("Turso Carol".to_string()),
            image: None,
            email_verified: Some(false),
            password: None,
            username: None,
            display_username: None,
            role: None,
            metadata: None,
        })
        .await
        .unwrap();

    let found = adapter
        .get_user_by_email("turso-carol@example.com")
        .await
        .unwrap();
    assert!(found.is_some());

    let not_found = adapter
        .get_user_by_email("nobody-turso@example.com")
        .await
        .unwrap();
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_turso_update_user() {
    let adapter = setup_turso().await;
    let id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(id.clone()),
            email: Some("turso-dave@example.com".to_string()),
            name: Some("Turso Dave".to_string()),
            image: None,
            email_verified: Some(false),
            password: None,
            username: None,
            display_username: None,
            role: Some("user".to_string()),
            metadata: None,
        })
        .await
        .unwrap();

    let updated = adapter
        .update_user(
            &id,
            UpdateUser {
                name: Some("Turso David".to_string()),
                role: Some("admin".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert_eq!(updated.name().unwrap(), "Turso David");
    assert_eq!(updated.role().unwrap(), "admin");
}

#[tokio::test]
async fn test_turso_delete_user() {
    let adapter = setup_turso().await;
    let id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(id.clone()),
            email: Some("turso-eve@example.com".to_string()),
            name: None,
            image: None,
            email_verified: None,
            password: None,
            username: None,
            display_username: None,
            role: None,
            metadata: None,
        })
        .await
        .unwrap();

    adapter.delete_user(&id).await.unwrap();
    let found = adapter.get_user_by_id(&id).await.unwrap();
    assert!(found.is_none());
}

// ── Session Tests (Turso) ───────────────────────────────────────────────────

#[tokio::test]
async fn test_turso_create_and_get_session() {
    let adapter = setup_turso().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("turso-session@example.com".to_string()),
            name: None,
            image: None,
            email_verified: None,
            password: None,
            username: None,
            display_username: None,
            role: None,
            metadata: None,
        })
        .await
        .unwrap();

    let session = adapter
        .create_session(CreateSession {
            user_id: user_id.clone(),
            expires_at: Utc::now() + Duration::hours(1),
            ip_address: Some("10.0.0.1".to_string()),
            user_agent: Some("turso-agent".to_string()),
            impersonated_by: None,
            active_organization_id: None,
        })
        .await
        .unwrap();

    assert!(session.token().starts_with("session_"));
    assert_eq!(session.user_id(), user_id);
    assert!(session.active());

    let found = adapter.get_session(session.token()).await.unwrap();
    assert!(found.is_some());
}

#[tokio::test]
async fn test_turso_delete_session() {
    let adapter = setup_turso().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("turso-del-sess@example.com".to_string()),
            name: None,
            image: None,
            email_verified: None,
            password: None,
            username: None,
            display_username: None,
            role: None,
            metadata: None,
        })
        .await
        .unwrap();

    let session = adapter
        .create_session(CreateSession {
            user_id,
            expires_at: Utc::now() + Duration::hours(1),
            ip_address: None,
            user_agent: None,
            impersonated_by: None,
            active_organization_id: None,
        })
        .await
        .unwrap();

    adapter.delete_session(session.token()).await.unwrap();
    let found = adapter.get_session(session.token()).await.unwrap();
    assert!(found.is_none());
}

// ── Account Tests (Turso) ───────────────────────────────────────────────────

#[tokio::test]
async fn test_turso_create_and_get_account() {
    let adapter = setup_turso().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("turso-acct@example.com".to_string()),
            name: None,
            image: None,
            email_verified: None,
            password: None,
            username: None,
            display_username: None,
            role: None,
            metadata: None,
        })
        .await
        .unwrap();

    let account = adapter
        .create_account(CreateAccount {
            user_id: user_id.clone(),
            account_id: "turso-google-id".to_string(),
            provider_id: "google".to_string(),
            access_token: Some("turso-access".to_string()),
            refresh_token: None,
            id_token: None,
            access_token_expires_at: None,
            refresh_token_expires_at: None,
            scope: Some("openid".to_string()),
            password: None,
        })
        .await
        .unwrap();

    assert_eq!(account.provider_id(), "google");

    let found = adapter
        .get_account("google", "turso-google-id")
        .await
        .unwrap();
    assert!(found.is_some());
}

#[tokio::test]
async fn test_turso_delete_account() {
    let adapter = setup_turso().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("turso-del-acct@example.com".to_string()),
            name: None,
            image: None,
            email_verified: None,
            password: None,
            username: None,
            display_username: None,
            role: None,
            metadata: None,
        })
        .await
        .unwrap();

    let account = adapter
        .create_account(CreateAccount {
            user_id,
            account_id: "turso-del-id".to_string(),
            provider_id: "test".to_string(),
            access_token: None,
            refresh_token: None,
            id_token: None,
            access_token_expires_at: None,
            refresh_token_expires_at: None,
            scope: None,
            password: None,
        })
        .await
        .unwrap();

    adapter.delete_account(account.id()).await.unwrap();
    let found = adapter.get_account("test", "turso-del-id").await.unwrap();
    assert!(found.is_none());
}

// ── Verification Tests (Turso) ──────────────────────────────────────────────

#[tokio::test]
async fn test_turso_create_and_get_verification() {
    let adapter = setup_turso().await;

    adapter
        .create_verification(CreateVerification {
            identifier: "turso-verify@example.com".to_string(),
            value: "turso-token-789".to_string(),
            expires_at: Utc::now() + Duration::hours(1),
        })
        .await
        .unwrap();

    let found = adapter
        .get_verification("turso-verify@example.com", "turso-token-789")
        .await
        .unwrap();
    assert!(found.is_some());
}

#[tokio::test]
async fn test_turso_consume_verification() {
    let adapter = setup_turso().await;

    adapter
        .create_verification(CreateVerification {
            identifier: "turso-consume@example.com".to_string(),
            value: "turso-consume-token".to_string(),
            expires_at: Utc::now() + Duration::hours(1),
        })
        .await
        .unwrap();

    let consumed = adapter
        .consume_verification("turso-consume@example.com", "turso-consume-token")
        .await
        .unwrap();
    assert!(consumed.is_some());

    let gone = adapter
        .get_verification("turso-consume@example.com", "turso-consume-token")
        .await
        .unwrap();
    assert!(gone.is_none());
}
