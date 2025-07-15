//! # ClickUp CLI Library
//!
//! This library provides a comprehensive command-line interface for the ClickUp API.
//! It includes modules for API communication, configuration management, error handling,
//! data models, rate limiting, and command implementations.
//!
//! ## Modules
//!
//! - **api**: ClickUp API client for making authenticated requests
//! - **commands**: CLI command implementations for all operations
//! - **config**: Configuration management and settings
//! - **constants**: Centralized constants and configuration values
//! - **error**: Error handling and custom error types
//! - **models**: Data structures for API communication
//! - **rate_limiter**: Rate limiting functionality for API requests
//!
//! ## Usage
//!
//! This library is primarily used by the `clickup-cli` binary, but can also be
//! used as a dependency in other Rust projects that need ClickUp API functionality.
//!
//! ```rust
//! use clickup_cli::{ClickUpApi, Config, ClickUpError};
//! use clickup_cli::constants::api::DEFAULT_TIMEOUT;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a default config (no authentication)
//!     let config = Config::default();
//!     
//!     // Create API client (will fail without auth, but shows the pattern)
//!     let api = ClickUpApi::new(config)?;
//!     
//!     // Use the API client (this would fail without proper authentication)
//!     match api.get_workspaces().await {
//!         Ok(workspaces) => println!("Found {} workspaces", workspaces.teams.len()),
//!         Err(ClickUpError::AuthError(_)) => println!("Authentication required"),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod commands;
pub mod config;
pub mod constants;
pub mod error;
pub mod models;
pub mod rate_limiter;
pub mod repository;

// Clean library exports - main public API
pub use api::ClickUpApi;
pub use config::Config;
pub use error::ClickUpError;
pub use models::*;
pub use repository::{ClickUpRepository, RepositoryFactory};

// Re-export commonly used constants for convenience
pub use constants::{
    api::{BASE_URL, DEFAULT_TIMEOUT, MAX_RETRIES},
    rate_limiting::{DEFAULT_RPM, DEFAULT_BUFFER, MAX_WAIT},
    validation::{MAX_TASK_NAME_LENGTH, MAX_TASK_DESCRIPTION_LENGTH},
};
