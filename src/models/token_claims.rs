use std::collections::HashMap;

use crate::models::{role::Role, token_scope::TokenScope, token_type::TokenType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub token_type: TokenType,
    pub iat: i64,
    pub exp: i64,
    pub iss: Option<String>,
    pub aud: Option<String>,
    pub jti: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
    pub scopes: Vec<TokenScope>,
    pub extra: HashMap<String, serde_json::Value>,
}

impl TokenClaims {
    pub fn new_access_token(
        user_id: String,
        email: String,
        role: Role,
        iat: i64,
        exp: i64,
        scopes: Vec<TokenScope>,
    ) -> Self {
        Self {
            sub: user_id,
            token_type: TokenType::Access,
            iat,
            exp,
            iss: Some("homeryland-api".to_string()),
            aud: Some("homeryland-client".to_string()),
            jti: Some(uuid::Uuid::new_v4().to_string()),
            email: Some(email),
            role: Some(role),
            scopes,
            extra: HashMap::new(),
        }
    }
    pub fn new_refresh_token(user_id: String, iat: i64, exp: i64) -> Self {
        Self {
            sub: user_id,
            token_type: TokenType::Refresh,
            iat,
            exp,
            iss: Some("homeryland-api".to_string()),
            aud: Some("homeryland-client".to_string()),
            jti: Some(uuid::Uuid::new_v4().to_string()),
            email: None,
            role: None,
            scopes: vec![TokenScope::Refresh],
            extra: HashMap::new(),
        }
    }
    pub fn new_email_verification_token(
        user_id: String,
        email: String,
        iat: i64,
        exp: i64,
    ) -> Self {
        Self {
            sub: user_id,
            token_type: TokenType::EmailVerification,
            iat,
            exp,
            iss: Some("homeryland-api".to_string()),
            aud: Some("homeryland-client".to_string()),
            jti: Some(uuid::Uuid::new_v4().to_string()),
            email: Some(email),
            role: None,
            scopes: vec![TokenScope::EmailVerify],
            extra: HashMap::new(),
        }
    }
    pub fn new_password_reset_token(user_id: String, email: String, iat: i64, exp: i64) -> Self {
        Self {
            sub: user_id,
            token_type: TokenType::PasswordReset,
            iat,
            exp,
            iss: Some("homeryland-api".to_string()),
            aud: Some("homeryland-client".to_string()),
            jti: Some(uuid::Uuid::new_v4().to_string()),
            email: Some(email),
            role: None,
            scopes: vec![TokenScope::PasswordReset],
            extra: HashMap::new(),
        }
    }
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().timestamp() > self.exp
    }
    pub fn has_scope(&self, scope: &TokenScope) -> bool {
        self.scopes.contains(scope)
    }
    pub fn has_any_scope(&self, scopes: &[TokenScope]) -> bool {
        scopes.iter().any(|scope| self.scopes.contains(scope))
    }
    pub fn has_all_scopes(&self, scopes: &[TokenScope]) -> bool {
        scopes.iter().all(|scope| self.scopes.contains(scope))
    }
    pub fn default_scopes_for_role(role: &Role) -> Vec<TokenScope> {
        match role {
            Role::Admin => vec![
                TokenScope::Read,
                TokenScope::Write,
                TokenScope::Delete,
                TokenScope::UserRead,
                TokenScope::UserWrite,
                TokenScope::UserDelete,
                TokenScope::AdminRead,
                TokenScope::AdminWrite,
                TokenScope::AdminDelete,
            ],
            Role::User => vec![
                TokenScope::Read,
                TokenScope::Write,
                TokenScope::UserRead,
                TokenScope::UserWrite,
            ],
        }
    }
}
