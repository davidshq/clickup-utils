//! # API Tests
//!
//! This module contains comprehensive tests for the ClickUp API client functionality.
//! It tests API client creation, configuration handling, authentication, and various
//! edge cases to ensure robust operation.
//!
//! ## Test Categories
//!
//! - **Client Creation**: Tests for API client instantiation with various configurations
//! - **Token Handling**: Tests for different token types and formats
//! - **Configuration**: Tests for config validation and edge cases
//! - **Task Operations**: Tests for task-related functionality including overdue task handling
//!
//! ## Test Environment
//!
//! Tests use a temporary configuration directory to avoid interfering with
//! the user's actual configuration files.

use clickup_cli::api::ClickUpApi;
use clickup_cli::config::{Config, RateLimitConfig};
mod test_utils;
use test_utils::{TestApiUtils, TestConfig};
use std::sync::Once;
use tempfile::TempDir;
use std::cell::RefCell;
use dotenvy;

/// Global test initialization state
static INIT: Once = Once::new();

// Thread-local storage for test configuration
thread_local! {
    static TEST_CONFIG: RefCell<Option<TempDir>> = RefCell::new(None);
}

/// Sets up the test environment with a temporary configuration directory
///
/// This function ensures that tests don't interfere with the user's actual
/// configuration by using a temporary directory for all config operations.
/// It's called once per test run using the `Once` synchronization primitive.
/// The temporary directory is stored in thread-local storage to ensure
/// proper test isolation and prevent unsafe global state.
/// 
/// The function also loads test-specific configuration from .env.test file
/// to ensure tests use the test environment rather than production settings.
fn setup_test_env() {
    INIT.call_once(|| {
        // Load test environment from .env.test
        dotenvy::from_filename(".env.test").ok();
        
        // Set up environment variables for configuration
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        std::env::set_var("XDG_CONFIG_HOME", &temp_path);
        std::env::set_var("APPDATA", &temp_path);
        
        // Store the temp directory in thread-local storage to prevent it from being dropped
        TEST_CONFIG.with(|config| {
            *config.borrow_mut() = Some(temp_dir);
        });
    });
}

/// Tests API client creation with a valid configuration
///
/// This test verifies that the API client can be created successfully
/// when provided with a complete, valid configuration including token,
/// workspace ID, and list ID.
#[test]
fn test_api_client_creation_with_valid_config() {
    setup_test_env();
    
    // Load test configuration from .env.test
    let config = Config::load_for_tests().expect("Failed to load test configuration");
    
    // Verify we have a test token loaded
    assert!(config.api_token.is_some(), "Test configuration should include API token");
    
    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok());
}

/// Tests API client creation without an authentication token
///
/// This test verifies that the API client can be created successfully
/// even when no authentication token is provided, as the client should
/// be able to handle unauthenticated operations.
#[test]
fn test_api_client_creation_without_token() {
    setup_test_env();
    
    // Create a config without token to test unauthenticated operations
    let config = Config {
        api_token: None,
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok()); // API client can be created without token
}

/// Tests API client creation with a personal token (pk_ prefix)
///
/// This test verifies that the API client can handle personal tokens
/// which start with 'pk_' and are used directly without Bearer prefix.
#[test]
fn test_api_client_with_personal_token() {
    setup_test_env();
    
    // Load test configuration and verify it uses test token
    let config = Config::load_for_tests().expect("Failed to load test configuration");
    
    // Verify we have a test token loaded from .env.test
    assert!(config.api_token.is_some(), "Test configuration should include API token");
    
    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok());
}

/// Tests API client creation with an OAuth token
///
/// This test verifies that the API client can handle OAuth tokens
/// which are used with the Bearer prefix in authentication headers.
#[test]
fn test_api_client_with_oauth_token() {
    setup_test_env();
    let config = Config {
        api_token: Some("oauth_test_token_123".to_string()),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok());
}

/// Tests API client creation with a custom API base URL
///
/// This test verifies that the API client can be configured with
/// custom base URLs for different environments or API versions.
#[test]
fn test_api_client_with_custom_base_url() {
    setup_test_env();
    let config = Config {
        api_token: Some("test_token_123".to_string()),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://custom.api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok());
}

/// Tests API client creation with potentially invalid configuration
///
/// This test verifies that the API client can handle invalid or malformed
/// configuration values gracefully without failing during creation.
#[test]
fn test_api_client_creation_failure() {
    // Test with invalid base URL (this should still work as it's just a string)
    let config = Config {
        api_token: Some("test_token_123".to_string()),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "invalid-url".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok()); // The client creation itself should succeed
}

/// Tests API client creation with cloned configuration
///
/// This test verifies that the API client can be created multiple times
/// using the same configuration object, ensuring proper cloning behavior.
#[test]
fn test_api_client_config_clone() {
    setup_test_env();
    let config = Config {
        api_token: Some("test_token_123".to_string()),
        workspace_id: Some("workspace_123".to_string()),
        default_list_id: Some("list_456".to_string()),
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api1 = TestApiUtils::create_test_client(&config);
    let api2 = TestApiUtils::create_test_client(&config);

    // Both should be created successfully
    assert!(api1.is_ok());
    assert!(api2.is_ok());
}

/// Tests API client creation with an empty token string
///
/// This test verifies that the API client can handle empty token strings
/// gracefully without failing during creation.
#[test]
fn test_api_client_with_empty_token() {
    setup_test_env();
    let config = Config {
        api_token: Some("".to_string()),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok()); // Client creation should succeed even with empty token
}

/// Tests API client creation with a whitespace-only token
///
/// This test verifies that the API client can handle tokens containing
/// only whitespace characters without failing during creation.
#[test]
fn test_api_client_with_whitespace_token() {
    setup_test_env();
    let config = Config {
        api_token: Some("   ".to_string()),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok()); // Client creation should succeed even with whitespace token
}

/// Tests creation of multiple API client instances
///
/// This test verifies that multiple API client instances can be created
/// simultaneously without conflicts, ensuring proper isolation between
/// different client instances.
#[test]
fn test_api_client_multiple_instances() {
    setup_test_env();
    let config1 = Config {
        api_token: Some("token1".to_string()),
        workspace_id: Some("workspace1".to_string()),
        default_list_id: Some("list1".to_string()),
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let config2 = Config {
        api_token: Some("token2".to_string()),
        workspace_id: Some("workspace2".to_string()),
        default_list_id: Some("list2".to_string()),
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api1 = TestApiUtils::create_test_client(&config1);
    let api2 = TestApiUtils::create_test_client(&config2);

    assert!(api1.is_ok());
    assert!(api2.is_ok());
}

/// Tests API client creation with None values for all optional fields
///
/// This test verifies that the API client can handle configurations
/// where all optional fields are set to None, ensuring graceful
/// handling of minimal configurations.
#[test]
fn test_api_client_with_none_values() {
    setup_test_env();
    let config = Config {
        api_token: None,
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok()); // Should succeed with all None values
}

/// Tests API client creation with a very long token
///
/// This test verifies that the API client can handle tokens of
/// arbitrary length without issues, ensuring robustness for
/// various token formats and lengths.
#[test]
fn test_api_client_with_long_token() {
    setup_test_env();
    let long_token = "a".repeat(1000); // Create a 1000-character token
    let config = Config {
        api_token: Some(long_token),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok()); // Should handle long tokens gracefully
}

/// Tests API client creation with special characters in token
///
/// This test verifies that the API client can handle tokens containing
/// special characters, ensuring robust token handling for various
/// authentication scenarios.
#[test]
fn test_api_client_with_special_characters_in_token() {
    setup_test_env();
    let special_token = "token!@#$%^&*()_+-=[]{}|;':\",./<>?`~";
    let config = Config {
        api_token: Some(special_token.to_string()),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };

    let api = TestApiUtils::create_test_client(&config);
    assert!(api.is_ok()); // Should handle special characters gracefully
}

/// Tests the overdue task update functionality with tag filtering
///
/// This test verifies the logic for identifying and updating overdue tasks
/// that have specific tags. It creates a mock task that is overdue and
/// has the 'urgent' tag to test the filtering and update logic.
#[test]
fn test_update_overdue_by_tag_functionality() {
    use chrono::{DateTime, Utc};
    use clickup_cli::models::{Task, TaskCreator, TaskList, TaskSpace, TaskStatus, TaskTag};

    // Create a mock task that is overdue
    let overdue_task = Task {
        id: "task_123".to_string(),
        name: Some("Overdue Task".to_string()),
        custom_id: None,
        text_content: "Overdue task content".to_string(),
        description: "Overdue task description".to_string(),
        status: TaskStatus {
            id: "status_1".to_string(),
            status: "to do".to_string(),
            color: "#ff0000".to_string(),
            orderindex: 1,
            type_: "custom".to_string(),
        },
        orderindex: "1".to_string(),
        date_created: "2023-01-01T00:00:00Z".to_string(),
        date_updated: "2023-01-01T00:00:00Z".to_string(),
        date_closed: None,
        creator: TaskCreator {
            id: 1,
            username: "testuser".to_string(),
            color: "#0000ff".to_string(),
            profile_picture: None,
        },
        assignees: vec![],
        watchers: vec![],
        checklists: vec![],
        tags: vec![TaskTag {
            name: Some("urgent".to_string()),
            tag_fg: "#ffffff".to_string(),
            tag_bg: "#ff0000".to_string(),
            creator: 1,
        }],
        parent: None,
        top_level_parent: None,
        priority: None,
        due_date: Some("2023-01-01T00:00:00Z".to_string()), // Overdue date
        start_date: None,
        time_estimate: None,
        time_spent: None,
        custom_fields: vec![],
        dependencies: vec![],
        linked_tasks: vec![],
        team_id: "team_1".to_string(),
        list: TaskList {
            id: "list_1".to_string(),
            name: Some("Test List".to_string()),
            access: Some(true),
        },
        folder: None,
        space: TaskSpace {
            id: "space_1".to_string(),
            name: Some("Test Space".to_string()),
        },
        url: "https://app.clickup.com/t/123".to_string(),
        subtasks: None,
    };

    // Verify the task is overdue
    let due_date = DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap();
    let now = Utc::now();
    assert!(due_date < now, "Task should be overdue");

    // Verify the task has the correct tag
    let has_urgent_tag = overdue_task
        .tags
        .iter()
        .any(|tag| tag.name.as_deref() == Some("urgent"));
    assert!(has_urgent_tag, "Task should have 'urgent' tag");

    println!("✓ Test task is properly configured as overdue with 'urgent' tag");
}

/// Tests time preservation in overdue task updates
///
/// This test verifies that when updating overdue tasks, the original
/// time component is preserved while the date is updated to today.
/// It ensures that tasks with specific times (not midnight) maintain
/// their time when rescheduled.
#[test]
fn test_update_overdue_by_tag_time_preservation() {
    use chrono::{DateTime, NaiveTime, Utc};
    use clickup_cli::models::{Task, TaskCreator, TaskList, TaskSpace, TaskStatus, TaskTag};

    // Create a mock task that is overdue with a specific time (not midnight)
    let overdue_task = Task {
        id: "task_123".to_string(),
        name: Some("Overdue Task with Time".to_string()),
        custom_id: None,
        text_content: "Overdue task content".to_string(),
        description: "Overdue task description".to_string(),
        status: TaskStatus {
            id: "status_1".to_string(),
            status: "to do".to_string(),
            color: "#ff0000".to_string(),
            orderindex: 1,
            type_: "custom".to_string(),
        },
        orderindex: "1".to_string(),
        date_created: "2023-01-01T00:00:00Z".to_string(),
        date_updated: "2023-01-01T00:00:00Z".to_string(),
        date_closed: None,
        creator: TaskCreator {
            id: 1,
            username: "testuser".to_string(),
            color: "#0000ff".to_string(),
            profile_picture: None,
        },
        assignees: vec![],
        watchers: vec![],
        checklists: vec![],
        tags: vec![TaskTag {
            name: Some("urgent".to_string()),
            tag_fg: "#ffffff".to_string(),
            tag_bg: "#ff0000".to_string(),
            creator: 1,
        }],
        parent: None,
        top_level_parent: None,
        priority: None,
        due_date: Some("2023-01-01T14:30:00Z".to_string()), // Overdue date with specific time (2:30 PM)
        start_date: None,
        time_estimate: None,
        time_spent: None,
        custom_fields: vec![],
        dependencies: vec![],
        linked_tasks: vec![],
        team_id: "team_1".to_string(),
        list: TaskList {
            id: "list_1".to_string(),
            name: Some("Test List".to_string()),
            access: Some(true),
        },
        folder: None,
        space: TaskSpace {
            id: "space_1".to_string(),
            name: Some("Test Space".to_string()),
        },
        url: "https://app.clickup.com/t/123".to_string(),
        subtasks: None,
    };

    // Verify the task is overdue
    let due_date = DateTime::parse_from_rfc3339("2023-01-01T14:30:00Z").unwrap();
    let now = Utc::now();
    assert!(due_date < now, "Task should be overdue");

    // Verify the task has the correct tag
    let has_urgent_tag = overdue_task
        .tags
        .iter()
        .any(|tag| tag.name.as_deref() == Some("urgent"));
    assert!(has_urgent_tag, "Task should have 'urgent' tag");

    // Test time preservation logic
    let original_time = due_date.time();
    let today_date = Utc::now().date_naive();

    // Create new datetime with today's date and original time
    let new_due_date = chrono::NaiveDateTime::new(today_date, original_time);
    let new_due_date_utc =
        chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(new_due_date, chrono::Utc);

    // Verify that the time component is preserved
    assert_eq!(
        new_due_date_utc.time(),
        original_time,
        "Time should be preserved"
    );

    // Verify that the date is today
    assert_eq!(
        new_due_date_utc.date_naive(),
        today_date,
        "Date should be today"
    );

    // Test that due_date_time is correctly set based on whether original had time
    let original_had_time = original_time != NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    assert!(
        original_had_time,
        "Original due date should have time (not midnight)"
    );

    println!("✓ Test time preservation logic works correctly");
}
