use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::{error, warn};
use uuid::Uuid;

use crate::errors::{
    api::ApiError, auth::AuthError, db::DatabaseError, jwt::JwtError, response::HttpError,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("JWT error: {0}")]
    Jwt(#[from] JwtError),
    #[error("Database error: {0}")]
    Db(#[from] DatabaseError),
    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),
    #[error("API error: {0}")]
    Api(#[from] ApiError),
    #[error("Validator error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    #[error("Internal server error: {message}")]
    Internal { message: String, trace_id: Uuid },
}

impl Error {
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
            trace_id: Uuid::new_v4(),
        }
    }

    pub fn trace_id(&self) -> Uuid {
        match self {
            Error::Internal { trace_id, .. } => *trace_id,
            _ => Uuid::new_v4(),
        }
    }

    pub fn error_code(&self) -> String {
        match self {
            Error::Jwt(err) => err.error_code().to_string(),
            Error::Db(err) => match err {
                DatabaseError::ConnectionError { .. } => "DB_CONNECTION_ERROR".to_string(),
                DatabaseError::QueryError { .. } => "DB_QUERY_ERROR".to_string(),
                DatabaseError::TransactionError { .. } => "DB_TRANSACTION_ERROR".to_string(),
                DatabaseError::NotFound(_) => "DB_NOT_FOUND".to_string(),
                DatabaseError::ConstraintViolation(_) => "DB_CONSTRAINT_VIOLATION".to_string(),
            },
            Error::Auth(err) => err.error_code().to_string(),
            Error::Api(err) => err.error_code().to_string(),
            Error::Validation(_) => "VALIDATION_ERROR".to_string(),
            Error::Internal { .. } => "INTERNAL_SERVER_ERROR".to_string(),
        }
    }

    pub fn log_error(&self) {
        let trace_id = self.trace_id();

        match self {
            Error::Jwt(err) => {
                warn!(
                    error = %err,
                    trace_id = %trace_id,
                    error_code = %self.error_code(),
                    "JWT error occurred"
                );
            }
            Error::Db(err) => {
                error!(
                    error = %err,
                    trace_id = %trace_id,
                    error_code = %self.error_code(),
                    "Database error occurred"
                );
            }
            Error::Auth(err) => match err {
                AuthError::InvalidCredentials | AuthError::TokenExpired => {
                    warn!(
                        error = %err,
                        trace_id = %trace_id,
                        error_code = %self.error_code(),
                        "Authentication error"
                    );
                }
                _ => {
                    error!(
                        error = %err,
                        trace_id = %trace_id,
                        error_code = %self.error_code(),
                        "Authentication error occurred"
                    );
                }
            },
            Error::Api(err) => {
                warn!(
                    error = %err,
                    trace_id = %trace_id,
                    error_code = %self.error_code(),
                    "API error occurred"
                );
            }
            Error::Validation(err) => {
                warn!(
                    error = %err,
                    trace_id = %trace_id,
                    error_code = %self.error_code(),
                    "Validation error occurred"
                );
            }
            Error::Internal { message, .. } => {
                error!(
                    message = %message,
                    trace_id = %trace_id,
                    error_code = %self.error_code(),
                    "Internal server error occurred"
                );
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let http_error = HttpError::from(self);
        http_error.into_response()
    }
}

impl From<Error> for HttpError {
    fn from(error: Error) -> Self {
        let trace_id = error.trace_id();

        error.log_error();

        match error {
            Error::Jwt(err) => match err {
                JwtError::TokenExpired => HttpError::with_trace_id(
                    "JWT token has expired",
                    axum::http::StatusCode::UNAUTHORIZED,
                    err.error_code(),
                    trace_id,
                ),
                JwtError::InvalidToken | JwtError::InvalidSignature | JwtError::InvalidFormat => {
                    HttpError::with_trace_id(
                        "Invalid JWT token",
                        axum::http::StatusCode::UNAUTHORIZED,
                        err.error_code(),
                        trace_id,
                    )
                }
                _ => HttpError::with_trace_id(
                    err.to_string(),
                    axum::http::StatusCode::BAD_REQUEST,
                    err.error_code(),
                    trace_id,
                ),
            },
            Error::Db(err) => match err {
                DatabaseError::NotFound(msg) => HttpError::with_trace_id(
                    msg,
                    axum::http::StatusCode::NOT_FOUND,
                    "DB_NOT_FOUND",
                    trace_id,
                ),
                DatabaseError::ConstraintViolation(msg) => HttpError::with_trace_id(
                    msg,
                    axum::http::StatusCode::CONFLICT,
                    "DB_CONSTRAINT_VIOLATION",
                    trace_id,
                ),
                _ => HttpError::server_error_with_trace_id("Database operation failed", trace_id),
            },

            Error::Auth(err) => match err {
                AuthError::InvalidCredentials => HttpError::with_trace_id(
                    "Invalid credentials provided",
                    axum::http::StatusCode::UNAUTHORIZED,
                    err.error_code(),
                    trace_id,
                ),
                AuthError::TokenExpired => HttpError::with_trace_id(
                    "Access token has expired",
                    axum::http::StatusCode::UNAUTHORIZED,
                    err.error_code(),
                    trace_id,
                ),
                AuthError::TokenNotProvided | AuthError::NotAuthenticated => {
                    HttpError::with_trace_id(
                        err.to_string(),
                        axum::http::StatusCode::UNAUTHORIZED,
                        err.error_code(),
                        trace_id,
                    )
                }
                AuthError::PermissionDenied => HttpError::with_trace_id(
                    err.to_string(),
                    axum::http::StatusCode::FORBIDDEN,
                    err.error_code(),
                    trace_id,
                ),
                AuthError::EmailAlreadyExists => HttpError::with_trace_id(
                    err.to_string(),
                    axum::http::StatusCode::CONFLICT,
                    err.error_code(),
                    trace_id,
                ),
                _ => HttpError::with_trace_id(
                    err.to_string(),
                    axum::http::StatusCode::BAD_REQUEST,
                    err.error_code(),
                    trace_id,
                ),
            },

            Error::Api(err) => match err {
                ApiError::NotFound => HttpError::with_trace_id(
                    err.to_string(),
                    axum::http::StatusCode::NOT_FOUND,
                    err.error_code(),
                    trace_id,
                ),
                ApiError::RateLimitExceeded => HttpError::with_trace_id(
                    err.to_string(),
                    axum::http::StatusCode::TOO_MANY_REQUESTS,
                    err.error_code(),
                    trace_id,
                ),
                ApiError::PayloadTooLarge => HttpError::with_trace_id(
                    err.to_string(),
                    axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                    err.error_code(),
                    trace_id,
                ),
                _ => HttpError::with_trace_id(
                    err.to_string(),
                    axum::http::StatusCode::BAD_REQUEST,
                    err.error_code(),
                    trace_id,
                ),
            },
            Error::Validation(validation_errors) => {
                let mut error_details = serde_json::Map::new();

                for (field, errors) in validation_errors.field_errors() {
                    let field_errors: Vec<String> = errors
                        .iter()
                        .map(|error| {
                            error
                                .message
                                .as_ref()
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| format!("Invalid value for field: {}", field))
                        })
                        .collect();
                    error_details.insert(
                        field.to_string(),
                        serde_json::Value::Array(
                            field_errors
                                .into_iter()
                                .map(serde_json::Value::String)
                                .collect(),
                        ),
                    );
                }
                HttpError::with_trace_id(
                    "Validation failed",
                    axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    "VALIDATION_ERROR",
                    trace_id,
                )
                .with_details(serde_json::Value::Object(error_details))
            }

            Error::Internal { .. } => {
                HttpError::server_error_with_trace_id("Internal server error", trace_id)
            }
        }
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Error::Jwt(JwtError::from(error))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
