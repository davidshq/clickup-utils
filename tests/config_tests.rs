//! # Configuration Tests
//!
//! This module contains comprehensive tests for the configuration management system.
//! It tests configuration loading, saving, validation, and various edge cases
//! to ensure robust configuration handling.
//!
//! ## Test Categories
//!
//! - **Default Configuration**: Tests for default configuration values
//! - **Token Management**: Tests for API token setting, getting, and validation
//! - **File Operations**: Tests for configuration file saving and loading
//! - **Validation**: Tests for input validation and error handling
//!
//! ## Test Environment
//!
//! Tests use temporary directories and isolated environment variables to avoid
//! interfering with the user's actual configuration files.

use clickup_cli::config::Config;
mod test_utils;
use test_utils::TestConfig;



/// Tests default configuration values
///
/// This test verifies that the default configuration has the expected
/// values for all fields, including the API base URL and empty optional fields.
#[test]
fn test_config_default() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    assert_eq!(config.api_base_url, "https://api.clickup.com/api/v2");
    assert!(config.api_token.is_none());
    assert!(config.workspace_id.is_none());
    assert!(config.default_list_id.is_none());
}

/// Tests setting and getting API tokens
///
/// This test verifies that API tokens can be set, saved to a file,
/// loaded from the file, and retrieved correctly.
#[test]
fn test_config_set_and_get_api_token() {
    let test_config = TestConfig::new();
    let mut config = Config { api_token: Some("test_token_456".to_string()), workspace_id: Some("workspace_123".to_string()), default_list_id: Some("list_456".to_string()), api_base_url: "https://test.api.clickup.com/api/v2".to_string(), ..Default::default() };
    let test_token = "test_token_123".to_string();
    let _result = config.set_api_token(test_token.clone());
    // Save to temp config file
    let save_result = config.save_with_path(Some(&test_config.config_file));
    assert!(save_result.is_ok());
    let loaded = Config::load_with_path(Some(&test_config.config_file)).unwrap();
    let retrieved_token = loaded.get_api_token().unwrap();
    assert_eq!(retrieved_token, test_token);
}

/// Tests getting API token when none is set
///
/// This test verifies that attempting to get an API token when none
/// is configured returns an appropriate authentication error.
#[test]
fn test_config_get_api_token_when_none() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    let result = config.get_api_token();
    assert!(result.is_err());
    match result {
        Err(clickup_cli::error::ClickUpError::AuthError(_)) => {}
        _ => panic!("Expected AuthError when no token is set"),
    }
}

/// Tests authentication status checking
///
/// This test verifies that the authentication status is correctly
/// determined based on whether an API token is set.
#[test]
fn test_config_is_authenticated() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    assert!(!config.is_authenticated());
    // Set the token directly without saving to avoid file system issues in tests
    config.api_token = Some("test_token".to_string());
    assert!(config.is_authenticated());
}

/// Tests configuration saving and loading
///
/// This test verifies that a complete configuration can be saved to a file
/// and then loaded back with all fields preserved correctly.
#[test]
fn test_config_save_and_load() {
    let test_config = TestConfig::new();
    // Create a config with all fields set
    let config = Config {
        api_token: Some("test_token_456".to_string()),
        workspace_id: Some("workspace_123".to_string()),
        default_list_id: Some("list_456".to_string()),
        api_base_url: "https://test.api.clickup.com/api/v2".to_string(),
        ..Default::default()
    };
    // Save the config to the temp file
    let save_result = config.save_with_path(Some(&test_config.config_file));
    assert!(
        save_result.is_ok(),
        "Failed to save config: {save_result:?}"
    );
    // Load the config and verify all fields
    let loaded_config = Config::load_with_path(Some(&test_config.config_file));
    assert!(
        loaded_config.is_ok(),
        "Failed to load config: {loaded_config:?}"
    );
    let loaded_config = loaded_config.unwrap();
    assert_eq!(loaded_config.api_token, config.api_token);
    assert_eq!(loaded_config.workspace_id, config.workspace_id);
    assert_eq!(loaded_config.default_list_id, config.default_list_id);
    assert_eq!(loaded_config.api_base_url, config.api_base_url);
}

/// Tests validation of empty API tokens
///
/// This test verifies that setting an empty string as an API token
/// is rejected with an appropriate validation error.
#[test]
fn test_config_validation_empty_token() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    let result = config.set_api_token("".to_string());
    assert!(result.is_err());
    match result {
        Err(clickup_cli::error::ClickUpError::ValidationError(_)) => {}
        _ => panic!("Expected ValidationError for empty token"),
    }
}

/// Tests validation of whitespace-only API tokens
///
/// This test verifies that setting a string containing only whitespace
/// as an API token is rejected with an appropriate validation error.
#[test]
fn test_config_validation_whitespace_token() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    let result = config.set_api_token("   ".to_string());
    assert!(result.is_err());
    match result {
        Err(clickup_cli::error::ClickUpError::ValidationError(_)) => {}
        _ => panic!("Expected ValidationError for whitespace-only token"),
    }
}
