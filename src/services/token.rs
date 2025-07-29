use crate::errors::core::Result;
use crate::models::role::Role;
use crate::{
    config::token::TokenConfig,
    models::{token_claims::TokenClaims, token_scope::TokenScope},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

#[derive(Debug, Clone)]
pub struct TokenService {
    pub config: TokenConfig,
}

impl TokenService {
    pub fn new(config: TokenConfig) -> Self {
        Self { config }
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

    pub fn verify_token(&self, token: &str, secret: &str) -> Result<TokenClaims> {
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_data = decode::<TokenClaims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}
