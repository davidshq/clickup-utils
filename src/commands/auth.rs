use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use clap::{Subcommand, Args};
use colored::*;
use log::{error, info};
use std::io::{self, Write};

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Set your ClickUp API token
    Set {
        /// API token (if not provided, will prompt for input)
        #[arg(short, long)]
        token: Option<String>,
    },
    /// Test your current authentication
    Test,
    /// Show current authentication status
    Status,
    /// Clear stored API token
    Clear,
}

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

async fn set_token(token: Option<String>, config: &mut Config) -> Result<(), ClickUpError> {
    let token = match token {
        Some(t) => t,
        None => {
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

    if token.is_empty() {
        return Err(ClickUpError::ValidationError("API token cannot be empty".to_string()));
    }

    config.set_api_token(token)?;
    println!("{}", "✓ API token saved successfully!".green());
    
    Ok(())
}

async fn test_auth(config: &Config) -> Result<(), ClickUpError> {
    if !config.is_authenticated() {
        println!("{}", "✗ No API token configured".red());
        println!("Use 'clickup-cli auth set' to configure your API token");
        return Ok(());
    }

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

fn show_status(config: &Config) -> Result<(), ClickUpError> {
    if config.is_authenticated() {
        println!("{}", "✓ API token is configured".green());
        if let Some(token) = &config.api_token {
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

fn clear_token(config: &mut Config) -> Result<(), ClickUpError> {
    config.api_token = None;
    config.save()?;
    println!("{}", "✓ API token cleared".green());
    Ok(())
} 