use clickup_cli::config::Config;
use tempfile::TempDir;
use std::sync::Once;
use std::fs;
use std::env;

static INIT: Once = Once::new();
static mut TEMP_DIR: Option<TempDir> = None;

fn setup_test_env() {
    INIT.call_once(|| {
        // Remove any existing config file first
        let real_config_dir = dirs::config_dir().unwrap().join("clickup-cli");
        let real_config_file = real_config_dir.join("config.toml");
        let _ = fs::remove_file(&real_config_file);
        
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Store original environment variables
        let original_xdg = env::var("XDG_CONFIG_HOME").ok();
        let original_appdata = env::var("APPDATA").ok();
        
        // Set environment variables to use temp directory
        env::set_var("XDG_CONFIG_HOME", &temp_path);
        env::set_var("APPDATA", &temp_path);
        
        // Create the config directory in temp
        let temp_config_dir = temp_path.join("clickup-cli");
        let _ = fs::create_dir_all(&temp_config_dir);
        
        unsafe {
            TEMP_DIR = Some(temp_dir);
        }
    });
}

fn cleanup_test_env() {
    unsafe {
        if let Some(temp_dir) = &TEMP_DIR {
            // Restore original environment variables
            env::remove_var("XDG_CONFIG_HOME");
            env::remove_var("APPDATA");
            
            // Clean up temp directory
            let _ = fs::remove_dir_all(temp_dir.path());
        }
    }
}

#[test]
fn test_config_default() {
    setup_test_env();
    let config = Config::default();
    assert_eq!(config.api_base_url, "https://api.clickup.com/api/v2");
    assert!(config.api_token.is_none());
    assert!(config.workspace_id.is_none());
    assert!(config.default_list_id.is_none());
    cleanup_test_env();
}

#[test]
fn test_config_set_and_get_api_token() {
    setup_test_env();
    let mut config = Config::default();
    let test_token = "test_token_123".to_string();
    let result = config.set_api_token(test_token.clone());
    assert!(result.is_ok());
    let retrieved_token = config.get_api_token().unwrap();
    assert_eq!(retrieved_token, test_token);
    cleanup_test_env();
}

#[test]
fn test_config_get_api_token_when_none() {
    setup_test_env();
    let config = Config::default();
    let result = config.get_api_token();
    assert!(result.is_err());
    match result {
        Err(clickup_cli::error::ClickUpError::AuthError(_)) => {},
        _ => panic!("Expected AuthError when no token is set"),
    }
    cleanup_test_env();
}

#[test]
fn test_config_is_authenticated() {
    setup_test_env();
    let mut config = Config::default();
    assert!(!config.is_authenticated());
    config.set_api_token("test_token".to_string()).unwrap();
    assert!(config.is_authenticated());
    cleanup_test_env();
}

#[test]
fn test_config_save_and_load() {
    setup_test_env();
    
    // Create a config with all fields set
    let config = Config {
        api_token: Some("test_token_456".to_string()),
        workspace_id: Some("workspace_123".to_string()),
        default_list_id: Some("list_456".to_string()),
        api_base_url: "https://test.api.clickup.com/api/v2".to_string(),
    };
    
    // Save the config
    let save_result = config.save();
    assert!(save_result.is_ok(), "Failed to save config: {:?}", save_result);
    
    // Load the config and verify all fields
    let loaded_config = Config::load();
    assert!(loaded_config.is_ok(), "Failed to load config: {:?}", loaded_config);
    
    let loaded_config = loaded_config.unwrap();
    assert_eq!(loaded_config.api_token, config.api_token);
    assert_eq!(loaded_config.workspace_id, config.workspace_id);
    assert_eq!(loaded_config.default_list_id, config.default_list_id);
    assert_eq!(loaded_config.api_base_url, config.api_base_url);
    
    cleanup_test_env();
}

#[test]
fn test_config_validation_empty_token() {
    setup_test_env();
    let mut config = Config::default();
    let result = config.set_api_token("".to_string());
    assert!(result.is_err());
    match result {
        Err(clickup_cli::error::ClickUpError::ValidationError(_)) => {},
        _ => panic!("Expected ValidationError for empty token"),
    }
    cleanup_test_env();
}

#[test]
fn test_config_validation_whitespace_token() {
    setup_test_env();
    let mut config = Config::default();
    let result = config.set_api_token("   ".to_string());
    assert!(result.is_err());
    match result {
        Err(clickup_cli::error::ClickUpError::ValidationError(_)) => {},
        _ => panic!("Expected ValidationError for whitespace-only token"),
    }
    cleanup_test_env();
} 