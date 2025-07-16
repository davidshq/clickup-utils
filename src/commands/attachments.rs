//! # Attachment Commands
//!
//! This module handles all attachment-related operations for the ClickUp CLI.
//! It provides commands for listing, uploading, and deleting attachments on tasks.
//!
//! ## Commands
//!
//! - **List**: Display all attachments for a task
//! - **Upload**: Upload a file attachment to a task
//! - **Link**: Create a link attachment on a task
//! - **Delete**: Remove an attachment permanently
//!
//! ## Features
//!
//! Attachments are displayed in formatted tables showing key information including
//! file name, size, type, and upload details. The module supports both file uploads
//! and link attachments with comprehensive error handling and user feedback.

use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::{CreateLinkAttachmentRequest};
use crate::repository::ClickUpRepository;
use crate::commands::utils::{CommandExecutor, DisplayUtils, TableBuilder, TableHeaders};
use clap::Subcommand;
use colored::*;
use log::info;

/// Parameters for uploading an attachment
///
/// This struct encapsulates all the parameters needed to upload a file attachment,
/// providing a clean interface for attachment upload operations.
struct UploadAttachmentParams {
    task_id: String,
    file_path: String,
    filename: Option<String>,
}

/// Parameters for creating a link attachment
///
/// This struct encapsulates all the parameters needed to create a link attachment,
/// providing a clean interface for link attachment operations.
struct CreateLinkAttachmentParams {
    task_id: String,
    link: String,
    title: Option<String>,
}

/// Attachment command variants
///
/// This enum defines all available attachment subcommands with their
/// associated parameters and help text. Each command variant includes
/// comprehensive help documentation for CLI usage.
#[derive(Subcommand)]
pub enum AttachmentCommands {
    /// List all attachments for a task
    List {
        /// Task ID
        #[arg(short, long)]
        task_id: String,
    },
    /// Upload a file attachment to a task
    Upload {
        /// Task ID
        #[arg(short, long)]
        task_id: String,
        /// File path to upload
        #[arg(short, long)]
        file_path: String,
        /// Custom filename (optional)
        #[arg(short, long)]
        filename: Option<String>,
    },
    /// Create a link attachment on a task
    Link {
        /// Task ID
        #[arg(short, long)]
        task_id: String,
        /// Link URL
        #[arg(short, long)]
        link: String,
        /// Custom title for the link (optional)
        #[arg(short, long)]
        title: Option<String>,
    },
    /// Delete an attachment
    Delete {
        /// Attachment ID
        #[arg(short, long)]
        id: String,
    },
}

impl CommandExecutor for AttachmentCommands {
    type Commands = AttachmentCommands;
    
    async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError> {
        let container = crate::commands::utils::RepositoryUtils::create_service_container(config)?;
        Self::handle_command(command, &container).await
    }
    
    async fn handle_command(command: Self::Commands, container: &crate::di::ServiceContainer) -> Result<(), ClickUpError> {
        let repo = container.repository();
        match command {
            AttachmentCommands::List { task_id } => {
                list_attachments(repo, &task_id).await?;
            }
            AttachmentCommands::Upload { task_id, file_path, filename } => {
                let params = UploadAttachmentParams {
                    task_id,
                    file_path,
                    filename,
                };
                upload_attachment(repo, params).await?;
            }
            AttachmentCommands::Link { task_id, link, title } => {
                let params = CreateLinkAttachmentParams {
                    task_id,
                    link,
                    title,
                };
                create_link_attachment(repo, params).await?;
            }
            AttachmentCommands::Delete { id } => {
                delete_attachment(repo, &id).await?;
            }
        }
        Ok(())
    }
}

/// List all attachments for a task
///
/// This function retrieves and displays all attachments for a specific task
/// in a formatted table showing key information like file name, size, type,
/// and upload details.
///
/// # Arguments
///
/// * `repo` - The repository to use for API operations
/// * `task_id` - The ID of the task to get attachments from
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return various errors including:
/// - Authentication errors (invalid token)
/// - Permission errors (no access to task)
/// - Network errors (connection issues)
/// - Not found errors (task doesn't exist)
async fn list_attachments(repo: &dyn ClickUpRepository, task_id: &str) -> Result<(), ClickUpError> {
    info!("Fetching attachments for task: {task_id}");
    
    let response = repo.get_attachments(task_id).await?;
    let attachments = response.attachments;
    
    if attachments.is_empty() {
        DisplayUtils::display_empty_message("attachments");
        return Ok(());
    }
    
    // Create table for attachments
    let mut table_builder = TableBuilder::new();
    table_builder.add_header(vec![
        TableHeaders::id(),
        "Name",
        "Type",
        "Size",
        "Uploaded",
        "User",
    ]);
    
    for attachment in &attachments {
        let name = attachment.title.as_deref()
            .or(attachment.filename.as_deref())
            .unwrap_or("Unknown");
        
        let file_type = attachment.mimetype.as_deref()
            .or(attachment.extension.as_deref())
            .unwrap_or("Unknown");
        
        let size = attachment.size
            .map(|s| format!("{s} B"))
            .unwrap_or_else(|| "Unknown".to_string());
        
        let uploaded = attachment.date_created
            .as_deref()
            .unwrap_or("Unknown");
        
        let user = attachment.user
            .as_ref()
            .map(|u| u.username.clone())
            .unwrap_or_else(|| "Unknown".to_string());
        
        table_builder.add_row(vec![
            attachment.id.clone(),
            name.to_string(),
            file_type.to_string(),
            size,
            uploaded.to_string(),
            user,
        ]);
    }
    
    println!("{}", "Attachments:".green().bold());
    table_builder.print();
    
    Ok(())
}

/// Upload a file attachment to a task
///
/// This function uploads a file attachment to a specific task with
/// comprehensive error handling and user feedback.
///
/// # Arguments
///
/// * `repo` - The repository to use for API operations
/// * `params` - The upload parameters containing task ID, file path, and optional filename
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return various errors including:
/// - File system errors (file not found, permission denied)
/// - Authentication errors (invalid token)
/// - Permission errors (no access to task)
/// - Network errors (connection issues)
/// - API errors (file too large, unsupported format)
async fn upload_attachment(repo: &dyn ClickUpRepository, params: UploadAttachmentParams) -> Result<(), ClickUpError> {
    info!("Uploading attachment to task: {}", params.task_id);
    info!("File path: {}", params.file_path);
    
    if let Some(ref filename) = params.filename {
        info!("Custom filename: {filename}");
    }
    
    // Validate file exists
    if !std::path::Path::new(&params.file_path).exists() {
        return Err(ClickUpError::NotFoundError(format!(
            "File not found: {}",
            params.file_path
        )));
    }
    
    // Upload the attachment
    let attachment = repo.upload_attachment(
        &params.task_id,
        &params.file_path,
        params.filename.as_deref(),
    ).await?;
    
    // Display success message with attachment details
    println!("{}", "✓ Attachment uploaded successfully!".green().bold());
    DisplayUtils::display_details_header("Attachment Details");
    println!("ID: {}", attachment.id);
    println!("Name: {}", attachment.title.as_deref().or(attachment.filename.as_deref()).unwrap_or("Unknown"));
    println!("Type: {}", attachment.mimetype.as_deref().unwrap_or("Unknown"));
    println!("Size: {} B", attachment.size.unwrap_or(0));
    println!("URL: {}", attachment.url.as_deref().unwrap_or("Not available"));
    
    if let Some(ref user) = attachment.user {
        println!("Uploaded by: {}", user.username);
    }
    
    Ok(())
}

/// Create a link attachment on a task
///
/// This function creates a link attachment on a specific task with
/// comprehensive error handling and user feedback.
///
/// # Arguments
///
/// * `repo` - The repository to use for API operations
/// * `params` - The link parameters containing task ID, link URL, and optional title
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return various errors including:
/// - Authentication errors (invalid token)
/// - Permission errors (no access to task)
/// - Network errors (connection issues)
/// - API errors (invalid URL, link preview failed)
async fn create_link_attachment(repo: &dyn ClickUpRepository, params: CreateLinkAttachmentParams) -> Result<(), ClickUpError> {
    info!("Creating link attachment on task: {}", params.task_id);
    info!("Link URL: {}", params.link);
    
    if let Some(ref title) = params.title {
        info!("Custom title: {title}");
    }
    
    // Create the link attachment request
    let link_data = CreateLinkAttachmentRequest {
        link: params.link,
        title: params.title,
    };
    
    // Create the attachment
    let attachment = repo.create_link_attachment(&params.task_id, link_data).await?;
    
    // Display success message with attachment details
    println!("{}", "✓ Link attachment created successfully!".green().bold());
    DisplayUtils::display_details_header("Link Attachment Details");
    println!("ID: {}", attachment.id);
    println!("Title: {}", attachment.title.as_deref().unwrap_or("Unknown"));
    println!("Link: {}", attachment.link.as_deref().unwrap_or("Unknown"));
    
    if let Some(ref link_preview) = attachment.link_preview {
        if let Some(ref title) = link_preview.title {
            println!("Preview Title: {title}");
        }
        if let Some(ref description) = link_preview.description {
            println!("Preview Description: {description}");
        }
        if let Some(ref domain) = link_preview.domain {
            println!("Domain: {domain}");
        }
    }
    
    if let Some(ref user) = attachment.user {
        println!("Created by: {}", user.username);
    }
    
    Ok(())
}

/// Delete an attachment
///
/// This function deletes a specific attachment with comprehensive
/// error handling and user feedback.
///
/// # Arguments
///
/// * `repo` - The repository to use for API operations
/// * `attachment_id` - The ID of the attachment to delete
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return various errors including:
/// - Authentication errors (invalid token)
/// - Permission errors (no access to attachment)
/// - Network errors (connection issues)
/// - Not found errors (attachment doesn't exist)
async fn delete_attachment(repo: &dyn ClickUpRepository, attachment_id: &str) -> Result<(), ClickUpError> {
    info!("Deleting attachment: {attachment_id}");
    
    // Delete the attachment
    repo.delete_attachment(attachment_id).await?;
    
    // Display success message
    println!("{}", "✓ Attachment deleted successfully!".green().bold());
    println!("Attachment ID: {attachment_id}");
    
    Ok(())
} 

/// Execute attachment commands
///
/// This function serves as the main entry point for attachment command execution.
/// It follows the standardized pattern used by all command modules.
///
/// # Arguments
///
/// * `command` - The attachment command to execute
/// * `config` - The application configuration
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `ClickUpError` on failure.
///
/// # Errors
///
/// This function can return various errors depending on the specific command:
/// - Authentication errors (invalid token)
/// - Permission errors (no access to resources)
/// - Network errors (connection issues)
/// - File system errors (file not found, permission denied)
/// - API errors (invalid data, rate limiting)
pub async fn execute(command: AttachmentCommands, config: &Config) -> Result<(), ClickUpError> {
    AttachmentCommands::execute(command, config).await
} 