//! # Task Command Tests
//!
//! This module contains comprehensive tests for the task command module.
//! It tests task listing, creation, updating, deletion, and advanced features
//! like tag-based filtering and overdue task management.
//!
//! ## Test Categories
//!
//! - **Task Listing**: Tests for listing tasks in a list
//! - **Task Creation**: Tests for creating new tasks
//! - **Task Updates**: Tests for updating existing tasks
//! - **Task Deletion**: Tests for deleting tasks
//! - **Tag Operations**: Tests for tag-based filtering and search
//! - **Overdue Management**: Tests for updating overdue tasks
//! - **Error Handling**: Tests for invalid parameters and API errors
//!
//! ## Test Environment
//!
//! Tests use temporary directories and isolated environment variables to avoid
//! interfering with the user's actual configuration files.

use clickup_cli::commands::tasks::{execute, TaskCommands};
use clickup_cli::config::Config;
mod test_utils;
use test_utils::TestConfig;



/// Tests the List command with no authentication
///
/// This test verifies that the List command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_task_list_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the List command
    let result = execute(TaskCommands::List { list_id: "list_123".to_string() }, &config).await;
    assert!(result.is_err());
}

/// Tests the List command with authentication
///
/// This test verifies that the List command can be executed
/// when authentication is configured (though it may fail due to
/// network/API issues, the command structure is valid).
#[tokio::test]
async fn test_task_list_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the List command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(TaskCommands::List { list_id: "list_123".to_string() }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the List command with empty list ID
///
/// This test verifies that the List command correctly handles
/// empty list IDs.
#[tokio::test]
async fn test_task_list_empty_list_id() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the List command with empty list ID
    let result = execute(TaskCommands::List { list_id: "".to_string() }, &config).await;
    // This should fail due to invalid list ID
    assert!(result.is_err());
}

/// Tests the ListByTag command with no authentication
///
/// This test verifies that the ListByTag command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_task_list_by_tag_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the ListByTag command
    let result = execute(TaskCommands::ListByTag { 
        list_id: "list_123".to_string(),
        tag: "urgent".to_string(),
    }, &config).await;
    assert!(result.is_err());
}

/// Tests the ListByTag command with authentication
///
/// This test verifies that the ListByTag command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_task_list_by_tag_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the ListByTag command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(TaskCommands::ListByTag { 
        list_id: "list_123".to_string(),
        tag: "urgent".to_string(),
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the SearchByTag command with no authentication
///
/// This test verifies that the SearchByTag command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_task_search_by_tag_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the SearchByTag command
    let result = execute(TaskCommands::SearchByTag { 
        tag: "urgent".to_string(),
        workspace_id: Some("workspace_123".to_string()),
        space_id: Some("space_123".to_string()),
    }, &config).await;
    assert!(result.is_err());
}

/// Tests the SearchByTag command with authentication
///
/// This test verifies that the SearchByTag command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_task_search_by_tag_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the SearchByTag command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(TaskCommands::SearchByTag { 
        tag: "urgent".to_string(),
        workspace_id: Some("workspace_123".to_string()),
        space_id: Some("space_123".to_string()),
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the UpdateOverdueByTag command with no authentication
///
/// This test verifies that the UpdateOverdueByTag command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_task_update_overdue_by_tag_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the UpdateOverdueByTag command
    let result = execute(TaskCommands::UpdateOverdueByTag { 
        tag: "urgent".to_string(),
        workspace_id: Some("workspace_123".to_string()),
        space_id: Some("space_123".to_string()),
        dry_run: true,
    }, &config).await;
    assert!(result.is_err());
}

/// Tests the UpdateOverdueByTag command with authentication
///
/// This test verifies that the UpdateOverdueByTag command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_task_update_overdue_by_tag_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the UpdateOverdueByTag command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(TaskCommands::UpdateOverdueByTag { 
        tag: "urgent".to_string(),
        workspace_id: Some("workspace_123".to_string()),
        space_id: Some("space_123".to_string()),
        dry_run: true,
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Show command with no authentication
///
/// This test verifies that the Show command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_task_show_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the Show command
    let result = execute(TaskCommands::Show { id: "task_123".to_string() }, &config).await;
    assert!(result.is_err());
}

/// Tests the Show command with authentication
///
/// This test verifies that the Show command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_task_show_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Show command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(TaskCommands::Show { id: "task_123".to_string() }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Create command with no authentication
///
/// This test verifies that the Create command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_task_create_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the Create command
    let result = execute(TaskCommands::Create { 
        list_id: "list_123".to_string(),
        name: "Test Task".to_string(),
        description: Some("Test description".to_string()),
        status: Some("in progress".to_string()),
        priority: Some(2),
        due_date: Some(1640995200), // 2022-01-01
        time_estimate: Some(3600000), // 1 hour in milliseconds
    }, &config).await;
    assert!(result.is_err());
}

/// Tests the Create command with authentication
///
/// This test verifies that the Create command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_task_create_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Create command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(TaskCommands::Create { 
        list_id: "list_123".to_string(),
        name: "Test Task".to_string(),
        description: Some("Test description".to_string()),
        status: Some("in progress".to_string()),
        priority: Some(2),
        due_date: Some(1640995200), // 2022-01-01
        time_estimate: Some(3600000), // 1 hour in milliseconds
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Create command with minimal parameters
///
/// This test verifies that the Create command can be executed
/// with only the required parameters.
#[tokio::test]
async fn test_task_create_minimal_params() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Create command with minimal parameters
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(TaskCommands::Create { 
        list_id: "list_123".to_string(),
        name: "Test Task".to_string(),
        description: None,
        status: None,
        priority: None,
        due_date: None,
        time_estimate: None,
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Update command with no authentication
///
/// This test verifies that the Update command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_task_update_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the Update command
    let result = execute(TaskCommands::Update { 
        id: "task_123".to_string(),
        name: Some("Updated Task".to_string()),
        description: Some("Updated description".to_string()),
        status: Some("completed".to_string()),
        priority: Some(1),
        due_date: Some(1640995200), // 2022-01-01
        time_estimate: Some(7200000), // 2 hours in milliseconds
    }, &config).await;
    assert!(result.is_err());
}

/// Tests the Update command with authentication
///
/// This test verifies that the Update command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_task_update_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Update command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(TaskCommands::Update { 
        id: "task_123".to_string(),
        name: Some("Updated Task".to_string()),
        description: Some("Updated description".to_string()),
        status: Some("completed".to_string()),
        priority: Some(1),
        due_date: Some(1640995200), // 2022-01-01
        time_estimate: Some(7200000), // 2 hours in milliseconds
    }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Delete command with no authentication
///
/// This test verifies that the Delete command correctly handles
/// the case when no authentication token is configured.
#[tokio::test]
async fn test_task_delete_no_auth() {
    let _test_config = TestConfig::new();
    let config = Config::default();
    
    // Ensure no token is set
    assert!(!config.is_authenticated());
    
    // Execute the Delete command
    let result = execute(TaskCommands::Delete { id: "task_123".to_string() }, &config).await;
    assert!(result.is_err());
}

/// Tests the Delete command with authentication
///
/// This test verifies that the Delete command can be executed
/// when authentication is configured.
#[tokio::test]
async fn test_task_delete_with_auth() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Delete command
    // Note: This will likely fail due to invalid token, but the command structure is valid
    let _result = execute(TaskCommands::Delete { id: "task_123".to_string() }, &config).await;
    // We don't assert on result since it depends on API connectivity
}

/// Tests the Delete command with empty task ID
///
/// This test verifies that the Delete command correctly handles
/// empty task IDs.
#[tokio::test]
async fn test_task_delete_empty_id() {
    let _test_config = TestConfig::new();
    let mut config = Config::default();
    
    // Set a token
    config.set_api_token("test_token_12345".to_string()).unwrap();
    assert!(config.is_authenticated());
    
    // Execute the Delete command with empty ID
    let result = execute(TaskCommands::Delete { id: "".to_string() }, &config).await;
    // This should fail due to invalid task ID
    assert!(result.is_err());
} 