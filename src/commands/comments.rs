//! # Comment Commands
//!
//! This module handles all comment-related operations for the ClickUp CLI.
//! It provides commands for listing, creating, updating, and deleting comments
//! on tasks.
//!
//! ## Commands
//!
//! - **List**: Display all comments for a specific task
//! - **Show**: Show detailed information about a specific comment
//! - **Create**: Add a new comment to a task
//! - **Update**: Modify an existing comment
//! - **Delete**: Remove a comment from a task
//!
//! ## Features
//!
//! Comments support rich text formatting, user mentions, and can be assigned
//! to specific team members. Comments are displayed in a formatted table
//! for easy reading.

use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::CreateCommentRequest;
use clap::Subcommand;
use colored::*;
use comfy_table::{Cell, Table};

/// Comment command variants
///
/// This enum defines all available comment subcommands with their
/// associated parameters and help text.
#[derive(Subcommand)]
pub enum CommentCommands {
    /// List all comments for a task
    List {
        /// Task ID
        #[arg(short, long)]
        task_id: String,
    },
    /// Show details of a specific comment
    Show {
        /// Comment ID
        #[arg(short, long)]
        id: String,
    },
    /// Create a new comment on a task
    Create {
        /// Task ID
        #[arg(short = 'i', long)]
        task_id: String,
        /// Comment text
        #[arg(short = 't', long)]
        text: String,
        /// Assignee ID (optional)
        #[arg(short, long)]
        assignee: Option<i64>,
        /// Notify all team members
        #[arg(short, long)]
        notify_all: Option<bool>,
    },
    /// Update an existing comment
    Update {
        /// Comment ID
        #[arg(short, long)]
        id: String,
        /// New comment text
        #[arg(short = 't', long)]
        text: String,
        /// Assignee ID (optional)
        #[arg(short, long)]
        assignee: Option<i64>,
        /// Notify all team members
        #[arg(short, long)]
        notify_all: Option<bool>,
    },
    /// Delete a comment
    Delete {
        /// Comment ID
        #[arg(short, long)]
        id: String,
    },
}

/// Execute comment commands
///
/// This function routes comment commands to their appropriate handlers
/// and manages the overall comment operations flow.
///
/// # Arguments
///
/// * `command` - The comment command to execute
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
/// - Not found errors for missing comments or tasks
pub async fn execute(command: CommentCommands, config: &Config) -> Result<(), ClickUpError> {
    let api = ClickUpApi::new(config.clone())?;

    match command {
        CommentCommands::List { task_id } => {
            list_comments(&api, &task_id).await?;
        }
        CommentCommands::Show { id } => {
            show_comment(&api, &id).await?;
        }
        CommentCommands::Create {
            task_id,
            text,
            assignee,
            notify_all,
        } => {
            create_comment(&api, &task_id, text, assignee, notify_all).await?;
        }
        CommentCommands::Update {
            id,
            text,
            assignee,
            notify_all,
        } => {
            update_comment(&api, &id, text, assignee, notify_all).await?;
        }
        CommentCommands::Delete { id } => {
            delete_comment(&api, &id).await?;
        }
    }
    Ok(())
}

/// List all comments for a task
///
/// This function retrieves and displays all comments for a specific task
/// in a formatted table. Long comments are truncated for display.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `task_id` - The ID of the task to list comments for
///
/// # Returns
///
/// Returns `Ok(())` on successful listing, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the task doesn't exist
async fn list_comments(api: &ClickUpApi, task_id: &str) -> Result<(), ClickUpError> {
    let comments = api.get_comments(task_id).await?;

    if comments.comments.is_empty() {
        println!("{}", "No comments found".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ID").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("User").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Comment").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Created").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Resolved").add_attribute(comfy_table::Attribute::Bold),
    ]);

    for comment in &comments.comments {
        let comment_text = if comment.comment_text.len() > 50 {
            format!("{}...", &comment.comment_text[..50])
        } else {
            comment.comment_text.clone()
        };

        table.add_row(vec![
            Cell::new(&comment.id),
            Cell::new(&comment.user.username),
            Cell::new(&comment_text),
            Cell::new(&comment.date),
            Cell::new(if comment.resolved { "Yes" } else { "No" }),
        ]);
    }

    println!("{table}");
    Ok(())
}

/// Show detailed information about a specific comment
///
/// This function searches for a comment across all accessible workspaces
/// and displays its detailed information. Note that this operation may
/// be slow as it searches through all tasks.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `comment_id` - The ID of the comment to show
///
/// # Returns
///
/// Returns `Ok(())` on successful display, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the comment doesn't exist
async fn show_comment(api: &ClickUpApi, comment_id: &str) -> Result<(), ClickUpError> {
    // For now, we'll need to search through tasks to find the comment
    // In a real implementation, you might want to store task_id in config
    let workspaces = api.get_workspaces().await?;

    for workspace in &workspaces.teams {
        let spaces = api.get_spaces(&workspace.id).await?;
        for space in &spaces.spaces {
            let lists = api.get_lists(&space.id).await?;
            for list in &lists.lists {
                let tasks = api.get_tasks(&list.id).await?;
                for task in &tasks.tasks {
                    let comments = api.get_comments(&task.id).await?;
                    if let Some(comment) =
                        comments.comments.into_iter().find(|c| c.id == comment_id)
                    {
                        println!("{}", "Comment Details".bold());
                        println!("ID: {}", comment.id);
                        println!("Task: {} ({})", task.name.as_deref().unwrap_or(""), task.id);
                        println!("User: {} ({})", comment.user.username, comment.user.id);
                        println!("Created: {}", comment.date);
                        println!("Updated: {}", comment.date);
                        println!("Resolved: {}", if comment.resolved { "Yes" } else { "No" });
                        println!("Text: {}", comment.comment_text);

                        if let Some(assignee) = &comment.assignee {
                            println!("Assignee: {assignee:?}");
                        }

                        if let Some(assignee_by) = &comment.assignee_by {
                            println!("Assigned by: {assignee_by:?}");
                        }

                        return Ok(());
                    }
                }
            }
        }
    }

    Err(ClickUpError::NotFoundError(format!(
        "Comment {comment_id} not found"
    )))
}

/// Update an existing comment
///
/// This function updates a comment with new text and optional parameters.
/// Only the provided fields will be updated.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `comment_id` - The ID of the comment to update
/// * `text` - The new comment text
/// * `assignee` - Optional assignee ID to mention
/// * `notify_all` - Optional flag to notify all team members
///
/// # Returns
///
/// Returns `Ok(())` on successful update, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the comment doesn't exist
/// - `ClickUpError::ValidationError` if the text is empty
async fn update_comment(
    api: &ClickUpApi,
    comment_id: &str,
    text: String,
    assignee: Option<i64>,
    notify_all: Option<bool>,
) -> Result<(), ClickUpError> {
    let comment_data = CreateCommentRequest {
        comment_text: text,
        assignee,
        notify_all,
    };

    api.update_comment(comment_id, comment_data).await?;

    println!("✓ Comment updated successfully!");
    Ok(())
}

/// Delete a comment
///
/// This function permanently removes a comment from a task.
/// This action cannot be undone.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `comment_id` - The ID of the comment to delete
///
/// # Returns
///
/// Returns `Ok(())` on successful deletion, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the comment doesn't exist
async fn delete_comment(api: &ClickUpApi, comment_id: &str) -> Result<(), ClickUpError> {
    api.delete_comment(comment_id).await?;

    println!("{}", "✓ Comment deleted successfully!".green());
    println!("Deleted comment ID: {comment_id}");

    Ok(())
}

/// Create a new comment on a task
///
/// This function creates a new comment on the specified task with
/// optional assignee mention and team notification.
///
/// # Arguments
///
/// * `api` - Reference to the ClickUp API client
/// * `task_id` - The ID of the task to comment on
/// * `text` - The comment text content
/// * `assignee` - Optional assignee ID to mention in the comment
/// * `notify_all` - Optional flag to notify all team members
///
/// # Returns
///
/// Returns `Ok(())` on successful creation, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return:
/// - `ClickUpError::NetworkError` if the API request fails
/// - `ClickUpError::NotFoundError` if the task doesn't exist
/// - `ClickUpError::ValidationError` if the text is empty
async fn create_comment(
    api: &ClickUpApi,
    task_id: &str,
    text: String,
    assignee: Option<i64>,
    notify_all: Option<bool>,
) -> Result<(), ClickUpError> {
    let comment_data = CreateCommentRequest {
        comment_text: text,
        assignee,
        notify_all,
    };

    api.create_comment(task_id, comment_data).await?;

    println!("✓ Comment created successfully!");
    Ok(())
}
