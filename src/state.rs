use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{config::Config, services::token::TokenService};

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db: Arc<Surreal<Client>>,
    pub token_service: TokenService,
}
