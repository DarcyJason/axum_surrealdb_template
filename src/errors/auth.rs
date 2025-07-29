use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials provided")]
    InvalidCredentials,
    #[error("Access token has expired")]
    TokenExpired,
    #[error("Authentication token not provided")]
    TokenNotProvided,
    #[error("Authentication token is invalid or malformed")]
    InvalidToken,
    #[error("User with this email already exists")]
    EmailAlreadyExists,
    #[error("User belonging to this token no longer exists")]
    UserNoLongerExists,
    #[error("Password cannot be empty")]
    EmptyPassword,
    #[error("Password must not be more than {max_length} characters")]
    PasswordTooLong { max_length: usize },
    #[error("Error while hashing password")]
    HashingError,
    #[error("Invalid password hash format")]
    InvalidHashFormat,
    #[error("You are not allowed to perform this action")]
    PermissionDenied,
    #[error("Authentication required. Please log in.")]
    NotAuthenticated,
}

impl AuthError {
    pub fn password_too_long(max_length: usize) -> Self {
        Self::PasswordTooLong { max_length }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            AuthError::InvalidCredentials => "AUTH_INVALID_CREDENTIALS",
            AuthError::TokenExpired => "AUTH_TOKEN_EXPIRED",
            AuthError::TokenNotProvided => "AUTH_TOKEN_NOT_PROVIDED",
            AuthError::InvalidToken => "AUTH_INVALID_TOKEN",
            AuthError::EmailAlreadyExists => "AUTH_EMAIL_EXISTS",
            AuthError::UserNoLongerExists => "AUTH_USER_NOT_EXISTS",
            AuthError::EmptyPassword => "AUTH_EMPTY_PASSWORD",
            AuthError::PasswordTooLong { .. } => "AUTH_PASSWORD_TOO_LONG",
            AuthError::HashingError => "AUTH_HASHING_ERROR",
            AuthError::InvalidHashFormat => "AUTH_INVALID_HASH_FORMAT",
            AuthError::PermissionDenied => "AUTH_PERMISSION_DENIED",
            AuthError::NotAuthenticated => "AUTH_NOT_AUTHENTICATED",
        }
    }
}
