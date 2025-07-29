use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::role::Role;

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