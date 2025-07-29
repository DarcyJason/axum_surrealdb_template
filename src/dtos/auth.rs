use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::token_type::TokenType;
use crate::models::user::User;
use crate::dtos::{NAME_REGEX, PASSWORD_REGEX, TOKEN_REGEX};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {

    #[validate(length(min = 1, max = 50), regex(path = "*NAME_REGEX"))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 20), regex(path = "*PASSWORD_REGEX"))]
    pub password: String,

    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 20), regex(path = "*PASSWORD_REGEX"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {

    #[validate(length(min = 32, max = 512), regex(path = "*TOKEN_REGEX"))]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyEmailRequest {

    #[validate(length(min = 32, max = 512), regex(path = "*TOKEN_REGEX"))]
    pub token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {

    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {

    #[validate(length(min = 32, max = 512), regex(path = "*TOKEN_REGEX"))]
    pub token: String,

    #[validate(length(min = 8, max = 20), regex(path = "*PASSWORD_REGEX"))]
    pub new_password: String,

    #[validate(must_match(other = "new_password"))]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {

    #[validate(length(min = 8, max = 20), regex(path = "*PASSWORD_REGEX"))]
    pub current_password: String,

    #[validate(length(min = 8, max = 20), regex(path = "*PASSWORD_REGEX"))]
    pub new_password: String,

    #[validate(must_match(other = "new_password"))]
    pub confirm_password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub tokens: TokenResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse { 
            id: user.id, 
            name: user.name, 
            email: user.email, 
            role: user.role.to_str().to_string(), 
            verified: user.verified, 
            created_at: user.created_at.map(|dt| dt.to_rfc3339()), 
            updated_at: user.updated_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

impl TokenResponse {
    pub fn new(access_token: String, refresh_token: String, expires_in: i64) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type: TokenType::Bearer.to_str().to_string(),
            expires_in,
        }
    }
}