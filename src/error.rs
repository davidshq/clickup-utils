//! # Error Handling
//! 
//! This module defines the error types used throughout the ClickUp CLI application.
//! It provides a comprehensive error handling system that covers all possible
//! failure scenarios including API errors, network issues, authentication problems,
//! and configuration errors.
//! 
//! ## Error Types
//! 
//! The `ClickUpError` enum provides specific error variants for different types
//! of failures, making it easy to handle errors appropriately in different contexts.
//! 
//! ## Error Conversion
//! 
//! This module also provides automatic conversion from external error types
//! (like `reqwest::Error`, `serde_json::Error`, etc.) to `ClickUpError`,
//! making it easy to use with the `?` operator.
//! 
//! ## Usage
//! 
//! ```rust
//! use crate::error::ClickUpError;
//! 
//! fn some_function() -> Result<(), ClickUpError> {
//!     // Function implementation
//!     Ok(())
//! }
//! ```

use thiserror::Error;

/// Comprehensive error type for ClickUp CLI operations
/// 
/// This enum defines all possible error types that can occur during ClickUp CLI
/// operations. Each variant represents a specific type of error with appropriate
/// context information.
/// 
/// The error type is designed to be user-friendly while providing enough detail
/// for debugging and error handling.
#[derive(Error, Debug)]
pub enum ClickUpError {
    /// API request failed with a specific error message
    /// 
    /// This error occurs when the ClickUp API returns an error response.
    /// The error message contains details about what went wrong.
    #[error("API request failed: {0}")]
    ApiError(String),

    /// Authentication failed with a specific reason
    /// 
    /// This error occurs when authentication fails, such as when an API token
    /// is invalid, missing, or expired.
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// Configuration error with details
    /// 
    /// This error occurs when there are problems with application configuration,
    /// such as missing config files or invalid configuration values.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Input validation error
    /// 
    /// This error occurs when user input is invalid or doesn't meet requirements.
    /// Examples include empty task names, invalid IDs, or malformed data.
    #[error("Invalid input: {0}")]
    ValidationError(String),

    /// Network-related error
    /// 
    /// This error occurs when there are network connectivity issues, timeouts,
    /// or other network-related problems.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Rate limit exceeded
    /// 
    /// This error occurs when the API rate limit has been exceeded.
    /// The application should wait before making additional requests.
    #[error("Rate limit exceeded")]
    RateLimitError,

    /// Resource not found
    /// 
    /// This error occurs when trying to access a resource that doesn't exist,
    /// such as a task, list, or workspace that has been deleted or doesn't exist.
    #[error("Resource not found: {0}")]
    NotFoundError(String),

    /// Permission denied
    /// 
    /// This error occurs when the user doesn't have permission to perform
    /// the requested operation on the specified resource.
    #[error("Permission denied: {0}")]
    PermissionError(String),

    /// Serialization error
    /// 
    /// This error occurs when data cannot be serialized to JSON format,
    /// typically when creating or updating resources.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error
    /// 
    /// This error occurs when API response data cannot be deserialized from JSON,
    /// typically due to unexpected response format or malformed JSON.
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// I/O error
    /// 
    /// This error occurs when there are file system or I/O related problems,
    /// such as reading configuration files or writing logs.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// HTTP client error
    /// 
    /// This error occurs when there are problems with HTTP requests,
    /// such as connection failures, timeouts, or malformed requests.
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON parsing error
    /// 
    /// This error occurs when JSON data cannot be parsed, typically when
    /// processing API responses or configuration files.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Configuration parsing error
    /// 
    /// This error occurs when configuration files cannot be parsed or contain
    /// invalid values.
    #[error("Configuration error: {0}")]
    ConfigParseError(#[from] config::ConfigError),
}

impl From<reqwest::Error> for ClickUpError {
    /// Converts `reqwest::Error` to `ClickUpError`
    /// 
    /// This implementation provides intelligent error conversion based on the
    /// type of HTTP error. It distinguishes between timeout errors, connection
    /// errors, and other HTTP-related issues.
    /// 
    /// # Arguments
    /// 
    /// * `err` - The reqwest error to convert
    /// 
    /// # Returns
    /// 
    /// Returns a `ClickUpError` variant appropriate for the type of HTTP error.
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
    /// Converts `serde_json::Error` to `ClickUpError`
    /// 
    /// This implementation converts JSON parsing errors to the appropriate
    /// `ClickUpError` variant.
    /// 
    /// # Arguments
    /// 
    /// * `err` - The serde_json error to convert
    /// 
    /// # Returns
    /// 
    /// Returns a `ClickUpError::JsonError` containing the original error.
    fn from(err: serde_json::Error) -> Self {
        ClickUpError::JsonError(err)
    }
}

impl From<std::io::Error> for ClickUpError {
    /// Converts `std::io::Error` to `ClickUpError`
    /// 
    /// This implementation converts I/O errors to the appropriate
    /// `ClickUpError` variant.
    /// 
    /// # Arguments
    /// 
    /// * `err` - The I/O error to convert
    /// 
    /// # Returns
    /// 
    /// Returns a `ClickUpError::IoError` containing the original error.
    fn from(err: std::io::Error) -> Self {
        ClickUpError::IoError(err)
    }
}

impl From<config::ConfigError> for ClickUpError {
    /// Converts `config::ConfigError` to `ClickUpError`
    /// 
    /// This implementation converts configuration parsing errors to the appropriate
    /// `ClickUpError` variant.
    /// 
    /// # Arguments
    /// 
    /// * `err` - The config error to convert
    /// 
    /// # Returns
    /// 
    /// Returns a `ClickUpError::ConfigParseError` containing the original error.
    fn from(err: config::ConfigError) -> Self {
        ClickUpError::ConfigParseError(err)
    }
} 