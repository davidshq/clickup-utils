use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use clap::Subcommand;
use colored::*;
use comfy_table::{Table, Cell};

#[derive(Subcommand)]
pub enum WorkspaceCommands {
    /// List all workspaces
    List,
    /// Show details of a specific workspace
    Show {
        /// Workspace ID
        #[arg(short, long)]
        id: String,
    },
}

pub async fn execute(command: WorkspaceCommands, config: &Config) -> Result<(), ClickUpError> {
    let api = ClickUpApi::new(config.clone())?;

    match command {
        WorkspaceCommands::List => {
            list_workspaces(&api).await?;
        }
        WorkspaceCommands::Show { id } => {
            show_workspace(&api, &id).await?;
        }
    }
    Ok(())
}

async fn list_workspaces(api: &ClickUpApi) -> Result<(), ClickUpError> {
    println!("Fetching workspaces from ClickUp API...");
    let workspaces = api.get_workspaces().await?;
    println!("Received {} workspaces", workspaces.teams.len());
    
    if workspaces.teams.is_empty() {
        println!("{}", "No workspaces found".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("ID").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Name").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Members").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Color").add_attribute(comfy_table::Attribute::Bold),
        ]);

    for workspace in &workspaces.teams {
        table.add_row(vec![
            Cell::new(&workspace.id),
            Cell::new(workspace.name.as_deref().unwrap_or("")),
            Cell::new(workspace.members.len().to_string()),
            Cell::new(workspace.color.as_deref().unwrap_or("None")),
        ]);
    }

    println!("{table}");
    Ok(())
}

async fn show_workspace(api: &ClickUpApi, workspace_id: &str) -> Result<(), ClickUpError> {
    let workspace = api.get_workspace(workspace_id).await?;

    println!("{}", "Workspace Details".bold());
    println!("ID: {}", workspace.id);
    println!("Name: {}", workspace.name.as_deref().unwrap_or(""));
    println!("Color: {}", workspace.color.as_deref().unwrap_or("None"));
    println!("Avatar: {}", workspace.avatar.as_deref().unwrap_or("None"));
    println!("Members: {}", workspace.members.len());
    
    if !workspace.members.is_empty() {
        println!("\n{}", "Members:".bold());
        for member in &workspace.members {
            if let (Some(username), Some(email)) = (&member.user.username, &member.user.email) {
                println!("  - {username} ({email})");
            } else if let Some(username) = &member.user.username {
                println!("  - {username} (no email)");
            } else {
                println!("  - Unknown user");
            }
        }
    }

    if let Some(roles) = &workspace.roles {
        if !roles.is_empty() {
            println!("\n{}", "Roles:".bold());
            for role in roles {
                println!("  - {} (ID: {})", role.name, role.id);
            }
        }
    }

    Ok(())
} 