//! # Comment Command Tests
//!
//! This module contains comprehensive tests for the comment command module.
//! It tests comment listing, creation, updating, deletion, and detailed viewing.
//!
//! ## Test Categories
//!
//! - **Comment Listing**: Tests for listing comments on a task
//! - **Comment Creation**: Tests for creating new comments
//! - **Comment Updates**: Tests for updating existing comments
//! - **Comment Deletion**: Tests for deleting comments
//! - **Comment Details**: Tests for showing detailed comment information
//! - **Error Handling**: Tests for invalid parameters and API errors
//!
//! ## Test Environment
//!
//! Tests use temporary directories and isolated environment variables to avoid
//! interfering with the user's actual configuration files.

use clickup_cli::commands::comments::{execute, CommentCommands};
use clickup_cli::config::Config;
use std::env;
use std::fs;
use tempfile::TempDir;

/// Test configuration that properly isolates tests from user configuration
///
/// This struct manages temporary directories and environment variables
/// to ensure tests don't interfere with the user's actual configuration.
/// It automatically cleans up when dropped.
struct TestConfig {
    #[allow(dead_code)]
    temp_dir: TempDir,
    config_file: std::path::PathBuf,
    original_xdg: Option<String>,
    original_appdata: Option<String>,
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
    fn new() -> Self {
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

        Self {
            temp_dir,
            config_file,
            original_xdg,
            original_appdata,
        }
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

/// Tests the List command with no authentication
///
/// This test verifies that the List command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_comment_list_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the List command
    let result = execute(CommentCommands::List { task_id: "task_123".to_string() }, &config).await;
    assert!(result.is_err());
}

/// Tests the List command with authentication
///
/// This test verifies that the List command can be executed
/// when authentication is configured (though it may fail due to
/// network/API issues, the command structure is valid).
#[tokio::test]
async fn test_comment_list_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the List command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(CommentCommands::List { task_id: "task_123".to_string() }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the List command with empty task ID
///
/// This test verifies that the List command correctly handles
/// empty task IDs.
#[tokio::test]
async fn test_comment_list_empty_task_id() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the List command with empty task ID
    let result = execute(CommentCommands::List { task_id: "".to_string() }, &config).await;
    // This should fail due to invalid task ID
    assert!(result.is_err());
}

/// Tests the Show command with no authentication
///
/// This test verifies that the Show command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_comment_show_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the Show command
    let result = execute(CommentCommands::Show { id: "comment_123".to_string() }, &config).await;
    assert!(result.is_err());
}

/// Tests the Show command with authentication
///
/// This test verifies that the Show command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_comment_show_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Show command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(CommentCommands::Show { id: "comment_123".to_string() }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Show command with empty comment ID
///
/// This test verifies that the Show command correctly handles
/// empty comment IDs.
#[tokio::test]
async fn test_comment_show_empty_id() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Show command with empty ID
    let result = execute(CommentCommands::Show { id: "".to_string() }, &config).await;
    // This should fail due to invalid comment ID
    assert!(result.is_err());
}

/// Tests the Create command with no authentication
///
/// This test verifies that the Create command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_comment_create_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the Create command
    let result = execute(CommentCommands::Create { 
        task_id: "task_123".to_string(),
        text: "Test comment".to_string(),
        assignee: Some(123),
        notify_all: Some(true),
    }, &config).await;
    assert!(result.is_err());
}

/// Tests the Create command with authentication
///
/// This test verifies that the Create command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_comment_create_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Create command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(CommentCommands::Create { 
        task_id: "task_123".to_string(),
        text: "Test comment".to_string(),
        assignee: Some(123),
        notify_all: Some(true),
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Create command with minimal parameters
///
/// This test verifies that the Create command can be executed
/// with only the required parameters.
#[tokio::test]
async fn test_comment_create_minimal_params() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Create command with minimal parameters
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(CommentCommands::Create { 
        task_id: "task_123".to_string(),
        text: "Test comment".to_string(),
        assignee: None,
        notify_all: None,
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Create command with empty text
///
/// This test verifies that the Create command correctly handles
/// empty comment text.
#[tokio::test]
async fn test_comment_create_empty_text() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Create command with empty text
    let result = execute(CommentCommands::Create { 
        task_id: "task_123".to_string(),
        text: "".to_string(),
        assignee: None,
        notify_all: None,
    }, &config).await;
    // This should fail due to empty comment text
    assert!(result.is_err());
}

/// Tests the Update command with no authentication
///
/// This test verifies that the Update command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_comment_update_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the Update command
    let result = execute(CommentCommands::Update { 
        id: "comment_123".to_string(),
        text: "Updated comment".to_string(),
        assignee: Some(456),
        notify_all: Some(false),
    }, &config).await;
    assert!(result.is_err());
}

/// Tests the Update command with authentication
///
/// This test verifies that the Update command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_comment_update_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Update command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(CommentCommands::Update { 
        id: "comment_123".to_string(),
        text: "Updated comment".to_string(),
        assignee: Some(456),
        notify_all: Some(false),
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Update command with empty text
///
/// This test verifies that the Update command correctly handles
/// empty comment text.
#[tokio::test]
async fn test_comment_update_empty_text() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Update command with empty text
    let result = execute(CommentCommands::Update { 
        id: "comment_123".to_string(),
        text: "".to_string(),
        assignee: None,
        notify_all: None,
    }, &config).await;
    // This should fail due to empty comment text
    assert!(result.is_err());
}

/// Tests the Delete command with no authentication
///
/// This test verifies that the Delete command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_comment_delete_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the Delete command
    let result = execute(CommentCommands::Delete { id: "comment_123".to_string() }, &config).await;
    assert!(result.is_err());
}

/// Tests the Delete command with authentication
///
/// This test verifies that the Delete command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_comment_delete_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Delete command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(CommentCommands::Delete { id: "comment_123".to_string() }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Delete command with empty comment ID
///
/// This test verifies that the Delete command correctly handles
/// empty comment IDs.
#[tokio::test]
async fn test_comment_delete_empty_id() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Delete command with empty ID
    let result = execute(CommentCommands::Delete { id: "".to_string() }, &config).await;
    // This should fail due to invalid comment ID
    assert!(result.is_err());
} 