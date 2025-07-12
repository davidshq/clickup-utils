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

/// Rate limiting configuration
/// 
/// This struct defines how the application should handle API rate limits.
/// It controls the maximum number of requests per minute and retry behavior.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimitConfig {
    /// Maximum number of requests allowed per minute
    /// 
    /// ClickUp free accounts are limited to 100 requests per minute.
    /// Paid accounts may have higher limits.
    pub requests_per_minute: u32,
    
    /// Whether to automatically retry rate-limited requests
    /// 
    /// When true, rate-limited requests will be automatically retried
    /// after waiting for the rate limit to reset.
    pub auto_retry: bool,
    
    /// Maximum number of retry attempts for rate-limited requests
    /// 
    /// This prevents infinite retry loops in case of persistent rate limiting.
    pub max_retries: u32,
    
    /// Buffer time in seconds to add to rate limit wait times
    /// 
    /// This provides a safety margin to ensure we don't hit rate limits
    /// immediately after they reset.
    pub buffer_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 100, // ClickUp free account limit
            auto_retry: true,
            max_retries: 3,
            buffer_seconds: 5, // 5 second buffer
        }
    }
}

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
    
    /// Rate limiting configuration
    /// 
    /// Controls how many requests can be made per minute and how to handle rate limits.
    pub rate_limit: RateLimitConfig,
}

impl Config {
    /// Loads configuration from all available sources
    ///
    /// This function loads configuration from multiple sources in order of precedence:
    /// 1. Environment variables (highest priority)
    /// 2. Configuration file (`config.toml` or the provided path)
    /// 3. Default values (lowest priority)
    ///
    /// The configuration directory and file are created automatically if they don't exist (unless a custom path is provided).
    ///
    /// # Arguments
    ///
    /// * `config_file_override` - Optional path to a config file. If `Some(path)`, loads from that file. If `None`, uses the default location.
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
    pub fn load_with_path(config_file_override: Option<&std::path::Path>) -> Result<Self, ClickUpError> {
        // Get the config file path
        let config_file = if let Some(path) = config_file_override {
            path.to_path_buf()
        } else {
            let config_dir = dirs::config_dir()
                .ok_or_else(|| ClickUpError::ConfigError("Could not find config directory".to_string()))?
                .join("clickup-cli");
            std::fs::create_dir_all(&config_dir).map_err(|e| {
                ClickUpError::ConfigError(format!("Failed to create config directory: {e}"))
            })?;
            config_dir.join("config.toml")
        };

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
            ClickUpError::ConfigError(format!("Failed to set default: {e}"))
        })?;
        
        // Set default values for rate limiting configuration
        builder = builder.set_default("rate_limit.requests_per_minute", 100).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to set rate limit default: {e}"))
        })?;
        builder = builder.set_default("rate_limit.auto_retry", true).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to set auto retry default: {e}"))
        })?;
        builder = builder.set_default("rate_limit.max_retries", 3).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to set max retries default: {e}"))
        })?;
        builder = builder.set_default("rate_limit.buffer_seconds", 5).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to set buffer seconds default: {e}"))
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

    /// Loads configuration from the default location
    ///
    /// This is a convenience wrapper for `load_with_path(None)`. See that method for details.
    pub fn load() -> Result<Self, ClickUpError> {
        Self::load_with_path(None)
    }

    /// Saves the current configuration to the config file
    ///
    /// This function serializes the current configuration to TOML format and
    /// writes it to the user's config directory or a custom path. The file is created if it
    /// doesn't exist, or overwritten if it does.
    ///
    /// # Arguments
    ///
    /// * `config_file_override` - Optional path to a config file. If `Some(path)`, saves to that file. If `None`, uses the default location.
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
    pub fn save_with_path(&self, config_file_override: Option<&std::path::Path>) -> Result<(), ClickUpError> {
        // Get the config file path
        let config_file = if let Some(path) = config_file_override {
            path.to_path_buf()
        } else {
            let config_dir = dirs::config_dir()
                .ok_or_else(|| ClickUpError::ConfigError("Could not find config directory".to_string()))?
                .join("clickup-cli");
            std::fs::create_dir_all(&config_dir).map_err(|e| {
                ClickUpError::ConfigError(format!("Failed to create config directory: {e}"))
            })?;
            config_dir.join("config.toml")
        };
        // Serialize the configuration to TOML format
        let config_str = toml::to_string_pretty(self).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize config: {e}"))
        })?;
        // Write the configuration to the file
        std::fs::write(config_file, config_str).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to write config file: {e}"))
        })?;
        Ok(())
    }

    /// Saves the current configuration to the default config file
    ///
    /// This is a convenience wrapper for `save_with_path(None)`. See that method for details.
    pub fn save(&self) -> Result<(), ClickUpError> {
        self.save_with_path(None)
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
        if token.trim().is_empty() {
            return Err(ClickUpError::ValidationError("API token cannot be empty or whitespace".to_string()));
        }
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
            rate_limit: RateLimitConfig::default(),
        }
    }
}