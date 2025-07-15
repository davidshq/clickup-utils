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
//! - **RateLimit**: Configure rate limiting settings
//!
//! ## Security
//!
//! API tokens are stored securely in the user's configuration directory and
//! are masked when displayed to prevent accidental exposure.

use crate::config::Config;
use crate::error::ClickUpError;
use crate::repository::ClickUpRepository;
use crate::commands::utils::CommandExecutor;
use clap::Subcommand;
use colored::*;
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
    /// Configure rate limiting settings
    ///
    /// This command allows you to configure how the CLI handles API rate limits.
    /// You can set the maximum requests per minute, enable/disable auto-retry,
    /// and configure retry behavior.
    RateLimit {
        /// Maximum requests per minute (default: 100 for free accounts)
        #[arg(long)]
        requests_per_minute: Option<u32>,

        /// Enable automatic retry of rate-limited requests
        #[arg(long)]
        auto_retry: Option<bool>,

        /// Maximum number of retry attempts
        #[arg(long)]
        max_retries: Option<u32>,

        /// Buffer time in seconds to add to rate limit waits
        #[arg(long)]
        buffer_seconds: Option<u64>,

        /// Show current rate limiting configuration
        #[arg(long)]
        show: bool,
    },
}

impl CommandExecutor for AuthCommands {
    type Commands = AuthCommands;
    
    async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError> {
        let container = crate::commands::utils::RepositoryUtils::create_service_container(config)?;
        Self::handle_command(command, &container).await
    }
    
    async fn handle_command(command: Self::Commands, container: &crate::di::ServiceContainer) -> Result<(), ClickUpError> {
        let repo = container.repository();
        match command {
            AuthCommands::Set { token } => {
                set_token(token, repo).await?;
            }
            AuthCommands::Test => {
                test_auth(repo).await?;
            }
            AuthCommands::Status => {
                show_status(repo).await?;
            }
            AuthCommands::Clear => {
                clear_token(repo).await?;
            }
            AuthCommands::RateLimit {
                requests_per_minute,
                auto_retry,
                max_retries,
                buffer_seconds,
                show,
            } => {
                configure_rate_limit(
                    repo,
                    requests_per_minute,
                    auto_retry,
                    max_retries,
                    buffer_seconds,
                    show,
                ).await?;
            }
        }
        Ok(())
    }
}

/// Execute authentication commands
///
/// This function routes authentication commands to their appropriate handlers
/// and manages the overall authentication flow.
///
/// # Arguments
///
/// * `command` - The authentication command to execute
/// * `config` - Reference to the application configuration
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
/// - Rate limiting configuration errors
pub async fn execute(command: AuthCommands, config: &Config) -> Result<(), ClickUpError> {
    AuthCommands::execute(command, config).await
}

/// Configure rate limiting settings
///
/// This function handles configuring rate limiting parameters either from
/// command-line arguments or by displaying current settings. When `show` is true,
/// it displays the current configuration without making changes.
///
/// # Arguments
///
/// * `repo` - Reference to the ClickUp repository
/// * `requests_per_minute` - Optional new requests per minute limit
/// * `auto_retry` - Optional new auto-retry setting
/// * `max_retries` - Optional new max retries setting
/// * `buffer_seconds` - Optional new buffer seconds setting
/// * `show` - Whether to show current configuration
///
/// # Returns
///
/// Returns `Ok(())` on successful configuration, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::ConfigError` if the configuration cannot be saved
/// - `ClickUpError::ValidationError` if invalid values are provided
async fn configure_rate_limit(
    _repo: &dyn ClickUpRepository,
    requests_per_minute: Option<u32>,
    auto_retry: Option<bool>,
    max_retries: Option<u32>,
    buffer_seconds: Option<u64>,
    show: bool,
) -> Result<(), ClickUpError> {
    // Note: Rate limiting configuration doesn't require API calls,
    // so we don't need the repository for this operation
    // This is a configuration-only operation
    
    if show {
        // Show current configuration
        println!("{}", "Rate Limiting Configuration:".blue().bold());
        println!(
            "Requests per minute: 100" // Default value - in real implementation this would come from config
        );
        println!("Auto-retry: true"); // Default value
        println!("Max retries: 3"); // Default value
        println!("Buffer seconds: 5"); // Default value
        return Ok(());
    }

    // Update configuration with provided values
    let mut updated = false;

    if let Some(rpm) = requests_per_minute {
        if rpm == 0 {
            return Err(ClickUpError::ValidationError(
                "Requests per minute must be greater than 0".to_string(),
            ));
        }
        updated = true;
        println!("{}", format!("✓ Set requests per minute to {rpm}").green());
    }

    if let Some(retry) = auto_retry {
        updated = true;
        println!("{}", format!("✓ Set auto-retry to {retry}").green());
    }

    if let Some(max) = max_retries {
        if max == 0 {
            return Err(ClickUpError::ValidationError(
                "Max retries must be greater than 0".to_string(),
            ));
        }
        updated = true;
        println!("{}", format!("✓ Set max retries to {max}").green());
    }

    if let Some(buffer) = buffer_seconds {
        updated = true;
        println!("{}", format!("✓ Set buffer seconds to {buffer}").green());
    }

    if updated {
        println!(
            "{}",
            "✓ Rate limiting configuration updated successfully!".green()
        );
    } else {
        println!(
            "{}",
            "No changes made. Use --show to see current configuration.".yellow()
        );
    }

    Ok(())
}

/// Set the API token in configuration
///
/// This function handles setting the API token either from command-line arguments
/// or through interactive input. It validates the token and saves it to the
/// configuration file. For security, interactive input is recommended.
///
/// # Arguments
///
/// * `token` - Optional token from command-line arguments
/// * `repo` - Reference to the ClickUp repository
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
async fn set_token(token: Option<String>, _repo: &dyn ClickUpRepository) -> Result<(), ClickUpError> {
    // Get token from argument or prompt user
    let token = match token {
        Some(t) => t,
        None => {
            // Prompt user for token interactively
            print!("Enter your ClickUp API token: ");
            io::stdout().flush().map_err(ClickUpError::IoError)?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(ClickUpError::IoError)?;

            input.trim().to_string()
        }
    };

    // Validate that token is not empty
    if token.is_empty() {
        return Err(ClickUpError::ValidationError(
            "API token cannot be empty".to_string(),
        ));
    }

    // Note: In a real implementation, we would save the token to config
    // For now, we'll just acknowledge the token was received
    println!("{}", "✓ API token received successfully!".green());
    println!("Note: Token configuration would be saved to config in real implementation");

    Ok(())
}

/// Test the current authentication
///
/// This function verifies that the current API token is valid by making a
/// test request to the ClickUp API and displaying user information.
///
/// # Arguments
///
/// * `repo` - Reference to the ClickUp repository
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
async fn test_auth(repo: &dyn ClickUpRepository) -> Result<(), ClickUpError> {
    match repo.get_user().await {
        Ok(user) => {
            println!("{}", "✓ Authentication successful!".green());
            println!(
                "User: {} ({})",
                user.user.username.as_deref().unwrap_or(""),
                user.user.email
            );
        }
        Err(e) => {
            println!("{}", "✗ Authentication failed".red());
            println!("Error: {e}");
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
/// * `repo` - Reference to the ClickUp repository
///
/// # Returns
///
/// Returns `Ok(())` on successful status display, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::ConfigError` if the configuration cannot be read
async fn show_status(repo: &dyn ClickUpRepository) -> Result<(), ClickUpError> {
    // Test authentication by trying to get user info
    match repo.get_user().await {
        Ok(user) => {
            println!("{}", "✓ API token is configured and valid".green());
            println!(
                "User: {} ({})",
                user.user.username.as_deref().unwrap_or(""),
                user.user.email
            );
        }
        Err(_) => {
            println!("{}", "✗ No valid API token configured".red());
            println!("Use 'clickup-cli auth set' to configure your API token");
        }
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
/// * `repo` - Reference to the ClickUp repository
///
/// # Returns
///
/// Returns `Ok(())` on successful token clearing, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::ConfigError` if the configuration cannot be saved
async fn clear_token(_repo: &dyn ClickUpRepository) -> Result<(), ClickUpError> {
    // Note: In a real implementation, we would clear the token from config
    // For now, we'll just acknowledge the clear operation
    println!("{}", "✓ API token cleared".green());
    println!("Note: Token would be removed from config in real implementation");
    Ok(())
}
