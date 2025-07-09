use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::{CreateTaskRequest, UpdateTaskRequest};
use clap::Subcommand;
use colored::*;
use comfy_table::{Table, Cell};

#[derive(Subcommand)]
pub enum TaskCommands {
    /// List all tasks in a list
    List {
        /// List ID
        #[arg(short, long)]
        list_id: String,
    },
    /// List tasks in a list filtered by tag
    ListByTag {
        /// List ID
        #[arg(short, long)]
        list_id: String,
        /// Tag name to filter by
        #[arg(short, long)]
        tag: String,
    },
    /// Search for tasks with a specific tag across all lists in a space
    SearchByTag {
        /// Tag name to search for
        #[arg(short, long)]
        tag: String,
        /// Workspace ID (optional - will prompt if not provided)
        #[arg(short, long)]
        workspace_id: Option<String>,
        /// Space ID (optional - will prompt if not provided)
        #[arg(short, long)]
        space_id: Option<String>,
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
        TaskCommands::ListByTag { list_id, tag } => {
            list_tasks_by_tag(&api, &list_id, &tag).await?;
        }
        TaskCommands::SearchByTag { tag, workspace_id, space_id } => {
            search_tasks_by_tag(&api, tag, workspace_id, space_id).await?;
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
    table.set_header(vec![
        Cell::new("ID").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Name").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Status").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Priority").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Due Date").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Assignees").add_attribute(comfy_table::Attribute::Bold),
    ]);

    for task in &tasks.tasks {
        let priority = task.priority.as_ref().map(|p| p.priority.as_str()).unwrap_or("None");
        let due_date = task.due_date.as_deref().unwrap_or("None");
        let assignees = if task.assignees.is_empty() {
            "None".to_string()
        } else {
            task.assignees.iter().map(|a| a.username.clone()).collect::<Vec<_>>().join(", ")
        };
        
        table.add_row(vec![
            Cell::new(&task.id),
            Cell::new(task.name.as_deref().unwrap_or("")),
            Cell::new(&task.status.status),
            Cell::new(priority),
            Cell::new(due_date),
            Cell::new(&assignees),
        ]);
    }

    println!("{}", table);
    Ok(())
}

async fn list_tasks_by_tag(api: &ClickUpApi, list_id: &str, tag: &str) -> Result<(), ClickUpError> {
    println!("{}", format!("Fetching tasks with tag '{}'...", tag).blue());
    let tasks = api.get_tasks_by_tag(list_id, tag).await?;
    
    if tasks.tasks.is_empty() {
        println!("{}", format!("No tasks found with tag '{}'", tag).yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ID").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Name").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Status").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Priority").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Due Date").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Assignees").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Tags").add_attribute(comfy_table::Attribute::Bold),
    ]);

    for task in &tasks.tasks {
        let priority = task.priority.as_ref().map(|p| p.priority.as_str()).unwrap_or("None");
        let due_date = task.due_date.as_deref().unwrap_or("None");
        let assignees = if task.assignees.is_empty() {
            "None".to_string()
        } else {
            task.assignees.iter().map(|a| a.username.clone()).collect::<Vec<_>>().join(", ")
        };
        
        let tag_names = task.tags.iter().filter_map(|t| t.name.as_deref()).collect::<Vec<_>>().join(", ");
        
        table.add_row(vec![
            Cell::new(&task.id),
            Cell::new(task.name.as_deref().unwrap_or("")),
            Cell::new(&task.status.status),
            Cell::new(priority),
            Cell::new(due_date),
            Cell::new(&assignees),
            Cell::new(&tag_names),
        ]);
    }

    println!("{}", format!("Tasks with tag '{}':", tag).bold());
    println!("{}", table);
    Ok(())
}

async fn search_tasks_by_tag(api: &ClickUpApi, tag: String, workspace_id: Option<String>, space_id: Option<String>) -> Result<(), ClickUpError> {
    println!("{}", format!("Searching for tasks with tag '{}'...", tag).blue());
    let tasks = api.search_tasks_by_tag(tag.clone(), workspace_id, space_id).await?;
    
    if tasks.tasks.is_empty() {
        println!("{}", format!("No tasks found with tag '{}'", tag).yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ID").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Name").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Status").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Priority").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Due Date").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Assignees").add_attribute(comfy_table::Attribute::Bold),
        Cell::new("Tags").add_attribute(comfy_table::Attribute::Bold),
    ]);

    for task in &tasks.tasks {
        let priority = task.priority.as_ref().map(|p| p.priority.as_str()).unwrap_or("None");
        let due_date = task.due_date.as_deref().unwrap_or("None");
        let assignees = if task.assignees.is_empty() {
            "None".to_string()
        } else {
            task.assignees.iter().map(|a| a.username.clone()).collect::<Vec<_>>().join(", ")
        };
        
        let tag_names = task.tags.iter().filter_map(|t| t.name.as_deref()).collect::<Vec<_>>().join(", ");
        
        table.add_row(vec![
            Cell::new(&task.id),
            Cell::new(task.name.as_deref().unwrap_or("")),
            Cell::new(&task.status.status),
            Cell::new(priority),
            Cell::new(due_date),
            Cell::new(&assignees),
            Cell::new(&tag_names),
        ]);
    }

    println!("{}", format!("Tasks with tag '{}':", tag).bold());
    println!("{}", table);
    Ok(())
}

async fn show_task(api: &ClickUpApi, task_id: &str) -> Result<(), ClickUpError> {
    let task = api.get_task(task_id).await?;

    println!("{}", "Task Details".bold());
    println!("ID: {}", task.id);
    println!("Name: {}", task.name.as_deref().unwrap_or(""));
    println!("Status: {} ({})", task.status.status, task.status.color);
    println!("Created: {}", task.date_created);
    println!("Updated: {}", task.date_updated);
    
    println!("Description: {}", task.description);
    
    if let Some(priority) = &task.priority {
        println!("Priority: {} ({})", priority.priority, priority.color);
    }
    
    if let Some(due_date) = &task.due_date {
        println!("Due Date: {}", due_date);
    }
    
    if let Some(start_date) = &task.start_date {
        println!("Start Date: {}", start_date);
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
            println!("  - {} ({}/{})", tag.name.as_deref().unwrap_or(""), tag.tag_fg, tag.tag_bg);
        }
    }
    
    if !task.checklists.is_empty() {
        println!("\n{}", "Checklists:".bold());
        for checklist in &task.checklists {
                    println!("    - {} [{}]", checklist.name.as_deref().unwrap_or(""), checklist.resolved.unwrap_or(false));
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
        start_date: None,
        start_date_time: None,
        points: None,
        notify_all: None,
    };

    let task = api.create_task(list_id, task_data).await?;
    
    println!("{}", "✓ Task created successfully!".green());
    println!("ID: {}", task.id);
    println!("Name: {}", task.name.as_deref().unwrap_or(""));
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
        start_date: None,
        start_date_time: None,
        points: None,
        notify_all: None,
    };

    let task = api.update_task(task_id, task_data).await?;
    
    println!("{}", "✓ Task updated successfully!".green());
    println!("ID: {}", task.id);
    println!("Name: {}", task.name.as_deref().unwrap_or(""));
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