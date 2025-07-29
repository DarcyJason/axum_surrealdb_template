use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenStatus {
    Active,
    Expired,
    Revoked,
    Used,
}

impl TokenStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            TokenStatus::Active => "active",
            TokenStatus::Expired => "expired",
            TokenStatus::Revoked => "revoked",
            TokenStatus::Used => "used",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(TokenStatus::Active),
            "expired" => Some(TokenStatus::Expired),
            "revoked" => Some(TokenStatus::Revoked),
            "used" => Some(TokenStatus::Used),
            _ => None,
        }
    }
}
