use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClickUpError {
    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Rate limit exceeded")]
    RateLimitError,

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Permission denied: {0}")]
    PermissionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigParseError(#[from] config::ConfigError),
}

impl From<reqwest::Error> for ClickUpError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            ClickUpError::NetworkError("Request timeout".to_string())
        } else if err.is_connect() {
            ClickUpError::NetworkError("Connection failed".to_string())
        } else {
            ClickUpError::HttpError(err)
        }
    }
}

impl From<serde_json::Error> for ClickUpError {
    fn from(err: serde_json::Error) -> Self {
        ClickUpError::JsonError(err)
    }
}

impl From<std::io::Error> for ClickUpError {
    fn from(err: std::io::Error) -> Self {
        ClickUpError::IoError(err)
    }
}

impl From<config::ConfigError> for ClickUpError {
    fn from(err: config::ConfigError) -> Self {
        ClickUpError::ConfigParseError(err)
    }
} 