use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::CreateCommentRequest;
use clap::Subcommand;
use colored::*;
use comfy_table::{Table, Cell};

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
        #[arg(short, long)]
        task_id: String,
        /// Comment text
        #[arg(short, long)]
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
        #[arg(short, long)]
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

pub async fn execute(command: CommentCommands, config: &Config) -> Result<(), ClickUpError> {
    let api = ClickUpApi::new(config.clone())?;

    match command {
        CommentCommands::List { task_id } => {
            list_comments(&api, &task_id).await?;
        }
        CommentCommands::Show { id } => {
            show_comment(&api, &id).await?;
        }
        CommentCommands::Create { task_id, text, assignee, notify_all } => {
            create_comment(&api, &task_id, text, assignee, notify_all).await?;
        }
        CommentCommands::Update { id, text, assignee, notify_all } => {
            update_comment(&api, &id, text, assignee, notify_all).await?;
        }
        CommentCommands::Delete { id } => {
            delete_comment(&api, &id).await?;
        }
    }
    Ok(())
}

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
            Cell::new(&comment.date_created),
            Cell::new(if comment.resolved { "Yes" } else { "No" }),
        ]);
    }

    println!("{table}");
    Ok(())
}

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
                    if let Some(comment) = comments.comments.into_iter().find(|c| c.id == comment_id) {
                        println!("{}", "Comment Details".bold());
                        println!("ID: {}", comment.id);
                        println!("Task: {} ({})", task.name.as_deref().unwrap_or(""), task.id);
                        println!("User: {} ({})", comment.user.username, comment.user.id);
                        println!("Created: {}", comment.date_created);
                        println!("Updated: {}", comment.date_updated);
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

    Err(ClickUpError::NotFoundError(format!("Comment {comment_id} not found")))
}

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

    let comment = api.update_comment(comment_id, comment_data).await?;
    
    println!("{}", "✓ Comment updated successfully!".green());
    println!("ID: {}", comment.id);
    println!("Text: {}", comment.comment_text);
    println!("Updated: {}", comment.date_updated);
    
    Ok(())
}

async fn delete_comment(api: &ClickUpApi, comment_id: &str) -> Result<(), ClickUpError> {
    api.delete_comment(comment_id).await?;
    
    println!("{}", "✓ Comment deleted successfully!".green());
    println!("Deleted comment ID: {comment_id}");
    
    Ok(())
}

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

    let comment = api.create_comment(task_id, comment_data).await?;
    
    println!("{}", "✓ Comment created successfully!".green());
    println!("ID: {}", comment.id);
    println!("User: {}", comment.user.username);
    println!("Text: {}", comment.comment_text);
    println!("Created: {}", comment.date_created);
    
    Ok(())
} 