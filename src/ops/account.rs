use async_trait::async_trait;
use better_auth_core::adapters::AccountOps;
use better_auth_core::error::{AuthError, AuthResult, DatabaseError};
use better_auth_core::types::{Account, CreateAccount, UpdateAccount};

use crate::adapter::ToastyAdapter;
use crate::conversions::create_account_to_model;
use crate::models::Account as AccountModel;

fn db_err(e: impl ToString) -> AuthError {
    AuthError::Database(DatabaseError::Query(e.to_string()))
}

#[async_trait]
impl AccountOps for ToastyAdapter {
    type Account = Account;

    async fn create_account(&self, data: CreateAccount) -> AuthResult<Self::Account> {
        let mut db = self.db.clone();
        let model = create_account_to_model(&data);
        let id = model.id.clone();

        toasty::create!(AccountModel {
            id: model.id,
            user_id: model.user_id,
            account_id: model.account_id,
            provider_id: model.provider_id,
            access_token: model.access_token,
            refresh_token: model.refresh_token,
            access_token_expires_at: model.access_token_expires_at,
            refresh_token_expires_at: model.refresh_token_expires_at,
            scope: model.scope,
            id_token: model.id_token,
            password: model.password,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
        .exec(&mut db)
        .await
        .map_err(db_err)?;

        AccountModel::get_by_id(&mut db, &id)
            .await
            .map(Account::from)
            .map_err(db_err)
    }

    async fn get_account(
        &self,
        provider: &str,
        provider_account_id: &str,
    ) -> AuthResult<Option<Self::Account>> {
        let mut db = self.db.clone();
        let result = toasty::query!(AccountModel filter .provider_id == #provider and .account_id == #provider_account_id)
            .first()
            .exec(&mut db)
            .await
            .map_err(db_err)?;
        Ok(result.map(Account::from))
    }

    async fn get_user_accounts(&self, user_id: &str) -> AuthResult<Vec<Self::Account>> {
        let mut db = self.db.clone();
        let results = toasty::query!(AccountModel filter .user_id == #user_id)
            .exec(&mut db)
            .await
            .map_err(db_err)?;
        Ok(results.into_iter().map(Account::from).collect())
    }

    async fn update_account(&self, id: &str, update: UpdateAccount) -> AuthResult<Self::Account> {
        let mut db = self.db.clone();
        let mut account = AccountModel::get_by_id(&mut db, id)
            .await
            .map_err(|_| AuthError::NotFound(format!("account {id} not found")))?;

        if let Some(access_token) = update.access_token {
            account.access_token = Some(access_token);
        }
        if let Some(refresh_token) = update.refresh_token {
            account.refresh_token = Some(refresh_token);
        }
        if let Some(id_token) = update.id_token {
            account.id_token = Some(id_token);
        }
        if let Some(dt) = update.access_token_expires_at {
            account.access_token_expires_at = Some(dt.timestamp_millis());
        }
        if let Some(dt) = update.refresh_token_expires_at {
            account.refresh_token_expires_at = Some(dt.timestamp_millis());
        }
        if let Some(scope) = update.scope {
            account.scope = Some(scope);
        }
        if let Some(password) = update.password {
            account.password = Some(password);
        }

        account.updated_at = crate::conversions::now_millis();

        account.update().exec(&mut db).await.map_err(db_err)?;

        AccountModel::get_by_id(&mut db, id)
            .await
            .map(Account::from)
            .map_err(db_err)
    }

    async fn delete_account(&self, id: &str) -> AuthResult<()> {
        let mut db = self.db.clone();
        let account = AccountModel::get_by_id(&mut db, id)
            .await
            .map_err(|_| AuthError::NotFound(format!("account {id} not found")))?;
        account.delete().exec(&mut db).await.map_err(db_err)?;
        Ok(())
    }
}
