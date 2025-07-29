use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{token_status::TokenStatus, token_type::TokenType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    pub token_type: TokenType,
    pub status: TokenStatus,
    pub token_hash: String,
    pub jti: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_ip: Option<String>,
    pub last_used_ip: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Token {
    pub fn new(
        user_id: String,
        token_type: TokenType,
        token_hash: String,
        expires_at: DateTime<Utc>,
        jti: Option<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            token_type,
            status: TokenStatus::Active,
            token_hash,
            jti,
            created_at: Utc::now(),
            expires_at,
            last_used_at: None,
            revoked_at: None,
            created_ip: None,
            last_used_ip: None,
            user_agent: None,
            metadata: HashMap::new(),
        }
    }
    pub fn is_valid(&self) -> bool {
        matches!(self.status, TokenStatus::Active) && !self.is_expired()
    }
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
    pub fn revoke(&mut self) {
        self.status = TokenStatus::Revoked;
        self.revoked_at = Some(Utc::now());
    }
    pub fn mark_as_used(&mut self) {
        self.status = TokenStatus::Used;
    }
    pub fn update_last_used(&mut self, ip: Option<String>) {
        self.last_used_at = Some(Utc::now());
        if let Some(ip) = ip {
            self.last_used_ip = Some(ip);
        }
    }
}
