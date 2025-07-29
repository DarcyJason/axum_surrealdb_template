use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenScope {
    Read,
    Write,
    Delete,
    UserRead,
    UserWrite,
    UserDelete,
    AdminRead,
    AdminWrite,
    AdminDelete,
    Refresh,
    EmailVerify,
    PasswordReset,
    Custom(String),
}

impl TokenScope {
    pub fn to_str(&self) -> String {
        match self {
            TokenScope::Read => "read".to_string(),
            TokenScope::Write => "write".to_string(),
            TokenScope::Delete => "delete".to_string(),
            TokenScope::UserRead => "user:read".to_string(),
            TokenScope::UserWrite => "user:write".to_string(),
            TokenScope::UserDelete => "user:delete".to_string(),
            TokenScope::AdminRead => "admin:read".to_string(),
            TokenScope::AdminWrite => "admin:write".to_string(),
            TokenScope::AdminDelete => "admin:delete".to_string(),
            TokenScope::Refresh => "refresh".to_string(),
            TokenScope::EmailVerify => "email:verify".to_string(),
            TokenScope::PasswordReset => "password:reset".to_string(),
            TokenScope::Custom(scope) => scope.clone(),
        }
    }
    pub fn from_str(scope: &str) -> Option<TokenScope> {
        match scope {
            "read" => Some(TokenScope::Read),
            "write" => Some(TokenScope::Write),
            "delete" => Some(TokenScope::Delete),
            "user:read" => Some(TokenScope::UserRead),
            "user:write" => Some(TokenScope::UserWrite),
            "user:delete" => Some(TokenScope::UserDelete),
            "admin:read" => Some(TokenScope::AdminRead),
            "admin:write" => Some(TokenScope::AdminWrite),
            "admin:delete" => Some(TokenScope::AdminDelete),
            "refresh" => Some(TokenScope::Refresh),
            "email:verify" => Some(TokenScope::EmailVerify),
            "password:reset" => Some(TokenScope::PasswordReset),
            _ => None,
        }
    }
}

impl Serialize for TokenScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

impl<'de> Deserialize<'de> for TokenScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(TokenScope::from_str(&s).unwrap_or(TokenScope::Custom(s)))
    }
}
