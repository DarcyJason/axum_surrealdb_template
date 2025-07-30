use crate::models::role::Role;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
    pub verified: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublicInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub verified: bool,
    pub created_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(id: String, name: String, email: String, password: String) -> Self {
        Self {
            id,
            name,
            email,
            password,
            role: Role::User,
            verified: false,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }
    pub fn is_admin(&self) -> bool {
        matches!(self.role, Role::Admin)
    }
    pub fn to_public_info(&self) -> UserPublicInfo {
        UserPublicInfo {
            id: self.id.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            role: self.role.clone(),
            verified: self.verified,
            created_at: self.created_at,
        }
    }
}
