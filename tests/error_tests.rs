use clickup_cli::error::ClickUpError;
use std::io;

#[test]
fn test_api_error() {
    let error = ClickUpError::ApiError("Test API error".to_string());
    assert_eq!(error.to_string(), "API request failed: Test API error");
}

#[test]
fn test_auth_error() {
    let error = ClickUpError::AuthError("Invalid token".to_string());
    assert_eq!(error.to_string(), "Authentication failed: Invalid token");
}

#[test]
fn test_config_error() {
    let error = ClickUpError::ConfigError("Config file not found".to_string());
    assert_eq!(error.to_string(), "Configuration error: Config file not found");
}

#[test]
fn test_validation_error() {
    let error = ClickUpError::ValidationError("Empty token".to_string());
    assert_eq!(error.to_string(), "Invalid input: Empty token");
}

#[test]
fn test_network_error() {
    let error = ClickUpError::NetworkError("Connection timeout".to_string());
    assert_eq!(error.to_string(), "Network error: Connection timeout");
}

#[test]
fn test_rate_limit_error() {
    let error = ClickUpError::RateLimitError;
    assert_eq!(error.to_string(), "Rate limit exceeded");
}

#[test]
fn test_not_found_error() {
    let error = ClickUpError::NotFoundError("Task not found".to_string());
    assert_eq!(error.to_string(), "Resource not found: Task not found");
}

#[test]
fn test_permission_error() {
    let error = ClickUpError::PermissionError("Access denied".to_string());
    assert_eq!(error.to_string(), "Permission denied: Access denied");
}

#[test]
fn test_serialization_error() {
    let error = ClickUpError::SerializationError("Invalid JSON".to_string());
    assert_eq!(error.to_string(), "Serialization error: Invalid JSON");
}

#[test]
fn test_deserialization_error() {
    let error = ClickUpError::DeserializationError("Malformed JSON".to_string());
    assert_eq!(error.to_string(), "Deserialization error: Malformed JSON");
}

#[test]
fn test_io_error_conversion() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let clickup_error: ClickUpError = io_error.into();
    assert!(matches!(clickup_error, ClickUpError::IoError(_)));
}

#[test]
fn test_json_error_conversion() {
    let json_str = "{ invalid json }";
    let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
    let clickup_error: ClickUpError = json_error.into();
    assert!(matches!(clickup_error, ClickUpError::JsonError(_)));
}

#[test]
fn test_config_parse_error_conversion() {
    let config_error = config::ConfigError::NotFound("Config not found".to_string());
    let clickup_error: ClickUpError = config_error.into();
    assert!(matches!(clickup_error, ClickUpError::ConfigParseError(_)));
}

#[test]
fn test_error_debug() {
    let error = ClickUpError::ApiError("Test error".to_string());
    let debug_str = format!("{error:?}");
    assert!(debug_str.contains("ApiError"));
    assert!(debug_str.contains("Test error"));
}

#[test]
fn test_error_equality() {
    let error1 = ClickUpError::ApiError("Test error".to_string());
    let error2 = ClickUpError::ApiError("Test error".to_string());
    let error3 = ClickUpError::ApiError("Different error".to_string());
    
    assert_eq!(error1.to_string(), error2.to_string());
    assert_ne!(error1.to_string(), error3.to_string());
}

#[test]
fn test_error_context() {
    let error = ClickUpError::ValidationError("Token cannot be empty".to_string());
    let error_str = error.to_string();
    assert!(error_str.contains("Invalid input"));
    assert!(error_str.contains("Token cannot be empty"));
}

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