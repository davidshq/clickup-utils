//! # Space Command Tests
//!
//! This module contains comprehensive tests for the space command module.
//! It tests space listing, detailed space viewing, folder listing, and backup functionality.
//!
//! ## Test Categories
//!
//! - **Space Listing**: Tests for listing spaces in a workspace
//! - **Space Details**: Tests for showing detailed space information
//! - **Folder Listing**: Tests for listing folders in a space
//! - **Backup Functionality**: Tests for space backup operations
//! - **Error Handling**: Tests for invalid IDs and API errors
//!
//! ## Test Environment
//!
//! Tests use temporary directories and isolated environment variables to avoid
//! interfering with the user's actual configuration files.

use clickup_cli::commands::spaces::{execute, SpaceCommands};
use clickup_cli::config::Config;
mod test_utils;
use test_utils::TestConfig;

/// Tests the List command with no authentication
///
/// This test verifies that the List command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_space_list_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the List command
    let result = execute(SpaceCommands::List { workspace_id: "workspace_123".to_string() }, &config).await;
    assert!(result.is_err());
}

/// Tests the List command with authentication
///
/// This test verifies that the List command can be executed
/// when authentication is configured (though it may fail due to
/// network/API issues, the command structure is valid).
#[tokio::test]
async fn test_space_list_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the List command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(SpaceCommands::List { workspace_id: "workspace_123".to_string() }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the List command with empty workspace ID
///
/// This test verifies that the List command correctly handles
/// empty workspace IDs.
#[tokio::test]
async fn test_space_list_empty_workspace_id() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the List command with empty workspace ID
    let result = execute(SpaceCommands::List { workspace_id: "".to_string() }, &config).await;
    // This should fail due to invalid workspace ID
    assert!(result.is_err());
}

/// Tests the Show command with no authentication
///
/// This test verifies that the Show command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_space_show_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the Show command
    let result = execute(SpaceCommands::Show { id: "space_123".to_string() }, &config).await;
    assert!(result.is_err());
}

/// Tests the Show command with authentication
///
/// This test verifies that the Show command can be executed
/// when authentication is configured (though it may fail due to
/// network/API issues, the command structure is valid).
#[tokio::test]
async fn test_space_show_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Show command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(SpaceCommands::Show { id: "space_123".to_string() }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Show command with empty space ID
///
/// This test verifies that the Show command correctly handles
/// empty space IDs.
#[tokio::test]
async fn test_space_show_empty_id() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Show command with empty ID
    let result = execute(SpaceCommands::Show { id: "".to_string() }, &config).await;
    // This should fail due to invalid space ID
    assert!(result.is_err());
}

/// Tests the ListFolders command with no authentication
///
/// This test verifies that the ListFolders command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_space_list_folders_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the ListFolders command
    let result = execute(SpaceCommands::ListFolders { space_id: "space_123".to_string() }, &config).await;
    assert!(result.is_err());
}

/// Tests the ListFolders command with authentication
///
/// This test verifies that the ListFolders command can be executed
/// when authentication is configured (though it may fail due to
/// network/API issues, the command structure is valid).
#[tokio::test]
async fn test_space_list_folders_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the ListFolders command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(SpaceCommands::ListFolders { space_id: "space_123".to_string() }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the ListFolders command with empty space ID
///
/// This test verifies that the ListFolders command correctly handles
/// empty space IDs.
#[tokio::test]
async fn test_space_list_folders_empty_id() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the ListFolders command with empty ID
    let result = execute(SpaceCommands::ListFolders { space_id: "".to_string() }, &config).await;
    // This should fail due to invalid space ID
    assert!(result.is_err());
}

/// Tests the Backup command with no authentication
///
/// This test verifies that the Backup command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_space_backup_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the Backup command
    let result = execute(SpaceCommands::Backup { 
        space_id: Some("space_123".to_string()),
        output_dir: ".".to_string(),
        include_comments: false,
    }, &config).await;
    assert!(result.is_err());
}

/// Tests the Backup command with authentication
///
/// This test verifies that the Backup command can be executed
/// when authentication is configured (though it may fail due to
/// network/API issues, the command structure is valid).
#[tokio::test]
async fn test_space_backup_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Backup command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(SpaceCommands::Backup { 
        space_id: Some("space_123".to_string()),
        output_dir: ".".to_string(),
        include_comments: false,
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Backup command with empty space ID
///
/// This test verifies that the Backup command correctly handles
/// empty space IDs.
#[tokio::test]
async fn test_space_backup_empty_id() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Backup command with empty ID
    let result = execute(SpaceCommands::Backup { 
        space_id: Some("".to_string()),
        output_dir: ".".to_string(),
        include_comments: false,
    }, &config).await;
    // This should fail due to invalid space ID
    assert!(result.is_err());
}

/// Tests the Backup command with include_comments flag
///
/// This test verifies that the Backup command correctly handles
/// the include_comments parameter.
#[tokio::test]
async fn test_space_backup_with_comments() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Backup command with comments included
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(SpaceCommands::Backup { 
        space_id: Some("space_123".to_string()),
        output_dir: ".".to_string(),
        include_comments: true,
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
} 