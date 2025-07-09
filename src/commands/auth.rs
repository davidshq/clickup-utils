//! # Authentication Commands
//! 
//! This module handles all authentication-related operations for the ClickUp CLI.
//! It provides commands for setting, testing, and managing API tokens, as well
//! as checking authentication status.
//! 
//! ## Commands
//! 
//! - **Set**: Configure API token (interactive or command-line)
//! - **Test**: Verify current authentication with ClickUp API
//! - **Status**: Display current authentication status
//! - **Clear**: Remove stored API token
//! 
//! ## Security
//! 
//! API tokens are stored securely in the user's configuration directory and
//! are masked when displayed to prevent accidental exposure.

use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use clap::{Subcommand, Args};
use colored::*;
use log::{error, info};
use std::io::{self, Write};

/// Authentication command variants
/// 
/// This enum defines all available authentication subcommands with their
/// associated parameters and help text.
#[derive(Subcommand)]
pub enum AuthCommands {
    /// Set your ClickUp API token
    /// 
    /// This command allows you to configure your ClickUp API token for authentication.
    /// If no token is provided as an argument, it will prompt you to enter one
    /// interactively (recommended for security).
    Set {
        /// API token (if not provided, will prompt for input)
        /// 
        /// For security reasons, it's recommended to omit this argument and
        /// enter the token interactively when prompted.
        #[arg(short, long)]
        token: Option<String>,
    },
    /// Test your current authentication
    /// 
    /// This command verifies that your current API token is valid by making
    /// a test request to the ClickUp API and displaying your user information.
    Test,
    /// Show current authentication status
    /// 
    /// This command displays whether you are currently authenticated and shows
    /// a masked version of your API token for verification.
    Status,
    /// Clear stored API token
    /// 
    /// This command removes your stored API token from the configuration,
    /// effectively logging you out of the CLI.
    Clear,
}

/// Execute authentication commands
/// 
/// This function routes authentication commands to their appropriate handlers
/// and manages the overall authentication flow.
/// 
/// # Arguments
/// 
/// * `command` - The authentication command to execute
/// * `config` - Mutable reference to the application configuration
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful execution, or a `ClickUpError` on failure.
/// 
/// # Errors
/// 
/// This function can return various errors including:
/// - Configuration errors when saving tokens
/// - Network errors when testing authentication
/// - Validation errors for invalid tokens
pub async fn execute(command: AuthCommands, config: &mut Config) -> Result<(), ClickUpError> {
    match command {
        AuthCommands::Set { token } => {
            set_token(token, config).await?;
        }
        AuthCommands::Test => {
            test_auth(config).await?;
        }
        AuthCommands::Status => {
            show_status(config)?;
        }
        AuthCommands::Clear => {
            clear_token(config)?;
        }
    }
    Ok(())
}

/// Set the API token in configuration
/// 
/// This function handles setting the API token either from command-line arguments
/// or through interactive input. It validates the token and saves it to the
/// configuration file.
/// 
/// # Arguments
/// 
/// * `token` - Optional token from command-line arguments
/// * `config` - Mutable reference to the application configuration
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful token setting, or a `ClickUpError` on failure.
/// 
/// # Errors
/// 
/// This function can return:
/// - `ClickUpError::ValidationError` if the token is empty
/// - `ClickUpError::ConfigError` if the configuration cannot be saved
/// - `ClickUpError::IoError` if interactive input fails
async fn set_token(token: Option<String>, config: &mut Config) -> Result<(), ClickUpError> {
    // Get token from argument or prompt user
    let token = match token {
        Some(t) => t,
        None => {
            // Prompt user for token interactively
            print!("Enter your ClickUp API token: ");
            io::stdout().flush().map_err(|e| {
                ClickUpError::IoError(e)
            })?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| {
                ClickUpError::IoError(e)
            })?;
            
            input.trim().to_string()
        }
    };

    // Validate that token is not empty
    if token.is_empty() {
        return Err(ClickUpError::ValidationError("API token cannot be empty".to_string()));
    }

    // Save token to configuration
    config.set_api_token(token)?;
    println!("{}", "✓ API token saved successfully!".green());
    
    Ok(())
}

/// Test the current authentication
/// 
/// This function verifies that the current API token is valid by making a
/// test request to the ClickUp API and displaying user information.
/// 
/// # Arguments
/// 
/// * `config` - Reference to the application configuration
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful authentication test, or a `ClickUpError` on failure.
/// 
/// # Errors
/// 
/// This function can return:
/// - `ClickUpError::AuthError` if no token is configured
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::ApiError` if the API returns an error
async fn test_auth(config: &Config) -> Result<(), ClickUpError> {
    // Check if user is authenticated
    if !config.is_authenticated() {
        println!("{}", "✗ No API token configured".red());
        println!("Use 'clickup-cli auth set' to configure your API token");
        return Ok(());
    }

    // Create API client and test authentication
    let api = ClickUpApi::new(config.clone())?;
    
    match api.get_user().await {
        Ok(user) => {
            println!("{}", "✓ Authentication successful!".green());
            println!("User: {} ({})", user.user.username, user.user.email);
        }
        Err(e) => {
            println!("{}", "✗ Authentication failed".red());
            println!("Error: {}", e);
        }
    }

    Ok(())
}

/// Show current authentication status
/// 
/// This function displays the current authentication status and shows a masked
/// version of the API token for verification.
/// 
/// # Arguments
/// 
/// * `config` - Reference to the application configuration
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful status display, or a `ClickUpError` on failure.
/// 
/// # Errors
/// 
/// This function can return:
/// - `ClickUpError::ConfigError` if the configuration cannot be read
fn show_status(config: &Config) -> Result<(), ClickUpError> {
    if config.is_authenticated() {
        println!("{}", "✓ API token is configured".green());
        if let Some(token) = &config.api_token {
            // Mask the token for security (show first 4 and last 4 characters)
            let masked_token = if token.len() > 8 {
                format!("{}...{}", &token[..4], &token[token.len()-4..])
            } else {
                "***".to_string()
            };
            println!("Token: {}", masked_token);
        }
    } else {
        println!("{}", "✗ No API token configured".red());
        println!("Use 'clickup-cli auth set' to configure your API token");
    }

    Ok(())
}

/// Clear the stored API token
/// 
/// This function removes the API token from the configuration, effectively
/// logging the user out of the CLI.
/// 
/// # Arguments
/// 
/// * `config` - Mutable reference to the application configuration
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful token clearing, or a `ClickUpError` on failure.
/// 
/// # Errors
/// 
/// This function can return:
/// - `ClickUpError::ConfigError` if the configuration cannot be saved
fn clear_token(config: &mut Config) -> Result<(), ClickUpError> {
    config.api_token = None;
    config.save()?;
    println!("{}", "✓ API token cleared".green());
    Ok(())
} 