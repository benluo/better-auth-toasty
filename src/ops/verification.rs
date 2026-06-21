use async_trait::async_trait;
use better_auth_core::adapters::VerificationOps;
use better_auth_core::error::{AuthError, AuthResult, DatabaseError};
use better_auth_core::types::{CreateVerification, Verification};

use crate::adapter::ToastyAdapter;
use crate::conversions::{create_verification_to_model, now_millis};
use crate::models::Verification as VerificationModel;

fn db_err(e: impl ToString) -> AuthError {
    AuthError::Database(DatabaseError::Query(e.to_string()))
}

#[async_trait]
impl VerificationOps for ToastyAdapter {
    type Verification = Verification;

    async fn create_verification(
        &self,
        data: CreateVerification,
    ) -> AuthResult<Self::Verification> {
        let mut db = self.db.clone();
        let model = create_verification_to_model(&data);
        let id = model.id.clone();

        toasty::create!(VerificationModel {
            id: model.id,
            identifier: model.identifier,
            value: model.value,
            expires_at: model.expires_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
        .exec(&mut db)
        .await
        .map_err(db_err)?;

        VerificationModel::get_by_id(&mut db, &id)
            .await
            .map(Verification::from)
            .map_err(db_err)
    }

    async fn get_verification(
        &self,
        identifier: &str,
        value: &str,
    ) -> AuthResult<Option<Self::Verification>> {
        let mut db = self.db.clone();
        let result =
            toasty::query!(VerificationModel filter .identifier == #identifier and .value == #value)
                .first()
                .exec(&mut db)
                .await
                .map_err(db_err)?;
        Ok(result.map(Verification::from))
    }

    async fn get_verification_by_value(
        &self,
        value: &str,
    ) -> AuthResult<Option<Self::Verification>> {
        let mut db = self.db.clone();
        let result = toasty::query!(VerificationModel filter .value == #value)
            .first()
            .exec(&mut db)
            .await
            .map_err(db_err)?;
        Ok(result.map(Verification::from))
    }

    async fn consume_verification(
        &self,
        identifier: &str,
        value: &str,
    ) -> AuthResult<Option<Self::Verification>> {
        let found = Self::get_verification(self, identifier, value).await?;
        if found.is_some() {
            let mut db = self.db.clone();
            let results = toasty::query!(VerificationModel filter .identifier == #identifier and .value == #value)
                .exec(&mut db)
                .await
                .map_err(db_err)?;
            for v in results {
                v.delete().exec(&mut db).await.map_err(db_err)?;
            }
        }
        Ok(found)
    }

    async fn delete_verification(&self, id: &str) -> AuthResult<()> {
        let mut db = self.db.clone();
        if let Ok(v) = VerificationModel::get_by_id(&mut db, id).await {
            v.delete().exec(&mut db).await.map_err(db_err)?;
        }
        Ok(())
    }

    async fn delete_expired_verifications(&self) -> AuthResult<usize> {
        let mut db = self.db.clone();
        let now = now_millis();
        let results: Vec<VerificationModel> =
            toasty::query!(VerificationModel filter .expires_at < #now)
                .exec(&mut db)
                .await
                .map_err(db_err)?;
        let count = results.len();
        for v in results {
            v.delete().exec(&mut db).await.map_err(db_err)?;
        }
        Ok(count)
    }
}
