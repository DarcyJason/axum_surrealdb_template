use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::user::User;
use crate::dtos::NAME_REGEX;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {

    #[validate(length(min = 1, max = 50), regex(path = "*NAME_REGEX"))]
    pub name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserDetailResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserSummary {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserSummary>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

impl From<User> for UserDetailResponse {
    fn from(user: User) -> Self {
        UserDetailResponse { 
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

impl From<User> for UserSummary {
    fn from(user: User) -> Self {
        UserSummary { 
            id: user.id, 
            name: user.name, 
            email: user.email, 
            role: user.role.to_str().to_string(), 
            verified: user.verified, 
            created_at: user.created_at.map(|dt| dt.to_rfc3339()), 
        }
    }
}