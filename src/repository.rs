//! # ClickUp Repository
//!
//! This module provides a repository pattern implementation for ClickUp API operations.
//! It abstracts the data access layer from business logic, making the codebase more
//! maintainable, testable, and extensible.
//!
//! ## Overview
//!
//! The repository pattern introduces an abstraction layer between business logic
//! and data access, providing several benefits:
//!
//! - **Separation of Concerns**: Business logic is decoupled from API implementation
//! - **Testability**: Easy to mock repository for unit testing
//! - **Extensibility**: Can add caching, logging, or other cross-cutting concerns
//! - **Maintainability**: API changes only affect repository implementation
//!
//! ## Architecture
//!
//! The repository pattern provides a clean abstraction layer:
//!
//! - Command Modules -> Repository Trait -> ClickUpApi -> HTTP Client -> ClickUp API
//! - Business Logic -> Repository Methods -> API Methods -> HTTP Requests
//!
//! ## Usage
//!
//! ```rust
//! use clickup_cli::repository::{ClickUpRepository, RepositoryFactory};
//! use clickup_cli::config::Config;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a default config (no authentication)
//!     let config = Config::default();
//!     let repo = RepositoryFactory::create(&config)?;
//!     
//!     // Use repository instead of direct API calls
//!     // Note: These calls will fail without proper authentication
//!     match repo.get_workspaces().await {
//!         Ok(workspaces) => println!("Found {} workspaces", workspaces.teams.len()),
//!         Err(e) => println!("Error: {}", e),
//!     }
//!     
//!     Ok(())
//! }
//! ```

use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::*;
use async_trait::async_trait;

/// Repository trait for ClickUp operations
///
/// This trait defines the contract for all ClickUp data operations,
/// providing a clean abstraction over the API layer. This allows
/// business logic to be decoupled from the specific API implementation.
///
/// ## Benefits
///
/// - **Abstraction**: Business logic doesn't need to know about HTTP details
/// - **Testability**: Easy to create mock implementations for testing
/// - **Extensibility**: Can add caching, logging, or other features transparently
/// - **Maintainability**: API changes only affect repository implementation
#[async_trait]
pub trait ClickUpRepository: Send + Sync {
    // User operations
    async fn get_user(&self) -> Result<User, ClickUpError>;
    
    // Workspace operations
    async fn get_workspaces(&self) -> Result<WorkspacesResponse, ClickUpError>;
    async fn get_workspace(&self, workspace_id: &str) -> Result<Workspace, ClickUpError>;
    
    // Space operations
    async fn get_spaces(&self, workspace_id: &str) -> Result<SpacesResponse, ClickUpError>;
    
    // List operations
    /// Retrieves a specific list by ID using the direct API endpoint
    async fn get_lists(&self, space_id: &str) -> Result<ListsResponse, ClickUpError>;
    async fn get_list(&self, list_id: &str) -> Result<List, ClickUpError>;
    
    // Folder operations
    async fn get_folders(&self, space_id: &str) -> Result<FoldersResponse, ClickUpError>;
    async fn get_folder_lists(&self, folder_id: &str) -> Result<ListsResponse, ClickUpError>;
    
    // Task operations
    async fn get_tasks(&self, list_id: &str) -> Result<TasksResponse, ClickUpError>;
    async fn get_task(&self, task_id: &str) -> Result<Task, ClickUpError>;
    async fn create_task(&self, list_id: &str, task: CreateTaskRequest) -> Result<Task, ClickUpError>;
    async fn update_task(&self, task_id: &str, task: UpdateTaskRequest) -> Result<Task, ClickUpError>;
    async fn delete_task(&self, task_id: &str) -> Result<(), ClickUpError>;
    async fn get_tasks_by_tag(&self, list_id: &str, tag: &str) -> Result<TasksResponse, ClickUpError>;
    async fn search_tasks_by_tag(&self, tag: String, workspace_id: Option<String>, space_id: Option<String>) -> Result<TasksResponse, ClickUpError>;
    
    // Comment operations
    async fn get_comments(&self, task_id: &str) -> Result<CommentsResponse, ClickUpError>;
    async fn get_comment(&self, comment_id: &str) -> Result<Comment, ClickUpError>;
    async fn create_comment(&self, task_id: &str, comment: CreateCommentRequest) -> Result<(), ClickUpError>;
    async fn delete_comment(&self, comment_id: &str) -> Result<(), ClickUpError>;
}

/// ClickUp API repository implementation
///
/// This struct implements the `ClickUpRepository` trait using the
/// `ClickUpApi` client. It provides a clean abstraction over the
/// API layer while maintaining all the functionality of the original API.
///
/// ## Features
///
/// - **Direct API Mapping**: All repository methods map directly to API methods
/// - **Error Propagation**: Proper error handling and propagation
/// - **Async Support**: Full async/await support for all operations
/// - **Thread Safety**: Safe for concurrent use across multiple threads
pub struct ClickUpApiRepository {
    api: ClickUpApi,
}

impl ClickUpApiRepository {
    /// Create a new repository instance
    ///
    /// # Arguments
    ///
    /// * `api` - The ClickUp API client to use for operations
    ///
    /// # Returns
    ///
    /// Returns a new `ClickUpApiRepository` instance
    pub fn new(api: ClickUpApi) -> Self {
        Self { api }
    }
}

#[async_trait]
impl ClickUpRepository for ClickUpApiRepository {
    // User operations
    async fn get_user(&self) -> Result<User, ClickUpError> {
        self.api.get_user().await
    }
    
    // Workspace operations
    async fn get_workspaces(&self) -> Result<WorkspacesResponse, ClickUpError> {
        self.api.get_workspaces().await
    }
    
    async fn get_workspace(&self, workspace_id: &str) -> Result<Workspace, ClickUpError> {
        self.api.get_workspace(workspace_id).await
    }
    
    // Space operations
    async fn get_spaces(&self, workspace_id: &str) -> Result<SpacesResponse, ClickUpError> {
        self.api.get_spaces(workspace_id).await
    }
    
    // List operations
    /// Retrieves a specific list by ID using the direct API endpoint
    async fn get_lists(&self, space_id: &str) -> Result<ListsResponse, ClickUpError> {
        self.api.get_lists(space_id).await
    }
    
    async fn get_list(&self, list_id: &str) -> Result<List, ClickUpError> {
        self.api.get_list(list_id).await
    }
    
    // Folder operations
    async fn get_folders(&self, space_id: &str) -> Result<FoldersResponse, ClickUpError> {
        self.api.get_folders(space_id).await
    }
    
    async fn get_folder_lists(&self, folder_id: &str) -> Result<ListsResponse, ClickUpError> {
        self.api.get_folder_lists(folder_id).await
    }
    
    // Task operations
    async fn get_tasks(&self, list_id: &str) -> Result<TasksResponse, ClickUpError> {
        self.api.get_tasks(list_id).await
    }
    
    async fn get_task(&self, task_id: &str) -> Result<Task, ClickUpError> {
        self.api.get_task(task_id).await
    }
    
    async fn create_task(&self, list_id: &str, task: CreateTaskRequest) -> Result<Task, ClickUpError> {
        self.api.create_task(list_id, task).await
    }
    
    async fn update_task(&self, task_id: &str, task: UpdateTaskRequest) -> Result<Task, ClickUpError> {
        self.api.update_task(task_id, task).await
    }
    
    async fn delete_task(&self, task_id: &str) -> Result<(), ClickUpError> {
        self.api.delete_task(task_id).await
    }
    
    async fn get_tasks_by_tag(&self, list_id: &str, tag: &str) -> Result<TasksResponse, ClickUpError> {
        self.api.get_tasks_by_tag(list_id, tag).await
    }
    
    async fn search_tasks_by_tag(&self, tag: String, workspace_id: Option<String>, space_id: Option<String>) -> Result<TasksResponse, ClickUpError> {
        self.api.search_tasks_by_tag(tag, workspace_id, space_id).await
    }
    
    // Comment operations
    async fn get_comments(&self, task_id: &str) -> Result<CommentsResponse, ClickUpError> {
        self.api.get_comments(task_id).await
    }
    
    async fn get_comment(&self, comment_id: &str) -> Result<Comment, ClickUpError> {
        self.api.get_comment(comment_id).await
    }
    
    async fn create_comment(&self, task_id: &str, comment: CreateCommentRequest) -> Result<(), ClickUpError> {
        self.api.create_comment(task_id, comment).await
    }
    

    
    async fn delete_comment(&self, comment_id: &str) -> Result<(), ClickUpError> {
        self.api.delete_comment(comment_id).await
    }
}

/// Factory for creating repository instances
///
/// This struct provides a factory pattern for creating repository instances
/// with the appropriate configuration and dependencies.
///
/// ## Usage
///
/// ```rust
/// use clickup_cli::repository::RepositoryFactory;
/// use clickup_cli::config::Config;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::load()?;
///     let repo = RepositoryFactory::create(&config)?;
///     Ok(())
/// }
/// ```
pub struct RepositoryFactory;

impl RepositoryFactory {
    /// Create a new repository instance
    ///
    /// This method creates a new `ClickUpApiRepository` instance with
    /// the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The application configuration
    ///
    /// # Returns
    ///
    /// Returns a new repository instance or a `ClickUpError` on failure
    ///
    /// # Errors
    ///
    /// This function can return:
    /// - `ClickUpError::NetworkError` if the API client cannot be created
    /// - `ClickUpError::AuthError` if authentication is invalid
    pub fn create(config: &Config) -> Result<Box<dyn ClickUpRepository>, ClickUpError> {
        let api = ClickUpApi::new(config.clone())?;
        let repository = ClickUpApiRepository::new(api);
        Ok(Box::new(repository))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    #[tokio::test]
    async fn test_repository_creation() {
        let config = Config::default();
        let repo = RepositoryFactory::create(&config);
        assert!(repo.is_ok());
    }
    
    #[tokio::test]
    async fn test_repository_trait_object() {
        let config = Config::default();
        let repo: Box<dyn ClickUpRepository> = RepositoryFactory::create(&config).unwrap();
        
        // Test that we can use the repository as a trait object
        assert!(repo.get_workspaces().await.is_err()); // Should fail without auth, but trait works
    }
} 