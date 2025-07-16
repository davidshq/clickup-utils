//! # Test Utilities
//!
//! This module contains shared test utilities and patterns used across all test files
//! to reduce code duplication and improve maintainability.
//!
//! ## Overview
//!
//! The utilities in this module provide standardized patterns for:
//! - Test configuration setup and teardown (`TestConfig`)
//! - API client creation for tests (`TestApiUtils`)
//! - Common test assertions and helpers
//! - Standardized error handling patterns
//!
//! ## Usage
//!
//! ```rust
//! use clickup_cli::tests::test_utils::{TestConfig, TestApiUtils};
//! use clickup_cli::config::Config;
//!
//! fn example() {
//!     let _test_config = TestConfig::new();
//!     let config = Config::default();
//!     let api = TestApiUtils::create_test_client(&config).unwrap();
//!     // ... test logic
//! }
//! ```

use clickup_cli::api::ClickUpApi;
use clickup_cli::config::Config;
use clickup_cli::error::ClickUpError;

use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test configuration for isolated test environments
///
/// This struct provides a standardized way to set up isolated test environments
/// with temporary configuration directories and proper cleanup.
///
/// ## Features
///
/// - **Isolated Environment**: Uses temporary directories for config files
/// - **Automatic Cleanup**: Implements Drop trait for proper resource cleanup
/// - **Environment Isolation**: Modifies environment variables for test isolation
/// - **Cross-Platform**: Works on both Unix and Windows systems
///
/// ## Usage Pattern
///
/// ```rust
/// fn example() {
///     let _test_config = TestConfig::new();
///     // Test logic here - config will be cleaned up automatically
/// }
/// ```
pub struct TestConfig {
    #[allow(dead_code)]
    temp_dir: TempDir,
    pub config_file: std::path::PathBuf,
    original_xdg: Option<String>,
    original_appdata: Option<String>,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TestConfig {
    /// Creates a new test configuration with isolated environment
    ///
    /// This function sets up a temporary directory and modifies environment
    /// variables to ensure tests use isolated configuration files.
    ///
    /// # Returns
    ///
    /// Returns a `TestConfig` instance that will clean up automatically.
    pub fn new() -> Self {
        let temp_dir = TestErrorHandler::create_temp_dir();
        let temp_path = temp_dir.path().to_path_buf();
        let config_file = temp_path.join("clickup-cli").join("config.toml");

        // Remove any existing config file first to ensure clean test state
        let real_config_dir = TestErrorHandler::get_config_dir();
        let real_config_file = real_config_dir.join("config.toml");
        let _ = fs::remove_file(&real_config_file);

        // Store original environment variables
        let original_xdg = env::var("XDG_CONFIG_HOME").ok();
        let original_appdata = env::var("APPDATA").ok();

        // Set environment variables to use temp directory
        env::set_var("XDG_CONFIG_HOME", &temp_path);
        env::set_var("APPDATA", &temp_path);

        // Create the config directory in temp
        let temp_config_dir = temp_path.join("clickup-cli");
        fs::create_dir_all(&temp_config_dir).expect("Failed to create config directory");

        // Create an empty config file to ensure it exists
        fs::write(&config_file, "").expect("Failed to create empty config file");

        Self {
            temp_dir,
            config_file,
            original_xdg,
            original_appdata,
        }
    }

    /// Get the path to the temporary config file
    pub fn config_file(&self) -> &std::path::Path {
        &self.config_file
    }
}

impl Drop for TestConfig {
    fn drop(&mut self) {
        // Restore original environment variables
        if let Some(xdg) = &self.original_xdg {
            env::set_var("XDG_CONFIG_HOME", xdg);
        } else {
            env::remove_var("XDG_CONFIG_HOME");
        }

        if let Some(appdata) = &self.original_appdata {
            env::set_var("APPDATA", appdata);
        } else {
            env::remove_var("APPDATA");
        }

        // Clean up temp directory (this happens automatically when temp_dir is dropped)
    }
}

/// Test error handling utilities for consistent error handling in tests
///
/// This struct provides standardized methods for handling errors in test environments,
/// ensuring consistent error handling patterns and avoiding `unwrap()` calls that
/// violate ADR 0012 (Code Quality Standards).
///
/// ## Features
///
/// - **Safe Error Handling**: Replaces `unwrap()` calls with proper error handling
/// - **Consistent Patterns**: Standardized error handling across all tests
/// - **ADR 0012 Compliance**: Aligns with zero clippy warnings policy
/// - **Test-Specific**: Optimized for test environments with clear error messages
///
/// ## Usage
///
/// ```rust
/// use clickup_cli::tests::test_utils::TestErrorHandler;
/// use clickup_cli::error::ClickUpError;
///
/// fn example() {
///     let result: Result<String, ClickUpError> = Ok("success".to_string());
///     let value = TestErrorHandler::expect_success(result, "test operation");
///     
///     let error_result: Result<String, ClickUpError> = Err(ClickUpError::AuthError("test".to_string()));
///     TestErrorHandler::expect_error(&error_result, "Authentication failed");
/// }
/// ```
pub struct TestErrorHandler;

impl TestErrorHandler {
    /// Expect a successful result or panic with a descriptive message
    ///
    /// This method replaces `unwrap()` calls in tests with proper error handling
    /// that provides clear context about what operation failed.
    ///
    /// # Arguments
    ///
    /// * `result` - The result to check
    /// * `context` - Description of the operation being tested
    ///
    /// # Returns
    ///
    /// Returns the success value, or panics with a descriptive error message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::TestErrorHandler;
    /// use clickup_cli::error::ClickUpError;
    ///
    /// fn example_success() {
    ///     let result: Result<String, ClickUpError> = Ok("test".to_string());
    ///     let value = TestErrorHandler::expect_success(result, "string creation");
    ///     assert_eq!(value, "test");
    /// }
    /// ```
    pub fn expect_success<T>(result: Result<T, ClickUpError>, context: &str) -> T {
        result.unwrap_or_else(|e| panic!("{} failed: {}", context, e))
    }
    
    /// Expect an error result or panic if successful
    ///
    /// This method provides a standardized way to check for expected errors
    /// in tests, ensuring consistent error validation patterns.
    ///
    /// # Arguments
    ///
    /// * `result` - The result to check
    /// * `expected_error` - The expected error message (partial match)
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::TestErrorHandler;
    /// use clickup_cli::error::ClickUpError;
    ///
    /// fn example_error() {
    ///     let result: Result<String, ClickUpError> = Err(ClickUpError::AuthError("Invalid token".to_string()));
    ///     TestErrorHandler::expect_error(&result, "Authentication failed");
    /// }
    /// ```
    pub fn expect_error<T>(result: &Result<T, ClickUpError>, expected_error: &str) {
        match result {
            Ok(_) => panic!("Expected error but got success"),
            Err(e) => assert!(e.to_string().contains(expected_error), 
                "Expected error containing '{}', but got: {}", expected_error, e),
        }
    }
    
    /// Create a temporary directory for testing
    ///
    /// This method provides a safe way to create temporary directories
    /// for testing, with proper error handling that aligns with ADR 0012.
    ///
    /// # Returns
    ///
    /// Returns a `TempDir` instance, or panics with a descriptive error message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::TestErrorHandler;
    ///
    /// fn example_temp_dir() {
    ///     let temp_dir = TestErrorHandler::create_temp_dir();
    ///     // Use temp_dir for testing...
    /// }
    /// ```
    pub fn create_temp_dir() -> TempDir {
        TempDir::new().expect("Failed to create temp directory")
    }
    
    /// Get the configuration directory path
    ///
    /// This method provides a safe way to get the configuration directory
    /// path, with proper error handling that aligns with ADR 0012.
    ///
    /// # Returns
    ///
    /// Returns the configuration directory path, or panics with a descriptive error message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::TestErrorHandler;
    ///
    /// fn example_config_dir() {
    ///     let config_dir = TestErrorHandler::get_config_dir();
    ///     // Use config_dir for testing...
    /// }
    /// ```
    pub fn get_config_dir() -> PathBuf {
        dirs::config_dir()
            .expect("Failed to get config directory")
            .join("clickup-cli")
    }
    
    /// Execute a command and expect success
    ///
    /// This method provides a safe way to execute commands in tests,
    /// with proper error handling that aligns with ADR 0012.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The command to execute
    /// * `context` - Description of the command being executed
    ///
    /// # Returns
    ///
    /// Returns the command output, or panics with a descriptive error message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::TestErrorHandler;
    /// use std::process::Command;
    ///
    /// fn example_command() {
    ///     let mut cmd = Command::new("echo");
    ///     cmd.arg("hello");
    ///     let output = TestErrorHandler::expect_command_success(cmd, "echo command");
    ///     assert!(output.status.success());
    /// }
    /// ```
    pub fn expect_command_success(mut cmd: std::process::Command, context: &str) -> std::process::Output {
        cmd.output().expect(&format!("{} failed to execute", context))
    }
    
    /// Set an API token safely in tests
    ///
    /// This method provides a safe way to set API tokens in test configurations,
    /// with proper error handling that aligns with ADR 0012.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to modify
    /// * `token` - The token to set
    /// * `context` - Description of the operation
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::TestErrorHandler;
    /// use clickup_cli::config::Config;
    ///
    /// fn example_set_token() {
    ///     let mut config = Config::default();
    ///     TestErrorHandler::set_api_token(&mut config, "test_token_12345", "test configuration");
    ///     assert!(config.is_authenticated());
    /// }
    /// ```
    pub fn set_api_token(config: &mut Config, token: &str, context: &str) {
        config.set_api_token(token.to_string())
            .expect(&format!("{} failed to set API token", context));
    }
}

/// Test API utilities for consistent API client creation in tests
///
/// This struct provides standardized methods for creating API clients
/// in test environments, ensuring consistent behavior across all tests.
///
/// ## Features
///
/// - **Standardized Creation**: Uses `ApiUtils::create_client()` for consistency
/// - **Error Handling**: Proper error propagation and handling
/// - **Test-Specific**: Optimized for test environments
/// - **Reusable**: Same pattern used across all test files
pub struct TestApiUtils;

impl TestApiUtils {
    /// Create a test API client using the standard ApiUtils pattern
    ///
    /// This method uses the same `ApiUtils::create_client()` pattern that
    /// is used throughout the application, ensuring consistency between
    /// tests and production code.
    ///
    /// # Arguments
    ///
    /// * `config` - Reference to the configuration to use for client creation
    ///
    /// # Returns
    ///
    /// Returns `Ok(ClickUpApi)` on successful creation, or a `ClickUpError` on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::{TestConfig, TestApiUtils};
    /// use clickup_cli::config::Config;
    ///
    /// fn example_api_client() {
    ///     let _test_config = TestConfig::new();
    ///     let config = Config::default();
    ///     let api = TestApiUtils::create_test_client(&config).unwrap();
    ///     // Use api for testing...
    /// }
    /// ```
    pub fn create_test_client(config: &Config) -> Result<ClickUpApi, ClickUpError> {
        ClickUpApi::new(config.clone())
    }

    /// Create a test API client with a custom token
    ///
    /// This method creates an API client with a specific token for testing
    /// authentication scenarios.
    ///
    /// # Arguments
    ///
    /// * `token` - The API token to use for authentication
    ///
    /// # Returns
    ///
    /// Returns `Ok(ClickUpApi)` on successful creation, or a `ClickUpError` on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::{TestConfig, TestApiUtils};
    ///
    /// fn example_with_token() {
    ///     let _test_config = TestConfig::new();
    ///     let api = TestApiUtils::create_test_client_with_token("test_token_123").unwrap();
    ///     // Test authenticated operations...
    /// }
    /// ```
    pub fn create_test_client_with_token(token: &str) -> Result<ClickUpApi, ClickUpError> {
        let mut config = Config::default();
        config.set_api_token(token.to_string())?;
        Self::create_test_client(&config)
    }

    /// Create a test API client without authentication
    ///
    /// This method creates an API client without any authentication token
    /// for testing unauthenticated operations.
    ///
    /// # Returns
    ///
    /// Returns `Ok(ClickUpApi)` on successful creation, or a `ClickUpError` on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::{TestConfig, TestApiUtils};
    ///
    /// fn example_without_auth() {
    ///     let _test_config = TestConfig::new();
    ///     let api = TestApiUtils::create_test_client_without_auth().unwrap();
    ///     // Test unauthenticated operations...
    /// }
    /// ```
    pub fn create_test_client_without_auth() -> Result<ClickUpApi, ClickUpError> {
        let config = Config::default();
        Self::create_test_client(&config)
    }
}

/// Common test assertions and helpers
///
/// This struct provides standardized test assertions and helper methods
/// to ensure consistent test behavior across all test files.
pub struct TestAssertions;

impl TestAssertions {
    /// Assert that a result is an authentication error
    ///
    /// This method provides a standardized way to check for authentication
    /// errors in tests, ensuring consistent error handling validation.
    ///
    /// # Arguments
    ///
    /// * `result` - The result to check
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::TestAssertions;
    ///
    /// fn example_auth_error() {
    ///     let result: Result<(), ClickUpError> = Err(ClickUpError::AuthError("Invalid token".to_string()));
    ///     TestAssertions::assert_auth_error(&result);
    /// }
    /// ```
    pub fn assert_auth_error<T>(result: &Result<T, ClickUpError>) {
        match result {
            Err(ClickUpError::AuthError(_)) => {}
            _ => panic!("Expected authentication error"),
        }
    }

    /// Assert that a result is a validation error
    ///
    /// This method provides a standardized way to check for validation
    /// errors in tests, ensuring consistent error handling validation.
    ///
    /// # Arguments
    ///
    /// * `result` - The result to check
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::TestAssertions;
    ///
    /// fn example_validation_error() {
    ///     let result: Result<(), ClickUpError> = Err(ClickUpError::ValidationError("Invalid input".to_string()));
    ///     TestAssertions::assert_validation_error(&result);
    /// }
    /// ```
    pub fn assert_validation_error<T>(result: &Result<T, ClickUpError>) {
        match result {
            Err(ClickUpError::ValidationError(_)) => {}
            _ => panic!("Expected validation error"),
        }
    }

    /// Assert that a result is a not found error
    ///
    /// This method provides a standardized way to check for not found
    /// errors in tests, ensuring consistent error handling validation.
    ///
    /// # Arguments
    ///
    /// * `result` - The result to check
    ///
    /// # Example
    ///
    /// ```rust
    /// use clickup_cli::tests::test_utils::TestAssertions;
    ///
    /// fn example_not_found_error() {
    ///     let result: Result<(), ClickUpError> = Err(ClickUpError::NotFoundError("Item not found".to_string()));
    ///     TestAssertions::assert_not_found_error(&result);
    /// }
    /// ```
    pub fn assert_not_found_error<T>(result: &Result<T, ClickUpError>) {
        match result {
            Err(ClickUpError::NotFoundError(_)) => {}
            _ => panic!("Expected not found error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_config_creation() {
        let test_config = TestConfig::new();
        assert!(test_config.config_file().exists() || test_config.config_file().parent().unwrap().exists());
    }

    #[test]
    fn test_api_utils_create_client() {
        let _test_config = TestConfig::new();
        let config = Config::default();
        let result = TestApiUtils::create_test_client(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_api_utils_create_client_with_token() {
        let _test_config = TestConfig::new();
        let result = TestApiUtils::create_test_client_with_token("test_token_123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_api_utils_create_client_without_auth() {
        let _test_config = TestConfig::new();
        let result = TestApiUtils::create_test_client_without_auth();
        assert!(result.is_ok());
    }

    #[test]
    fn test_assertions_auth_error() {
        let result: Result<(), ClickUpError> = Err(ClickUpError::AuthError("Invalid token".to_string()));
        TestAssertions::assert_auth_error(&result);
    }

    #[test]
    fn test_assertions_validation_error() {
        let result: Result<(), ClickUpError> = Err(ClickUpError::ValidationError("Invalid input".to_string()));
        TestAssertions::assert_validation_error(&result);
    }

    #[test]
    fn test_assertions_not_found_error() {
        let result: Result<(), ClickUpError> = Err(ClickUpError::NotFoundError("Item not found".to_string()));
        TestAssertions::assert_not_found_error(&result);
    }

    // TestErrorHandler tests
    #[test]
    fn test_error_handler_expect_success() {
        let result: Result<String, ClickUpError> = Ok("test".to_string());
        let value = TestErrorHandler::expect_success(result, "string creation");
        assert_eq!(value, "test");
    }

    #[test]
    fn test_error_handler_expect_error() {
        let result: Result<String, ClickUpError> = Err(ClickUpError::AuthError("Invalid token".to_string()));
        TestErrorHandler::expect_error(&result, "Authentication failed");
    }

    #[test]
    fn test_error_handler_create_temp_dir() {
        let temp_dir = TestErrorHandler::create_temp_dir();
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn test_error_handler_get_config_dir() {
        let config_dir = TestErrorHandler::get_config_dir();
        assert!(config_dir.to_string_lossy().contains("clickup-cli"));
    }

    #[test]
    fn test_error_handler_set_api_token() {
        let mut config = Config::default();
        TestErrorHandler::set_api_token(&mut config, "test_token_12345", "test configuration");
        assert!(config.is_authenticated());
    }

    #[test]
    #[should_panic(expected = "string creation failed")]
    fn test_error_handler_expect_success_panic() {
        let result: Result<String, ClickUpError> = Err(ClickUpError::AuthError("Invalid token".to_string()));
        let _value = TestErrorHandler::expect_success(result, "string creation");
    }

    #[test]
    #[should_panic(expected = "Expected error but got success")]
    fn test_error_handler_expect_error_panic() {
        let result: Result<String, ClickUpError> = Ok("test".to_string());
        TestErrorHandler::expect_error(&result, "Authentication failed");
    }
} 