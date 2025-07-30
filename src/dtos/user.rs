use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub verified: bool,
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub active_sessions: usize,
}

#[derive(Debug, Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    pub location: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_active_at: chrono::DateTime<chrono::Utc>,
    pub is_current: bool,
}
