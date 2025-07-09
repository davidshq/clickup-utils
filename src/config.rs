use crate::error::ClickUpError;
use config::{Config as ConfigFile, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_token: Option<String>,
    pub workspace_id: Option<String>,
    pub default_list_id: Option<String>,
    pub api_base_url: String,
}

impl Config {
    pub fn load() -> Result<Self, ClickUpError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| ClickUpError::ConfigError("Could not find config directory".to_string()))?
            .join("clickup-cli");

        
        std::fs::create_dir_all(&config_dir).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to create config directory: {}", e))
        })?;

        let config_file = config_dir.join("config.toml");

        let mut config = ConfigFile::new();

        // Load from config file if it exists
        if config_file.exists() {
            config = config.merge(File::from(config_file.as_path()))?;
        }

        // Load from environment variables
        config = config.merge(Environment::with_prefix("CLICKUP").separator("_"))?;

        // Set defaults
        config.set_default("api_base_url", "https://api.clickup.com/api/v2")?;

        let config: Config = config.try_into()?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), ClickUpError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| ClickUpError::ConfigError("Could not find config directory".to_string()))?
            .join("clickup-cli");

        std::fs::create_dir_all(&config_dir).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to create config directory: {}", e))
        })?;

        let config_file = config_dir.join("config.toml");
        let config_str = toml::to_string_pretty(self).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize config: {}", e))
        })?;

        std::fs::write(config_file, config_str).map_err(|e| {
            ClickUpError::ConfigError(format!("Failed to write config file: {}", e))
        })?;

        Ok(())
    }

    pub fn set_api_token(&mut self, token: String) -> Result<(), ClickUpError> {
        self.api_token = Some(token);
        self.save()
    }

    pub fn get_api_token(&self) -> Result<&str, ClickUpError> {
        self.api_token
            .as_deref()
            .ok_or_else(|| ClickUpError::AuthError("API token not configured".to_string()))
    }

    pub fn is_authenticated(&self) -> bool {
        self.api_token.is_some()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_token: None,
            workspace_id: None,
            default_list_id: None,
            api_base_url: "https://api.clickup.com/api/v2".to_string(),
        }
    }
} 