use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSession {
    pub id: String,
    pub user_id: String,
    pub access_token_jti: String,
    pub refresh_token_jti: String,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub is_active: bool,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    pub location: Option<String>,
}

impl TokenSession {
    pub fn new(user_id: String, access_jti: String, refresh_jti: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            access_token_jti: access_jti,
            refresh_token_jti: refresh_jti,
            created_at: Utc::now(),
            last_active_at: Utc::now(),
            is_active: true,
            device_info: None,
            ip_address: None,
            location: None,
        }
    }
}
