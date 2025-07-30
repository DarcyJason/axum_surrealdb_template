use axum::Extension;

use crate::{
    errors::{core::Result, db::DatabaseError},
    models::token_session::TokenSession,
    state::AppState,
};
use std::sync::Arc;

pub struct TokenRepository;

impl TokenRepository {
    pub fn new() -> Self {
        Self
    }
    pub async fn create_session(
        &self,
        app_state: Arc<AppState>,
        session: TokenSession,
    ) -> Result<TokenSession> {
        let created: Option<TokenSession> = app_state
            .db
            .create(("token_sessions", &session.id))
            .content(session)
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("CREATE token_sessions".to_string()))
            })?;
        created.ok_or(DatabaseError::NotFound("Failed to create token session".to_string()).into())
    }
    pub async fn find_by_access_token_jti(
        &self,
        app_state: Arc<AppState>,
        jti: String,
    ) -> Result<Option<TokenSession>> {
        let sessions: Vec<TokenSession> = app_state
            .db
            .query("SELECT * FROM token_sessions WHERE access_token_jti = $jti")
            .bind(("jti", jti))
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("SELECT by access_token_jti".to_string()))
            })?
            .take(0)
            .map_err(|e| DatabaseError::query_failed(e, Some("Take query result".to_string())))?;
        Ok(sessions.into_iter().next())
    }
    pub async fn find_by_refresh_token_jti(
        &self,
        app_state: Arc<AppState>,
        jti: String,
    ) -> Result<Option<TokenSession>> {
        let sessions: Vec<TokenSession> = app_state
            .db
            .query("SELECT * FROM token_sessions WHERE refresh_token_jti = $jti")
            .bind(("jti", jti))
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("SELECT by refresh_token_jti".to_string()))
            })?
            .take(0)
            .map_err(|e| DatabaseError::query_failed(e, Some("Take query result".to_string())))?;
        Ok(sessions.into_iter().next())
    }
    pub async fn revoke_session(&self, app_state: Arc<AppState>, session_id: String) -> Result<()> {
        let _: Option<TokenSession> = app_state
            .db
            .update(("token_sessions", session_id.as_str()))
            .merge(serde_json::json!({
                "is_active": false
            }))
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("UPDATE session to revoke".to_string()))
            })?;
        Ok(())
    }
    pub async fn revoke_all_user_sessions(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
    ) -> Result<()> {
        let _: Vec<TokenSession> = app_state
            .db
            .query("UPDATE token_sessions SET is_active = false WHERE user_id = $user_id")
            .bind(("user_id", user_id))
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("UPDATE all user sessions".to_string()))
            })?
            .take(0)
            .map_err(|e| DatabaseError::query_failed(e, Some("Take query result".to_string())))?;
        Ok(())
    }
    pub async fn update_last_active(
        &self,
        app_state: Arc<AppState>,
        session_id: String,
    ) -> Result<()> {
        let _: Option<TokenSession> = app_state
            .db
            .update(("token_sessions", session_id.as_str()))
            .merge(serde_json::json!({
                "last_active_at": chrono::Utc::now()
            }))
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("UPDATE last_active_at".to_string()))
            })?;
        Ok(())
    }
    pub async fn find_by_id(
        &self,
        app_state: Arc<AppState>,
        session_id: String,
    ) -> Result<Option<TokenSession>> {
        let session: Option<TokenSession> = app_state
            .db
            .select(("token_sessions", session_id.as_str()))
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("SELECT session by id".to_string()))
            })?;
        Ok(session)
    }
    pub async fn get_active_sessions_by_user(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
    ) -> Result<Vec<TokenSession>> {
        let sessions: Vec<TokenSession> = app_state
            .db
            .query("SELECT * FROM token_sessions WHERE user_id = $user_id AND is_active = true")
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("SELCT active sessions by user".to_string()))
            })?
            .take(0)
            .map_err(|e| DatabaseError::query_failed(e, Some("Take query result".to_string())))?;
        Ok(sessions)
    }
    pub async fn cleanup_expired_sessions(&self, app_state: Arc<AppState>) -> Result<usize> {
        let now = chrono::Utc::now();
        let cutoff_time = now - chrono::Duration::days(30);
        let deleted: Vec<TokenSession> = app_state
            .db
            .query("DELETE token_sessions WHERE created_at < $cutoff_time RETURN BEFORE")
            .bind(("cutoff_time", cutoff_time))
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("DELETE expired sessions".to_string()))
            })?
            .take(0)
            .map_err(|e| DatabaseError::query_failed(e, Some("Take query result".to_string())))?;
        Ok(deleted.len())
    }
    pub async fn is_session_active(
        &self,
        app_state: Arc<AppState>,
        session_id: String,
    ) -> Result<bool> {
        if let Some(session) = self.find_by_id(app_state, session_id).await? {
            Ok(session.is_active)
        } else {
            Ok(false)
        }
    }
}
