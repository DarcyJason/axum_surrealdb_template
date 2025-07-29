use serde::{Deserialize, Serialize};
use crate::models::{role::Role, token_type::TokenType};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String, // user_id
    pub email: String,
    pub role: Role,
    pub exp: usize,
    pub iat: usize,
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String, // user_id
    pub exp: usize,
    pub iat: usize,
    pub token_type: TokenType,
}
