use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use crate::{
    models::{token_claims::TokenClaims, token_scope::TokenScope},
    services::token::TokenService,
    state::AppState,
};

pub async fn auth_middleware(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let token =
        TokenService::extract_token_from_header(auth_header).ok_or(StatusCode::UNAUTHORIZED)?;
    let token_service = &app_state.token_service;
    let claims = token_service
        .verify_access_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if claims.is_expired() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

pub fn require_scopes(
    required_scopes: Vec<TokenScope>,
) -> impl Clone
+ Send
+ Sync
+ 'static
+ Fn(Request, Next) -> Pin<Box<dyn Future<Output = Result<Response, StatusCode>> + Send>> {
    move |request: Request, next: Next| {
        let required_scopes = required_scopes.clone();
        Box::pin(async move {
            // 从请求扩展中获取 claims
            let claims = request
                .extensions()
                .get::<TokenClaims>()
                .ok_or(StatusCode::UNAUTHORIZED)?;

            // 检查是否有所需的权限
            if !claims.has_any_scope(&required_scopes) {
                return Err(StatusCode::FORBIDDEN);
            }

            Ok(next.run(request).await)
        })
    }
}

pub async fn admin_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    let claims = request
        .extensions()
        .get::<TokenClaims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let admin_scopes = vec![
        TokenScope::AdminRead,
        TokenScope::AdminWrite,
        TokenScope::AdminDelete,
    ];

    if !claims.has_any_scope(&admin_scopes) {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

pub async fn optional_auth_middleware(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    // 尝试获取 Authorization header
    if let Some(auth_header) = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
    {
        if let Some(token) = TokenService::extract_token_from_header(auth_header) {
            let token_service = &app_state.token_service;

            if let Ok(claims) = token_service.verify_access_token(token) {
                if !claims.is_expired() {
                    request.extensions_mut().insert(claims);
                }
            }
        }
    }

    next.run(request).await
}

pub async fn require_read_scope(request: Request, next: Next) -> Result<Response, StatusCode> {
    let claims = request
        .extensions()
        .get::<TokenClaims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !claims.has_scope(&TokenScope::Read) {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

pub async fn require_write_scope(request: Request, next: Next) -> Result<Response, StatusCode> {
    let claims = request
        .extensions()
        .get::<TokenClaims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !claims.has_scope(&TokenScope::Write) {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

pub async fn require_delete_scope(request: Request, next: Next) -> Result<Response, StatusCode> {
    let claims = request
        .extensions()
        .get::<TokenClaims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !claims.has_scope(&TokenScope::Delete) {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

pub async fn require_user_read_scope(request: Request, next: Next) -> Result<Response, StatusCode> {
    let claims = request
        .extensions()
        .get::<TokenClaims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !claims.has_scope(&TokenScope::UserRead) {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}
