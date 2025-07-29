use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Access,
    Refresh,
    Bearer,
}

impl TokenType {
    pub fn to_str(&self) -> &str {
        match self {
            TokenType::Access => "Access",
            TokenType::Refresh => "Refresh",
            TokenType::Bearer => "Bearer"
        }
    }
}