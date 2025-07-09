//! # Configuration Management
//! 
//! This module handles application configuration including API tokens, workspace settings,
//! and other user preferences. It provides a unified interface for loading, saving,
//! and managing configuration data.
//! 
//! ## Features
//! 
//! - **Multi-source Configuration**: Loads from files, environment variables, and defaults
//! - **Secure Storage**: Stores sensitive data like API tokens in user's config directory
//! - **Environment Variables**: Supports `CLICKUP_*` environment variables
//! - **Automatic Creation**: Creates config directories and files as needed
//! 
//! ## Configuration Sources
//! 
//! The configuration is loaded from multiple sources in order of precedence:
//! 1. Environment variables (highest priority)
//! 2. Configuration file (`config.toml`)
//! 3. Default values (lowest priority)
//! 
//! ## Environment Variables
//! 
//! The following environment variables are supported:
//! - `CLICKUP_API_TOKEN` - Your ClickUp API token
//! - `CLICKUP_WORKSPACE_ID` - Default workspace ID
//! - `CLICKUP_DEFAULT_LIST_ID` - Default list ID
//! - `CLICKUP_API_BASE_URL` - API base URL (defaults to ClickUp v2 API)
//! 
//! ## Configuration File
//! 
//! The configuration file is stored in the user's config directory:
//! - Windows: `%APPDATA%\clickup-cli\config.toml`
//! - macOS: `~/Library/Application Support/clickup-cli/config.toml`
//! - Linux: `~/.config/clickup-cli/config.toml`

use crate::error::ClickUpError;
use config::{Config as ConfigFile, Environment, File};
use serde::{Deserialize, Serialize};

/// Application configuration structure
/// 
/// This struct holds all configuration data for the ClickUp CLI application.
/// It includes API tokens, workspace settings, and other user preferences.
/// 
/// The configuration can be loaded from multiple sources and automatically
/// saves changes to the user's config directory.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// ClickUp API token for authentication
    /// 
    /// This token is used to authenticate all API requests. It can be set
    /// via the configuration file, environment variable, or CLI command.
    pub api_token: Option<String>,
    
    /// Default workspace ID for convenience
    /// 
    /// When specified, this workspace ID will be used as the default for
    /// commands that require a workspace context.
    pub workspace_id: Option<String>,
    
    /// Default list ID for task operations
    /// 
    /// When specified, this list ID will be used as the default for
    /// task creation and management commands.
    pub default_list_id: Option<String>,
    
    /// ClickUp API base URL
    /// 
    /// The base URL for all API requests. Defaults to the ClickUp v2 API.
    pub api_base_url: String,
}

impl Config {
    /// Loads configuration from all available sources
    /// 
    /// This function loads configuration from multiple sources in order of precedence:
    /// 1. Environment variables (highest priority)
    /// 2. Configuration file (`config.toml`)
    /// 3. Default values (lowest priority)
    /// 
    /// The configuration directory and file are created automatically if they don't exist.
    /// 
    /// # Returns
    /// 
    /// Returns a `Config` instance with the loaded configuration, or a `ClickUpError`
    /// if the configuration cannot be loaded.
    /// 
    /// # Errors
    /// 
    /// This function can return:
    /// - `ClickUpError::ConfigError` if the config directory cannot be created
    /// - `ClickUpError::ConfigParseError` if the configuration file is invalid
    pub fn load() -> Result<Self, ClickUpError> {
        // Get the user's config directory
        let config_dir = dirs::config_dir()
            .ok_or_else(|| ClickUpError::ConfigError("Could not find config directory".to_string()))?
            .join("clickup-cli");

        // Create the config directory if it doesn't exist
        std::fs::create_dir_all(&config_dir).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to create config directory: {}", e))
        })?;

        let config_file = config_dir.join("config.toml");

        // Use ConfigBuilder to avoid deprecated methods
        let mut builder = ConfigFile::builder();

        // Load configuration from file if it exists
        if config_file.exists() {
            builder = builder.add_source(File::from(config_file.as_path()));
        }

        // Load configuration from environment variables
        builder = builder.add_source(Environment::with_prefix("CLICKUP").separator("_"));

        // Set default values for required fields
        builder = builder.set_default("api_base_url", "https://api.clickup.com/api/v2").map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to set default: {}", e))
        })?;

        // Build the config and deserialize
        let config = builder.build().map_err(|e| {
            ClickUpError::ConfigParseError(e)
        })?;
        let config: Config = config.try_deserialize().map_err(|_| {
            ClickUpError::ConfigParseError(config::ConfigError::NotFound("Failed to parse config".to_string()))
        })?;

        Ok(config)
    }

    /// Saves the current configuration to the config file
    /// 
    /// This function serializes the current configuration to TOML format and
    /// writes it to the user's config directory. The file is created if it
    /// doesn't exist, or overwritten if it does.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on successful save, or a `ClickUpError` on failure.
    /// 
    /// # Errors
    /// 
    /// This function can return:
    /// - `ClickUpError::ConfigError` if the config directory cannot be created
    /// - `ClickUpError::SerializationError` if the configuration cannot be serialized
    /// - `ClickUpError::IoError` if the file cannot be written
    pub fn save(&self) -> Result<(), ClickUpError> {
        // Get the user's config directory
        let config_dir = dirs::config_dir()
            .ok_or_else(|| ClickUpError::ConfigError("Could not find config directory".to_string()))?
            .join("clickup-cli");

        // Create the config directory if it doesn't exist
        std::fs::create_dir_all(&config_dir).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to create config directory: {}", e))
        })?;

        let config_file = config_dir.join("config.toml");
        
        // Serialize the configuration to TOML format
        let config_str = toml::to_string_pretty(self).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize config: {}", e))
        })?;

        // Write the configuration to the file
        std::fs::write(config_file, config_str).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to write config file: {}", e))
        })?;

        Ok(())
    }

    /// Sets the API token and saves the configuration
    /// 
    /// This function updates the API token in the configuration and immediately
    /// saves the changes to the config file.
    /// 
    /// # Arguments
    /// 
    /// * `token` - The ClickUp API token to store
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on successful save, or a `ClickUpError` on failure.
    /// 
    /// # Errors
    /// 
    /// This function can return the same errors as `save()`.
    pub fn set_api_token(&mut self, token: String) -> Result<(), ClickUpError> {
        self.api_token = Some(token);
        self.save()
    }

    /// Retrieves the API token from configuration
    /// 
    /// This function returns a reference to the stored API token. If no token
    /// is configured, it returns an authentication error.
    /// 
    /// # Returns
    /// 
    /// Returns a reference to the API token string, or a `ClickUpError` if
    /// no token is configured.
    /// 
    /// # Errors
    /// 
    /// This function can return:
    /// - `ClickUpError::AuthError` if no API token is configured
    pub fn get_api_token(&self) -> Result<&str, ClickUpError> {
        self.api_token
            .as_deref()
            .ok_or_else(|| ClickUpError::AuthError("API token not configured".to_string()))
    }

    /// Checks if the user is authenticated
    /// 
    /// This function returns `true` if an API token is configured, indicating
    /// that the user is authenticated and can make API requests.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if an API token is configured, `false` otherwise.
    pub fn is_authenticated(&self) -> bool {
        self.api_token.is_some()
    }
}

impl Default for Config {
    /// Creates a default configuration
    /// 
    /// This function creates a new `Config` instance with default values.
    /// The API token is set to `None` (unauthenticated), and the API base URL
    /// is set to the ClickUp v2 API endpoint.
    fn default() -> Self {
        Self {
            api_token: None,
            workspace_id: None,
            default_list_id: None,
            api_base_url: "https://api.clickup.com/api/v2".to_string(),
        }
    }
}