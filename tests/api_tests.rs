use clickup_cli::api::ClickUpApi;
use clickup_cli::config::Config;
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
    };
    
    let config2 = Config {
        api_token: Some("token2".to_string()),
        workspace_id: None,
        default_list_id: None,
        api_base_url: "https://api.clickup.com/api/v2".to_string(),
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
    };
    
    let api = ClickUpApi::new(config);
    assert!(api.is_ok());
} 