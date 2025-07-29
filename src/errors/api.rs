use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Invalid API request: {message}")]
    InvalidRequest { message: String },
    #[error("Requested resource not found")]
    NotFound,
    #[error("API rate limit exceeded")]
    RateLimitExceeded,
    #[error("Unsupported media type")]
    UnsupportedMediaType,
    #[error("Request payload too large")]
    PayloadTooLarge,
}

impl ApiError {
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::InvalidRequest {
            message: message.into(),
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            ApiError::InvalidRequest { .. } => "API_INVALID_REQUEST",
            ApiError::NotFound => "API_NOT_FOUND",
            ApiError::RateLimitExceeded => "API_RATE_LIMIT_EXCEEDED",
            ApiError::UnsupportedMediaType => "API_UNSUPPORTED_MEDIA_TYPE",
            ApiError::PayloadTooLarge => "API_PAYLOAD_TOO_LARGE",
        }
    }
}
