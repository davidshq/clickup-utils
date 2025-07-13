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
//! ## Command Structure
//!
//! Each command module follows a consistent pattern:
//! 1. Command enum definition with clap attributes and comprehensive help text
//! 2. Main execution function that routes to specific handlers
//! 3. Individual handler functions for each subcommand with detailed documentation
//! 4. Error handling and user feedback with colored output
//!
//! ## Usage
//!
//! Commands are executed through the main CLI application and automatically
//! handle authentication, error reporting, and user feedback. All commands
//! support help text and provide detailed error messages when operations fail.

pub mod auth;
pub mod comments;
pub mod lists;
pub mod spaces;
pub mod tasks;
pub mod teams;
pub mod workspaces;
