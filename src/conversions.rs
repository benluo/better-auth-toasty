use better_auth_core::types::{
    Account, CreateAccount, CreateSession, CreateUser, CreateVerification, Session,
    User, Verification,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::{
    Account as AccountRow, Session as SessionRow, User as UserModel,
    Verification as VerificationRow,
};

pub(crate) fn ts(millis: i64) -> DateTime<Utc> {
    DateTime::from_timestamp_millis(millis).unwrap_or_default()
}

pub(crate) fn millis(dt: DateTime<Utc>) -> i64 {
    dt.timestamp_millis()
}

pub(crate) fn now_millis() -> i64 {
    Utc::now().timestamp_millis()
}

// -- User --

impl From<UserModel> for User {
    fn from(row: UserModel) -> Self {
        let metadata = row
            .metadata
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        User {
            id: row.id,
            name: row.name,
            email: row.email,
            email_verified: row.email_verified,
            image: row.image,
            created_at: ts(row.created_at),
            updated_at: ts(row.updated_at),
            username: row.username,
            display_username: row.display_username,
            two_factor_enabled: row.two_factor_enabled,
            role: row.role,
            banned: row.banned,
            ban_reason: row.ban_reason,
            ban_expires: row.ban_expires.map(ts),
            metadata,
        }
    }
}

pub(crate) fn create_user_to_model(data: &CreateUser) -> UserModel {
    let now = now_millis();
    let metadata_str = data
        .metadata
        .as_ref()
        .and_then(|v| serde_json::to_string(v).ok());
    UserModel {
        id: data
            .id
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string()),
        name: data.name.clone(),
        email: data.email.clone(),
        email_verified: data.email_verified.unwrap_or(false),
        image: data.image.clone(),
        username: data.username.clone(),
        display_username: data.display_username.clone(),
        role: data.role.clone(),
        banned: false,
        ban_reason: None,
        ban_expires: None,
        two_factor_enabled: false,
        metadata: metadata_str,
        created_at: now,
        updated_at: now,
    }
}

// -- Session --

impl From<SessionRow> for Session {
    fn from(row: SessionRow) -> Self {
        Session {
            id: row.id,
            expires_at: ts(row.expires_at),
            token: row.token,
            created_at: ts(row.created_at),
            updated_at: ts(row.updated_at),
            ip_address: row.ip_address,
            user_agent: row.user_agent,
            user_id: row.user_id,
            impersonated_by: row.impersonated_by,
            active_organization_id: row.active_organization_id,
            active: row.active,
        }
    }
}

pub(crate) fn create_session_to_model(data: &CreateSession) -> SessionRow {
    let now = now_millis();
    let token = format!("session_{}", Uuid::new_v4().to_string().replace('-', ""));
    SessionRow {
        id: Uuid::new_v4().to_string(),
        user_id: data.user_id.clone(),
        token,
        expires_at: millis(data.expires_at),
        ip_address: data.ip_address.clone(),
        user_agent: data.user_agent.clone(),
        impersonated_by: data.impersonated_by.clone(),
        active_organization_id: data.active_organization_id.clone(),
        active: true,
        created_at: now,
        updated_at: now,
    }
}

// -- Account --

impl From<AccountRow> for Account {
    fn from(row: AccountRow) -> Self {
        Account {
            id: row.id,
            user_id: row.user_id,
            account_id: row.account_id,
            provider_id: row.provider_id,
            access_token: row.access_token,
            refresh_token: row.refresh_token,
            id_token: row.id_token,
            access_token_expires_at: row.access_token_expires_at.map(ts),
            refresh_token_expires_at: row.refresh_token_expires_at.map(ts),
            scope: row.scope,
            password: row.password,
            created_at: ts(row.created_at),
            updated_at: ts(row.updated_at),
        }
    }
}

pub(crate) fn create_account_to_model(data: &CreateAccount) -> AccountRow {
    let now = now_millis();
    AccountRow {
        id: Uuid::new_v4().to_string(),
        user_id: data.user_id.clone(),
        account_id: data.account_id.clone(),
        provider_id: data.provider_id.clone(),
        access_token: data.access_token.clone(),
        refresh_token: data.refresh_token.clone(),
        access_token_expires_at: data.access_token_expires_at.map(millis),
        refresh_token_expires_at: data.refresh_token_expires_at.map(millis),
        scope: data.scope.clone(),
        id_token: data.id_token.clone(),
        password: data.password.clone(),
        created_at: now,
        updated_at: now,
    }
}

// -- Verification --

impl From<VerificationRow> for Verification {
    fn from(row: VerificationRow) -> Self {
        Verification {
            id: row.id,
            identifier: row.identifier,
            value: row.value,
            expires_at: ts(row.expires_at),
            created_at: ts(row.created_at),
            updated_at: ts(row.updated_at),
        }
    }
}

pub(crate) fn create_verification_to_model(data: &CreateVerification) -> VerificationRow {
    let now = now_millis();
    VerificationRow {
        id: Uuid::new_v4().to_string(),
        identifier: data.identifier.clone(),
        value: data.value.clone(),
        expires_at: millis(data.expires_at),
        created_at: now,
        updated_at: now,
    }
}
