use std::sync::Arc;

use crate::{
    errors::{core::Result, db::DatabaseError},
    models::user::User,
    state::AppState,
};

#[derive(Debug, Clone)]
pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        Self
    }
    pub async fn create(&self, app_state: Arc<AppState>, user: User) -> Result<User> {
        let created: Option<User> = app_state
            .db
            .create(("users", &user.id))
            .content(user)
            .await
            .map_err(|e| DatabaseError::query_failed(e, Some("CREATE user".to_string())))?;
        created.ok_or(DatabaseError::NotFound("Failed to create user".to_string()).into())
    }
    pub async fn find_by_email(
        &self,
        app_state: Arc<AppState>,
        email: String,
    ) -> Result<Option<User>> {
        let users: Vec<User> = app_state
            .db
            .query("SELECT * FROM users WHERE email = $email LIMIT 1")
            .bind(("email", email))
            .await
            .map_err(|e| DatabaseError::query_failed(e, Some("SELECT user by email".to_string())))?
            .take(0)
            .map_err(|e| DatabaseError::query_failed(e, Some("Take query result".to_string())))?;
        Ok(users.into_iter().next())
    }
    pub async fn find_by_id(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
    ) -> Result<Option<User>> {
        let user: Option<User> = app_state
            .db
            .select(("users", user_id.as_str()))
            .await
            .map_err(|e| DatabaseError::query_failed(e, Some("SELECT user by id".to_string())))?;
        Ok(user)
    }
    pub async fn update_verification_status(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
        verified: bool,
    ) -> Result<User> {
        let updated: Option<User> = app_state
            .db
            .update(("users", user_id.as_str()))
            .merge(serde_json::json!({
                "verified": verified,
                "updated_at": chrono::Utc::now()
            }))
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("UPDATE user verification".to_string()))
            })?;
        updated.ok_or(
            DatabaseError::NotFound("User not found for verification update".to_string()).into(),
        )
    }
    pub async fn update_password(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
        new_password_hash: String,
    ) -> Result<User> {
        let updated: Option<User> = app_state
            .db
            .update(("users", user_id.as_str()))
            .merge(serde_json::json!({
                "password": new_password_hash,
                "updated_at": chrono::Utc::now(),
            }))
            .await
            .map_err(|e| {
                DatabaseError::query_failed(e, Some("UPDATE user password".to_string()))
            })?;
        updated
            .ok_or(DatabaseError::NotFound("User not found for password update".to_string()).into())
    }
    pub async fn update_profile(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
        name: Option<String>,
        email: Option<String>,
    ) -> Result<User> {
        let mut update_data = serde_json::json!({
            "updated_at": chrono::Utc::now()
        });
        if let Some(name) = name {
            update_data["name"] = serde_json::Value::String(name);
        }
        if let Some(email) = email {
            update_data["email"] = serde_json::Value::String(email);
            update_data["verified"] = serde_json::Value::Bool(false);
        }
        let updated: Option<User> = app_state
            .db
            .update(("users", user_id.as_str()))
            .merge(update_data)
            .await
            .map_err(|e| DatabaseError::query_failed(e, Some("UPDATE user profile".to_string())))?;
        updated
            .ok_or(DatabaseError::NotFound("User not found for profile update".to_string()).into())
    }
    pub async fn delete(&self, app_state: Arc<AppState>, user_id: String) -> Result<()> {
        let _: Option<User> = app_state
            .db
            .delete(("users", user_id.as_str()))
            .await
            .map_err(|e| DatabaseError::query_failed(e, Some("DELETE user".to_string())))?;
        Ok(())
    }
    pub async fn email_exists(&self, app_state: Arc<AppState>, email: String) -> Result<bool> {
        let count: Vec<serde_json::Value> = app_state
            .db
            .query("SELECT count() FROM users WHERE email = $email GROUP ALL")
            .bind(("email", email))
            .await
            .map_err(|e| DatabaseError::query_failed(e, Some("COUNT users by email".to_string())))?
            .take(0)
            .map_err(|e| DatabaseError::query_failed(e, Some("Take query result".to_string())))?;
        if let Some(result) = count.first() {
            if let Some(count_val) = result.get("count") {
                if let Some(count_num) = count_val.as_u64() {
                    return Ok(count_num > 0);
                }
            }
        }
        Ok(false)
    }
}
