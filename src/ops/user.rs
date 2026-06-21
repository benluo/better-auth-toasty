use async_trait::async_trait;
use better_auth_core::adapters::UserOps;
use better_auth_core::error::{AuthError, AuthResult, DatabaseError};
use better_auth_core::types::{CreateUser, ListUsersParams, UpdateUser, User};

use crate::adapter::ToastyAdapter;
use crate::conversions::create_user_to_model;
use crate::models::User as UserModel;

fn db_err(e: impl ToString) -> AuthError {
    AuthError::Database(DatabaseError::Query(e.to_string()))
}

#[async_trait]
impl UserOps for ToastyAdapter {
    type User = User;

    async fn create_user(&self, data: CreateUser) -> AuthResult<Self::User> {
        let mut db = self.db.clone();
        let model = create_user_to_model(&data);
        let id = model.id.clone();

        toasty::create!(UserModel {
            id: model.id,
            name: model.name,
            email: model.email,
            email_verified: model.email_verified,
            image: model.image,
            username: model.username,
            display_username: model.display_username,
            role: model.role,
            banned: model.banned,
            ban_reason: model.ban_reason,
            ban_expires: model.ban_expires,
            two_factor_enabled: model.two_factor_enabled,
            metadata: model.metadata,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
        .exec(&mut db)
        .await
        .map_err(db_err)?;

        UserModel::get_by_id(&mut db, &id)
            .await
            .map(User::from)
            .map_err(db_err)
    }

    async fn get_user_by_id(&self, id: &str) -> AuthResult<Option<Self::User>> {
        let mut db = self.db.clone();
        UserModel::get_by_id(&mut db, id)
            .await
            .map(|u| Some(User::from(u)))
            .or(Ok(None))
    }

    async fn get_user_by_email(&self, email: &str) -> AuthResult<Option<Self::User>> {
        let mut db = self.db.clone();
        UserModel::get_by_email(&mut db, email)
            .await
            .map(|u| Some(User::from(u)))
            .or(Ok(None))
    }

    async fn get_user_by_username(&self, _username: &str) -> AuthResult<Option<Self::User>> {
        todo!("add #[unique] or #[index] on username and use UserModel::get_by_username")
    }

    async fn update_user(&self, id: &str, update: UpdateUser) -> AuthResult<Self::User> {
        let mut db = self.db.clone();
        let mut user = UserModel::get_by_id(&mut db, id)
            .await
            .map_err(|_| AuthError::UserNotFound)?;

        let mut builder = user.update();
        if let Some(email) = update.email {
            builder = builder.email(email);
        }
        if let Some(name) = update.name {
            builder = builder.name(name);
        }
        if let Some(image) = update.image {
            builder = builder.image(image);
        }
        if let Some(email_verified) = update.email_verified {
            builder = builder.email_verified(email_verified);
        }
        if let Some(username) = update.username {
            builder = builder.username(username);
        }
        if let Some(display_username) = update.display_username {
            builder = builder.display_username(display_username);
        }
        if let Some(role) = update.role {
            builder = builder.role(role);
        }
        if let Some(banned) = update.banned {
            builder = builder.banned(banned);
            if !banned {
                builder = builder.ban_reason(None::<String>);
                builder = builder.ban_expires(None::<i64>);
            }
        }
        if let Some(ban_reason) = update.ban_reason {
            builder = builder.ban_reason(ban_reason);
        }
        if let Some(ban_expires) = update.ban_expires {
            builder = builder.ban_expires(ban_expires.timestamp_millis());
        }
        if let Some(two_factor_enabled) = update.two_factor_enabled {
            builder = builder.two_factor_enabled(two_factor_enabled);
        }
        if let Some(metadata) = update.metadata {
            builder = builder.metadata(serde_json::to_string(&metadata).ok());
        }

        builder
            .updated_at(crate::conversions::now_millis())
            .exec(&mut db)
            .await
            .map_err(db_err)?;

        UserModel::get_by_id(&mut db, id)
            .await
            .map(User::from)
            .map_err(db_err)
    }

    async fn delete_user(&self, id: &str) -> AuthResult<()> {
        let mut db = self.db.clone();
        let user = UserModel::get_by_id(&mut db, id)
            .await
            .map_err(|_| AuthError::UserNotFound)?;
        user.delete().exec(&mut db).await.map_err(db_err)?;
        Ok(())
    }

    async fn list_users(&self, _params: ListUsersParams) -> AuthResult<(Vec<Self::User>, usize)> {
        todo!("use UserModel::all() with pagination/filtering")
    }
}
