use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use clap::Subcommand;
use colored::*;
use prettytable::{Table, Row, Cell};

#[derive(Subcommand)]
pub enum TeamCommands {
    /// List all teams
    List,
    /// Show details of a specific team
    Show {
        /// Team ID
        #[arg(short, long)]
        id: String,
    },
}

pub async fn execute(command: TeamCommands, config: &Config) -> Result<(), ClickUpError> {
    let api = ClickUpApi::new(config.clone())?;

    match command {
        TeamCommands::List => {
            list_teams(&api).await?;
        }
        TeamCommands::Show { id } => {
            show_team(&api, &id).await?;
        }
    }
    Ok(())
}

async fn list_teams(api: &ClickUpApi) -> Result<(), ClickUpError> {
    let workspaces = api.get_workspaces().await?;
    
    if workspaces.teams.is_empty() {
        println!("{}", "No teams found".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID").style_spec("bFg"),
        Cell::new("Name").style_spec("bFg"),
        Cell::new("Members").style_spec("bFg"),
        Cell::new("Color").style_spec("bFg"),
    ]));

    for team in &workspaces.teams {
        table.add_row(Row::new(vec![
            Cell::new(&team.id),
            Cell::new(&team.name),
            Cell::new(&team.members.len().to_string()),
            Cell::new(team.color.as_deref().unwrap_or("None")),
        ]));
    }

    table.printstd();
    Ok(())
}

async fn show_team(api: &ClickUpApi, team_id: &str) -> Result<(), ClickUpError> {
    let workspaces = api.get_workspaces().await?;
    
    let team = workspaces.teams
        .into_iter()
        .find(|t| t.id == team_id)
        .ok_or_else(|| ClickUpError::NotFoundError(format!("Team {} not found", team_id)))?;

    println!("{}", "Team Details".bold());
    println!("ID: {}", team.id);
    println!("Name: {}", team.name);
    println!("Color: {}", team.color.as_deref().unwrap_or("None"));
    println!("Avatar: {}", team.avatar.as_deref().unwrap_or("None"));
    println!("Members: {}", team.members.len());
    
    if !team.members.is_empty() {
        println!("\n{}", "Members:".bold());
        for member in &team.members {
                if let (Some(username), Some(email)) = (&member.user.username, &member.user.email) {
                println!("  - {} ({})", username, email);
            } else if let Some(username) = &member.user.username {
                println!("  - {} (no email)", username);
            } else {
                println!("  - Unknown user");
            }
        }
    }

    if !team.roles.is_empty() {
        println!("\n{}", "Roles:".bold());
        for role in &team.roles {
            println!("  - {} ({})", role.name, role.key);
        }
    }

    Ok(())
} 