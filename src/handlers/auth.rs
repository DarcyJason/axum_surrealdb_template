use axum::{Extension, extract::State, http::StatusCode, response::Json};
use std::sync::Arc;
use validator::Validate;

use crate::{
    dtos::auth::{
        ChangePasswordRequest, ForgotPasswordRequest, LoginRequest, LoginResponse, LogoutRequest,
        LogoutResponse, RefreshTokenRequest, RefreshTokenResponse, RegisterRequest,
        ResetPasswordRequest, UserInfo,
    },
    errors::{auth::AuthError, core::Result},
    models::token_claims::TokenClaims,
    services::user::UserService,
    state::AppState,
};

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserInfo>)> {
    payload.validate()?;

    let user_service = UserService::new();
    let user = user_service
        .create_user(
            app_state.clone(),
            payload.name,
            payload.email,
            payload.password,
        )
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(UserInfo {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role.to_str().to_string(),
            created_at: user.created_at.unwrap_or_default(),
        }),
    ))
}

pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    payload.validate()?;

    let user_service = UserService::new();
    let user = user_service
        .authenticate_user(app_state.clone(), payload.email, payload.password)
        .await?;

    // 使用TokenService创建会话
    let (access_token, refresh_token, _session) = app_state
        .token_service
        .create_session(
            app_state.clone(),
            &user.id,
            &user.email,
            &user.role,
            payload.device_info,
            None,
        )
        .await?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: app_state.env.token_config.access_token_expires_in,
        user: UserInfo {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role.to_str().to_string(),
            created_at: user.created_at.unwrap_or_default(),
        },
    }))
}

/// 刷新访问令牌
pub async fn refresh_token(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>> {
    let (new_access_token, new_refresh_token) = app_state
        .token_service
        .refresh_session(app_state.clone(), &payload.refresh_token)
        .await?;

    Ok(Json(RefreshTokenResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: app_state.env.token_config.access_token_expires_in,
    }))
}

pub async fn logout(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Json<LogoutResponse>> {
    // 如果提供了refresh_token，通过它找到session并撤销
    if let Some(refresh_token) = payload.refresh_token {
        let refresh_claims = app_state
            .token_service
            .verify_refresh_token(&refresh_token)?;

        if let Some(refresh_jti) = refresh_claims.jti {
            if let Some(session) = app_state
                .token_service
                .token_repo
                .find_by_refresh_token_jti(app_state.clone(), refresh_jti)
                .await?
            {
                app_state
                    .token_service
                    .revoke_session(app_state.clone(), session.id)
                    .await?;
            }
        }
    } else {
        // 如果没有提供refresh_token，通过access_token的jti找到session
        if let Some(access_jti) = claims.jti {
            if let Some(session) = app_state
                .token_service
                .token_repo
                .find_by_access_token_jti(app_state.clone(), access_jti)
                .await?
            {
                app_state
                    .token_service
                    .revoke_session(app_state.clone(), session.id)
                    .await?;
            }
        }
    }

    Ok(Json(LogoutResponse {
        message: "Successfully logged out".to_string(),
    }))
}

pub async fn change_password(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>)> {
    payload.validate()?;

    let user_service = UserService::new();
    let _updated_user = user_service
        .change_password(
            app_state.clone(),
            claims.sub.clone(),
            payload.current_password,
            payload.new_password,
        )
        .await?;

    // 修改密码后，撤销用户所有现有会话（强制重新登录）
    app_state
        .token_service
        .revoke_all_user_sessions(app_state.clone(), claims.sub)
        .await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Password changed successfully. Please log in again."
        })),
    ))
}

pub async fn forgot_password(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<serde_json::Value>> {
    payload.validate()?;

    let user_service = UserService::new();

    // 检查用户是否存在
    if let Some(user) = user_service
        .find_by_email(app_state.clone(), payload.email.clone())
        .await?
    {
        // 生成密码重置令牌
        let reset_token = app_state
            .token_service
            .generate_password_reset_token(&user.id, &user.email)?;

        // TODO: 在实际应用中，这里应该发送邮件
        // email_service.send_password_reset_email(&user.email, &reset_token).await?;

        tracing::info!(
            "Password reset token generated for user {}: {}",
            user.email,
            reset_token
        );
    }

    // 无论用户是否存在，都返回相同的消息（防止用户枚举攻击）
    Ok(Json(serde_json::json!({
        "message": "If the email exists, a password reset link has been sent."
    })))
}

pub async fn reset_password(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>> {
    payload.validate()?;

    let claims = app_state
        .token_service
        .verify_password_reset_token(&payload.token)?;

    if claims.is_expired() {
        return Err(AuthError::TokenExpired.into());
    }

    let user_service = UserService::new();
    let _updated_user = user_service
        .reset_password(app_state.clone(), claims.sub.clone(), payload.new_password)
        .await?;

    app_state
        .token_service
        .revoke_all_user_sessions(app_state.clone(), claims.sub)
        .await?;

    Ok(Json(serde_json::json!({
        "message": "Password reset successfully. Please log in with your new password."
    })))
}

pub async fn verify_email(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>> {
    let token = payload
        .get("token")
        .and_then(|t| t.as_str())
        .ok_or_else(|| AuthError::TokenNotProvided)?;

    let claims = app_state
        .token_service
        .verify_email_verification_token(token)?;

    if claims.is_expired() {
        return Err(AuthError::TokenExpired.into());
    }

    let user_service = UserService::new();
    let _updated_user = user_service
        .verify_email(app_state.clone(), claims.sub)
        .await?;

    Ok(Json(serde_json::json!({
        "message": "Email verified successfully."
    })))
}

pub async fn get_user_sessions(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
) -> Result<Json<serde_json::Value>> {
    let sessions = app_state
        .token_service
        .get_user_active_sessions(app_state.clone(), claims.sub.clone())
        .await?;

    let current_jti = claims.jti.as_ref();

    let session_info: Vec<serde_json::Value> = sessions
        .into_iter()
        .map(|session| {
            let is_current = current_jti
                .map(|jti| jti == &session.access_token_jti)
                .unwrap_or(false);

            serde_json::json!({
                "id": session.id,
                "device_info": session.device_info,
                "ip_address": session.ip_address,
                "location": session.location,
                "created_at": session.created_at,
                "last_active_at": session.last_active_at,
                "is_current": is_current
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "sessions": session_info,
        "total": session_info.len()
    })))
}

pub async fn revoke_all_sessions(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
) -> Result<Json<serde_json::Value>> {
    app_state
        .token_service
        .revoke_all_user_sessions(app_state.clone(), claims.sub)
        .await?;

    Ok(Json(serde_json::json!({
        "message": "All sessions have been revoked successfully."
    })))
}

pub async fn revoke_session(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>> {
    let session_id = payload
        .get("session_id")
        .and_then(|s| s.as_str())
        .ok_or_else(|| AuthError::InvalidCredentials)?;

    // 验证会话属于当前用户
    if let Some(session) = app_state
        .token_service
        .token_repo
        .find_by_id(app_state.clone(), session_id.to_string())
        .await?
    {
        if session.user_id != claims.sub {
            return Err(AuthError::PermissionDenied.into());
        }

        app_state
            .token_service
            .revoke_session(app_state.clone(), session_id.to_string())
            .await?;

        Ok(Json(serde_json::json!({
            "message": "Session revoked successfully."
        })))
    } else {
        Err(crate::errors::db::DatabaseError::NotFound("Session not found".to_string()).into())
    }
}

pub async fn resend_verification_email(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<TokenClaims>,
) -> Result<Json<serde_json::Value>> {
    let user_service = UserService::new();
    let user = user_service
        .find_by_id(app_state.clone(), claims.sub)
        .await?
        .ok_or(AuthError::UserNoLongerExists)?;

    if user.verified {
        return Ok(Json(serde_json::json!({
            "message": "Email is already verified."
        })));
    }

    let verification_token = app_state
        .token_service
        .generate_email_verification_token(&user.id, &user.email)?;

    // TODO: 在实际应用中，这里应该发送邮件
    // email_service.send_verification_email(&user.email, &verification_token).await?;

    tracing::info!(
        "Email verification token generated for user {}: {}",
        user.email,
        verification_token
    );

    Ok(Json(serde_json::json!({
        "message": "Verification email has been sent."
    })))
}
