use std::sync::Arc;

use crate::{
    config::token::TokenConfig,
    database::token::TokenRepository,
    errors::core::Result,
    models::{
        role::Role, token_claims::TokenClaims, token_scope::TokenScope, token_session::TokenSession,
    },
    state::AppState,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

#[derive(Debug, Clone)]
pub struct TokenService {
    pub config: TokenConfig,
    pub token_repo: TokenRepository,
}

impl TokenService {
    pub fn new(config: TokenConfig) -> Self {
        Self {
            config,
            token_repo: TokenRepository::new(),
        }
    }

    pub fn generate_access_token(
        &self,
        user_id: &str,
        email: &str,
        role: &Role,
        custom_scopes: Option<Vec<TokenScope>>,
    ) -> Result<String> {
        let now = Utc::now();
        let expires_at = now + Duration::seconds(self.config.access_token_expires_in);
        let scopes = custom_scopes.unwrap_or_else(|| TokenClaims::default_scopes_for_role(role));
        let claims = TokenClaims::new_access_token(
            user_id.to_string(),
            email.to_string(),
            role.clone(),
            now.timestamp(),
            expires_at.timestamp(),
            scopes,
        );
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.config.jwt_access_secret.as_bytes());
        encode(&header, &claims, &encoding_key).map_err(Into::into)
    }

    pub fn generate_refresh_token(&self, user_id: &str) -> Result<String> {
        let now = Utc::now();
        let expires_at = now + Duration::seconds(self.config.refresh_token_expires_in);
        let claims = TokenClaims::new_refresh_token(
            user_id.to_string(),
            now.timestamp(),
            expires_at.timestamp(),
        );
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.config.jwt_refresh_secret.as_bytes());
        encode(&header, &claims, &encoding_key).map_err(Into::into)
    }

    pub fn generate_email_verification_token(&self, user_id: &str, email: &str) -> Result<String> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24);
        let claims = TokenClaims::new_email_verification_token(
            user_id.to_string(),
            email.to_string(),
            now.timestamp(),
            expires_at.timestamp(),
        );
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let encoding_key =
            EncodingKey::from_secret(self.config.email_verification_secret.as_bytes());
        encode(&header, &claims, &encoding_key).map_err(Into::into)
    }

    pub fn generate_password_reset_token(&self, user_id: &str, email: &str) -> Result<String> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(1);
        let claims = TokenClaims::new_password_reset_token(
            user_id.to_string(),
            email.to_string(),
            now.timestamp(),
            expires_at.timestamp(),
        );
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.config.password_reset_secret.as_bytes());
        encode(&header, &claims, &encoding_key).map_err(Into::into)
    }

    pub async fn create_session(
        &self,
        app_state: Arc<AppState>,
        user_id: &str,
        email: &str,
        role: &Role,
        device_info: Option<String>,
        custom_scopes: Option<Vec<TokenScope>>,
    ) -> Result<(String, String, TokenSession)> {
        let access_token = self.generate_access_token(user_id, email, role, custom_scopes)?;
        let refresh_token = self.generate_refresh_token(user_id)?;

        let access_claims = self.verify_access_token(&access_token)?;
        let refresh_claims = self.verify_refresh_token(&refresh_token)?;

        let access_jti = access_claims.jti.unwrap_or_default();
        let refresh_jti = refresh_claims.jti.unwrap_or_default();

        let mut session = TokenSession::new(user_id.to_string(), access_jti, refresh_jti);
        session.device_info = device_info;

        let created_session = self.token_repo.create_session(app_state, session).await?;

        Ok((access_token, refresh_token, created_session))
    }

    pub async fn refresh_session(
        &self,
        app_state: Arc<AppState>,
        refresh_token: &str,
    ) -> Result<(String, String)> {
        let refresh_claims = self.verify_refresh_token(refresh_token)?;
        let refresh_jti = refresh_claims.jti.as_ref().unwrap();

        let session = self
            .token_repo
            .find_by_refresh_token_jti(app_state.clone(), refresh_jti.clone())
            .await?
            .ok_or_else(|| crate::errors::auth::AuthError::InvalidToken)?;

        if !session.is_active {
            return Err(crate::errors::auth::AuthError::InvalidToken.into());
        }

        let new_access_token = self.generate_access_token(
            &session.user_id,
            "",
            &crate::models::role::Role::User,
            None,
        )?;
        let new_refresh_token = self.generate_refresh_token(&session.user_id)?;

        self.token_repo
            .update_last_active(app_state, session.id)
            .await?;

        Ok((new_access_token, new_refresh_token))
    }

    pub async fn verify_access_token_with_session(
        &self,
        app_state: Arc<AppState>,
        token: &str,
    ) -> Result<TokenClaims> {
        let claims = self.verify_access_token(token)?;

        if let Some(jti) = &claims.jti {
            if let Some(session) = self
                .token_repo
                .find_by_access_token_jti(app_state.clone(), jti.clone())
                .await?
            {
                if !session.is_active {
                    return Err(crate::errors::auth::AuthError::InvalidToken.into());
                }

                self.token_repo
                    .update_last_active(app_state, session.id)
                    .await?;
            } else {
                return Err(crate::errors::auth::AuthError::InvalidToken.into());
            }
        }

        Ok(claims)
    }

    pub async fn revoke_session(&self, app_state: Arc<AppState>, session_id: String) -> Result<()> {
        self.token_repo.revoke_session(app_state, session_id).await
    }

    pub async fn revoke_all_user_sessions(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
    ) -> Result<()> {
        self.token_repo
            .revoke_all_user_sessions(app_state, user_id)
            .await
    }

    pub async fn get_user_active_sessions(
        &self,
        app_state: Arc<AppState>,
        user_id: String,
    ) -> Result<Vec<TokenSession>> {
        self.token_repo
            .get_active_sessions_by_user(app_state, user_id)
            .await
    }

    pub async fn cleanup_expired_sessions(&self, app_state: Arc<AppState>) -> Result<usize> {
        self.token_repo.cleanup_expired_sessions(app_state).await
    }

    pub fn verify_access_token(&self, token: &str) -> Result<TokenClaims> {
        self.verify_token(token, &self.config.jwt_access_secret)
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<TokenClaims> {
        self.verify_token(token, &self.config.jwt_refresh_secret)
    }

    pub fn verify_email_verification_token(&self, token: &str) -> Result<TokenClaims> {
        self.verify_token(token, &self.config.email_verification_secret)
    }

    pub fn verify_password_reset_token(&self, token: &str) -> Result<TokenClaims> {
        self.verify_token(token, &self.config.password_reset_secret)
    }

    pub fn extract_token_from_header(auth_header: &str) -> Option<&str> {
        if auth_header.starts_with("Bearer ") {
            Some(&auth_header[7..])
        } else {
            None
        }
    }

    pub fn generate_token_pair(
        &self,
        user_id: &str,
        email: &str,
        role: &Role,
        custom_scopes: Option<Vec<TokenScope>>,
    ) -> Result<(String, String)> {
        let access_token = self.generate_access_token(user_id, email, role, custom_scopes)?;
        let refresh_token = self.generate_refresh_token(user_id)?;
        Ok((access_token, refresh_token))
    }

    fn verify_token(&self, token: &str, secret: &str) -> Result<TokenClaims> {
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_data = decode::<TokenClaims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}
