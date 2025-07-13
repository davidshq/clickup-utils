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
use clap::Subcommand;
use colored::*;
use comfy_table::{Cell, Table};

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
    let api = ClickUpApi::new(config.clone())?;

    match command {
        WorkspaceCommands::List => {
            list_workspaces(&api).await?;
        }
        WorkspaceCommands::Show { id } => {
            show_workspace(&api, &id).await?;
        }
    }
    Ok(())
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
        println!("{}", "No workspaces found".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ID").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Name").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Members").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Color").add_attribute(comfy_table::Attribute::Bold),
    ]);

    for workspace in &workspaces.teams {
        table.add_row(vec![
            Cell::new(&workspace.id),
            Cell::new(workspace.name.as_deref().unwrap_or("")),
            Cell::new(workspace.members.len().to_string()),
            Cell::new(workspace.color.as_deref().unwrap_or("None")),
        ]);
    }

    println!("{table}");
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

    println!("{}", "Workspace Details".bold());
    println!("ID: {}", workspace.id);
    println!("Name: {}", workspace.name.as_deref().unwrap_or(""));
    println!("Color: {}", workspace.color.as_deref().unwrap_or("None"));
    println!("Avatar: {}", workspace.avatar.as_deref().unwrap_or("None"));
    println!("Members: {}", workspace.members.len());

    if !workspace.members.is_empty() {
        println!("\n{}", "Members:".bold());
        for member in &workspace.members {
            if let (Some(username), Some(email)) = (&member.user.username, &member.user.email) {
                println!("  - {username} ({email})");
            } else if let Some(username) = &member.user.username {
                println!("  - {username} (no email)");
            } else {
                println!("  - Unknown user");
            }
        }
    }

    if let Some(roles) = &workspace.roles {
        if !roles.is_empty() {
            println!("\n{}", "Roles:".bold());
            for role in roles {
                println!("  - {} (ID: {})", role.name, role.id);
            }
        }
    }

    Ok(())
}
