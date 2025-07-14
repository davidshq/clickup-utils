//! # List Commands
//!
//! This module handles all list-related operations for the ClickUp CLI.
//! It provides commands for listing and viewing details of lists within
//! ClickUp spaces.
//!
//! ## Commands
//!
//! - **List**: Display all lists in a specific space
//! - **Show**: Show detailed information about a specific list
//!
//! ## Features
//!
//! Lists are displayed in a formatted table showing key information
//! including task counts, folder organization, and content descriptions.

use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::commands::utils::{ApiUtils, CommandExecutor, DisplayUtils, ErrorUtils, TableBuilder, TableHeaders};
use clap::Subcommand;

/// List command variants
///
/// This enum defines all available list subcommands with their
/// associated parameters and help text.
#[derive(Subcommand)]
pub enum ListCommands {
    /// List all lists in a space
    List {
        /// Space ID
        #[arg(short, long)]
        space_id: String,
    },
    /// Show details of a specific list
    Show {
        /// List ID
        #[arg(short, long)]
        id: String,
    },
}

impl CommandExecutor for ListCommands {
    type Commands = ListCommands;
    
    async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError> {
        let api = ApiUtils::create_client(config)?;
        Self::handle_command(command, &api).await
    }
    
    async fn handle_command(command: Self::Commands, api: &ClickUpApi) -> Result<(), ClickUpError> {
        match command {
            ListCommands::List { space_id } => {
                list_lists(api, &space_id).await?;
            }
            ListCommands::Show { id } => {
                show_list(api, &id).await?;
            }
        }
        Ok(())
    }
}

/// Execute list commands
///
/// This function routes list commands to their appropriate handlers
/// and manages the overall list operations flow.
///
/// # Arguments
///
/// * `command` - The list command to execute
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
/// - Not found errors for missing lists or spaces
pub async fn execute(command: ListCommands, config: &Config) -> Result<(), ClickUpError> {
    ListCommands::execute(command, config).await
}

/// List all lists in a space
///
/// This function retrieves and displays all lists for a specific space
/// in a formatted table showing key information like task counts and
/// folder organization.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `space_id` - The ID of the space to list lists for
///
/// # Returns
///
/// Returns `Ok(())` on successful listing, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the space doesn't exist
async fn list_lists(api: &ClickUpApi, space_id: &str) -> Result<(), ClickUpError> {
    let lists = api.get_lists(space_id).await?;

    if lists.lists.is_empty() {
        DisplayUtils::display_empty_message("lists");
        return Ok(());
    }

    let mut table_builder = TableBuilder::new();
    table_builder.add_header(vec![
        TableHeaders::id(),
        TableHeaders::name(),
        TableHeaders::content(),
        TableHeaders::task_count(),
        TableHeaders::folder(),
    ]);

    for list in &lists.lists {
        let folder_name = list
            .folder
            .as_ref()
            .map(|f| f.name.as_str())
            .unwrap_or("None");
        let task_count = list.task_count.map_or("".to_string(), |c| c.to_string());
        let content = list.content.as_deref().unwrap_or("");

        table_builder.add_row(vec![
            list.id.clone(),
            list.name.as_deref().unwrap_or("").to_string(),
            content.to_string(),
            task_count,
            folder_name.to_string(),
        ]);
    }

    table_builder.print();
    Ok(())
}

/// Show detailed information about a specific list
///
/// This function searches for a list across all accessible workspaces
/// and displays its detailed information. Note that this operation may
/// be slow as it searches through all spaces.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `list_id` - The ID of the list to show
///
/// # Returns
///
/// Returns `Ok(())` on successful display, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the list doesn't exist
async fn show_list(api: &ClickUpApi, list_id: &str) -> Result<(), ClickUpError> {
    // For now, we'll need to search through spaces to find the list
    // In a real implementation, you might want to store space_id in config
    let workspaces = api.get_workspaces().await?;

    for workspace in &workspaces.teams {
        let spaces = api.get_spaces(&workspace.id).await?;
        for space in &spaces.spaces {
            let lists = api.get_lists(&space.id).await?;
            if let Some(list) = lists.lists.into_iter().find(|l| l.id == list_id) {
                DisplayUtils::display_details_header("List");
                println!("ID: {}", list.id);
                println!("Name: {}", list.name.as_deref().unwrap_or(""));
                println!("Content: {}", list.content.as_deref().unwrap_or(""));
                println!("Order Index: {}", list.orderindex);
                println!("Task Count: {}", list.task_count.map_or(0, |c| c));

                if let Some(folder) = &list.folder {
                    println!("Folder: {} ({})", folder.name, folder.id);
                }

                println!("Space: {} ({})", list.space.name, list.space.id);

                return Ok(());
            }
        }
    }

    Err(ErrorUtils::not_found_error("List", list_id))
}
