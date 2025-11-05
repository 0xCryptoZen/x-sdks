use thiserror::Error;

/// Errors that can occur when using the X SDK
#[derive(Debug, Error)]
pub enum XError {
    /// Authentication failed (401, 403)
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Rate limit exceeded (429)
    #[error("Rate limit exceeded. Retry after {retry_after:?} seconds")]
    RateLimitExceeded {
        retry_after: Option<u64>,
        message: String,
    },

    /// Invalid request (400, 422)
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// API error with status code and message
    #[error("API error (status {code}): {message}")]
    ApiError { code: u16, message: String },

    /// Network error from reqwest
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// OAuth signature generation error
    #[error("OAuth error: {0}")]
    OAuthError(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl XError {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            XError::RateLimitExceeded { .. }
                | XError::ApiError {
                    code: 500..=599,
                    ..
                }
                | XError::NetworkError(_)
        )
    }

    /// Get retry delay in seconds if applicable
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            XError::RateLimitExceeded { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}

/// Result type for X SDK operations
pub type XResult<T> = Result<T, XError>;
