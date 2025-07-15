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

use crate::config::Config;
use crate::error::ClickUpError;
use crate::repository::ClickUpRepository;
use crate::commands::utils::{CommandExecutor, DisplayUtils, ErrorUtils, TableBuilder, TableHeaders};
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
    /// Show detailed information about a specific list
    Show {
        /// List ID
        #[arg(short, long)]
        list_id: String,
    },
}

impl CommandExecutor for ListCommands {
    type Commands = ListCommands;
    
    async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError> {
        let repo = crate::repository::RepositoryFactory::create(config)?;
        Self::handle_command(command, &*repo).await
    }
    
    async fn handle_command(command: Self::Commands, repo: &dyn ClickUpRepository) -> Result<(), ClickUpError> {
        match command {
            ListCommands::List { space_id } => {
                list_lists(repo, &space_id).await?;
            }
            ListCommands::Show { list_id } => {
                show_list(repo, &list_id).await?;
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
/// * `repo` - Reference to the ClickUp repository
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
async fn list_lists(repo: &dyn ClickUpRepository, space_id: &str) -> Result<(), ClickUpError> {
    let lists = repo.get_lists(space_id).await?;

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
/// This function retrieves and displays detailed information about a
/// specific list, including its name, content, task count, and folder.
///
/// # Arguments
///
/// * `repo` - Reference to the ClickUp repository
/// * `list_id` - The ID of the list to show
///
/// # Returns
///
/// Returns `Ok(())` on successful showing, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the list doesn't exist
async fn show_list(repo: &dyn ClickUpRepository, list_id: &str) -> Result<(), ClickUpError> {
    let list = repo.get_list(list_id).await?;

    let mut table_builder = TableBuilder::new();
    table_builder.add_header(vec![
        TableHeaders::id(),
        TableHeaders::name(),
        TableHeaders::content(),
        TableHeaders::task_count(),
        TableHeaders::folder(),
    ]);

    table_builder.add_row(vec![
        list.id.clone(),
        list.name.as_deref().unwrap_or("").to_string(),
        list.content.as_deref().unwrap_or("").to_string(),
        list.task_count.map_or("".to_string(), |c| c.to_string()),
        list.folder
            .as_ref()
            .map(|f| f.name.as_str())
            .unwrap_or("None")
            .to_string(),
    ]);

    table_builder.print();
    Ok(())
}


