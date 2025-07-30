use chrono::Utc;
use regex::Regex;
use uuid::Uuid;

use crate::{
    database::user::UserRepository,
    errors::{auth::AuthError, core::Result},
    models::{role::Role, user::User},
    state::AppState,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            user_repo: UserRepository::new(),
        }
    }
    fn hash_password(&self, password: &str) -> Result<String> {
        use bcrypt::{DEFAULT_COST, hash};
        hash(password, DEFAULT_COST).map_err(|_| AuthError::HashingError.into())
    }
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        use bcrypt::verify;
        verify(password, hash).map_err(|_| AuthError::InvalidHashFormat.into())
    }
    fn validate_user_input(&self, name: &str, email: &str, password: &str) -> Result<()> {
        self.validate_name(name)?;
        self.validate_email(email)?;
        self.validate_password(password)?;
        Ok(())
    }
    fn validate_name(&self, name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(AuthError::InvalidCredentials.into());
        }
        if name.len() > 100 {
            return Err(AuthError::InvalidCredentials.into());
        }
        Ok(())
    }
    fn validate_email(&self, email: &str) -> Result<()> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| AuthError::InvalidCredentials)?;
        if !email_regex.is_match(email) {
            return Err(AuthError::InvalidCredentials.into());
        }
        Ok(())
    }
    fn validate_password(&self, password: &str) -> Result<()> {
        if password.is_empty() {
            return Err(AuthError::EmptyPassword.into());
        }
        if password.len() < 8 {
            return Err(AuthError::InvalidCredentials.into());
        }
        if password.len() > 128 {
            return Err(AuthError::password_too_long(128).into());
        }
        Ok(())
    }
    pub async fn create_user(
        &self,
        app_state: Arc<AppState>,
        name: String,
        email: String,
        password: String,
    ) -> Result<User> {
        self.validate_user_input(&name, &email, &password)?;
        if self
            .user_repo
            .email_exists(app_state.clone(), email.clone())
            .await?
        {
            return Err(AuthError::EmailAlreadyExists.into());
        }
        let password_hash = self.hash_password(&password)?;
        let user = User {
            id: Uuid::new_v4().to_string(),
            name,
            email,
            password: password_hash,
            role: Role::User,
            verified: false,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        self.user_repo.create(app_state, user).await
    }
    pub async fn authenticate_user(
        &self,
        app_state: Arc<AppState>,
        email: String,
        password: String,
    ) -> Result<User> {
        let user = self
            .user_repo
            .find_by_email(app_state, email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;
        if !self.verify_password(&password, &user.password)? {
            return Err(AuthError::InvalidCredentials.into());
        }
        Ok(user)
    }
    pub async fn find_by_email(
        &self,
        app_state: Arc<AppState>,
        email: String,
    ) -> Result<Option<User>> {
        self.user_repo.find_by_email(app_state, email).await
    }
    pub async fn find_by_id(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
    ) -> Result<Option<User>> {
        self.user_repo.find_by_id(app_state, user_id).await
    }
    pub async fn verify_email(&self, app_state: Arc<AppState>, user_id: String) -> Result<User> {
        self.user_repo
            .update_verification_status(app_state, user_id, true)
            .await
    }
    pub async fn change_password(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
        current_password: String,
        new_password: String,
    ) -> Result<User> {
        let user = self
            .user_repo
            .find_by_id(app_state.clone(), user_id.clone())
            .await?
            .ok_or(AuthError::UserNoLongerExists)?;
        if !self.verify_password(&current_password, &user.password)? {
            return Err(AuthError::InvalidCredentials.into());
        }
        self.validate_password(&new_password)?;
        let new_password_hash = self.hash_password(&new_password)?;
        self.user_repo
            .update_password(app_state, user_id, new_password_hash)
            .await
    }
    pub async fn reset_password(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
        new_password: String,
    ) -> Result<User> {
        self.validate_password(&new_password)?;
        let new_password_hash = self.hash_password(&new_password)?;
        self.user_repo
            .update_password(app_state, user_id, new_password_hash)
            .await
    }
    pub async fn update_profile(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
        name: Option<String>,
        email: Option<String>,
    ) -> Result<User> {
        if let Some(ref new_email) = email {
            let current_user = self
                .user_repo
                .find_by_id(app_state.clone(), user_id.clone())
                .await?
                .ok_or(AuthError::UserNoLongerExists)?;
            if new_email != &current_user.email {
                if self
                    .user_repo
                    .email_exists(app_state.clone(), new_email.clone())
                    .await?
                {
                    return Err(AuthError::EmailAlreadyExists.into());
                }
                self.validate_email(new_email)?;
            }
        }
        if let Some(ref new_name) = name {
            self.validate_name(new_name)?;
        }
        self.user_repo
            .update_profile(app_state, user_id, name, email)
            .await
    }
    pub async fn delete_user(&self, app_state: Arc<AppState>, user_id: String) -> Result<()> {
        self.user_repo.delete(app_state, user_id).await
    }
}
