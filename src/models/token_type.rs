use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    Access,
    Refresh,
    EmailVerification,
    PasswordReset,
    Invitation,
}

impl TokenType {
    pub fn to_str(&self) -> &str {
        match self {
            TokenType::Access => "access",
            TokenType::Refresh => "refresh",
            TokenType::EmailVerification => "email_verification",
            TokenType::PasswordReset => "password_reset",
            TokenType::Invitation => "invitation",
        }
    }
    pub fn from_str(s: &str) -> Option<TokenType> {
        match s {
            "access" => Some(TokenType::Access),
            "refresh" => Some(TokenType::Refresh),
            "email_verification" => Some(TokenType::EmailVerification),
            "password_reset" => Some(TokenType::PasswordReset),
            "invitation" => Some(TokenType::Invitation),
            _ => None,
        }
    }
}
