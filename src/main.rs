//! # ClickUp CLI
//!
//! A command-line interface for interacting with the ClickUp API.
//!
//! This application provides a comprehensive CLI tool for managing ClickUp workspaces,
//! teams, spaces, lists, tasks, and comments. It supports authentication, workspace
//! management, task creation and updates, and comment management.
//!
//! ## Features
//!
//! - **Authentication**: Secure API token management with local storage
//! - **Workspace Management**: List and manage workspaces and teams
//! - **Space Management**: Navigate and manage spaces within workspaces
//! - **List Management**: View and manage lists within spaces
//! - **Task Management**: Create, read, update, and delete tasks
//! - **Comment Management**: Add and view comments on tasks
//!
//! ## Usage
//!
//! ```bash
//! # Set up authentication
//! clickup-cli auth set
//!
//! # List workspaces
//! clickup-cli workspaces list
//!
//! # Create a task
//! clickup-cli tasks create --list-id <list-id> --name "My Task"
//! ```
//!
//! ## Configuration
//!
//! The application stores configuration in the user's config directory:
//! - Windows: `%APPDATA%\clickup-cli\config.toml`
//! - macOS: `~/Library/Application Support/clickup-cli/config.toml`
//! - Linux: `~/.config/clickup-cli/config.toml`

use clap::{Parser, Subcommand};
use log::{error, info};

mod api;
mod commands;
mod config;
mod error;
mod models;
mod rate_limiter;

use crate::commands::{auth, comments, lists, spaces, tasks, teams, workspaces};
use crate::config::Config;

/// Main CLI application structure
///
/// This struct defines the command-line interface using clap for argument parsing.
/// It supports various subcommands for different ClickUp operations and includes
/// global options like debug logging.
#[derive(Parser)]
#[command(
    name = "clickup-cli",
    about = "A command-line interface for ClickUp API",
    version,
    author
)]
struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    command: Commands,

    /// Enable debug logging for detailed output
    #[arg(long, global = true)]
    debug: bool,
}

/// Available subcommands for the ClickUp CLI
///
/// Each variant represents a different category of operations that can be
/// performed on the ClickUp API.
#[derive(Subcommand)]
enum Commands {
    /// Authentication and API token management
    Auth {
        /// The authentication subcommand to execute
        #[command(subcommand)]
        command: auth::AuthCommands,
    },
    /// Workspace and team management operations
    Workspaces {
        /// The workspace subcommand to execute
        #[command(subcommand)]
        command: workspaces::WorkspaceCommands,
    },
    /// Team management within workspaces
    Teams {
        /// The team subcommand to execute
        #[command(subcommand)]
        command: teams::TeamCommands,
    },
    /// Space management within workspaces
    Spaces {
        /// The space subcommand to execute
        #[command(subcommand)]
        command: spaces::SpaceCommands,
    },
    /// List management within spaces
    Lists {
        /// The list subcommand to execute
        #[command(subcommand)]
        command: lists::ListCommands,
    },
    /// Task management operations
    Tasks {
        /// The task subcommand to execute
        #[command(subcommand)]
        command: tasks::TaskCommands,
    },
    /// Comment management on tasks
    Comments {
        /// The comment subcommand to execute
        #[command(subcommand)]
        command: comments::CommentCommands,
    },
}

/// Main application entry point
///
/// This function initializes the CLI application, sets up logging,
/// loads configuration, and routes commands to their appropriate handlers.
///
/// # Returns
///
/// Returns `Ok(())` on successful execution, or an error wrapped in a box.
///
/// # Errors
///
/// This function can return various errors including:
/// - Configuration loading errors
/// - Command execution errors
/// - Network and API errors
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Initialize logging based on debug flag
    if cli.debug {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("Starting ClickUp CLI...");

    // Load application configuration
    let mut config = Config::load().map_err(|e| {
        error!("Failed to load configuration: {e}");
        e
    })?;

    // Route the command to the appropriate handler
    match cli.command {
        Commands::Auth { command } => {
            auth::execute(command, &mut config).await?;
        }
        Commands::Workspaces { command } => {
            workspaces::execute(command, &config).await?;
        }
        Commands::Teams { command } => {
            teams::execute(command, &config).await?;
        }
        Commands::Spaces { command } => {
            spaces::execute(command, &config).await?;
        }
        Commands::Lists { command } => {
            lists::execute(command, &config).await?;
        }
        Commands::Tasks { command } => {
            tasks::execute(command, &config).await?;
        }
        Commands::Comments { command } => {
            comments::execute(command, &config).await?;
        }
    }

    Ok(())
}
