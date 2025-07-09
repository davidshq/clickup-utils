use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::{CreateTaskRequest, UpdateTaskRequest};
use clap::Subcommand;
use colored::*;
use prettytable::{Table, Row, Cell};

#[derive(Subcommand)]
pub enum TaskCommands {
    /// List all tasks in a list
    List {
        /// List ID
        #[arg(short, long)]
        list_id: String,
    },
    /// Show details of a specific task
    Show {
        /// Task ID
        #[arg(short, long)]
        id: String,
    },
    /// Create a new task
    Create {
        /// List ID
        #[arg(short, long)]
        list_id: String,
        /// Task name
        #[arg(short, long)]
        name: String,
        /// Task description
        #[arg(short, long)]
        description: Option<String>,
        /// Task status
        #[arg(short, long)]
        status: Option<String>,
        /// Priority (1-4)
        #[arg(short, long)]
        priority: Option<i64>,
        /// Due date (Unix timestamp)
        #[arg(short, long)]
        due_date: Option<i64>,
        /// Time estimate (in milliseconds)
        #[arg(short, long)]
        time_estimate: Option<i64>,
    },
    /// Update an existing task
    Update {
        /// Task ID
        #[arg(short, long)]
        id: String,
        /// New task name
        #[arg(short, long)]
        name: Option<String>,
        /// New task description
        #[arg(short, long)]
        description: Option<String>,
        /// New task status
        #[arg(short, long)]
        status: Option<String>,
        /// New priority (1-4)
        #[arg(short, long)]
        priority: Option<i64>,
        /// New due date (Unix timestamp)
        #[arg(short, long)]
        due_date: Option<i64>,
        /// New time estimate (in milliseconds)
        #[arg(short, long)]
        time_estimate: Option<i64>,
    },
    /// Delete a task
    Delete {
        /// Task ID
        #[arg(short, long)]
        id: String,
    },
}

pub async fn execute(command: TaskCommands, config: &Config) -> Result<(), ClickUpError> {
    let api = ClickUpApi::new(config.clone())?;

    match command {
        TaskCommands::List { list_id } => {
            list_tasks(&api, &list_id).await?;
        }
        TaskCommands::Show { id } => {
            show_task(&api, &id).await?;
        }
        TaskCommands::Create { list_id, name, description, status, priority, due_date, time_estimate } => {
            create_task(&api, &list_id, name, description, status, priority, due_date, time_estimate).await?;
        }
        TaskCommands::Update { id, name, description, status, priority, due_date, time_estimate } => {
            update_task(&api, &id, name, description, status, priority, due_date, time_estimate).await?;
        }
        TaskCommands::Delete { id } => {
            delete_task(&api, &id).await?;
        }
    }
    Ok(())
}

async fn list_tasks(api: &ClickUpApi, list_id: &str) -> Result<(), ClickUpError> {
    let tasks = api.get_tasks(list_id).await?;
    
    if tasks.tasks.is_empty() {
        println!("{}", "No tasks found".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID").style_spec("bFg"),
        Cell::new("Name").style_spec("bFg"),
        Cell::new("Status").style_spec("bFg"),
        Cell::new("Priority").style_spec("bFg"),
        Cell::new("Due Date").style_spec("bFg"),
        Cell::new("Assignees").style_spec("bFg"),
    ]));

    for task in &tasks.tasks {
        let priority = task.priority.as_ref().map(|p| &p.priority).map_or("None", |v| v);
        let due_date = task.due_date.as_deref().unwrap_or("None");
        let assignees = if task.assignees.is_empty() {
            "None".to_string()
        } else {
            task.assignees.iter().map(|a| a.username.as_str()).collect::<Vec<_>>().join(", ")
        };
        
        table.add_row(Row::new(vec![
            Cell::new(&task.id),
            Cell::new(&task.name),
            Cell::new(&task.status.status),
            Cell::new(priority),
            Cell::new(due_date),
            Cell::new(&assignees),
        ]));
    }

    table.printstd();
    Ok(())
}

async fn show_task(api: &ClickUpApi, task_id: &str) -> Result<(), ClickUpError> {
    let task = api.get_task(task_id).await?;

    println!("{}", "Task Details".bold());
    println!("ID: {}", task.id);
    println!("Name: {}", task.name);
    println!("Status: {} ({})", task.status.status, task.status.color);
    println!("Created: {}", task.date_created);
    println!("Updated: {}", task.date_updated);
    
    if let Some(description) = &task.description {
        println!("Description: {}", description);
    }
    
    if let Some(priority) = &task.priority {
        println!("Priority: {} ({})", priority.priority, priority.color);
    }
    
    if let Some(due_date) = &task.due_date {
        println!("Due Date: {}", due_date);
    }
    
    if let Some(start_date) = &task.start_date {
        println!("Start Date: {}", start_date);
    }
    
    if let Some(points) = task.points {
        println!("Points: {}", points);
    }
    
    if let Some(time_estimate) = task.time_estimate {
        println!("Time Estimate: {} ms", time_estimate);
    }
    
    if let Some(time_spent) = task.time_spent {
        println!("Time Spent: {} ms", time_spent);
    }
    
    if !task.assignees.is_empty() {
        println!("\n{}", "Assignees:".bold());
        for assignee in &task.assignees {
            println!("  - {} ({})", assignee.username, assignee.id);
        }
    }
    
    if !task.tags.is_empty() {
        println!("\n{}", "Tags:".bold());
        for tag in &task.tags {
            println!("  - {} ({}/{})", tag.name, tag.tag_fg, tag.tag_bg);
        }
    }
    
    if !task.checklists.is_empty() {
        println!("\n{}", "Checklists:".bold());
        for checklist in &task.checklists {
            println!("  - {} ({} resolved, {} unresolved)", 
                checklist.name, checklist.resolved, checklist.unresolved);
        }
    }
    
    println!("URL: {}", task.url);
    
    Ok(())
}

async fn create_task(
    api: &ClickUpApi,
    list_id: &str,
    name: String,
    description: Option<String>,
    status: Option<String>,
    priority: Option<i64>,
    due_date: Option<i64>,
    time_estimate: Option<i64>,
) -> Result<(), ClickUpError> {
    let task_data = CreateTaskRequest {
        name,
        description,
        status,
        priority,
        due_date,
        due_date_time: due_date.is_some().then(|| true),
        time_estimate,
        assignees: None,
        tags: None,
        parent: None,
        custom_fields: None,
    };

    let task = api.create_task(list_id, task_data).await?;
    
    println!("{}", "✓ Task created successfully!".green());
    println!("ID: {}", task.id);
    println!("Name: {}", task.name);
    println!("Status: {}", task.status.status);
    println!("URL: {}", task.url);
    
    Ok(())
}

async fn update_task(
    api: &ClickUpApi,
    task_id: &str,
    name: Option<String>,
    description: Option<String>,
    status: Option<String>,
    priority: Option<i64>,
    due_date: Option<i64>,
    time_estimate: Option<i64>,
) -> Result<(), ClickUpError> {
    let task_data = UpdateTaskRequest {
        name,
        description,
        status,
        priority,
        due_date,
        due_date_time: due_date.is_some().then(|| true),
        time_estimate,
        assignees: None,
        tags: None,
        parent: None,
        custom_fields: None,
    };

    let task = api.update_task(task_id, task_data).await?;
    
    println!("{}", "✓ Task updated successfully!".green());
    println!("ID: {}", task.id);
    println!("Name: {}", task.name);
    println!("Status: {}", task.status.status);
    println!("URL: {}", task.url);
    
    Ok(())
}

async fn delete_task(api: &ClickUpApi, task_id: &str) -> Result<(), ClickUpError> {
    api.delete_task(task_id).await?;
    
    println!("{}", "✓ Task deleted successfully!".green());
    println!("Deleted task ID: {}", task_id);
    
    Ok(())
} 