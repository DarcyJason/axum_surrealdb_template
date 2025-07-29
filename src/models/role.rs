use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

impl Role {
    pub fn to_str(&self) -> &str {
        match self {
            Role::Admin => "Admin",
            Role::User => "User",
        }
    }
}