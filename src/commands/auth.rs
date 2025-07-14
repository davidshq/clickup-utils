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

use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;

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
/// - Rate limiting configuration errors
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
        AuthCommands::RateLimit {
            requests_per_minute,
            auto_retry,
            max_retries,
            buffer_seconds,
            show,
        } => {
            configure_rate_limit(
                config,
                requests_per_minute,
                auto_retry,
                max_retries,
                buffer_seconds,
                show,
            )?;
        }
    }
    Ok(())
}

/// Configure rate limiting settings
///
/// This function handles configuring rate limiting parameters either from
/// command-line arguments or by displaying current settings. When `show` is true,
/// it displays the current configuration without making changes.
///
/// # Arguments
///
/// * `config` - Mutable reference to the application configuration
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
fn configure_rate_limit(
    config: &mut Config,
    requests_per_minute: Option<u32>,
    auto_retry: Option<bool>,
    max_retries: Option<u32>,
    buffer_seconds: Option<u64>,
    show: bool,
) -> Result<(), ClickUpError> {
    if show {
        // Show current configuration
        println!("{}", "Rate Limiting Configuration:".blue().bold());
        println!(
            "Requests per minute: {}",
            config.rate_limit.requests_per_minute
        );
        println!("Auto-retry: {}", config.rate_limit.auto_retry);
        println!("Max retries: {}", config.rate_limit.max_retries);
        println!("Buffer seconds: {}", config.rate_limit.buffer_seconds);
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
        config.rate_limit.requests_per_minute = rpm;
        updated = true;
        println!("{}", format!("✓ Set requests per minute to {rpm}").green());
    }

    if let Some(retry) = auto_retry {
        config.rate_limit.auto_retry = retry;
        updated = true;
        println!("{}", format!("✓ Set auto-retry to {retry}").green());
    }

    if let Some(max) = max_retries {
        if max == 0 {
            return Err(ClickUpError::ValidationError(
                "Max retries must be greater than 0".to_string(),
            ));
        }
        config.rate_limit.max_retries = max;
        updated = true;
        println!("{}", format!("✓ Set max retries to {max}").green());
    }

    if let Some(buffer) = buffer_seconds {
        config.rate_limit.buffer_seconds = buffer;
        updated = true;
        println!("{}", format!("✓ Set buffer seconds to {buffer}").green());
    }

    if updated {
        config.save()?;
        println!(
            "{}",
            "✓ Rate limiting configuration saved successfully!".green()
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
                format!("{}...{}", &token[..4], &token[token.len() - 4..])
            } else {
                "***".to_string()
            };
            println!("Token: {masked_token}");
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
