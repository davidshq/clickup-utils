//! # Workspace Commands
//!
//! This module handles all workspace-related operations for the ClickUp CLI.
//! It provides commands for listing and viewing details of workspaces that
//! the authenticated user has access to.
//!
//! ## Commands
//!
//! - **List**: Display all accessible workspaces
//! - **Show**: Show detailed information about a specific workspace
//!
//! ## Features
//!
//! Workspaces are displayed in formatted tables showing key information
//! including member counts, colors, and roles. Detailed workspace views
//! include member lists and role information.

use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::commands::utils::{ApiUtils, CommandExecutor, DisplayUtils, TableBuilder, TableHeaders};
use clap::Subcommand;

/// Workspace command variants
///
/// This enum defines all available workspace subcommands with their
/// associated parameters and help text. Each command variant includes
/// comprehensive help documentation for CLI usage.
#[derive(Subcommand)]
pub enum WorkspaceCommands {
    /// List all workspaces
    List,
    /// Show details of a specific workspace
    Show {
        /// Workspace ID
        #[arg(short, long)]
        id: String,
    },
}

impl CommandExecutor for WorkspaceCommands {
    type Commands = WorkspaceCommands;
    
    async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError> {
        let api = ApiUtils::create_client(config)?;
        Self::handle_command(command, &api).await
    }
    
    async fn handle_command(command: Self::Commands, api: &ClickUpApi) -> Result<(), ClickUpError> {
        match command {
            WorkspaceCommands::List => {
                list_workspaces(api).await?;
            }
            WorkspaceCommands::Show { id } => {
                show_workspace(api, &id).await?;
            }
        }
        Ok(())
    }
}

/// Execute workspace commands
///
/// This function routes workspace commands to their appropriate handlers
/// and manages the overall workspace operations flow.
///
/// # Arguments
///
/// * `command` - The workspace command to execute
/// * `config` - Reference to the application configuration
///
/// # Returns
///
/// Returns `Ok(())` on successful execution, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return various errors including:
/// - Network errors when communicating with the API
/// - Authentication errors if not properly authenticated
/// - Validation errors for invalid parameters
/// - Not found errors for missing workspaces
pub async fn execute(command: WorkspaceCommands, config: &Config) -> Result<(), ClickUpError> {
    WorkspaceCommands::execute(command, config).await
}

/// List all workspaces
///
/// This function retrieves and displays all workspaces accessible to the
/// authenticated user in a formatted table showing key information like
/// member counts and colors.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
///
/// # Returns
///
/// Returns `Ok(())` on successful listing, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::AuthError` if not properly authenticated
async fn list_workspaces(api: &ClickUpApi) -> Result<(), ClickUpError> {
    println!("Fetching workspaces from ClickUp API...");
    let workspaces = api.get_workspaces().await?;
    println!("Received {} workspaces", workspaces.teams.len());

    if workspaces.teams.is_empty() {
        DisplayUtils::display_empty_message("workspaces");
        return Ok(());
    }

    let mut table_builder = TableBuilder::new();
    table_builder.add_header(vec![
        TableHeaders::id(),
        TableHeaders::name(),
        TableHeaders::members(),
        TableHeaders::color(),
    ]);

    for workspace in &workspaces.teams {
        table_builder.add_row(vec![
            workspace.id.clone(),
            workspace.name.as_deref().unwrap_or("").to_string(),
            workspace.members.len().to_string(),
            workspace.color.as_deref().unwrap_or("None").to_string(),
        ]);
    }

    table_builder.print();
    Ok(())
}

/// Show details of a specific workspace
///
/// This function retrieves and displays comprehensive information about
/// a specific workspace including member details and role information.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `workspace_id` - The ID of the workspace to show
///
/// # Returns
///
/// Returns `Ok(())` on successful display, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the workspace doesn't exist
async fn show_workspace(api: &ClickUpApi, workspace_id: &str) -> Result<(), ClickUpError> {
    let workspace = api.get_workspace(workspace_id).await?;

    DisplayUtils::display_details_header("Workspace");
    println!("ID: {}", workspace.id);
    println!("Name: {}", workspace.name.as_deref().unwrap_or(""));
    println!("Color: {}", workspace.color.as_deref().unwrap_or("None"));
    println!("Avatar: {}", workspace.avatar.as_deref().unwrap_or("None"));
    println!("Members: {}", workspace.members.len());

    // Display members
    let members: Vec<(String, Option<String>)> = workspace
        .members
        .iter()
        .map(|m| {
            let username = m.user.username.as_deref().unwrap_or("Unknown user").to_string();
            let email = m.user.email.clone();
            (username, email)
        })
        .collect();
    DisplayUtils::display_members(&members);

    // Display roles
    if let Some(roles) = &workspace.roles {
        if !roles.is_empty() {
            DisplayUtils::display_section_header("Roles");
            for role in roles {
                println!("  - {} (ID: {})", role.name, role.id);
            }
        }
    }

    Ok(())
}
