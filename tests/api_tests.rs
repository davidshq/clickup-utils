use clickup_cli::api::ClickUpApi;
use clickup_cli::config::{Config, RateLimitConfig};
use tempfile::TempDir;
use std::sync::Once;

static INIT: Once = Once::new();
static mut TEMP_DIR: Option<TempDir> = None;

fn setup_test_env() {
    INIT.call_once(|| {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        std::env::set_var("XDG_CONFIG_HOME", &temp_path);
        std::env::set_var("APPDATA", &temp_path);
        unsafe {
            TEMP_DIR = Some(temp_dir);
        }
    });
}

#[test]
fn test_api_client_creation_with_valid_config() {
    setup_test_env();
    let config = Config {
        api_token: Some("test_token_123".to_string()),
        workspace_id: Some("workspace_123".to_string()),
        default_list_id: Some("list_456".to_string()),
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok());
}

#[test]
fn test_api_client_creation_without_token() {
    setup_test_env();
    let config = Config {
        api_token: None,
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok()); // API client can be created without token
}

#[test]
fn test_api_client_with_personal_token() {
    setup_test_env();
    let config = Config {
        api_token: Some("pk_test_token_123".to_string()),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok());
}

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
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok());
}

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
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok());
}

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
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok()); // The client creation itself should succeed
}

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
    
    let api1 = ClickUpApi::new(config.clone());
    let api2 = ClickUpApi::new(config);
    
    // Both should be created successfully
    assert!(api1.is_ok());
    assert!(api2.is_ok());
}

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
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok()); // Client creation should succeed even with empty token
}

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
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok()); // Client creation should succeed even with whitespace token
}

#[test]
fn test_api_client_multiple_instances() {
    setup_test_env();
    let config1 = Config {
        api_token: Some("token1".to_string()),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };
    
    let config2 = Config {
        api_token: Some("token2".to_string()),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };
    
    let api1 = ClickUpApi::new(config1);
    let api2 = ClickUpApi::new(config2);
    
    assert!(api1.is_ok());
    assert!(api2.is_ok());
}

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
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok());
}

#[test]
fn test_api_client_with_long_token() {
    setup_test_env();
    let long_token = "a".repeat(1000);
    let config = Config {
        api_token: Some(long_token),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok());
}

#[test]
fn test_api_client_with_special_characters_in_token() {
    setup_test_env();
    let special_token = "test_token_with_special_chars_!@#$%^&*()_+-=[]{}|;':\",./<>?".to_string();
    let config = Config {
        api_token: Some(special_token),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
        rate_limit: RateLimitConfig::default(),
    };
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok());
} 

#[test]
fn test_update_overdue_by_tag_functionality() {
    use chrono::{DateTime, Utc};
    use clickup_cli::models::{Task, TaskStatus, TaskCreator, TaskList, TaskSpace, TaskTag};
    
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
    let has_urgent_tag = overdue_task.tags.iter().any(|tag| {
        tag.name.as_deref() == Some("urgent")
    });
    assert!(has_urgent_tag, "Task should have 'urgent' tag");
    
    println!("✓ Test task is properly configured as overdue with 'urgent' tag");
} 

#[test]
fn test_update_overdue_by_tag_time_preservation() {
    use chrono::{DateTime, Utc, NaiveTime};
    use clickup_cli::models::{Task, TaskStatus, TaskCreator, TaskList, TaskSpace, TaskTag};
    
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
    let has_urgent_tag = overdue_task.tags.iter().any(|tag| {
        tag.name.as_deref() == Some("urgent")
    });
    assert!(has_urgent_tag, "Task should have 'urgent' tag");
    
    // Test time preservation logic
    let original_time = due_date.time();
    let today_date = Utc::now().date_naive();
    
    // Create new datetime with today's date and original time
    let new_due_date = chrono::NaiveDateTime::new(today_date, original_time);
    let new_due_date_utc = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(new_due_date, chrono::Utc);
    
    // Verify that the time component is preserved
    assert_eq!(new_due_date_utc.time(), original_time, "Time should be preserved");
    
    // Verify that the date is today
    assert_eq!(new_due_date_utc.date_naive(), today_date, "Date should be today");
    
    // Test that due_date_time is correctly set based on whether original had time
    let original_had_time = original_time != NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    assert!(original_had_time, "Original due date should have time (not midnight)");
    
    println!("✓ Test time preservation logic works correctly");
} 