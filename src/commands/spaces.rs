use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use clap::Subcommand;
use colored::*;
use prettytable::{Table, Row, Cell};

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
    table.add_row(Row::new(vec![
        Cell::new("ID").style_spec("bFg"),
        Cell::new("Name").style_spec("bFg"),
        Cell::new("Private").style_spec("bFg"),
        Cell::new("Statuses").style_spec("bFg"),
        Cell::new("Multiple Assignees").style_spec("bFg"),
    ]));

    for space in &spaces.spaces {
        table.add_row(Row::new(vec![
            Cell::new(&space.id),
            Cell::new(&space.name),
            Cell::new(if space.private { "Yes" } else { "No" }),
            Cell::new(&space.statuses.len().to_string()),
            Cell::new(if space.multiple_assignees { "Yes" } else { "No" }),
        ]));
    }

    table.printstd();
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
            println!("Name: {}", space.name);
            println!("Private: {}", if space.private { "Yes" } else { "No" });
            println!("Multiple Assignees: {}", if space.multiple_assignees { "Yes" } else { "No" });
            println!("Statuses: {}", space.statuses.len());
            
            if !space.statuses.is_empty() {
                println!("\n{}", "Statuses:".bold());
                for status in &space.statuses {
                    println!("  - {} ({})", status.status, status.type_);
                }
            }

            println!("\n{}", "Features:".bold());
            println!("  Due Dates: {}", if space.features.due_dates.enabled { "Enabled" } else { "Disabled" });
            println!("  Time Tracking: {}", if space.features.time_tracking.enabled { "Enabled" } else { "Disabled" });
            println!("  Points: {}", if space.features.points.enabled { "Enabled" } else { "Disabled" });
            println!("  Custom Fields: {}", if space.features.custom_fields.enabled { "Enabled" } else { "Disabled" });
            
            return Ok(());
        }
    }

    Err(ClickUpError::NotFoundError(format!("Space {} not found", space_id)))
} 