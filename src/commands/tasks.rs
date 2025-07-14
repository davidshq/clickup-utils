//! # Task Commands
//!
//! This module handles all task-related operations for the ClickUp CLI.
//! It provides commands for listing, creating, updating, and deleting tasks,
//! as well as advanced features like tag-based filtering and overdue task management.
//!
//! ## Commands
//!
//! - **List**: Display all tasks in a list
//! - **ListByTag**: Display tasks filtered by a specific tag
//! - **SearchByTag**: Search for tasks with a tag across multiple spaces
//! - **UpdateOverdueByTag**: Update overdue tasks with a tag to today's date
//! - **Show**: Show detailed information about a specific task
//! - **Create**: Create a new task with various parameters
//! - **Update**: Update an existing task with new values
//! - **Delete**: Remove a task permanently
//!
//! ## Features
//!
//! Tasks are displayed in formatted tables showing key information including
//! status, priority, due dates, and assignees. Advanced features include
//! tag-based filtering, overdue task management with dry-run support, and
//! comprehensive task details display.

use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::{CreateTaskRequest, UpdateTaskRequest};
use crate::commands::utils::{ApiUtils, CommandExecutor, DisplayUtils, TableBuilder, TableHeaders};
use chrono::NaiveTime;
use clap::Subcommand;
use colored::*;

/// Parameters for creating a task
///
/// This struct encapsulates all the parameters needed to create a new task,
/// providing a clean interface for task creation operations.
struct CreateTaskParams {
    list_id: String,
    name: String,
    description: Option<String>,
    status: Option<String>,
    priority: Option<i64>,
    due_date: Option<i64>,
    time_estimate: Option<i64>,
}

/// Parameters for updating a task
///
/// This struct encapsulates all the parameters needed to update an existing task,
/// providing a clean interface for task update operations.
struct UpdateTaskParams {
    task_id: String,
    name: Option<String>,
    description: Option<String>,
    status: Option<String>,
    priority: Option<i64>,
    due_date: Option<i64>,
    time_estimate: Option<i64>,
}

/// Task command variants
///
/// This enum defines all available task subcommands with their
/// associated parameters and help text. Each command variant includes
/// comprehensive help documentation for CLI usage.
#[derive(Subcommand)]
pub enum TaskCommands {
    /// List all tasks in a list
    List {
        /// List ID
        #[arg(short, long)]
        list_id: String,
    },
    /// List tasks in a list filtered by tag
    ListByTag {
        /// List ID
        #[arg(short, long)]
        list_id: String,
        /// Tag name to filter by
        #[arg(short, long)]
        tag: String,
    },
    /// Search for tasks with a specific tag across all lists in a space
    SearchByTag {
        /// Tag name to search for
        #[arg(short, long)]
        tag: String,
        /// Workspace ID (optional - will prompt if not provided)
        #[arg(short, long)]
        workspace_id: Option<String>,
        /// Space ID (optional - will prompt if not provided)
        #[arg(short, long)]
        space_id: Option<String>,
    },
    /// Update overdue tasks with a specific tag to today's date
    UpdateOverdueByTag {
        /// Tag name to filter by
        #[arg(short, long)]
        tag: String,
        /// Workspace ID (optional - will prompt if not provided)
        #[arg(short, long)]
        workspace_id: Option<String>,
        /// Space ID (optional - will prompt if not provided)
        #[arg(short, long)]
        space_id: Option<String>,
        /// Dry run mode - show what would be updated without making changes
        #[arg(short, long)]
        dry_run: bool,
    },
    /// Show details of a specific task
    Show {
        /// Task ID
        #[arg(short, long)]
        id: String,
    },
    /// Create a new task
    Create {
        /// List ID
        #[arg(short, long)]
        list_id: String,
        /// Task name
        #[arg(short, long)]
        name: String,
        /// Task description
        #[arg(long)]
        description: Option<String>,
        /// Task status
        #[arg(short, long)]
        status: Option<String>,
        /// Priority (1-4)
        #[arg(short, long)]
        priority: Option<i64>,
        /// Due date (Unix timestamp)
        #[arg(short, long)]
        due_date: Option<i64>,
        /// Time estimate (in milliseconds)
        #[arg(short, long)]
        time_estimate: Option<i64>,
    },
    /// Update an existing task
    Update {
        /// Task ID
        #[arg(short, long)]
        id: String,
        /// New task name
        #[arg(short, long)]
        name: Option<String>,
        /// New task description
        #[arg(long)]
        description: Option<String>,
        /// New task status
        #[arg(short, long)]
        status: Option<String>,
        /// New priority (1-4)
        #[arg(short, long)]
        priority: Option<i64>,
        /// New due date (Unix timestamp)
        #[arg(short, long)]
        due_date: Option<i64>,
        /// New time estimate (in milliseconds)
        #[arg(short, long)]
        time_estimate: Option<i64>,
    },
    /// Delete a task
    Delete {
        /// Task ID
        #[arg(short, long)]
        id: String,
    },
}

impl CommandExecutor for TaskCommands {
    type Commands = TaskCommands;
    
    async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError> {
        let api = ApiUtils::create_client(config)?;
        Self::handle_command(command, &api).await
    }
    
    async fn handle_command(command: Self::Commands, api: &ClickUpApi) -> Result<(), ClickUpError> {
        match command {
            TaskCommands::List { list_id } => {
                list_tasks(api, &list_id).await?;
            }
            TaskCommands::ListByTag { list_id, tag } => {
                list_tasks_by_tag(api, &list_id, &tag).await?;
            }
            TaskCommands::SearchByTag {
                tag,
                workspace_id,
                space_id,
            } => {
                search_tasks_by_tag(api, tag, workspace_id, space_id).await?;
            }
            TaskCommands::UpdateOverdueByTag {
                tag,
                workspace_id,
                space_id,
                dry_run,
            } => {
                update_overdue_by_tag(api, tag, workspace_id, space_id, dry_run).await?;
            }
            TaskCommands::Show { id } => {
                show_task(api, &id).await?;
            }
            TaskCommands::Create {
                list_id,
                name,
                description,
                status,
                priority,
                due_date,
                time_estimate,
            } => {
                let params = CreateTaskParams {
                    list_id: list_id.clone(),
                    name,
                    description,
                    status,
                    priority,
                    due_date,
                    time_estimate,
                };
                create_task(api, params).await?;
            }
            TaskCommands::Update {
                id,
                name,
                description,
                status,
                priority,
                due_date,
                time_estimate,
            } => {
                let params = UpdateTaskParams {
                    task_id: id.clone(),
                    name,
                    description,
                    status,
                    priority,
                    due_date,
                    time_estimate,
                };
                update_task(api, params).await?;
            }
            TaskCommands::Delete { id } => {
                delete_task(api, &id).await?;
            }
        }
        Ok(())
    }
}

/// Execute task commands
///
/// This function routes task commands to their appropriate handlers
/// and manages the overall task operations flow.
///
/// # Arguments
///
/// * `command` - The task command to execute
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
/// - Not found errors for missing tasks or lists
/// - Date parsing errors for overdue task operations
pub async fn execute(command: TaskCommands, config: &Config) -> Result<(), ClickUpError> {
    TaskCommands::execute(command, config).await
}

/// List all tasks in a list
///
/// This function retrieves and displays all tasks for a specific list
/// in a formatted table showing key information like status, priority,
/// due dates, and assignees.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `list_id` - The ID of the list to list tasks for
///
/// # Returns
///
/// Returns `Ok(())` on successful listing, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the list doesn't exist
async fn list_tasks(api: &ClickUpApi, list_id: &str) -> Result<(), ClickUpError> {
    let tasks = api.get_tasks(list_id).await?;

    if tasks.tasks.is_empty() {
        DisplayUtils::display_empty_message("tasks");
        return Ok(());
    }

    let mut table_builder = TableBuilder::new();
    table_builder.add_header(vec![
        TableHeaders::id(),
        TableHeaders::name(),
        TableHeaders::status(),
        TableHeaders::priority(),
        TableHeaders::due_date(),
        "Assignees",
    ]);

    for task in &tasks.tasks {
        let priority = task
            .priority
            .as_ref()
            .map(|p| p.priority.as_str())
            .unwrap_or("None");
        let due_date = task.due_date.as_deref().unwrap_or("None");
        let assignees = if task.assignees.is_empty() {
            "None".to_string()
        } else {
            task.assignees
                .iter()
                .map(|a| a.username.clone())
                .collect::<Vec<_>>()
                .join(", ")
        };

        table_builder.add_row(vec![
            task.id.clone(),
            task.name.as_deref().unwrap_or("").to_string(),
            task.status.status.clone(),
            priority.to_string(),
            due_date.to_string(),
            assignees,
        ]);
    }

    table_builder.print();
    Ok(())
}

/// List tasks in a list filtered by tag
///
/// This function retrieves and displays tasks for a specific list that
/// have a particular tag, showing key information in a formatted table.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `list_id` - The ID of the list to search in
/// * `tag` - The tag name to filter by
///
/// # Returns
///
/// Returns `Ok(())` on successful listing, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the list doesn't exist
async fn list_tasks_by_tag(api: &ClickUpApi, list_id: &str, tag: &str) -> Result<(), ClickUpError> {
    println!("{}", format!("Fetching tasks with tag '{tag}'...").blue());
    let tasks = api.get_tasks_by_tag(list_id, tag).await?;

    if tasks.tasks.is_empty() {
        println!("{}", format!("No tasks found with tag '{tag}'").yellow());
        return Ok(());
    }

    let mut table_builder = TableBuilder::new();
    table_builder.add_header(vec![
        TableHeaders::id(),
        TableHeaders::name(),
        TableHeaders::status(),
        TableHeaders::priority(),
        TableHeaders::due_date(),
        "Assignees",
        "Tags",
    ]);

    for task in &tasks.tasks {
        let priority = task
            .priority
            .as_ref()
            .map(|p| p.priority.as_str())
            .unwrap_or("None");
        let due_date = task.due_date.as_deref().unwrap_or("None");
        let assignees = if task.assignees.is_empty() {
            "None".to_string()
        } else {
            task.assignees
                .iter()
                .map(|a| a.username.clone())
                .collect::<Vec<_>>()
                .join(", ")
        };

        let tag_names = task
            .tags
            .iter()
            .filter_map(|t| t.name.as_deref())
            .collect::<Vec<_>>()
            .join(", ");

        table_builder.add_row(vec![
            task.id.clone(),
            task.name.as_deref().unwrap_or("").to_string(),
            task.status.status.clone(),
            priority.to_string(),
            due_date.to_string(),
            assignees,
            tag_names,
        ]);
    }

    println!("{}", format!("Tasks with tag '{tag}':").bold());
    table_builder.print();
    Ok(())
}

/// Search for tasks with a specific tag across all lists in a space
///
/// This function searches for tasks with a particular tag across multiple
/// spaces and lists, displaying results in a formatted table. This is
/// useful for finding tasks across a large workspace.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `tag` - The tag name to search for
/// * `workspace_id` - Optional workspace ID to limit search scope
/// * `space_id` - Optional space ID to limit search scope
///
/// # Returns
///
/// Returns `Ok(())` on successful search, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the workspace or space doesn't exist
async fn search_tasks_by_tag(
    api: &ClickUpApi,
    tag: String,
    workspace_id: Option<String>,
    space_id: Option<String>,
) -> Result<(), ClickUpError> {
    println!(
        "{}",
        format!("Searching for tasks with tag '{tag}'...").blue()
    );
    let tasks = api
        .search_tasks_by_tag(tag.clone(), workspace_id, space_id)
        .await?;

    if tasks.tasks.is_empty() {
        println!("{}", format!("No tasks found with tag '{tag}'").yellow());
        return Ok(());
    }

    let mut table_builder = TableBuilder::new();
    table_builder.add_header(vec![
        TableHeaders::id(),
        TableHeaders::name(),
        TableHeaders::status(),
        TableHeaders::priority(),
        TableHeaders::due_date(),
        "Assignees",
        "Tags",
    ]);

    for task in &tasks.tasks {
        let priority = task
            .priority
            .as_ref()
            .map(|p| p.priority.as_str())
            .unwrap_or("None");
        let due_date = task.due_date.as_deref().unwrap_or("None");
        let assignees = if task.assignees.is_empty() {
            "None".to_string()
        } else {
            task.assignees
                .iter()
                .map(|a| a.username.clone())
                .collect::<Vec<_>>()
                .join(", ")
        };

        let tag_names = task
            .tags
            .iter()
            .filter_map(|t| t.name.as_deref())
            .collect::<Vec<_>>()
            .join(", ");

        table_builder.add_row(vec![
            task.id.clone(),
            task.name.as_deref().unwrap_or("").to_string(),
            task.status.status.clone(),
            priority.to_string(),
            due_date.to_string(),
            assignees,
            tag_names,
        ]);
    }

    println!("{}", format!("Tasks with tag '{tag}':").bold());
    table_builder.print();
    Ok(())
}

/// Update overdue tasks with a specific tag to today's date
///
/// This function finds tasks with a specific tag that are overdue and
/// updates their due dates to today while preserving the original time.
/// Supports dry-run mode to preview changes without making them.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `tag` - The tag name to filter by
/// * `workspace_id` - Optional workspace ID to limit search scope
/// * `space_id` - Optional space ID to limit search scope
/// * `dry_run` - Whether to show what would be updated without making changes
///
/// # Returns
///
/// Returns `Ok(())` on successful operation, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::ValidationError` if date parsing fails
/// - `ClickUpError::NotFoundError` if the workspace or space doesn't exist
async fn update_overdue_by_tag(
    api: &ClickUpApi,
    tag: String,
    workspace_id: Option<String>,
    space_id: Option<String>,
    dry_run: bool,
) -> Result<(), ClickUpError> {
    println!(
        "{}",
        format!("Searching for overdue tasks with tag '{tag}'...").blue()
    );
    let tasks = api
        .search_tasks_by_tag(tag.clone(), workspace_id, space_id)
        .await?;

    if tasks.tasks.is_empty() {
        println!(
            "{}",
            format!("No tasks found with tag '{tag}' to check.").yellow()
        );
        return Ok(());
    }

    let mut updated_count = 0;
    let mut checked_count = 0;

    for task in &tasks.tasks {
        checked_count += 1;

        if let Some(due_date_str) = &task.due_date {
            // Try to parse as Unix timestamp first (milliseconds)
            let due_date_time = if let Ok(timestamp_ms) = due_date_str.parse::<i64>() {
                // Convert milliseconds to DateTime
                chrono::DateTime::from_timestamp_millis(timestamp_ms).ok_or_else(|| {
                    ClickUpError::ValidationError(format!("Invalid timestamp: {timestamp_ms}"))
                })?
            } else {
                // Try to parse as RFC3339 string format and convert to Utc
                chrono::DateTime::parse_from_rfc3339(due_date_str)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .map_err(|e| {
                        ClickUpError::ValidationError(format!(
                            "Could not parse due date '{due_date_str}': {e}"
                        ))
                    })?
            };

            let today = chrono::Utc::now();

            if due_date_time < today {
                println!(
                    "{}",
                    format!(
                        "Task '{}' (ID: {}) is overdue (due: {}).",
                        task.name.as_deref().unwrap_or("Unnamed"),
                        task.id,
                        due_date_str
                    )
                    .red()
                );

                if !dry_run {
                    // Preserve the original time from the due date, but set the date to today
                    let today_date = chrono::Utc::now().date_naive();
                    let original_time = due_date_time.time();

                    // Create new datetime with today's date and original time
                    let new_due_date = chrono::NaiveDateTime::new(today_date, original_time);
                    let new_due_date_utc =
                        chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                            new_due_date,
                            chrono::Utc,
                        );
                    let new_due_timestamp = new_due_date_utc.timestamp_millis();

                    // Determine if the original due date had time (not just date)
                    // Check if the time is not midnight (00:00:00)
                    let original_time = due_date_time.time();
                    let original_had_time =
                        original_time != NaiveTime::from_hms_opt(0, 0, 0).unwrap();

                    let update_data = UpdateTaskRequest {
                        name: None,
                        description: None,
                        status: Some(task.status.status.clone()), // Required by API
                        priority: None,
                        due_date: Some(new_due_timestamp), // Today's date with original time
                        due_date_time: Some(original_had_time), // Only true if original had time
                        time_estimate: None,
                        assignees: None,
                        tags: None,
                        parent: None,
                        custom_fields: None,
                        start_date: None,
                        start_date_time: None,
                        points: None,
                        notify_all: None,
                    };

                    match api.update_task(&task.id, update_data).await {
                        Ok(_) => {
                            println!(
                                "{}",
                                format!(
                                    "✓ Updated task '{}' due date to today.",
                                    task.name.as_deref().unwrap_or("Unnamed")
                                )
                                .green()
                            );
                            updated_count += 1;
                        }
                        Err(e) => {
                            println!(
                                "{}",
                                format!(
                                    "✗ Failed to update task '{}': {}",
                                    task.name.as_deref().unwrap_or("Unnamed"),
                                    e
                                )
                                .red()
                            );
                        }
                    }
                } else {
                    println!(
                        "{}",
                        format!(
                            "[DRY RUN] Would update task '{}' due date to today.",
                            task.name.as_deref().unwrap_or("Unnamed")
                        )
                        .yellow()
                    );
                    updated_count += 1;
                }
            } else {
                println!(
                    "{}",
                    format!(
                        "Task '{}' (ID: {}) is not overdue (due: {}).",
                        task.name.as_deref().unwrap_or("Unnamed"),
                        task.id,
                        due_date_str
                    )
                    .green()
                );
            }
        } else {
            println!(
                "{}",
                format!(
                    "Task '{}' (ID: {}) has no due date.",
                    task.name.as_deref().unwrap_or("Unnamed"),
                    task.id
                )
                .blue()
            );
        }
    }

    println!("\n{}", "Summary:".bold());
    println!("  Tasks checked: {checked_count}");
    println!("  Tasks updated: {updated_count}");

    if updated_count > 0 {
        if dry_run {
            println!(
                "{}",
                format!(
                    "[DRY RUN] Would have updated {updated_count} overdue tasks with tag '{tag}'."
                )
                .yellow()
            );
        } else {
            println!(
                "{}",
                format!("✓ Successfully updated {updated_count} overdue tasks with tag '{tag}'.")
                    .green()
            );
        }
    } else {
        println!(
            "{}",
            format!("No overdue tasks found with tag '{tag}' to update.").yellow()
        );
    }

    Ok(())
}

/// Show details of a specific task
///
/// This function retrieves and displays comprehensive information about
/// a specific task including all metadata, assignees, tags, and checklists.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `task_id` - The ID of the task to show
///
/// # Returns
///
/// Returns `Ok(())` on successful display, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the task doesn't exist
async fn show_task(api: &ClickUpApi, task_id: &str) -> Result<(), ClickUpError> {
    let task = api.get_task(task_id).await?;

    println!("{}", "Task Details".bold());
    println!("ID: {}", task.id);
    println!("Name: {}", task.name.as_deref().unwrap_or(""));
    println!("Status: {} ({})", task.status.status, task.status.color);
    println!("Created: {}", task.date_created);
    println!("Updated: {}", task.date_updated);

    println!("Description: {}", task.description);

    if let Some(priority) = &task.priority {
        println!("Priority: {} ({})", priority.priority, priority.color);
    }

    if let Some(due_date) = &task.due_date {
        println!("Due Date: {due_date}");
    }

    if let Some(start_date) = &task.start_date {
        println!("Start Date: {start_date}");
    }

    if let Some(time_estimate) = task.time_estimate {
        println!("Time Estimate: {time_estimate} ms");
    }

    if let Some(time_spent) = task.time_spent {
        println!("Time Spent: {time_spent} ms");
    }

    if !task.assignees.is_empty() {
        println!("\n{}", "Assignees:".bold());
        for assignee in &task.assignees {
            println!("  - {} ({})", assignee.username, assignee.id);
        }
    }

    if !task.tags.is_empty() {
        println!("\n{}", "Tags:".bold());
        for tag in &task.tags {
            println!(
                "  - {} ({}/{})",
                tag.name.as_deref().unwrap_or(""),
                tag.tag_fg,
                tag.tag_bg
            );
        }
    }

    if !task.checklists.is_empty() {
        println!("\n{}", "Checklists:".bold());
        for checklist in &task.checklists {
            println!(
                "    - {} [{}]",
                checklist.name.as_deref().unwrap_or(""),
                checklist.resolved.unwrap_or(false)
            );
        }
    }

    println!("URL: {}", task.url);

    Ok(())
}

/// Create a new task
///
/// This function creates a new task with the specified parameters including
/// name, description, status, priority, due date, and time estimates.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `params` - Task creation parameters
///
/// # Returns
///
/// Returns `Ok(())` on successful creation, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::ValidationError` if required parameters are missing
/// - `ClickUpError::NotFoundError` if the list doesn't exist
async fn create_task(api: &ClickUpApi, params: CreateTaskParams) -> Result<(), ClickUpError> {
    let task_data = CreateTaskRequest {
        name: params.name,
        description: params.description,
        status: params.status,
        priority: params.priority,
        due_date: params.due_date,
        due_date_time: params.due_date.is_some().then_some(true),
        time_estimate: params.time_estimate,
        assignees: None,
        tags: None,
        parent: None,
        custom_fields: None,
        start_date: None,
        start_date_time: None,
        points: None,
        notify_all: None,
    };

    let task = api.create_task(&params.list_id, task_data).await?;

    println!("{}", "✓ Task created successfully!".green());
    println!("ID: {}", task.id);
    println!("Name: {}", task.name.as_deref().unwrap_or(""));
    println!("Status: {}", task.status.status);
    println!("URL: {}", task.url);

    Ok(())
}

/// Update an existing task
///
/// This function updates an existing task with new values for any of the
/// provided parameters. Only the specified fields will be updated.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `params` - Task update parameters
///
/// # Returns
///
/// Returns `Ok(())` on successful update, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::ValidationError` if invalid parameters are provided
/// - `ClickUpError::NotFoundError` if the task doesn't exist
async fn update_task(api: &ClickUpApi, params: UpdateTaskParams) -> Result<(), ClickUpError> {
    let task_data = UpdateTaskRequest {
        name: params.name,
        description: params.description,
        status: params.status,
        priority: params.priority,
        due_date: params.due_date,
        due_date_time: params.due_date.is_some().then_some(true),
        time_estimate: params.time_estimate,
        assignees: None,
        tags: None,
        parent: None,
        custom_fields: None,
        start_date: None,
        start_date_time: None,
        points: None,
        notify_all: None,
    };

    let task = api.update_task(&params.task_id, task_data).await?;

    println!("{}", "✓ Task updated successfully!".green());
    println!("ID: {}", task.id);
    println!("Name: {}", task.name.as_deref().unwrap_or(""));
    println!("Status: {}", task.status.status);
    println!("URL: {}", task.url);

    Ok(())
}

/// Delete a task
///
/// This function permanently removes a task from ClickUp.
/// This action cannot be undone.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `task_id` - The ID of the task to delete
///
/// # Returns
///
/// Returns `Ok(())` on successful deletion, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the task doesn't exist
async fn delete_task(api: &ClickUpApi, task_id: &str) -> Result<(), ClickUpError> {
    api.delete_task(task_id).await?;

    println!("{}", "✓ Task deleted successfully!".green());
    println!("Deleted task ID: {task_id}");

    Ok(())
}
