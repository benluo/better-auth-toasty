use toasty::Model;

#[derive(Clone, Debug, Model)]
pub struct User {
    #[key]
    pub id: String,
    pub name: Option<String>,
    #[unique]
    pub email: Option<String>,
    pub email_verified: bool,
    pub image: Option<String>,
    pub username: Option<String>,
    pub display_username: Option<String>,
    pub role: Option<String>,
    pub banned: bool,
    pub ban_reason: Option<String>,
    pub ban_expires: Option<i64>,
    pub two_factor_enabled: bool,
    pub metadata: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Model)]
pub struct Session {
    #[key]
    pub id: String,
    #[index]
    pub user_id: String,
    #[unique]
    pub token: String,
    pub expires_at: i64,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub impersonated_by: Option<String>,
    pub active_organization_id: Option<String>,
    pub active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Model)]
pub struct Account {
    #[key]
    pub id: String,
    #[index]
    pub user_id: String,
    pub account_id: String,
    pub provider_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub access_token_expires_at: Option<i64>,
    pub refresh_token_expires_at: Option<i64>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub password: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Model)]
pub struct Verification {
    #[key]
    pub id: String,
    pub identifier: String,
    pub value: String,
    pub expires_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
