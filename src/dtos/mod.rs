use once_cell::sync::Lazy;
use regex::Regex;

pub mod auth;
pub mod user;

pub static NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z\s]+$").unwrap());
pub static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,20}$").unwrap());
pub static TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z0-9_-]+$").unwrap());