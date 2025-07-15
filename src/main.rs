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

use clap::Parser;
use log::{error, info};

mod app;

use app::{ClickUpApp, Cli};

/// Main application entry point
///
/// This function initializes the CLI application, sets up logging,
/// creates the application instance, and runs the command.
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
    ClickUpApp::init_logging(cli.debug);

    info!("Starting ClickUp CLI...");

    // Create and run application
    let mut app = ClickUpApp::new().map_err(|e| {
        error!("Failed to initialize application: {e}");
        e
    })?;

    // Perform startup tasks
    app.startup().await.map_err(|e| {
        error!("Startup failed: {e}");
        e
    })?;

    // Run the application
    app.run(cli).await.map_err(|e| {
        error!("Application error: {e}");
        e
    })?;

    // Perform shutdown tasks
    app.shutdown().await.map_err(|e| {
        error!("Shutdown failed: {e}");
        e
    })?;

    info!("ClickUp CLI completed successfully");
    Ok(())
}
