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
//! 2. `.env` file (loaded automatically if present)
//! 3. Configuration file (`config.toml`)
//! 4. Default values (lowest priority)
//!
//! ## Environment Variables and .env Files
//!
//! The following environment variables are supported (can be set in `.env` file or system environment):
//! - `CLICKUP_API_TOKEN` - Your ClickUp API token
//! - `CLICKUP_API_TOKEN_TEST` - Test API token (used when running tests)
//! - `CLICKUP_WORKSPACE_ID` - Default workspace ID
//! - `CLICKUP_DEFAULT_LIST_ID` - Default list ID
//! - `CLICKUP_API_BASE_URL` - API base URL (defaults to ClickUp v2 API)
//! - `CLICKUP_RATE_LIMIT__REQUESTS_PER_MINUTE` - Rate limit requests per minute
//! - `CLICKUP_RATE_LIMIT__AUTO_RETRY` - Whether to auto-retry rate-limited requests
//! - `CLICKUP_RATE_LIMIT__MAX_RETRIES` - Maximum retry attempts
//! - `CLICKUP_RATE_LIMIT__BUFFER_SECONDS` - Buffer time for rate limiting
//!
//! ## .env Files
//!
//! For local development, you can use a `.env` file in your project root:
//! 1. Copy `.env.example` to `.env`
//! 2. Fill in your configuration values
//! 3. The `.env` file will be loaded automatically
//!
//! **Note**: `.env` files are ignored by git for security. Never commit your `.env` file.
//!
//! ## Configuration File
//!
//! The configuration file is stored in the user's config directory:
//! - Windows: `%APPDATA%\clickup-cli\config.toml`
//! - macOS: `~/Library/Application Support/clickup-cli/config.toml`
//! - Linux: `~/.config/clickup-cli/config.toml`

use crate::error::ClickUpError;
use config::{Config as ConfigFile, Environment, File};
use dotenvy::dotenv;
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
    pub fn load_with_path(
        config_file_override: Option<&std::path::Path>,
    ) -> Result<Self, ClickUpError> {
        // Load .env file if it exists (highest priority), unless skipped for tests
        if std::env::var("CLICKUP_SKIP_ENV_FILE").is_err() {
            dotenv().ok();
        }

        // Get the config file path
        let config_file = if let Some(path) = config_file_override {
            path.to_path_buf()
        } else {
            let config_dir = dirs::config_dir()
                .ok_or_else(|| {
                    ClickUpError::ConfigError("Could not find config directory".to_string())
                })?
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
        builder = builder
            .set_default("api_base_url", "https://api.clickup.com/api/v2")
            .map_err(|e| ClickUpError::ConfigError(format!("Failed to set default: {e}")))?;

        // Set default values for rate limiting configuration
        builder = builder
            .set_default("rate_limit.requests_per_minute", 100)
            .map_err(|e| {
                ClickUpError::ConfigError(format!("Failed to set rate limit default: {e}"))
            })?;
        builder = builder
            .set_default("rate_limit.auto_retry", true)
            .map_err(|e| {
                ClickUpError::ConfigError(format!("Failed to set auto retry default: {e}"))
            })?;
        builder = builder
            .set_default("rate_limit.max_retries", 3)
            .map_err(|e| {
                ClickUpError::ConfigError(format!("Failed to set max retries default: {e}"))
            })?;
        builder = builder
            .set_default("rate_limit.buffer_seconds", 5)
            .map_err(|e| {
                ClickUpError::ConfigError(format!("Failed to set buffer seconds default: {e}"))
            })?;
        // Build the config and deserialize
        let config = builder.build().map_err(ClickUpError::ConfigParseError)?;
        #[allow(unused_mut)]
        let mut config: Config = config.try_deserialize().map_err(|_| {
            ClickUpError::ConfigParseError(config::ConfigError::NotFound(
                "Failed to parse config".to_string(),
            ))
        })?;

        // Load API tokens from environment variables if not already set
        if config.api_token.is_none() {
            if let Ok(token) = std::env::var("CLICKUP_API_TOKEN") {
                if !token.trim().is_empty() {
                    config.api_token = Some(token);
                }
            }
        }

        // Check for test API token (always check, not just in test context)
        if let Ok(test_token) = std::env::var("CLICKUP_API_TOKEN_TEST") {
            if !test_token.trim().is_empty() {
                config.api_token = Some(test_token);
            }
        }

        Ok(config)
    }

    /// Loads configuration from the default location
    ///
    /// This is a convenience wrapper for `load_with_path(None)`. See that method for details.
    pub fn load() -> Result<Self, ClickUpError> {
        Self::load_with_path(None)
    }

    /// Loads configuration specifically for tests from .env.test
    ///
    /// This method loads configuration from .env.test file and environment variables,
    /// ensuring tests use the test environment settings rather than production settings.
    ///
    /// # Returns
    ///
    /// Returns a `Config` instance with test configuration, or a `ClickUpError`
    /// if the configuration cannot be loaded.
    ///
    /// # Errors
    ///
    /// This function can return the same errors as `load_with_path()`.
    pub fn save_with_path(
        &self,
        config_file_override: Option<&std::path::Path>,
    ) -> Result<(), ClickUpError> {
        // Get the config file path
        let config_file = if let Some(path) = config_file_override {
            path.to_path_buf()
        } else {
            let config_dir = dirs::config_dir()
                .ok_or_else(|| {
                    ClickUpError::ConfigError("Could not find config directory".to_string())
                })?
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
        std::fs::write(config_file, config_str)
            .map_err(|e| ClickUpError::ConfigError(format!("Failed to write config file: {e}")))?;
        Ok(())
    }

    /// Saves the current configuration to the default config file
    ///
    /// This is a convenience wrapper for `save_with_path(None)`. See that method for details.
    pub fn save(&self) -> Result<(), ClickUpError> {
        self.save_with_path(None)
    }

    /// Creates a .env file from the .env.example template
    ///
    /// This is useful for first-time setup. The method copies the .env.example
    /// file to .env if it doesn't already exist.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the file was created successfully, or a `ClickUpError`
    /// if the operation fails.
    #[allow(dead_code)]
    pub fn create_env_file() -> Result<(), ClickUpError> {
        let env_file = std::path::Path::new(".env");
        let example_file = std::path::Path::new(".env.example");

        if env_file.exists() {
            return Ok(()); // .env already exists
        }

        if !example_file.exists() {
            return Err(ClickUpError::ConfigError(
                ".env.example file not found. Please create it first.".to_string(),
            ));
        }

        std::fs::copy(example_file, env_file)
            .map_err(|e| ClickUpError::ConfigError(format!("Failed to create .env file: {e}")))?;

        Ok(())
    }

    /// Checks if a .env file exists and provides helpful setup instructions
    ///
    /// This method is useful for providing better error messages when
    /// configuration is missing.
    #[allow(dead_code)]
    pub fn check_env_setup() -> Result<(), ClickUpError> {
        let env_file = std::path::Path::new(".env");
        let example_file = std::path::Path::new(".env.example");

        if !env_file.exists() {
            if example_file.exists() {
                return Err(ClickUpError::ConfigError(
                    "No .env file found. Please copy .env.example to .env and configure your settings.".to_string()
                ));
            } else {
                return Err(ClickUpError::ConfigError(
                    "No .env file found and no .env.example template available.".to_string(),
                ));
            }
        }

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
        if token.trim().is_empty() {
            return Err(ClickUpError::ValidationError(
                "API token cannot be empty or whitespace".to_string(),
            ));
        }
        self.api_token = Some(token);
        self.save()
    }

    /// Retrieves the API token from configuration
    ///
    /// This function returns a reference to the stored API token. If no token
    /// is configured, it returns an authentication error.
    ///
    /// The function checks for tokens in the following order:
    /// 1. Environment variable `CLICKUP_API_TOKEN_TEST` (when available)
    /// 2. Environment variable `CLICKUP_API_TOKEN` (for regular use)
    /// 3. Stored API token in configuration
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
        // Fall back to stored token in configuration
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
