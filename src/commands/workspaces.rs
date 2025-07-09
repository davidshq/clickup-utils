use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use clap::Subcommand;
use colored::*;
use prettytable::{Table, Row, Cell};

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
    let workspaces = api.get_workspaces().await?;
    
    if workspaces.teams.is_empty() {
        println!("{}", "No workspaces found".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID").style_spec("bFg"),
        Cell::new("Name").style_spec("bFg"),
        Cell::new("Members").style_spec("bFg"),
        Cell::new("Color").style_spec("bFg"),
    ]));

    for workspace in &workspaces.teams {
        table.add_row(Row::new(vec![
            Cell::new(&workspace.id),
            Cell::new(&workspace.name),
            Cell::new(&workspace.members.len().to_string()),
            Cell::new(workspace.color.as_deref().unwrap_or("None")),
        ]));
    }

    table.printstd();
    Ok(())
}

async fn show_workspace(api: &ClickUpApi, workspace_id: &str) -> Result<(), ClickUpError> {
    let workspaces = api.get_workspaces().await?;
    
    let workspace = workspaces.teams
        .into_iter()
        .find(|w| w.id == workspace_id)
        .ok_or_else(|| ClickUpError::NotFoundError(format!("Workspace {} not found", workspace_id)))?;

    println!("{}", "Workspace Details".bold());
    println!("ID: {}", workspace.id);
    println!("Name: {}", workspace.name);
    println!("Color: {}", workspace.color.as_deref().unwrap_or("None"));
    println!("Avatar: {}", workspace.avatar.as_deref().unwrap_or("None"));
    println!("Members: {}", workspace.members.len());
    
    if !workspace.members.is_empty() {
        println!("\n{}", "Members:".bold());
        for member in &workspace.members {
            println!("  - {} ({})", member.username, member.email);
        }
    }

    if !workspace.roles.is_empty() {
        println!("\n{}", "Roles:".bold());
        for role in &workspace.roles {
            println!("  - {} ({})", role.name, role.key);
        }
    }

    Ok(())
} 