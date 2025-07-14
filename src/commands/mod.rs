//! # Command Modules
//!
//! This module contains all the command implementations for the ClickUp CLI.
//! Each submodule handles a specific category of operations and provides
//! the necessary command structures and execution logic.
//!
//! ## Command Categories
//!
//! The commands are organized into logical categories:
//! - **Auth**: Authentication, API token management, and rate limiting configuration
//! - **Workspaces**: Workspace and team management
//! - **Teams**: Team-specific operations within workspaces
//! - **Spaces**: Space management and navigation
//! - **Lists**: List operations and organization
//! - **Tasks**: Task creation, updates, and management
//! - **Comments**: Comment system operations
//!
//! ## Standardized Command Architecture
//!
//! All command modules follow a consistent, standardized pattern to reduce
//! code duplication and improve maintainability:
//!
//! ### 1. CommandExecutor Trait Pattern
//!
//! Each command module implements the `CommandExecutor` trait, which provides:
//! - Standardized command execution flow
//! - Centralized API client creation
//! - Consistent error handling
//! - Separation of concerns between execution and business logic
//!
//! ```rust
//! impl CommandExecutor for XCommands {
//!     type Commands = XCommands;
//!     
//!     async fn execute(command: Self::Commands, config: &Config) -> Result<(), ClickUpError> {
//!         let api = ApiUtils::create_client(config)?;
//!         Self::handle_command(command, &api).await
//!     }
//!     
//!     async fn handle_command(command: Self::Commands, api: &ClickUpApi) -> Result<(), ClickUpError> {
//!         match command {
//!             // Command-specific matches
//!         }
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ### 2. Utility Modules
//!
//! The `utils` module provides standardized utilities used across all command modules:
//!
//! - **TableBuilder**: Builder pattern for consistent table creation and formatting
//! - **DisplayUtils**: Standardized output formatting for details, empty messages, and members
//! - **ErrorUtils**: Consistent error creation and handling patterns
//! - **ApiUtils**: Centralized API client creation
//! - **TableHeaders**: Standardized table header constants
//!
//! ### 3. Standardized Patterns
//!
//! All command modules implement these consistent patterns:
//!
//! #### Table Creation Pattern
//! ```rust
//! let mut table_builder = TableBuilder::new();
//! table_builder.add_header(vec![
//!     TableHeaders::id(),
//!     TableHeaders::name(),
//!     // ... other headers
//! ]);
//!
//! for item in &items {
//!     table_builder.add_row(vec![
//!         item.id.clone(),
//!         item.name.as_deref().unwrap_or("").to_string(),
//!         // ... other fields
//!     ]);
//! }
//!
//! table_builder.print();
//! ```
//!
//! #### Empty Results Handling
//! ```rust
//! if items.is_empty() {
//!     DisplayUtils::display_empty_message("items");
//!     return Ok(());
//! }
//! ```
//!
//! #### Details Display Pattern
//! ```rust
//! DisplayUtils::display_details_header("Item");
//! println!("ID: {}", item.id);
//! println!("Name: {}", item.name.as_deref().unwrap_or(""));
//! // ... other fields
//! ```
//!
//! #### Member Display Pattern
//! ```rust
//! let members: Vec<(String, Option<String>)> = workspace
//!     .members
//!     .iter()
//!     .map(|m| {
//!         let username = m.user.username.as_deref().unwrap_or("Unknown user").to_string();
//!         let email = m.user.email.clone();
//!         (username, email)
//!     })
//!     .collect();
//! DisplayUtils::display_members(&members);
//! ```
//!
//! #### Error Handling Pattern
//! ```rust
//! Err(ErrorUtils::not_found_error("Item", id))
//! ```
//!
//! ## Command Structure
//!
//! Each command module follows this consistent structure:
//! 1. **Command enum definition** with clap attributes and comprehensive help text
//! 2. **CommandExecutor implementation** for standardized execution flow
//! 3. **Individual handler functions** for each subcommand with detailed documentation
//! 4. **Utility usage** for consistent table creation, display formatting, and error handling
//!
//! ## Benefits of Standardization
//!
//! The standardized patterns provide several benefits:
//! - **Reduced Code Duplication**: ~200+ lines of duplicate code eliminated
//! - **Improved Consistency**: All commands follow the same patterns
//! - **Enhanced Maintainability**: Changes to common patterns only need to be made in one place
//! - **Better Readability**: Command logic is cleaner and more focused
//! - **Easier Development**: New commands can follow established patterns
//!
//! ## Usage
//!
//! Commands are executed through the main CLI application and automatically
//! handle authentication, error reporting, and user feedback. All commands
//! support help text and provide detailed error messages when operations fail.
//!
//! The standardized architecture ensures that all commands provide consistent
//! user experience with formatted tables, proper error handling, and clear
//! output formatting.

pub mod auth;
pub mod comments;
pub mod lists;
pub mod spaces;
pub mod tasks;
pub mod teams;
pub mod workspaces;
pub mod utils;
