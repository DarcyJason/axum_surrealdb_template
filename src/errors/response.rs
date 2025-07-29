use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
    pub trace_id: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(code: String, message: String, trace_id: Option<Uuid>) -> Self {
        Self {
            error: ErrorDetail {
                code,
                message,
                details: None,
            },
            trace_id: trace_id.map(|id| id.to_string()),
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn with_details(
        code: String,
        message: String,
        details: serde_json::Value,
        trace_id: Option<Uuid>,
    ) -> Self {
        Self {
            error: ErrorDetail {
                code,
                message,
                details: Some(details),
            },
            trace_id: trace_id.map(|id| id.to_string()),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
    pub code: String,
    pub trace_id: Option<Uuid>,
    pub details: Option<serde_json::Value>,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode, code: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status,
            code: code.into(),
            trace_id: Some(Uuid::new_v4()),
            details: None,
        }
    }

    pub fn with_trace_id(
        message: impl Into<String>,
        status: StatusCode,
        code: impl Into<String>,
        trace_id: Uuid,
    ) -> Self {
        HttpError {
            message: message.into(),
            status,
            code: code.into(),
            trace_id: Some(trace_id),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError::new(
            message,
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR",
        )
    }

    pub fn server_error_with_trace_id(message: impl Into<String>, trace_id: Uuid) -> Self {
        HttpError::with_trace_id(
            message,
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR",
            trace_id,
        )
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError::new(message, StatusCode::BAD_REQUEST, "BAD_REQUEST")
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError::new(message, StatusCode::CONFLICT, "CONFLICT")
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError::new(message, StatusCode::UNAUTHORIZED, "UNAUTHORIZED")
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        HttpError::new(message, StatusCode::FORBIDDEN, "FORBIDDEN")
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        HttpError::new(message, StatusCode::NOT_FOUND, "NOT_FOUND")
    }

    pub fn unprocessable_entity(message: impl Into<String>) -> Self {
        HttpError::new(
            message,
            StatusCode::UNPROCESSABLE_ENTITY,
            "UNPROCESSABLE_ENTITY",
        )
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse::new(
            self.code.clone(),
            self.message.clone(),
            self.trace_id,
        ));
        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: code: {}, message: {}, status: {}, trace_id: {:?}",
            self.code, self.message, self.status, self.trace_id
        )
    }
}

impl std::error::Error for HttpError {}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}
