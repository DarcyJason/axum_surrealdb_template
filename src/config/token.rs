#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub access_token_expires_in: i64,
    pub refresh_token_expires_in: i64,
    pub token_cleanup_interval: i64,
}

impl Default for TokenConfig {
    fn default() -> Self {
        TokenConfig {
            jwt_access_secret: std::env::var("JWT_ACCESS_SECRET")
                .expect("JWT_ACCESS_SECRET must be set"),
            jwt_refresh_secret: std::env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET"),
            access_token_expires_in: std::env::var("ACCESS_TOKEN_EXPIRES_IN")
                .expect("ACCESS_TOKEN_EXPIRES_IN")
                .parse::<i64>()
                .expect("ACCESS_TOKEN_EXPIRES_IN should be a i64 number"),
            refresh_token_expires_in: std::env::var("REFRESH_TOKEN_EXPIRES_IN")
                .expect("REFRESH_TOKEN_EXPIRES_IN")
                .parse::<i64>()
                .expect("REFRESH_TOKEN_EXPIRES_IN should be a i64 number"),
            token_cleanup_interval: std::env::var("TOKEN_CLEANUP_INTERVAL")
                .expect("TOKEN_CLEANUP_INTERVAL")
                .parse::<i64>()
                .expect("TOKEN_CLEANUP_INTERVAL should be a i64 number"),
        }
    }
}

impl TokenConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
