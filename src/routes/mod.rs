use crate::routes::protected::protected_routes;
use crate::routes::public::public_routes;
use crate::state::AppState;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::{Extension, Router};
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::Level;

pub mod protected;
pub mod public;

pub fn all_routes(app_state: Arc<AppState>) -> Router {
    let frontend_url = app_state.env.frontend_config.frontend_url.clone();

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(10)
        .finish()
        .unwrap();

    let api_routes = Router::new()
        .merge(public_routes())
        .merge(protected_routes());

    Router::new().nest("/api/v1", api_routes)
        .layer(CorsLayer::new()
            .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
            .allow_methods([Method::GET, Method::POST, Method::PUT]))
        .layer(TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new()
                .level(Level::INFO))
            .on_request(trace::DefaultOnRequest::new()
                .level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new()
                .level(Level::INFO)))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        .layer(GovernorLayer{
            config: Arc::new(governor_conf)
        })
        .layer(Extension(app_state))
}