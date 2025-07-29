use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Field '{field}' cannot be empty")]
    EmptyField { field: String },
    #[error("Field '{field}' has invalid format: {reason}")]
    InvalidFormat { field: String, reason: String },
    #[error("Field '{field}' value is out of range: {reason}")]
    OutOfRange { field: String, reason: String },
    #[error("Field '{field}' exceeds maximum length of {max_length}")]
    TooLong { field: String, max_length: usize },
    #[error("Field '{field}' is below minimum length of {min_length}")]
    TooShort { field: String, min_length: usize },
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Multiple validation errors occurred")]
    Multiple(Vec<ValidationError>),
}

impl ValidationError {
    pub fn error_code(&self) -> &'static str {
        match self {
            ValidationError::EmptyField { .. } => "VALIDATION_EMPTY_FIELD",
            ValidationError::InvalidFormat { .. } => "VALIDATION_INVALID_FORMAT",
            ValidationError::OutOfRange { .. } => "VALIDATION_OUT_OF_RANGE",
            ValidationError::TooLong { .. } => "VALIDATION_TOO_LONG",
            ValidationError::TooShort { .. } => "VALIDATION_TOO_SHORT",
            ValidationError::InvalidEmail => "VALIDATION_INVALID_EMAIL",
            ValidationError::Multiple(_) => "VALIDATION_MULTIPLE_ERRORS",
        }
    }

    pub fn to_details(&self) -> Option<serde_json::Value> {
        match self {
            ValidationError::Multiple(errors) => {
                let error_details: Vec<_> = errors
                    .iter()
                    .map(|e| {
                        json!({
                            "code": e.error_code(),
                            "message": e.to_string()
                        })
                    })
                    .collect();
                Some(json!({ "errors": error_details }))
            }
            _ => None,
        }
    }
}
