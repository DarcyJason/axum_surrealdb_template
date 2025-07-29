use crate::{config::Config, services::token::TokenService};

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub token_service: TokenService,
}
