//! # Command Utilities
//!
//! This module contains common utilities and patterns used across command modules
//! to reduce code duplication and improve maintainability.

use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use clap::Subcommand;
use colored::*;
use comfy_table::{Cell, Table};

/// Common command execution trait
///
/// This trait standardizes the command execution pattern used across all
/// command modules, reducing boilerplate code and ensuring consistency.
#[allow(async_fn_in_trait)]
pub trait CommandExecutor {
    type Commands: Subcommand;
    
    /// Execute a command with the given configuration
    ///
    /// This function provides the standard pattern for command execution,
    /// including API client creation and error handling.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute
    /// * `config` - Reference to the application configuration
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful execution, or a `ClickUpError` on failure.
    async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError>;
    
    /// Handle the specific command logic
    ///
    /// This function should be implemented by each command module to handle
    /// the specific logic for each command variant.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute
    /// * `api` - Reference to the ClickUp API client
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful execution, or a `ClickUpError` on failure.
    async fn handle_command(command: Self::Commands, api: &ClickUpApi) -> Result<(), ClickUpError>;
}



/// Table builder for consistent table creation
///
/// This struct provides a builder pattern for creating formatted tables
/// with consistent styling and structure.
pub struct TableBuilder {
    table: Table,
}

impl TableBuilder {
    /// Create a new table builder
    pub fn new() -> Self {
        Self {
            table: Table::new(),
        }
    }
    
    /// Add a header row with bold formatting
    ///
    /// # Arguments
    ///
    /// * `headers` - Vector of header strings
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` for method chaining
    pub fn add_header(&mut self, headers: Vec<&str>) -> &mut Self {
        let header_cells: Vec<Cell> = headers
            .into_iter()
            .map(|h| Cell::new(h).add_attribute(comfy_table::Attribute::Bold))
            .collect();
        self.table.set_header(header_cells);
        self
    }
    
    /// Add a data row
    ///
    /// # Arguments
    ///
    /// * `row` - Vector of row data strings
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` for method chaining
    pub fn add_row(&mut self, row: Vec<String>) -> &mut Self {
        let row_cells: Vec<Cell> = row.into_iter().map(Cell::new).collect();
        self.table.add_row(row_cells);
        self
    }
    
    /// Print the table and consume the builder
    pub fn print(self) {
        println!("{}", self.table);
    }
    

}

impl Default for TableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Display utilities for consistent output formatting
pub struct DisplayUtils;

impl DisplayUtils {
    /// Display an empty results message
    ///
    /// # Arguments
    ///
    /// * `item_type` - The type of items that were not found
    pub fn display_empty_message(item_type: &str) {
        println!("{}", format!("No {} found", item_type).yellow());
    }
    
    /// Display a details header
    ///
    /// # Arguments
    ///
    /// * `item_type` - The type of item being displayed
    pub fn display_details_header(item_type: &str) {
        println!("{}", format!("{} Details", item_type).bold());
    }
    
    /// Display a section header
    ///
    /// # Arguments
    ///
    /// * `section_name` - The name of the section
    pub fn display_section_header(section_name: &str) {
        println!("\n{}", format!("{}:", section_name).bold());
    }
    
    /// Display a member with username and email
    ///
    /// # Arguments
    ///
    /// * `username` - The member's username
    /// * `email` - The member's email (optional)
    pub fn display_member(username: &str, email: Option<&str>) {
        match email {
            Some(email) => println!("  - {} ({})", username, email),
            None => println!("  - {} (no email)", username),
        }
    }
    
    /// Display a list of members
    ///
    /// # Arguments
    ///
    /// * `members` - Vector of member data with username and email
    pub fn display_members(members: &[(String, Option<String>)]) {
        if !members.is_empty() {
            Self::display_section_header("Members");
            for (username, email) in members {
                Self::display_member(username, email.as_deref());
            }
        }
    }
}

/// Error utilities for consistent error handling
pub struct ErrorUtils;

impl ErrorUtils {
    /// Create a not found error
    ///
    /// # Arguments
    ///
    /// * `item_type` - The type of item that was not found
    /// * `id` - The ID of the item that was not found
    ///
    /// # Returns
    ///
    /// Returns a `ClickUpError::NotFoundError`
    pub fn not_found_error(item_type: &str, id: &str) -> ClickUpError {
        ClickUpError::NotFoundError(format!("{} {} not found", item_type, id))
    }
    

}

/// API client utilities for consistent client creation
pub struct ApiUtils;

impl ApiUtils {
    /// Create a new ClickUp API client
    ///
    /// # Arguments
    ///
    /// * `config` - Reference to the application configuration
    ///
    /// # Returns
    ///
    /// Returns a `ClickUpApi` instance or a `ClickUpError`
    pub fn create_client(config: &Config) -> Result<ClickUpApi, ClickUpError> {
        ClickUpApi::new(config.clone())
    }
}

/// Common table headers for consistent column names
pub struct TableHeaders;

impl TableHeaders {
    /// Get standard ID header
    pub fn id() -> &'static str {
        "ID"
    }
    
    /// Get standard Name header
    pub fn name() -> &'static str {
        "Name"
    }
    
    /// Get standard Members header
    pub fn members() -> &'static str {
        "Members"
    }
    
    /// Get standard Color header
    pub fn color() -> &'static str {
        "Color"
    }
    
    /// Get standard Status header
    pub fn status() -> &'static str {
        "Status"
    }
    
    /// Get standard Priority header
    pub fn priority() -> &'static str {
        "Priority"
    }
    
    /// Get standard Due Date header
    pub fn due_date() -> &'static str {
        "Due Date"
    }
    
    /// Get standard Created header
    pub fn created() -> &'static str {
        "Created"
    }
    
    /// Get standard User header
    pub fn user() -> &'static str {
        "User"
    }
    
    /// Get standard Comment header
    pub fn comment() -> &'static str {
        "Comment"
    }
    
    /// Get standard Resolved header
    pub fn resolved() -> &'static str {
        "Resolved"
    }
    
    /// Get standard Content header
    pub fn content() -> &'static str {
        "Content"
    }
    
    /// Get standard Task Count header
    pub fn task_count() -> &'static str {
        "Task Count"
    }
    
    /// Get standard Folder header
    pub fn folder() -> &'static str {
        "Folder"
    }
    
    /// Get standard Private header
    pub fn private() -> &'static str {
        "Private"
    }
    
    /// Get standard Statuses header
    pub fn statuses() -> &'static str {
        "Statuses"
    }
    
    /// Get standard Multiple Assignees header
    pub fn multiple_assignees() -> &'static str {
        "Multiple Assignees"
    }
} 