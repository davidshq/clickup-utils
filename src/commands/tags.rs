//! # Tag Commands
//!
//! This module handles all tag-related operations for the ClickUp CLI.
//! It provides commands for listing and creating tags within spaces.
//!
//! ## Commands
//!
//! - **List**: Display all tags in a space
//! - **Create**: Create a new tag with specified name and colors
//!
//! ## Features
//!
//! Tags are displayed in formatted tables showing key information including
//! name, colors, and creator. Tags can be created with custom foreground
//! and background colors for visual organization.

use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::CreateTagRequest;
use crate::repository::ClickUpRepository;
use crate::commands::utils::{CommandExecutor, TableBuilder};
use clap::Subcommand;
use colored::*;

/// Parameters for creating a tag
///
/// This struct encapsulates all the parameters needed to create a new tag,
/// providing a clean interface for tag creation operations.
struct CreateTagParams {
    space_id: String,
    name: String,
    tag_fg: String,
    tag_bg: String,
}

/// Tag command variants
///
/// This enum defines all available tag subcommands with their
/// associated parameters and help text. Each command variant includes
/// comprehensive help documentation for CLI usage.
#[derive(Subcommand)]
pub enum TagCommands {
    /// List all tags in a space
    List {
        /// Space ID
        #[arg(short, long)]
        space_id: String,
    },
    /// Create a new tag
    Create {
        /// Space ID
        #[arg(short, long)]
        space_id: String,
        /// Tag name
        #[arg(short, long)]
        name: String,
        /// Tag foreground color (hex format, e.g., "#ffffff")
        #[arg(short, long, default_value = "#ffffff")]
        tag_fg: String,
        /// Tag background color (hex format, e.g., "#ff0000")
        #[arg(short, long, default_value = "#ff0000")]
        tag_bg: String,
    },
}

impl CommandExecutor for TagCommands {
    type Commands = TagCommands;
    
    async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError> {
        let container = crate::commands::utils::RepositoryUtils::create_service_container(config)?;
        Self::handle_command(command, &container).await
    }
    
    async fn handle_command(command: Self::Commands, container: &crate::di::ServiceContainer) -> Result<(), ClickUpError> {
        let repo = container.repository();
        match command {
            TagCommands::List { space_id } => {
                list_tags(repo, &space_id).await?;
            }
            TagCommands::Create {
                space_id,
                name,
                tag_fg,
                tag_bg,
            } => {
                let params = CreateTagParams {
                    space_id: space_id.clone(),
                    name,
                    tag_fg,
                    tag_bg,
                };
                create_tag(repo, params).await?;
            }
        }
        Ok(())
    }
}

/// Helper function to execute tag commands
///
/// This function provides a convenient way to execute tag commands
/// for testing purposes.
pub async fn execute(command: TagCommands, config: &Config) -> Result<(), ClickUpError> {
    TagCommands::execute(command, config).await
}

/// List all tags in a space
///
/// This function retrieves and displays all tags within a specific space
/// in a formatted table. This is useful for understanding the available
/// tags for task organization.
///
/// # Arguments
///
/// * `repo` - Reference to the ClickUp repository
/// * `space_id` - The ID of the space to list tags from
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
async fn list_tags(repo: &dyn ClickUpRepository, space_id: &str) -> Result<(), ClickUpError> {
    println!("{}", format!("Fetching tags for space '{space_id}'...").blue());
    let tags = repo.get_tags(space_id).await?;

    if tags.tags.is_empty() {
        println!("{}", "No tags found in this space".yellow());
        return Ok(());
    }

    let mut table_builder = TableBuilder::new();
    table_builder.add_header(vec![
        "Name",
        "Foreground Color",
        "Background Color",
        "Creator ID",
    ]);

    for tag in &tags.tags {
        table_builder.add_row(vec![
            tag.name.as_deref().unwrap_or("").to_string(),
            tag.tag_fg.clone(),
            tag.tag_bg.clone(),
            tag.creator.to_string(),
        ]);
    }

    println!("{}", format!("Tags in space '{space_id}':").bold());
    table_builder.print();
    Ok(())
}

/// Create a new tag
///
/// This function creates a new tag with the specified parameters including
/// name, foreground color, and background color.
///
/// # Arguments
///
/// * `repo` - Reference to the ClickUp repository
/// * `params` - Tag creation parameters
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
/// - `ClickUpError::NotFoundError` if the space doesn't exist
async fn create_tag(repo: &dyn ClickUpRepository, params: CreateTagParams) -> Result<(), ClickUpError> {
    println!(
        "{}",
        format!("Creating tag '{}' in space '{}'...", params.name, params.space_id).blue()
    );

    let tag_request = CreateTagRequest {
        name: params.name.clone(),
        tag_fg: params.tag_fg.clone(),
        tag_bg: params.tag_bg.clone(),
    };

    let created_tag = repo.create_tag(&params.space_id, tag_request).await?;

    println!("{}", "Tag created successfully!".green().bold());
    println!("Name: {}", created_tag.name.as_deref().unwrap_or(""));
    println!("Foreground Color: {}", created_tag.tag_fg);
    println!("Background Color: {}", created_tag.tag_bg);
    println!("Creator ID: {}", created_tag.creator);

    Ok(())
} 