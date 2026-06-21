use async_trait::async_trait;
use better_auth_core::adapters::SessionOps;
use better_auth_core::error::{AuthError, AuthResult, DatabaseError};
use better_auth_core::types::{CreateSession, Session};
use chrono::{DateTime, Utc};

use crate::adapter::ToastyAdapter;
use crate::conversions::{create_session_to_model, now_millis};
use crate::models::Session as SessionModel;

fn db_err(e: impl ToString) -> AuthError {
    AuthError::Database(DatabaseError::Query(e.to_string()))
}

#[async_trait]
impl SessionOps for ToastyAdapter {
    type Session = Session;

    async fn create_session(&self, data: CreateSession) -> AuthResult<Self::Session> {
        let mut db = self.db.clone();
        let model = create_session_to_model(&data);
        let token = model.token.clone();

        toasty::create!(SessionModel {
            id: model.id,
            user_id: model.user_id,
            token: model.token,
            expires_at: model.expires_at,
            ip_address: model.ip_address,
            user_agent: model.user_agent,
            impersonated_by: model.impersonated_by,
            active_organization_id: model.active_organization_id,
            active: model.active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
        .exec(&mut db)
        .await
        .map_err(db_err)?;

        SessionModel::get_by_token(&mut db, &token)
            .await
            .map(Session::from)
            .map_err(db_err)
    }

    async fn get_session(&self, token: &str) -> AuthResult<Option<Self::Session>> {
        let mut db = self.db.clone();
        let result = toasty::query!(SessionModel filter .token == #token and .active == true)
            .first()
            .exec(&mut db)
            .await
            .map_err(db_err)?;
        Ok(result.map(Session::from))
    }

    async fn get_user_sessions(&self, user_id: &str) -> AuthResult<Vec<Self::Session>> {
        let mut db = self.db.clone();
        let results = toasty::query!(SessionModel filter .user_id == #user_id and .active == true)
            .exec(&mut db)
            .await
            .map_err(db_err)?;
        Ok(results.into_iter().map(Session::from).collect())
    }

    async fn update_session_expiry(
        &self,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> AuthResult<()> {
        let mut db = self.db.clone();
        let mut session = SessionModel::get_by_token(&mut db, token)
            .await
            .map_err(|_| AuthError::SessionNotFound)?;
        session.expires_at = expires_at.timestamp_millis();
        session.updated_at = now_millis();
        session.update().exec(&mut db).await.map_err(db_err)?;
        Ok(())
    }

    async fn delete_session(&self, token: &str) -> AuthResult<()> {
        let mut db = self.db.clone();
        if let Ok(session) = SessionModel::get_by_token(&mut db, token).await {
            session.delete().exec(&mut db).await.map_err(db_err)?;
        }
        Ok(())
    }

    async fn delete_user_sessions(&self, user_id: &str) -> AuthResult<()> {
        let mut db = self.db.clone();
        let sessions: Vec<SessionModel> =
            toasty::query!(SessionModel filter .user_id == #user_id)
                .exec(&mut db)
                .await
                .map_err(db_err)?;
        for session in sessions {
            session.delete().exec(&mut db).await.map_err(db_err)?;
        }
        Ok(())
    }

    async fn delete_expired_sessions(&self) -> AuthResult<usize> {
        let mut db = self.db.clone();
        let now = now_millis();
        let sessions: Vec<SessionModel> =
            toasty::query!(SessionModel filter .expires_at < #now or .active == false)
                .exec(&mut db)
                .await
                .map_err(db_err)?;
        let count = sessions.len();
        for session in sessions {
            session.delete().exec(&mut db).await.map_err(db_err)?;
        }
        Ok(count)
    }

    async fn update_session_active_organization(
        &self,
        token: &str,
        organization_id: Option<&str>,
    ) -> AuthResult<Self::Session> {
        let mut db = self.db.clone();
        let mut session = SessionModel::get_by_token(&mut db, token)
            .await
            .map_err(|_| AuthError::SessionNotFound)?;
        session.active_organization_id = organization_id.map(String::from);
        session.updated_at = now_millis();
        session.update().exec(&mut db).await.map_err(db_err)?;

        SessionModel::get_by_token(&mut db, token)
            .await
            .map(Session::from)
            .map_err(db_err)
    }
}
