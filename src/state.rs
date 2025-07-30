use crate::{config::Config, services::token::TokenService};
use surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db: Surreal<Client>,
    pub token_service: TokenService,
}
