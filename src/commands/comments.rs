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

use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::CreateCommentRequest;
use crate::repository::ClickUpRepository;
use crate::commands::utils::{CommandExecutor, DisplayUtils, TableBuilder, TableHeaders};
use clap::Subcommand;
use colored::*;

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

    /// Delete a comment
    Delete {
        /// Comment ID
        #[arg(short, long)]
        id: String,
    },
}

impl CommandExecutor for CommentCommands {
    type Commands = CommentCommands;
    
    async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError> {
        let repo = crate::repository::RepositoryFactory::create(config)?;
        Self::handle_command(command, &*repo).await
    }
    
    async fn handle_command(command: Self::Commands, repo: &dyn ClickUpRepository) -> Result<(), ClickUpError> {
        match command {
            CommentCommands::List { task_id } => {
                list_comments(repo, &task_id).await?;
            }
            CommentCommands::Show { id } => {
                show_comment(repo, &id).await?;
            }
            CommentCommands::Create {
                task_id,
                text,
                assignee,
                notify_all,
            } => {
                create_comment(repo, &task_id, text, assignee, notify_all).await?;
            }

            CommentCommands::Delete { id } => {
                delete_comment(repo, &id).await?;
            }
        }
        Ok(())
    }
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
    CommentCommands::execute(command, config).await
}

/// List all comments for a task
///
/// This function retrieves and displays all comments for a specific task
/// in a formatted table. Long comments are truncated for display.
///
/// # Arguments
///
/// * `repo` - Reference to the ClickUp repository
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
async fn list_comments(repo: &dyn ClickUpRepository, task_id: &str) -> Result<(), ClickUpError> {
    let comments = repo.get_comments(task_id).await?;

    if comments.comments.is_empty() {
        DisplayUtils::display_empty_message("comments");
        return Ok(());
    }

    let mut table_builder = TableBuilder::new();
    table_builder.add_header(vec![
        TableHeaders::id(),
        TableHeaders::user(),
        TableHeaders::comment(),
        TableHeaders::created(),
        TableHeaders::resolved(),
    ]);

    for comment in &comments.comments {
        let comment_text = if comment.comment_text.len() > 50 {
            format!("{}...", &comment.comment_text[..50])
        } else {
            comment.comment_text.clone()
        };

        table_builder.add_row(vec![
            comment.id.clone(),
            comment.user.username.clone(),
            comment_text,
            comment.date.clone(),
            if comment.resolved { "Yes" } else { "No" }.to_string(),
        ]);
    }

    table_builder.print();
    Ok(())
}

/// Show detailed information about a specific comment
///
/// This function uses an efficient search strategy to find a comment by ID
/// and displays its detailed information. The search is performed concurrently
/// across workspaces for better performance.
///
/// # Arguments
///
/// * `repo` - Reference to the ClickUp repository
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
async fn show_comment(repo: &dyn ClickUpRepository, comment_id: &str) -> Result<(), ClickUpError> {
    // Use the efficient comment search method
    let comment = repo.get_comment(comment_id).await?;

    DisplayUtils::display_details_header("Comment");
    println!("ID: {}", comment.id);
    println!("User: {} ({})", comment.user.username, comment.user.id);
    println!("Created: {}", comment.date);
    println!("Updated: {}", comment.date_updated.as_deref().unwrap_or(&comment.date));
    println!("Resolved: {}", if comment.resolved { "Yes" } else { "No" });
    println!("Text: {}", comment.comment_text);

    if let Some(assignee) = &comment.assignee {
        println!("Assignee: {assignee:?}");
    }

    if let Some(assignee_by) = &comment.assignee_by {
        println!("Assigned by: {assignee_by:?}");
    }

    if let Some(parent) = &comment.parent {
        println!("Parent Comment: {parent:?}");
    }

    if !comment.children.is_empty() {
        println!("Child Comments: {} replies", comment.reply_count);
    }

    if !comment.reactions.is_empty() {
        println!("Reactions: {} reactions", comment.reactions.len());
    }

    Ok(())
}



/// Delete a comment
///
/// This function permanently removes a comment from a task.
/// This action cannot be undone.
///
/// # Arguments
///
/// * `repo` - Reference to the ClickUp repository
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
async fn delete_comment(repo: &dyn ClickUpRepository, comment_id: &str) -> Result<(), ClickUpError> {
    repo.delete_comment(comment_id).await?;

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
/// * `repo` - Reference to the ClickUp repository
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
    repo: &dyn ClickUpRepository,
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

    repo.create_comment(task_id, comment_data).await?;

    println!("✓ Comment created successfully!");
    Ok(())
}
