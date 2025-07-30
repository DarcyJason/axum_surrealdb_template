mod config;
mod database;
mod dtos;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod services;
mod state;

use crate::config::Config;
use crate::config::token::TokenConfig;
use crate::routes::all_routes;
use crate::services::token::TokenService;
use crate::state::AppState;
use axum::serve;
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

pub async fn run() {
    dotenvy::dotenv().ok();

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .pretty()
        .init();

    let config = Config::new();

    let db: Surreal<Client> = Surreal::<Client>::init();
    db.connect::<Ws>(&config.db_config.surreal_url)
        .await
        .unwrap();
    db.signin(Root {
        username: &config.db_config.surreal_root_username,
        password: &config.db_config.surreal_root_password,
    })
    .await
    .unwrap();
    db.use_ns(&config.db_config.surreal_root_ns)
        .use_db(&config.db_config.surreal_root_db)
        .await
        .unwrap();

    let port = config.server_config.server_port;
    info!(
        "{}",
        format!("✅ The server is running on http://localhost:{port}")
    );
    info!("✅ You can press Ctrl+C to shut it down.");

    let app_state = AppState {
        env: config.clone(),
        db: db,
        token_service: TokenService::new(TokenConfig::new()),
    };

    let app_router = all_routes(Arc::new(app_state.clone()));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    serve(listener, app_router)
        .with_graceful_shutdown(async {
            match tokio::signal::ctrl_c().await {
                Ok(()) => {
                    println!();
                    info!("✅ The server has been shut down gracefully by Ctrl+C.");
                }
                Err(e) => {
                    println!();
                    error!("❌ Error: {}", e);
                }
            }
        })
        .await
        .unwrap();
}
