mod common;

use better_auth_core::adapters::{AccountOps, SessionOps, UserOps, VerificationOps};
use better_auth_core::types::{CreateAccount, CreateSession, CreateUser, CreateVerification, UpdateAccount, UpdateUser};
use better_auth_core::{AuthAccount, AuthSession, AuthUser, AuthVerification};
use chrono::{Duration, Utc};
use uuid::Uuid;

use better_auth_toasty::ToastyAdapter;

async fn setup() -> ToastyAdapter {
    let dir = std::env::temp_dir();
    let id = Uuid::new_v4();
    let path = dir.join(format!("better_auth_test_{id}.db"));
    common::setup_adapter(&format!("sqlite:{}", path.display())).await
}

// ── User Tests ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_user() {
    let adapter = setup().await;
    let create = CreateUser {
        id: Some(Uuid::new_v4().to_string()),
        email: Some("alice@example.com".to_string()),
        name: Some("Alice".to_string()),
        image: None,
        email_verified: Some(true),
        password: None,
        username: Some("alice".to_string()),
        display_username: None,
        role: Some("user".to_string()),
        metadata: None,
    };
    let user = adapter.create_user(create).await.unwrap();
    assert_eq!(user.email().unwrap(), "alice@example.com");
    assert_eq!(user.name().unwrap(), "Alice");
    assert_eq!(user.username().unwrap(), "alice");
    assert_eq!(user.role().unwrap(), "user");
    assert!(user.email_verified());
}

#[tokio::test]
async fn test_get_user_by_id() {
    let adapter = setup().await;
    let id = Uuid::new_v4().to_string();
    let create = CreateUser {
        id: Some(id.clone()),
        email: Some("bob@example.com".to_string()),
        name: Some("Bob".to_string()),
        image: None,
        email_verified: Some(false),
        password: None,
        username: None,
        display_username: None,
        role: None,
        metadata: None,
    };
    adapter.create_user(create).await.unwrap();

    let found = adapter.get_user_by_id(&id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().email().unwrap(), "bob@example.com");
}

#[tokio::test]
async fn test_get_user_by_email() {
    let adapter = setup().await;
    let create = CreateUser {
        id: Some(Uuid::new_v4().to_string()),
        email: Some("carol@example.com".to_string()),
        name: Some("Carol".to_string()),
        image: None,
        email_verified: Some(false),
        password: None,
        username: None,
        display_username: None,
        role: None,
        metadata: None,
    };
    adapter.create_user(create).await.unwrap();

    let found = adapter.get_user_by_email("carol@example.com").await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name().unwrap(), "Carol");

    let not_found = adapter.get_user_by_email("nobody@example.com").await.unwrap();
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_update_user() {
    let adapter = setup().await;
    let id = Uuid::new_v4().to_string();
    let create = CreateUser {
        id: Some(id.clone()),
        email: Some("dave@example.com".to_string()),
        name: Some("Dave".to_string()),
        image: None,
        email_verified: Some(false),
        password: None,
        username: None,
        display_username: None,
        role: Some("user".to_string()),
        metadata: None,
    };
    adapter.create_user(create).await.unwrap();

    let update = UpdateUser {
        name: Some("David".to_string()),
        role: Some("admin".to_string()),
        email_verified: Some(true),
        ..Default::default()
    };
    let updated = adapter.update_user(&id, update).await.unwrap();
    assert_eq!(updated.name().unwrap(), "David");
    assert_eq!(updated.role().unwrap(), "admin");
    assert!(updated.email_verified());
}

#[tokio::test]
async fn test_delete_user() {
    let adapter = setup().await;
    let id = Uuid::new_v4().to_string();
    let create = CreateUser {
        id: Some(id.clone()),
        email: Some("eve@example.com".to_string()),
        name: Some("Eve".to_string()),
        image: None,
        email_verified: Some(false),
        password: None,
        username: None,
        display_username: None,
        role: None,
        metadata: None,
    };
    adapter.create_user(create).await.unwrap();

    adapter.delete_user(&id).await.unwrap();
    let found = adapter.get_user_by_id(&id).await.unwrap();
    assert!(found.is_none());
}

// ── Session Tests ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_session() {
    let adapter = setup().await;
    let user_id = Uuid::new_v4().to_string();
    let create_user = CreateUser {
        id: Some(user_id.clone()),
        email: Some("session@example.com".to_string()),
        name: Some("Session User".to_string()),
        image: None,
        email_verified: Some(false),
        password: None,
        username: None,
        display_username: None,
        role: None,
        metadata: None,
    };
    adapter.create_user(create_user).await.unwrap();

    let expires_at = Utc::now() + Duration::hours(1);
    let create_session = CreateSession {
        user_id: user_id.clone(),
        expires_at,
        ip_address: Some("127.0.0.1".to_string()),
        user_agent: Some("test-agent".to_string()),
        impersonated_by: None,
        active_organization_id: None,
    };
    let session = adapter.create_session(create_session).await.unwrap();
    assert!(session.token().starts_with("session_"));
    assert_eq!(session.user_id(), user_id);
    assert!(session.active());

    let found = adapter.get_session(session.token()).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().token(), session.token());
}

#[tokio::test]
async fn test_get_user_sessions() {
    let adapter = setup().await;
    let user_id = Uuid::new_v4().to_string();
    let create_user = CreateUser {
        id: Some(user_id.clone()),
        email: Some("multi@example.com".to_string()),
        name: None,
        image: None,
        email_verified: None,
        password: None,
        username: None,
        display_username: None,
        role: None,
        metadata: None,
    };
    adapter.create_user(create_user).await.unwrap();

    let expires_at = Utc::now() + Duration::hours(1);
    for _ in 0..3 {
        adapter
            .create_session(CreateSession {
                user_id: user_id.clone(),
                expires_at,
                ip_address: None,
                user_agent: None,
                impersonated_by: None,
                active_organization_id: None,
            })
            .await
            .unwrap();
    }

    let sessions = adapter.get_user_sessions(&user_id).await.unwrap();
    assert_eq!(sessions.len(), 3);
}

#[tokio::test]
async fn test_update_session_expiry() {
    let adapter = setup().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("expiry@example.com".to_string()),
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

    let new_expiry = Utc::now() + Duration::hours(24);
    adapter
        .update_session_expiry(session.token(), new_expiry)
        .await
        .unwrap();

    let updated = adapter.get_session(session.token()).await.unwrap().unwrap();
    assert_eq!(
        updated.expires_at().timestamp_millis(),
        new_expiry.timestamp_millis()
    );
}

#[tokio::test]
async fn test_delete_session() {
    let adapter = setup().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("del-session@example.com".to_string()),
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

#[tokio::test]
async fn test_delete_expired_sessions() {
    let adapter = setup().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("expired@example.com".to_string()),
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

    // Create an already-expired session
    adapter
        .create_session(CreateSession {
            user_id: user_id.clone(),
            expires_at: Utc::now() - Duration::hours(1),
            ip_address: None,
            user_agent: None,
            impersonated_by: None,
            active_organization_id: None,
        })
        .await
        .unwrap();

    // Create a valid session
    let valid = adapter
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

    let deleted = adapter.delete_expired_sessions().await.unwrap();
    assert!(deleted >= 1);

    // Valid session should still exist
    let still_there = adapter.get_session(valid.token()).await.unwrap();
    assert!(still_there.is_some());
}

// ── Account Tests ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_account() {
    let adapter = setup().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("acct@example.com".to_string()),
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

    let create = CreateAccount {
        user_id: user_id.clone(),
        account_id: "google-12345".to_string(),
        provider_id: "google".to_string(),
        access_token: Some("access_tok".to_string()),
        refresh_token: Some("refresh_tok".to_string()),
        id_token: None,
        access_token_expires_at: Some(Utc::now() + Duration::hours(1)),
        refresh_token_expires_at: None,
        scope: Some("openid profile email".to_string()),
        password: None,
    };
    let account = adapter.create_account(create).await.unwrap();
    assert_eq!(account.provider_id(), "google");
    assert_eq!(account.account_id(), "google-12345");
    assert_eq!(account.access_token().unwrap(), "access_tok");

    let found = adapter.get_account("google", "google-12345").await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().user_id(), user_id);
}

#[tokio::test]
async fn test_get_user_accounts() {
    let adapter = setup().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("multi-acct@example.com".to_string()),
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

    for provider in &["google", "github"] {
        adapter
            .create_account(CreateAccount {
                user_id: user_id.clone(),
                account_id: format!("{}-id", provider),
                provider_id: provider.to_string(),
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
    }

    let accounts = adapter.get_user_accounts(&user_id).await.unwrap();
    assert_eq!(accounts.len(), 2);
}

#[tokio::test]
async fn test_update_account() {
    let adapter = setup().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("upd-acct@example.com".to_string()),
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
            account_id: "google-upd".to_string(),
            provider_id: "google".to_string(),
            access_token: Some("old_token".to_string()),
            refresh_token: None,
            id_token: None,
            access_token_expires_at: None,
            refresh_token_expires_at: None,
            scope: None,
            password: None,
        })
        .await
        .unwrap();

    let updated = adapter
        .update_account(
            account.id(),
            UpdateAccount {
                access_token: Some("new_token".to_string()),
                scope: Some("openid".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert_eq!(updated.access_token().unwrap(), "new_token");
    assert_eq!(updated.scope().unwrap(), "openid");
}

#[tokio::test]
async fn test_delete_account() {
    let adapter = setup().await;
    let user_id = Uuid::new_v4().to_string();
    adapter
        .create_user(CreateUser {
            id: Some(user_id.clone()),
            email: Some("del-acct@example.com".to_string()),
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
            account_id: "to-delete".to_string(),
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
    let found = adapter.get_account("test", "to-delete").await.unwrap();
    assert!(found.is_none());
}

// ── Verification Tests ──────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_verification() {
    let adapter = setup().await;

    let create = CreateVerification {
        identifier: "alice@example.com".to_string(),
        value: "token-abc-123".to_string(),
        expires_at: Utc::now() + Duration::hours(1),
    };
    let verification = adapter.create_verification(create).await.unwrap();
    assert_eq!(verification.identifier(), "alice@example.com");
    assert_eq!(verification.value(), "token-abc-123");

    let found = adapter
        .get_verification("alice@example.com", "token-abc-123")
        .await
        .unwrap();
    assert!(found.is_some());
}

#[tokio::test]
async fn test_get_verification_by_value() {
    let adapter = setup().await;

    adapter
        .create_verification(CreateVerification {
            identifier: "user@example.com".to_string(),
            value: "unique-token-456".to_string(),
            expires_at: Utc::now() + Duration::hours(1),
        })
        .await
        .unwrap();

    let found = adapter
        .get_verification_by_value("unique-token-456")
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().identifier(), "user@example.com");

    let not_found = adapter
        .get_verification_by_value("nonexistent")
        .await
        .unwrap();
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_consume_verification() {
    let adapter = setup().await;

    adapter
        .create_verification(CreateVerification {
            identifier: "consume@example.com".to_string(),
            value: "consume-token".to_string(),
            expires_at: Utc::now() + Duration::hours(1),
        })
        .await
        .unwrap();

    let consumed = adapter
        .consume_verification("consume@example.com", "consume-token")
        .await
        .unwrap();
    assert!(consumed.is_some());

    // Should no longer exist
    let gone = adapter
        .get_verification("consume@example.com", "consume-token")
        .await
        .unwrap();
    assert!(gone.is_none());
}

#[tokio::test]
async fn test_delete_expired_verifications() {
    let adapter = setup().await;

    // Create an expired verification
    adapter
        .create_verification(CreateVerification {
            identifier: "old@example.com".to_string(),
            value: "expired-token".to_string(),
            expires_at: Utc::now() - Duration::hours(1),
        })
        .await
        .unwrap();

    // Create a valid verification
    adapter
        .create_verification(CreateVerification {
            identifier: "new@example.com".to_string(),
            value: "valid-token".to_string(),
            expires_at: Utc::now() + Duration::hours(1),
        })
        .await
        .unwrap();

    let deleted = adapter.delete_expired_verifications().await.unwrap();
    assert!(deleted >= 1);

    // Valid one should remain
    let still_there = adapter
        .get_verification_by_value("valid-token")
        .await
        .unwrap();
    assert!(still_there.is_some());
}
