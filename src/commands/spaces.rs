//! # Space Commands
//!
//! This module handles all space-related operations for the ClickUp CLI.
//! It provides commands for listing, viewing, and backing up spaces and
//! their contents including folders, lists, tasks, and comments.
//!
//! ## Commands
//!
//! - **List**: Display all spaces in a workspace
//! - **Show**: Show detailed information about a specific space
//! - **ListFolders**: Display all folders in a space
//! - **Backup**: Create a comprehensive backup of a space with all content
//!
//! ## Features
//!
//! Spaces are displayed in formatted tables showing key information.
//! The backup feature creates complete JSON backups including all nested
//! content with optional comment inclusion.

use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::{Comment, Folder, List, Space, Task};
use chrono::Utc;
use clap::Subcommand;
use colored::*;
use comfy_table::{Cell, Table};
use serde_json;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Space command variants
///
/// This enum defines all available space subcommands with their
/// associated parameters and help text. Each command variant includes
/// comprehensive help documentation for CLI usage.
#[derive(Subcommand)]
pub enum SpaceCommands {
    /// List all spaces in a workspace
    List {
        /// Workspace ID
        #[arg(short, long)]
        workspace_id: String,
    },
    /// Show details of a specific space
    Show {
        /// Space ID
        #[arg(short, long)]
        id: String,
    },
    /// List all folders in a space
    ListFolders {
        /// Space ID
        #[arg(short, long)]
        space_id: String,
    },
    /// Create a comprehensive backup of a space
    Backup {
        /// Space ID (if not provided, you will be prompted to select)
        #[arg(short, long)]
        space_id: Option<String>,
        /// Output directory for backup files (default: current directory)
        #[arg(short, long, default_value = ".")]
        output_dir: String,
        /// Include task comments in backup
        #[arg(long, default_value = "false")]
        include_comments: bool,
    },
}

/// Execute space commands
///
/// This function routes space commands to their appropriate handlers
/// and manages the overall space operations flow.
///
/// # Arguments
///
/// * `command` - The space command to execute
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
/// - Not found errors for missing spaces or workspaces
/// - File system errors during backup operations
pub async fn execute(command: SpaceCommands, config: &Config) -> Result<(), ClickUpError> {
    let api = ClickUpApi::new(config.clone())?;

    match command {
        SpaceCommands::List { workspace_id } => {
            list_spaces(&api, &workspace_id).await?;
        }
        SpaceCommands::Show { id } => {
            show_space(&api, &id).await?;
        }
        SpaceCommands::ListFolders { space_id } => {
            list_folders(&api, &space_id).await?;
        }
        SpaceCommands::Backup {
            space_id,
            output_dir,
            include_comments,
        } => {
            let space_id = match space_id {
                Some(id) => id,
                None => select_space_interactive(&api).await?,
            };
            backup_space(&api, &space_id, &output_dir, include_comments).await?;
        }
    }
    Ok(())
}

/// List all spaces in a workspace
///
/// This function retrieves and displays all spaces for a specific workspace
/// in a formatted table showing key information like privacy settings and
/// feature configurations.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `workspace_id` - The ID of the workspace to list spaces for
///
/// # Returns
///
/// Returns `Ok(())` on successful listing, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the workspace doesn't exist
async fn list_spaces(api: &ClickUpApi, workspace_id: &str) -> Result<(), ClickUpError> {
    let spaces = api.get_spaces(workspace_id).await?;

    if spaces.spaces.is_empty() {
        println!("{}", "No spaces found".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ID").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Name").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Private").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Statuses").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Multiple Assignees").add_attribute(comfy_table::Attribute::Bold),
    ]);

    for space in &spaces.spaces {
        table.add_row(vec![
            Cell::new(&space.id),
            Cell::new(space.name.as_deref().unwrap_or("")),
            Cell::new(if space.private { "Yes" } else { "No" }),
            Cell::new(space.statuses.len().to_string()),
            Cell::new(if space.multiple_assignees {
                "Yes"
            } else {
                "No"
            }),
        ]);
    }

    println!("{table}");
    Ok(())
}

/// Show detailed information about a specific space
///
/// This function searches for a space across all accessible workspaces
/// and displays its detailed information including features and statuses.
/// Note that this operation may be slow as it searches through all workspaces.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `space_id` - The ID of the space to show
///
/// # Returns
///
/// Returns `Ok(())` on successful display, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the space doesn't exist
async fn show_space(api: &ClickUpApi, space_id: &str) -> Result<(), ClickUpError> {
    // For now, we'll need to search through workspaces to find the space
    // In a real implementation, you might want to store workspace_id in config
    let workspaces = api.get_workspaces().await?;

    for workspace in &workspaces.teams {
        let spaces = api.get_spaces(&workspace.id).await?;
        if let Some(space) = spaces.spaces.into_iter().find(|s| s.id == space_id) {
            println!("{}", "Space Details".bold());
            println!("ID: {}", space.id);
            println!("Name: {}", space.name.as_deref().unwrap_or(""));
            println!("Private: {}", if space.private { "Yes" } else { "No" });
            println!(
                "Multiple Assignees: {}",
                if space.multiple_assignees {
                    "Yes"
                } else {
                    "No"
                }
            );
            println!("Statuses: {}", space.statuses.len());

            if !space.statuses.is_empty() {
                println!("\n{}", "Statuses:".bold());
                for status in &space.statuses {
                    println!("  - {} ({})", status.status, status.type_);
                }
            }

            if let Some(features) = &space.features {
                println!("\n{}", "Features:".bold());
                if let Some(due_dates) = &features.due_dates {
                    println!(
                        "  Due Dates: {}",
                        if due_dates.enabled {
                            "Enabled"
                        } else {
                            "Disabled"
                        }
                    );
                }
                if let Some(time_tracking) = &features.time_tracking {
                    println!(
                        "  Time Tracking: {}",
                        if time_tracking.enabled {
                            "Enabled"
                        } else {
                            "Disabled"
                        }
                    );
                }
                if let Some(custom_fields) = &features.custom_fields {
                    println!(
                        "  Custom Fields: {}",
                        if custom_fields.enabled {
                            "Enabled"
                        } else {
                            "Disabled"
                        }
                    );
                }
            }

            return Ok(());
        }
    }

    Err(ClickUpError::NotFoundError(format!(
        "Space {space_id} not found"
    )))
}

/// List all folders in a space
///
/// This function retrieves and displays all folders for a specific space
/// in a formatted table showing key information like visibility and
/// archival status.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `space_id` - The ID of the space to list folders for
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
async fn list_folders(api: &ClickUpApi, space_id: &str) -> Result<(), ClickUpError> {
    let folders = api.get_folders(space_id).await?;

    if folders.folders.is_empty() {
        println!("{}", "No folders found".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ID").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Name").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Content").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Hidden").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Archived").add_attribute(comfy_table::Attribute::Bold),
    ]);

    for folder in &folders.folders {
        let content = folder.content.as_deref().unwrap_or("");
        let hidden = if folder.hidden.unwrap_or(false) {
            "Yes"
        } else {
            "No"
        };
        let archived = if folder.archived.unwrap_or(false) {
            "Yes"
        } else {
            "No"
        };

        table.add_row(vec![
            Cell::new(&folder.id),
            Cell::new(folder.name.as_deref().unwrap_or("")),
            Cell::new(content),
            Cell::new(hidden),
            Cell::new(archived),
        ]);
    }

    println!("{table}");
    Ok(())
}

/// Backup data structure for a complete space backup
#[derive(serde::Serialize)]
struct SpaceBackup {
    /// Backup metadata
    metadata: BackupMetadata,
    /// Space information
    space: Option<Space>,
    /// Folders in the space
    folders: Vec<Folder>,
    /// Lists in the space (including those in folders)
    lists: Vec<List>,
    /// Tasks in all lists
    tasks: Vec<Task>,
    /// Comments for tasks (if included)
    comments: Vec<TaskComments>,
}

/// Backup metadata
#[derive(serde::Serialize)]
struct BackupMetadata {
    /// Backup creation timestamp
    created_at: String,
    /// Space ID that was backed up
    space_id: String,
    /// Whether comments were included
    include_comments: bool,
    /// Total number of folders
    folder_count: usize,
    /// Total number of lists
    list_count: usize,
    /// Total number of tasks
    task_count: usize,
    /// Total number of comments
    comment_count: usize,
}

/// Task comments structure
#[derive(serde::Serialize)]
struct TaskComments {
    /// Task ID
    task_id: String,
    /// Task name
    task_name: String,
    /// Comments for this task
    comments: Vec<Comment>,
}

/// Create a comprehensive backup of a space
///
/// This function creates a complete backup of a space including all folders,
/// lists, tasks, and optionally comments. The backup is saved as a JSON file
/// with timestamped filename in the specified output directory.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `space_id` - The ID of the space to backup
/// * `output_dir` - Directory to save the backup file
/// * `include_comments` - Whether to include task comments in the backup
///
/// # Returns
///
/// Returns `Ok(())` on successful backup, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if API requests fail or file operations fail
/// - `ClickUpError::NotFoundError` if the space doesn't exist
/// - File system errors if the output directory cannot be created or written to
async fn backup_space(
    api: &ClickUpApi,
    space_id: &str,
    output_dir: &str,
    include_comments: bool,
) -> Result<(), ClickUpError> {
    println!("{}", "Starting space backup...".bold().green());

    // Create output directory if it doesn't exist
    let output_path = Path::new(output_dir);
    if !output_path.exists() {
        fs::create_dir_all(output_path).map_err(|e| {
            ClickUpError::NetworkError(format!("Failed to create output directory: {e}"))
        })?;
    }

    // Get space information
    println!("Fetching space information...");
    let space = get_space_info(api, space_id).await?;

    // Get folders
    println!("Fetching folders...");
    let folders = api.get_folders(space_id).await?;

    // Get lists (both in space and in folders)
    println!("Fetching lists...");
    let mut all_lists = Vec::new();

    // Get lists directly in the space
    let space_lists = api.get_lists(space_id).await?;
    all_lists.extend(space_lists.lists);

    // Get lists in folders
    for folder in &folders.folders {
        let folder_lists = api.get_folder_lists(&folder.id).await?;
        all_lists.extend(folder_lists.lists);
    }

    // Get tasks from all lists
    println!("Fetching tasks...");
    let mut all_tasks = Vec::new();
    let mut all_comments = Vec::new();

    for list in &all_lists {
        println!(
            "  Fetching tasks from list: {}",
            list.name.as_deref().unwrap_or("Unknown")
        );
        let tasks = api.get_tasks(&list.id).await?;
        all_tasks.extend(tasks.tasks.clone());

        // Get comments for each task if requested
        if include_comments {
            for task in &tasks.tasks {
                match api.get_comments(&task.id).await {
                    Ok(comments_response) => {
                        if !comments_response.comments.is_empty() {
                            all_comments.push(TaskComments {
                                task_id: task.id.clone(),
                                task_name: task
                                    .name
                                    .clone()
                                    .unwrap_or_else(|| "Unknown Task".to_string()),
                                comments: comments_response.comments,
                            });
                        }
                    }
                    Err(e) => {
                        println!(
                            "  Warning: Failed to fetch comments for task {}: {}",
                            task.id, e
                        );
                    }
                }
            }
        }
    }

    // Create backup data structure
    let backup = SpaceBackup {
        metadata: BackupMetadata {
            created_at: Utc::now().to_rfc3339(),
            space_id: space_id.to_string(),
            include_comments,
            folder_count: folders.folders.len(),
            list_count: all_lists.len(),
            task_count: all_tasks.len(),
            comment_count: all_comments.len(),
        },
        space: Some(space),
        folders: folders.folders,
        lists: all_lists,
        tasks: all_tasks,
        comments: all_comments,
    };

    // Generate filename with timestamp
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("clickup_space_backup_{space_id}_{timestamp}.json");
    let filepath = output_path.join(&filename);

    // Write backup to file
    println!("Writing backup to file: {}", filepath.display());
    let json_data = serde_json::to_string_pretty(&backup)
        .map_err(|e| ClickUpError::NetworkError(format!("Failed to serialize backup data: {e}")))?;

    fs::write(&filepath, json_data)
        .map_err(|e| ClickUpError::NetworkError(format!("Failed to write backup file: {e}")))?;

    // Print summary
    println!("\n{}", "Backup completed successfully!".bold().green());
    println!("File: {}", filepath.display());
    println!("Space: {}", backup.metadata.space_id);
    println!("Folders: {}", backup.metadata.folder_count);
    println!("Lists: {}", backup.metadata.list_count);
    println!("Tasks: {}", backup.metadata.task_count);
    if include_comments {
        println!("Comments: {}", backup.metadata.comment_count);
    }

    Ok(())
}

/// Helper function to get space information
///
/// This function searches for a space across all accessible workspaces
/// and returns its complete information. Used internally by backup operations.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `space_id` - The ID of the space to retrieve
///
/// # Returns
///
/// Returns the space information on success, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the space doesn't exist
async fn get_space_info(api: &ClickUpApi, space_id: &str) -> Result<Space, ClickUpError> {
    // Search through workspaces to find the space
    let workspaces = api.get_workspaces().await?;

    for workspace in &workspaces.teams {
        let spaces = api.get_spaces(&workspace.id).await?;
        if let Some(space) = spaces.spaces.into_iter().find(|s| s.id == space_id) {
            return Ok(space);
        }
    }

    Err(ClickUpError::NotFoundError(format!(
        "Space {space_id} not found"
    )))
}

/// Prompt the user to select a workspace and then a space interactively
///
/// This function provides an interactive selection process for choosing
/// a space when none is specified. It lists available workspaces and
/// spaces, then prompts for user input.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
///
/// # Returns
///
/// Returns the selected space ID on success, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if no workspaces or spaces are found
/// - `ClickUpError::IoError` if user input fails
async fn select_space_interactive(api: &ClickUpApi) -> Result<String, ClickUpError> {
    // List workspaces
    let workspaces = api.get_workspaces().await?;
    if workspaces.teams.is_empty() {
        return Err(ClickUpError::NotFoundError(
            "No workspaces found".to_string(),
        ));
    }
    println!("\nAvailable Workspaces:");
    for (i, ws) in workspaces.teams.iter().enumerate() {
        println!(
            "  {}. {} (ID: {})",
            i + 1,
            ws.name.as_deref().unwrap_or("Unnamed Workspace"),
            ws.id
        );
    }
    print!("Select a workspace by number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let ws_index: usize = input.trim().parse().unwrap_or(0);
    if ws_index == 0 || ws_index > workspaces.teams.len() {
        return Err(ClickUpError::NotFoundError(
            "Invalid workspace selection".to_string(),
        ));
    }
    let workspace = &workspaces.teams[ws_index - 1];

    // List spaces in the selected workspace
    let spaces = api.get_spaces(&workspace.id).await?;
    if spaces.spaces.is_empty() {
        return Err(ClickUpError::NotFoundError(
            "No spaces found in selected workspace".to_string(),
        ));
    }
    println!(
        "\nAvailable Spaces in '{}':",
        workspace.name.as_deref().unwrap_or("Unnamed Workspace")
    );
    for (i, sp) in spaces.spaces.iter().enumerate() {
        println!(
            "  {}. {} (ID: {})",
            i + 1,
            sp.name.as_deref().unwrap_or("Unnamed Space"),
            sp.id
        );
    }
    print!("Select a space by number: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let sp_index: usize = input.trim().parse().unwrap_or(0);
    if sp_index == 0 || sp_index > spaces.spaces.len() {
        return Err(ClickUpError::NotFoundError(
            "Invalid space selection".to_string(),
        ));
    }
    let space = &spaces.spaces[sp_index - 1];
    Ok(space.id.clone())
}
