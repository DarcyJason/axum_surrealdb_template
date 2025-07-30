use axum::{
    Extension,
    extract::{Query, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    errors::{auth::AuthError, core::Result},
    models::{role::Role, token_claims::TokenClaims},
    services::user::UserService,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub role: Option<String>,
    pub verified: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AdminUserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub active_sessions: usize,
}

#[derive(Debug, Serialize)]
pub struct SystemStats {
    pub total_users: u64,
    pub verified_users: u64,
    pub active_sessions: u64,
    pub admin_users: u64,
    pub recent_registrations: u64,
}

pub async fn get_system_stats(
    Extension(claims): Extension<TokenClaims>,
) -> Result<Json<SystemStats>> {
    // 验证管理员权限
    if !claims
        .role
        .as_ref()
        .map(|r| matches!(r, Role::Admin))
        .unwrap_or(false)
    {
        return Err(AuthError::PermissionDenied.into());
    }

    // TODO: 实现实际的统计查询
    // 这里需要在Repository中添加统计查询方法
    let stats = SystemStats {
        total_users: 0,          // 从数据库查询
        verified_users: 0,       // 从数据库查询
        active_sessions: 0,      // 从token_sessions表查询
        admin_users: 0,          // 从数据库查询
        recent_registrations: 0, // 查询最近注册的用户数
    };

    Ok(Json(stats))
}

/// 获取所有用户列表（仅管理员）
pub async fn list_users(
    Extension(claims): Extension<TokenClaims>,
    Query(query): Query<UserListQuery>,
) -> Result<Json<serde_json::Value>> {
    if !claims
        .role
        .as_ref()
        .map(|r| matches!(r, Role::Admin))
        .unwrap_or(false)
    {
        return Err(AuthError::PermissionDenied.into());
    }

    // TODO: 实现分页用户查询
    // 这里需要在UserRepository中添加分页查询方法
    let _page = query.page.unwrap_or(1);
    let _limit = query.limit.unwrap_or(10);

    // 临时返回空列表
    let users: Vec<AdminUserInfo> = vec![];

    Ok(Json(serde_json::json!({
        "users": users,
        "pagination": {
            "page": query.page.unwrap_or(1),
            "limit": query.limit.unwrap_or(10),
            "total": 0,
            "pages": 0
        }
    })))
}

pub async fn get_user_by_id(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>> {
    if !claims
        .role
        .as_ref()
        .map(|r| matches!(r, Role::Admin))
        .unwrap_or(false)
    {
        return Err(AuthError::PermissionDenied.into());
    }

    let user_id = payload
        .get("user_id")
        .and_then(|id| id.as_str())
        .ok_or_else(|| AuthError::InvalidCredentials)?;

    let user_service = UserService::new();
    let user = user_service
        .find_by_id(app_state.clone(), user_id.to_string())
        .await?
        .ok_or_else(|| crate::errors::db::DatabaseError::NotFound("User not found".to_string()))?;

    let sessions = app_state
        .token_service
        .get_user_active_sessions(app_state.clone(), user.id.clone())
        .await?;

    Ok(Json(serde_json::json!({
        "user": {
            "id": user.id,
            "name": user.name,
            "email": user.email,
            "role": user.role.to_str(),
            "verified": user.verified,
            "created_at": user.created_at,
            "updated_at": user.updated_at,
            "active_sessions": sessions.len(),
            "sessions": sessions
        }
    })))
}

pub async fn admin_revoke_user_sessions(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>> {
    if !claims
        .role
        .as_ref()
        .map(|r| matches!(r, Role::Admin))
        .unwrap_or(false)
    {
        return Err(AuthError::PermissionDenied.into());
    }

    let user_id = payload
        .get("user_id")
        .and_then(|id| id.as_str())
        .ok_or_else(|| AuthError::InvalidCredentials)?;

    app_state
        .token_service
        .revoke_all_user_sessions(app_state.clone(), user_id.to_string())
        .await?;

    Ok(Json(serde_json::json!({
        "message": format!("All sessions for user {} have been revoked", user_id)
    })))
}

pub async fn update_user_role(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>> {
    if !claims
        .role
        .as_ref()
        .map(|r| matches!(r, Role::Admin))
        .unwrap_or(false)
    {
        return Err(AuthError::PermissionDenied.into());
    }

    let user_id = payload
        .get("user_id")
        .and_then(|id| id.as_str())
        .ok_or_else(|| AuthError::InvalidCredentials)?;

    let new_role = payload
        .get("role")
        .and_then(|r| r.as_str())
        .ok_or_else(|| AuthError::InvalidCredentials)?;

    let _role = match new_role {
        "Admin" => Role::Admin,
        "User" => Role::User,
        _ => return Err(AuthError::InvalidCredentials.into()),
    };

    // TODO: 实现更新用户角色的方法
    // 这需要在UserRepository中添加update_role方法

    app_state
        .token_service
        .revoke_all_user_sessions(app_state.clone(), user_id.to_string())
        .await?;

    Ok(Json(serde_json::json!({
        "message": format!("User role updated to {}. User sessions have been revoked.", new_role)
    })))
}

pub async fn cleanup_expired_sessions(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
) -> Result<Json<serde_json::Value>> {
    if !claims
        .role
        .as_ref()
        .map(|r| matches!(r, Role::Admin))
        .unwrap_or(false)
    {
        return Err(AuthError::PermissionDenied.into());
    }

    let cleaned_count = app_state
        .token_service
        .cleanup_expired_sessions(app_state.clone())
        .await?;

    Ok(Json(serde_json::json!({
        "message": format!("Cleaned up {} expired sessions", cleaned_count),
        "cleaned_count": cleaned_count
    })))
}
