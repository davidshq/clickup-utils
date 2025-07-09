use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use clap::Subcommand;
use colored::*;
use comfy_table::{Table, Cell};

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
}

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
    }
    Ok(())
}

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
            Cell::new(&space.statuses.len().to_string()),
            Cell::new(if space.multiple_assignees { "Yes" } else { "No" }),
        ]);
    }

    println!("{}", table);
    Ok(())
}

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
            println!("Multiple Assignees: {}", if space.multiple_assignees { "Yes" } else { "No" });
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
                    println!("  Due Dates: {}", if due_dates.enabled { "Enabled" } else { "Disabled" });
                }
                if let Some(time_tracking) = &features.time_tracking {
                    println!("  Time Tracking: {}", if time_tracking.enabled { "Enabled" } else { "Disabled" });
                }
                if let Some(custom_fields) = &features.custom_fields {
                    println!("  Custom Fields: {}", if custom_fields.enabled { "Enabled" } else { "Disabled" });
                }
            }
            
            return Ok(());
        }
    }

    Err(ClickUpError::NotFoundError(format!("Space {} not found", space_id)))
}

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
        let hidden = if folder.hidden.unwrap_or(false) { "Yes" } else { "No" };
        let archived = if folder.archived.unwrap_or(false) { "Yes" } else { "No" };
        
        table.add_row(vec![
            Cell::new(&folder.id),
            Cell::new(folder.name.as_deref().unwrap_or("")),
            Cell::new(content),
            Cell::new(hidden),
            Cell::new(archived),
        ]);
    }

    println!("{}", table);
    Ok(())
} 