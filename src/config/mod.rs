use crate::config::database::DatabaseConfig;
use crate::config::frontend::FrontendConfig;
use crate::config::server::ServerConfig;
use crate::config::token::TokenConfig;

pub mod server;
pub mod database;
pub mod frontend;
pub mod token;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_config: ServerConfig,
    pub db_config: DatabaseConfig,
    pub frontend_config: FrontendConfig,
    pub token_config: TokenConfig
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server_config: ServerConfig::new(),
            db_config: DatabaseConfig::new(),
            frontend_config: FrontendConfig::new(),
            token_config: TokenConfig::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}