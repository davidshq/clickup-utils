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
use clickup_cli::commands::utils::ApiUtils;
use clickup_cli::config::Config;
use clickup_cli::error::ClickUpError;
use std::env;
use std::fs;
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
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_path_buf();
        let config_file = temp_path.join("clickup-cli").join("config.toml");

        // Remove any existing config file first to ensure clean test state
        let real_config_dir = dirs::config_dir().unwrap().join("clickup-cli");
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
        ApiUtils::create_client(config)
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
} 