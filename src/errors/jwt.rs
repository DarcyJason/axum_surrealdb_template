use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Invalid JWT token")]
    InvalidToken,
    #[error("JWT token has expired")]
    TokenExpired,
    #[error("Invalid JWT signature")]
    InvalidSignature,
    #[error("Invalid JWT key")]
    InvalidKey,
    #[error("Invalid JWT algorithm")]
    InvalidAlgorithm,
    #[error("Invalid JWT format")]
    InvalidFormat,
    #[error("JWT encoding error")]
    EncodingError,
    #[error("JWT decoding error")]
    DecodingError,
    #[error("Invalid JWT header")]
    InvalidHeader,
    #[error("Invalid JWT payload")]
    InvalidPayload,
}

impl JwtError {
    pub fn error_code(&self) -> &'static str {
        match self {
            JwtError::InvalidToken => "JWT_INVALID_TOKEN",
            JwtError::TokenExpired => "JWT_TOKEN_EXPIRED",
            JwtError::InvalidSignature => "JWT_INVALID_SIGNATURE",
            JwtError::InvalidKey => "JWT_INVALID_KEY",
            JwtError::InvalidAlgorithm => "JWT_INVALID_ALGORITHM",
            JwtError::InvalidFormat => "JWT_INVALID_FORMAT",
            JwtError::EncodingError => "JWT_ENCODING_ERROR",
            JwtError::DecodingError => "JWT_DECODING_ERROR",
            JwtError::InvalidHeader => "JWT_INVALID_HEADER",
            JwtError::InvalidPayload => "JWT_INVALID_PAYLOAD",
        }
    }
}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;

        match error.kind() {
            ErrorKind::InvalidToken => JwtError::InvalidToken,
            ErrorKind::ExpiredSignature => JwtError::TokenExpired,
            ErrorKind::InvalidSignature => JwtError::InvalidSignature,
            ErrorKind::InvalidKeyFormat => JwtError::InvalidKey,
            ErrorKind::InvalidAlgorithm => JwtError::InvalidAlgorithm,
            ErrorKind::InvalidAlgorithmName => JwtError::InvalidAlgorithm,
            ErrorKind::MissingRequiredClaim(_) => JwtError::InvalidPayload,
            ErrorKind::InvalidIssuer => JwtError::InvalidPayload,
            ErrorKind::InvalidAudience => JwtError::InvalidPayload,
            ErrorKind::InvalidSubject => JwtError::InvalidPayload,
            ErrorKind::ImmatureSignature => JwtError::TokenExpired,
            ErrorKind::Json(_) => JwtError::InvalidFormat,
            ErrorKind::Utf8(_) => JwtError::InvalidFormat,
            ErrorKind::Crypto(_) => JwtError::InvalidSignature,
            ErrorKind::Base64(_) => JwtError::InvalidFormat,
            _ => JwtError::DecodingError,
        }
    }
}
