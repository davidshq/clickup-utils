use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use clap::Subcommand;
use colored::*;
use prettytable::{Table, Row, Cell};

#[derive(Subcommand)]
pub enum ListCommands {
    /// List all lists in a space
    List {
        /// Space ID
        #[arg(short, long)]
        space_id: String,
    },
    /// Show details of a specific list
    Show {
        /// List ID
        #[arg(short, long)]
        id: String,
    },
}

pub async fn execute(command: ListCommands, config: &Config) -> Result<(), ClickUpError> {
    let api = ClickUpApi::new(config.clone())?;

    match command {
        ListCommands::List { space_id } => {
            list_lists(&api, &space_id).await?;
        }
        ListCommands::Show { id } => {
            show_list(&api, &id).await?;
        }
    }
    Ok(())
}

async fn list_lists(api: &ClickUpApi, space_id: &str) -> Result<(), ClickUpError> {
    let lists = api.get_lists(space_id).await?;
    
    if lists.lists.is_empty() {
        println!("{}", "No lists found".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID").style_spec("bFg"),
        Cell::new("Name").style_spec("bFg"),
        Cell::new("Content").style_spec("bFg"),
        Cell::new("Task Count").style_spec("bFg"),
        Cell::new("Folder").style_spec("bFg"),
    ]));

    for list in &lists.lists {
        let folder_name = list.folder.as_ref().map(|f| f.name.as_str()).unwrap_or("None");
        let task_count = list.task_count.as_deref().unwrap_or("0");
        
        table.add_row(Row::new(vec![
            Cell::new(&list.id),
            Cell::new(&list.name),
            Cell::new(&list.content),
            Cell::new(task_count),
            Cell::new(folder_name),
        ]));
    }

    table.printstd();
    Ok(())
}

async fn show_list(api: &ClickUpApi, list_id: &str) -> Result<(), ClickUpError> {
    // For now, we'll need to search through spaces to find the list
    // In a real implementation, you might want to store space_id in config
    let workspaces = api.get_workspaces().await?;
    
    for workspace in &workspaces.teams {
        let spaces = api.get_spaces(&workspace.id).await?;
        for space in &spaces.spaces {
            let lists = api.get_lists(&space.id).await?;
            if let Some(list) = lists.lists.into_iter().find(|l| l.id == list_id) {
                println!("{}", "List Details".bold());
                println!("ID: {}", list.id);
                println!("Name: {}", list.name);
                println!("Content: {}", list.content);
                println!("Order Index: {}", list.orderindex);
                println!("Task Count: {}", list.task_count.as_deref().unwrap_or("0"));
                
                if let Some(folder) = &list.folder {
                    println!("Folder: {} ({})", folder.name, folder.id);
                }
                
                if let Some(space) = &list.space {
                    println!("Space: {} ({})", space.name, space.id);
                }
                
                if let Some(inbound_address) = &list.inbound_address {
                    println!("Inbound Address: {}", inbound_address);
                }
                
                return Ok(());
            }
        }
    }

    Err(ClickUpError::NotFoundError(format!("List {} not found", list_id)))
} 