use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

pub struct TokenRepository {
    db: Arc<Surreal<Client>>
}

impl TokenRepository {
    
}