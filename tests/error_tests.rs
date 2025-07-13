//! # Error Handling Tests
//!
//! This module contains comprehensive tests for the error handling system.
//! It tests all error types, their string representations, conversions,
//! and various error scenarios to ensure robust error handling.
//!
//! ## Test Categories
//!
//! - **Error Types**: Tests for all ClickUpError variants
//! - **String Representations**: Tests for error message formatting
//! - **Error Conversions**: Tests for automatic conversion from external error types
//! - **Error Context**: Tests for error context and debugging information
//!
//! ## Error Coverage
//!
//! Tests cover all error variants including API errors, authentication errors,
//! configuration errors, validation errors, network errors, and more.

use clickup_cli::error::ClickUpError;
use std::io;

/// Tests API error creation and string representation
///
/// This test verifies that API errors are created correctly and
/// have the expected string representation.
#[test]
fn test_api_error() {
    let error = ClickUpError::ApiError("Test API error".to_string());
    assert_eq!(error.to_string(), "API request failed: Test API error");
}

/// Tests authentication error creation and string representation
///
/// This test verifies that authentication errors are created correctly and
/// have the expected string representation.
#[test]
fn test_auth_error() {
    let error = ClickUpError::AuthError("Invalid token".to_string());
    assert_eq!(error.to_string(), "Authentication failed: Invalid token");
}

/// Tests configuration error creation and string representation
///
/// This test verifies that configuration errors are created correctly and
/// have the expected string representation.
#[test]
fn test_config_error() {
    let error = ClickUpError::ConfigError("Config file not found".to_string());
    assert_eq!(
        error.to_string(),
        "Configuration error: Config file not found"
    );
}

/// Tests validation error creation and string representation
///
/// This test verifies that validation errors are created correctly and
/// have the expected string representation.
#[test]
fn test_validation_error() {
    let error = ClickUpError::ValidationError("Empty token".to_string());
    assert_eq!(error.to_string(), "Invalid input: Empty token");
}

/// Tests network error creation and string representation
///
/// This test verifies that network errors are created correctly and
/// have the expected string representation.
#[test]
fn test_network_error() {
    let error = ClickUpError::NetworkError("Connection timeout".to_string());
    assert_eq!(error.to_string(), "Network error: Connection timeout");
}

/// Tests rate limit error creation and string representation
///
/// This test verifies that rate limit errors are created correctly and
/// have the expected string representation.
#[test]
fn test_rate_limit_error() {
    let error = ClickUpError::RateLimitError;
    assert_eq!(error.to_string(), "Rate limit exceeded");
}

/// Tests not found error creation and string representation
///
/// This test verifies that not found errors are created correctly and
/// have the expected string representation.
#[test]
fn test_not_found_error() {
    let error = ClickUpError::NotFoundError("Task not found".to_string());
    assert_eq!(error.to_string(), "Resource not found: Task not found");
}

/// Tests permission error creation and string representation
///
/// This test verifies that permission errors are created correctly and
/// have the expected string representation.
#[test]
fn test_permission_error() {
    let error = ClickUpError::PermissionError("Access denied".to_string());
    assert_eq!(error.to_string(), "Permission denied: Access denied");
}

/// Tests serialization error creation and string representation
///
/// This test verifies that serialization errors are created correctly and
/// have the expected string representation.
#[test]
fn test_serialization_error() {
    let error = ClickUpError::SerializationError("Invalid JSON".to_string());
    assert_eq!(error.to_string(), "Serialization error: Invalid JSON");
}

/// Tests deserialization error creation and string representation
///
/// This test verifies that deserialization errors are created correctly and
/// have the expected string representation.
#[test]
fn test_deserialization_error() {
    let error = ClickUpError::DeserializationError("Malformed JSON".to_string());
    assert_eq!(error.to_string(), "Deserialization error: Malformed JSON");
}

/// Tests automatic conversion from IO errors
///
/// This test verifies that IO errors are automatically converted
/// to ClickUpError::IoError when using the `?` operator.
#[test]
fn test_io_error_conversion() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let clickup_error: ClickUpError = io_error.into();
    assert!(matches!(clickup_error, ClickUpError::IoError(_)));
}

/// Tests automatic conversion from JSON errors
///
/// This test verifies that JSON parsing errors are automatically converted
/// to ClickUpError::JsonError when using the `?` operator.
#[test]
fn test_json_error_conversion() {
    let json_str = "{ invalid json }";
    let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
    let clickup_error: ClickUpError = json_error.into();
    assert!(matches!(clickup_error, ClickUpError::JsonError(_)));
}

/// Tests automatic conversion from configuration parsing errors
///
/// This test verifies that configuration parsing errors are automatically converted
/// to ClickUpError::ConfigParseError when using the `?` operator.
#[test]
fn test_config_parse_error_conversion() {
    let config_error = config::ConfigError::NotFound("Config not found".to_string());
    let clickup_error: ClickUpError = config_error.into();
    assert!(matches!(clickup_error, ClickUpError::ConfigParseError(_)));
}

/// Tests error debug formatting
///
/// This test verifies that errors can be formatted for debugging
/// and contain the expected information.
#[test]
fn test_error_debug() {
    let error = ClickUpError::ApiError("Test error".to_string());
    let debug_str = format!("{error:?}");
    assert!(debug_str.contains("ApiError"));
    assert!(debug_str.contains("Test error"));
}

/// Tests error equality and string comparison
///
/// This test verifies that errors with the same content have equal
/// string representations and different errors have different representations.
#[test]
fn test_error_equality() {
    let error1 = ClickUpError::ApiError("Test error".to_string());
    let error2 = ClickUpError::ApiError("Test error".to_string());
    let error3 = ClickUpError::ApiError("Different error".to_string());

    assert_eq!(error1.to_string(), error2.to_string());
    assert_ne!(error1.to_string(), error3.to_string());
}

/// Tests error context and message content
///
/// This test verifies that error messages contain the expected
/// context information and error details.
#[test]
fn test_error_context() {
    let error = ClickUpError::ValidationError("Token cannot be empty".to_string());
    let error_str = error.to_string();
    assert!(error_str.contains("Invalid input"));
    assert!(error_str.contains("Token cannot be empty"));
}

/// Tests all error types for completeness
///
/// This test verifies that all error variants can be created and
/// have non-empty string representations.
#[test]
fn test_multiple_error_types() {
    let errors = vec![
        ClickUpError::ApiError("API error".to_string()),
        ClickUpError::AuthError("Auth error".to_string()),
        ClickUpError::ConfigError("Config error".to_string()),
        ClickUpError::ValidationError("Validation error".to_string()),
        ClickUpError::NetworkError("Network error".to_string()),
        ClickUpError::RateLimitError,
        ClickUpError::NotFoundError("Not found".to_string()),
        ClickUpError::PermissionError("Permission denied".to_string()),
        ClickUpError::SerializationError("Serialization error".to_string()),
        ClickUpError::DeserializationError("Deserialization error".to_string()),
    ];

    for error in errors {
        let error_str = error.to_string();
        assert!(!error_str.is_empty());
        assert!(!error_str.is_empty());
    }
}
