//! # Team Commands
//!
//! This module handles all team-related operations for the ClickUp CLI.
//! It provides commands for listing and viewing details of teams within
//! ClickUp workspaces.
//!
//! ## Commands
//!
//! - **List**: Display all teams in accessible workspaces
//! - **Show**: Show detailed information about a specific team
//!
//! ## Features
//!
//! Teams are displayed in formatted tables showing key information
//! including member counts, colors, and roles. Detailed team views
//! include member lists and role information.

use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::commands::utils::{ApiUtils, CommandExecutor, DisplayUtils, ErrorUtils, TableBuilder, TableHeaders};
use clap::Subcommand;

/// Team command variants
///
/// This enum defines all available team subcommands with their
/// associated parameters and help text. Each command variant includes
/// comprehensive help documentation for CLI usage.
#[derive(Subcommand)]
pub enum TeamCommands {
    /// List all teams
    List,
    /// Show details of a specific team
    Show {
        /// Team ID
        #[arg(short, long)]
        id: String,
    },
}

impl CommandExecutor for TeamCommands {
    type Commands = TeamCommands;
    
    async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError> {
        let api = ApiUtils::create_client(config)?;
        Self::handle_command(command, &api).await
    }
    
    async fn handle_command(command: Self::Commands, api: &ClickUpApi) -> Result<(), ClickUpError> {
        match command {
            TeamCommands::List => {
                list_teams(api).await?;
            }
            TeamCommands::Show { id } => {
                show_team(api, &id).await?;
            }
        }
        Ok(())
    }
}

/// Execute team commands
///
/// This function routes team commands to their appropriate handlers
/// and manages the overall team operations flow.
///
/// # Arguments
///
/// * `command` - The team command to execute
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
/// - Not found errors for missing teams
pub async fn execute(command: TeamCommands, config: &Config) -> Result<(), ClickUpError> {
    TeamCommands::execute(command, config).await
}

/// List all teams
///
/// This function retrieves and displays all teams from accessible workspaces
/// in a formatted table showing key information like member counts and colors.
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
async fn list_teams(api: &ClickUpApi) -> Result<(), ClickUpError> {
    let workspaces = api.get_workspaces().await?;

    if workspaces.teams.is_empty() {
        DisplayUtils::display_empty_message("teams");
        return Ok(());
    }

    let mut table_builder = TableBuilder::new();
    table_builder.add_header(vec![
        TableHeaders::id(),
        TableHeaders::name(),
        TableHeaders::members(),
        TableHeaders::color(),
    ]);

    for team in &workspaces.teams {
        table_builder.add_row(vec![
            team.id.clone(),
            team.name.as_deref().unwrap_or("").to_string(),
            team.members.len().to_string(),
            team.color.as_deref().unwrap_or("None").to_string(),
        ]);
    }

    table_builder.print();
    Ok(())
}

/// Show details of a specific team
///
/// This function retrieves and displays comprehensive information about
/// a specific team including member details and role information.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `team_id` - The ID of the team to show
///
/// # Returns
///
/// Returns `Ok(())` on successful display, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the team doesn't exist
async fn show_team(api: &ClickUpApi, team_id: &str) -> Result<(), ClickUpError> {
    let workspaces = api.get_workspaces().await?;

    let team = workspaces
        .teams
        .into_iter()
        .find(|t| t.id == team_id)
        .ok_or_else(|| ErrorUtils::not_found_error("Team", team_id))?;

    DisplayUtils::display_details_header("Team");
    println!("ID: {}", team.id);
    println!("Name: {}", team.name.as_deref().unwrap_or(""));
    println!("Color: {}", team.color.as_deref().unwrap_or("None"));
    println!("Avatar: {}", team.avatar.as_deref().unwrap_or("None"));
    println!("Members: {}", team.members.len());

    // Display members
    let members: Vec<(String, Option<String>)> = team
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
    if let Some(roles) = &team.roles {
        if !roles.is_empty() {
            DisplayUtils::display_section_header("Roles");
            for role in roles {
                println!("  - {} (ID: {})", role.name, role.id);
            }
        }
    }

    Ok(())
}
