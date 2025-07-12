use clickup_cli::config::Config;
use tempfile::TempDir;
use std::fs;
use std::env;
use std::path::PathBuf;

// Test configuration that properly isolates tests
struct TestConfig {
    temp_dir: TempDir,
    config_file: std::path::PathBuf,
    original_xdg: Option<String>,
    original_appdata: Option<String>,
}

impl TestConfig {
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

#[test]
fn test_config_default() {
    let test_config = TestConfig::new();
    let config = Config::default();
    assert_eq!(config.api_base_url, "https://api.clickup.com/api/v2");
    assert!(config.api_token.is_none());
    assert!(config.workspace_id.is_none());
    assert!(config.default_list_id.is_none());
}

#[test]
fn test_config_set_and_get_api_token() {
    let test_config = TestConfig::new();
    let mut config = Config::default();
    let test_token = "test_token_123".to_string();
    let result = config.set_api_token(test_token.clone());
    // Save to temp config file
    let save_result = config.save_with_path(Some(&test_config.config_file));
    assert!(save_result.is_ok());
    let loaded = Config::load_with_path(Some(&test_config.config_file)).unwrap();
    let retrieved_token = loaded.get_api_token().unwrap();
    assert_eq!(retrieved_token, test_token);
}

#[test]
fn test_config_get_api_token_when_none() {
    let test_config = TestConfig::new();
    let config = Config::default();
    let result = config.get_api_token();
    assert!(result.is_err());
    match result {
        Err(clickup_cli::error::ClickUpError::AuthError(_)) => {},
        _ => panic!("Expected AuthError when no token is set"),
    }
}

#[test]
fn test_config_is_authenticated() {
    let test_config = TestConfig::new();
    let mut config = Config::default();
    assert!(!config.is_authenticated());
    config.set_api_token("test_token".to_string()).unwrap();
    assert!(config.is_authenticated());
}

#[test]
fn test_config_save_and_load() {
    let test_config = TestConfig::new();
    // Create a config with all fields set
    let config = Config {
        api_token: Some("test_token_456".to_string()),
        workspace_id: Some("workspace_123".to_string()),
        default_list_id: Some("list_456".to_string()),
        api_base_url: "https://test.api.clickup.com/api/v2".to_string(),
    };
    // Save the config to the temp file
    let save_result = config.save_with_path(Some(&test_config.config_file));
    assert!(save_result.is_ok(), "Failed to save config: {:?}", save_result);
    // Load the config and verify all fields
    let loaded_config = Config::load_with_path(Some(&test_config.config_file));
    assert!(loaded_config.is_ok(), "Failed to load config: {:?}", loaded_config);
    let loaded_config = loaded_config.unwrap();
    assert_eq!(loaded_config.api_token, config.api_token);
    assert_eq!(loaded_config.workspace_id, config.workspace_id);
    assert_eq!(loaded_config.default_list_id, config.default_list_id);
    assert_eq!(loaded_config.api_base_url, config.api_base_url);
}

#[test]
fn test_config_validation_empty_token() {
    let test_config = TestConfig::new();
    let mut config = Config::default();
    let result = config.set_api_token("".to_string());
    assert!(result.is_err());
    match result {
        Err(clickup_cli::error::ClickUpError::ValidationError(_)) => {},
        _ => panic!("Expected ValidationError for empty token"),
    }
}

#[test]
fn test_config_validation_whitespace_token() {
    let test_config = TestConfig::new();
    let mut config = Config::default();
    let result = config.set_api_token("   ".to_string());
    assert!(result.is_err());
    match result {
        Err(clickup_cli::error::ClickUpError::ValidationError(_)) => {},
        _ => panic!("Expected ValidationError for whitespace-only token"),
    }
} 