use clap::{Parser, Subcommand};
use colored::*;
use log::{error, info};

mod api;
mod commands;
mod config;
mod error;
mod models;

use crate::commands::{
    auth, comments, lists, spaces, tasks, teams, workspaces,
};
use crate::config::Config;
use crate::error::ClickUpError;

#[derive(Parser)]
#[command(
    name = "clickup-cli",
    about = "A command-line interface for ClickUp API",
    version,
    author
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable debug logging
    #[arg(long, global = true)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        command: auth::AuthCommands,
    },
    /// Workspace management
    Workspaces {
        #[command(subcommand)]
        command: workspaces::WorkspaceCommands,
    },
    /// Team management
    Teams {
        #[command(subcommand)]
        command: teams::TeamCommands,
    },
    /// Space management
    Spaces {
        #[command(subcommand)]
        command: spaces::SpaceCommands,
    },
    /// List management
    Lists {
        #[command(subcommand)]
        command: lists::ListCommands,
    },
    /// Task management
    Tasks {
        #[command(subcommand)]
        command: tasks::TaskCommands,
    },
    /// Comment management
    Comments {
        #[command(subcommand)]
        command: comments::CommentCommands,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.debug {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("Starting ClickUp CLI...");

    // Load configuration
    let mut config = Config::load().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        e
    })?;

    // Execute command
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