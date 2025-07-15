//! # Application Layer
//!
//! This module provides the application layer for the ClickUp CLI binary.
//! It handles CLI-specific logic, command routing, and application lifecycle
//! management, separating binary concerns from the core library functionality.
//!
//! ## Architecture
//!
//! The application layer sits between the CLI interface (main.rs) and the core
//! library functionality, providing a clean separation of concerns:
//!
//! ```
//! main.rs -> app.rs -> library modules (api, commands, etc.)
//! ```

use clickup_cli::{ClickUpApi, Config, ClickUpError, constants};
use clickup_cli::commands::{auth, comments, lists, spaces, tasks, teams, workspaces};
use clap::{Parser, Subcommand};
use log::{error, info, warn};

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
pub struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Commands,

    /// Enable debug logging for detailed output
    #[arg(long, global = true)]
    pub debug: bool,
}

/// Available subcommands for the ClickUp CLI
///
/// Each variant represents a different category of operations that can be
/// performed on the ClickUp API.
#[derive(Subcommand)]
pub enum Commands {
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

/// ClickUp CLI Application
///
/// This struct encapsulates the application state and provides methods for
/// managing the application lifecycle and command execution.
pub struct ClickUpApp {
    /// Application configuration
    config: Config,
    /// API client for making requests
    api: ClickUpApi,
}

impl ClickUpApp {
    /// Creates a new ClickUp CLI application
    ///
    /// This function initializes the application by loading configuration
    /// and creating the API client.
    ///
    /// # Returns
    ///
    /// Returns a new `ClickUpApp` instance on success, or a `ClickUpError` on failure.
    ///
    /// # Errors
    ///
    /// This function can return:
    /// - `ClickUpError::ConfigError` if configuration cannot be loaded
    /// - `ClickUpError::NetworkError` if the API client cannot be created
    pub fn new() -> Result<Self, ClickUpError> {
        info!("Initializing ClickUp CLI application...");
        
        // Load application configuration
        let config = Config::load().map_err(|e| {
            error!("Failed to load configuration: {e}");
            e
        })?;
        
        // Create API client
        let api = ClickUpApi::new(config.clone()).map_err(|e| {
            error!("Failed to create API client: {e}");
            e
        })?;
        
        info!("Application initialized successfully");
        
        Ok(Self { config, api })
    }
    
    /// Runs the application with the given CLI arguments
    ///
    /// This function routes commands to their appropriate handlers and manages
    /// the application lifecycle.
    ///
    /// # Arguments
    ///
    /// * `cli` - The parsed CLI arguments
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful execution, or a `ClickUpError` on failure.
    ///
    /// # Errors
    ///
    /// This function can return various errors including:
    /// - Configuration errors
    /// - Command execution errors
    /// - Network and API errors
    pub async fn run(&mut self, cli: Cli) -> Result<(), ClickUpError> {
        info!("Executing command");
        
        match cli.command {
            Commands::Auth { command } => {
                self.handle_auth(command).await
            }
            Commands::Workspaces { command } => {
                self.handle_workspaces(command).await
            }
            Commands::Teams { command } => {
                self.handle_teams(command).await
            }
            Commands::Spaces { command } => {
                self.handle_spaces(command).await
            }
            Commands::Lists { command } => {
                self.handle_lists(command).await
            }
            Commands::Tasks { command } => {
                self.handle_tasks(command).await
            }
            Commands::Comments { command } => {
                self.handle_comments(command).await
            }
        }
    }
    
    /// Handles authentication commands
    ///
    /// # Arguments
    ///
    /// * `command` - The authentication command to execute
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `ClickUpError` on failure.
    async fn handle_auth(&mut self, command: auth::AuthCommands) -> Result<(), ClickUpError> {
        auth::execute(command, &self.config).await
    }
    
    /// Handles workspace commands
    ///
    /// # Arguments
    ///
    /// * `command` - The workspace command to execute
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `ClickUpError` on failure.
    async fn handle_workspaces(&self, command: workspaces::WorkspaceCommands) -> Result<(), ClickUpError> {
        workspaces::execute(command, &self.config).await
    }
    
    /// Handles team commands
    ///
    /// # Arguments
    ///
    /// * `command` - The team command to execute
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `ClickUpError` on failure.
    async fn handle_teams(&self, command: teams::TeamCommands) -> Result<(), ClickUpError> {
        teams::execute(command, &self.config).await
    }
    
    /// Handles space commands
    ///
    /// # Arguments
    ///
    /// * `command` - The space command to execute
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `ClickUpError` on failure.
    async fn handle_spaces(&self, command: spaces::SpaceCommands) -> Result<(), ClickUpError> {
        spaces::execute(command, &self.config).await
    }
    
    /// Handles list commands
    ///
    /// # Arguments
    ///
    /// * `command` - The list command to execute
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `ClickUpError` on failure.
    async fn handle_lists(&self, command: lists::ListCommands) -> Result<(), ClickUpError> {
        lists::execute(command, &self.config).await
    }
    
    /// Handles task commands
    ///
    /// # Arguments
    ///
    /// * `command` - The task command to execute
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `ClickUpError` on failure.
    async fn handle_tasks(&self, command: tasks::TaskCommands) -> Result<(), ClickUpError> {
        tasks::execute(command, &self.config).await
    }
    
    /// Handles comment commands
    ///
    /// # Arguments
    ///
    /// * `command` - The comment command to execute
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `ClickUpError` on failure.
    async fn handle_comments(&self, command: comments::CommentCommands) -> Result<(), ClickUpError> {
        comments::execute(command, &self.config).await
    }
    
}

/// Application lifecycle management
impl ClickUpApp {
    /// Initializes logging based on debug flag
    ///
    /// # Arguments
    ///
    /// * `debug` - Whether to enable debug logging
    pub fn init_logging(debug: bool) {
        if debug {
            std::env::set_var("RUST_LOG", constants::logging::DEBUG_LOG_LEVEL);
        } else {
            std::env::set_var("RUST_LOG", constants::logging::DEFAULT_LOG_LEVEL);
        }
        env_logger::init();
    }
    
    /// Performs application startup tasks
    ///
    /// This function handles any necessary startup tasks such as
    /// configuration validation, API connectivity checks, etc.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `ClickUpError` on failure.
    pub async fn startup(&self) -> Result<(), ClickUpError> {
        info!("Performing application startup tasks...");
        
        // Validate configuration
        if !self.config.is_authenticated() {
            warn!("No API token configured. Some commands may fail.");
        }
        
        // Test API connectivity if authenticated
        if self.config.is_authenticated() {
            match self.api.get_user().await {
                Ok(user) => {
                    info!("Successfully connected to ClickUp API as: {}", 
                          user.user.username.unwrap_or_else(|| "Unknown".to_string()));
                }
                Err(e) => {
                    warn!("Failed to connect to ClickUp API: {e}");
                }
            }
        }
        
        info!("Startup tasks completed");
        Ok(())
    }
    
    /// Performs application shutdown tasks
    ///
    /// This function handles any necessary cleanup tasks such as
    /// saving configuration, closing connections, etc.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or a `ClickUpError` on failure.
    pub async fn shutdown(&self) -> Result<(), ClickUpError> {
        info!("Performing application shutdown tasks...");
        
        // Save configuration if modified
        if let Err(e) = self.config.save() {
            warn!("Failed to save configuration: {e}");
        }
        
        info!("Shutdown tasks completed");
        Ok(())
    }
} 