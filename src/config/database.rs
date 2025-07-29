#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub surreal_url: String,
    pub surreal_root_username: String,
    pub surreal_root_password: String,
    pub surreal_root_ns: String,
    pub surreal_root_db: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            surreal_url: std::env::var("SURREAL_URL").expect("SURREAL_URL must be set"),
            surreal_root_username: std::env::var("SURREAL_ROOT_USERNAME").expect("SURREAL_ROOT_USERNAME must be set"),
            surreal_root_password: std::env::var("SURREAL_ROOT_PASSWORD").expect("SURREAL_ROOT_PASSWORD must be set"),
            surreal_root_ns: std::env::var("SURREAL_ROOT_NS").expect("SURREAL_ROOT_NS must be set"),
            surreal_root_db: std::env::var("SURREAL_ROOT_DB").expect("SURREAL_ROOT_DB must be set"),
        }
    }
}

impl DatabaseConfig {
    pub fn new() -> Self {
        Self::default()
    }
}