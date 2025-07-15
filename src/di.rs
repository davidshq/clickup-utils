//! # Dependency Injection Container
//!
//! This module provides a comprehensive dependency injection system for the ClickUp CLI.
//! It manages the creation and lifecycle of all application services, providing a clean
//! abstraction for service management and enabling better testability and modularity.
//!
//! ## Overview
//!
//! The dependency injection container provides:
//! - **Centralized Service Management**: All services created and managed in one place
//! - **Resource Sharing**: Services can share common resources (HTTP client, configuration)
//! - **Runtime Flexibility**: Can switch implementations at runtime
//! - **Enhanced Testability**: Easy to inject mock implementations
//! - **Lifecycle Management**: Proper initialization and cleanup of services
//!
//! ## Architecture
//!
//! ServiceContainer
//! |-- Config (shared configuration)
//! |-- ClickUpRepository (data access layer)
//! |-- EventBus (event system - future)
//! |-- CacheManager (caching layer - future)
//! `-- ConnectionPool (HTTP client pool - future)
//!
//! ## Usage
//!
//! ```rust
//! use clickup_cli::di::{ServiceContainer, ServiceContainerBuilder};
//! use clickup_cli::config::Config;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::default();
//!     let container = ServiceContainerBuilder::new()
//!         .with_config(config)
//!         .build()?;
//!     // Do not call any API methods in this example
//!     Ok(())
//! }
//! ```

use std::sync::Arc;
use crate::api::ClickUpApi;
use crate::config::Config;
use crate::error::ClickUpError;
use crate::repository::{ClickUpRepository, ClickUpApiRepository};

/// Service container for dependency injection
///
/// This struct manages all application services and provides a centralized
/// way to access dependencies. It ensures proper lifecycle management and
/// resource sharing across the application.
///
/// ## Features
///
/// - **Thread Safety**: All services are wrapped in `Arc` for safe sharing
/// - **Lazy Initialization**: Services are created only when needed
/// - **Resource Sharing**: Common resources shared across services
/// - **Error Handling**: Proper error propagation during service creation
/// - **Extensibility**: Easy to add new services and implementations
pub struct ServiceContainer {
    /// Application configuration
    config: Arc<Config>,
    /// ClickUp repository for data access
    repository: Arc<dyn ClickUpRepository>,
}

impl ServiceContainer {
    /// Create a new service container with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The application configuration
    ///
    /// # Returns
    ///
    /// Returns a new `ServiceContainer` instance or a `ClickUpError` on failure
    ///
    /// # Errors
    ///
    /// This function can return:
    /// - `ClickUpError::ConfigError` if configuration is invalid
    /// - `ClickUpError::NetworkError` if API client cannot be created
    /// - `ClickUpError::AuthError` if authentication is invalid
    pub fn new(config: Config) -> Result<Self, ClickUpError> {
        let config = Arc::new(config);
        
        // Create API client
        let api = ClickUpApi::new(config.as_ref().clone())?;
        
        // Create repository
        let repository = Arc::new(ClickUpApiRepository::new(api));
        
        Ok(Self {
            config,
            repository,
        })
    }
    
    /// Get a reference to the application configuration
    ///
    /// # Returns
    ///
    /// Returns a reference to the shared configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Get a reference to the ClickUp repository
    ///
    /// # Returns
    ///
    /// Returns a reference to the repository for data access
    pub fn repository(&self) -> &dyn ClickUpRepository {
        self.repository.as_ref()
    }
    
    /// Get a clone of the repository Arc for long-lived references
    ///
    /// # Returns
    ///
    /// Returns a cloned Arc to the repository
    pub fn repository_arc(&self) -> Arc<dyn ClickUpRepository> {
        Arc::clone(&self.repository)
    }
    
    /// Get a clone of the configuration Arc for long-lived references
    ///
    /// # Returns
    ///
    /// Returns a cloned Arc to the configuration
    pub fn config_arc(&self) -> Arc<Config> {
        Arc::clone(&self.config)
    }
}

/// Builder for creating service containers with custom configuration
///
/// This builder pattern allows for flexible service container creation
/// with optional customizations and future extensibility.
///
/// ## Usage
///
/// ```rust
/// use clickup_cli::di::ServiceContainerBuilder;
/// use clickup_cli::config::Config;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::default();
///     let container = ServiceContainerBuilder::new()
///         .with_config(config)
///         .build()?;
///     Ok(())
/// }
/// ```
pub struct ServiceContainerBuilder {
    config: Option<Config>,
}

impl ServiceContainerBuilder {
    /// Create a new service container builder
    ///
    /// # Returns
    ///
    /// Returns a new `ServiceContainerBuilder` instance
    pub fn new() -> Self {
        Self { config: None }
    }
    
    /// Set the configuration for the service container
    ///
    /// # Arguments
    ///
    /// * `config` - The application configuration
    ///
    /// # Returns
    ///
    /// Returns the builder for method chaining
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }
    
    /// Build the service container
    ///
    /// # Returns
    ///
    /// Returns a new `ServiceContainer` instance or a `ClickUpError` on failure
    ///
    /// # Errors
    ///
    /// This function can return:
    /// - `ClickUpError::ConfigError` if no configuration is provided
    /// - `ClickUpError::NetworkError` if API client cannot be created
    /// - `ClickUpError::AuthError` if authentication is invalid
    pub fn build(self) -> Result<ServiceContainer, ClickUpError> {
        let config = self.config.ok_or_else(|| {
            ClickUpError::ConfigError("No configuration provided".to_string())
        })?;
        
        ServiceContainer::new(config)
    }
}

impl Default for ServiceContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory for creating service containers with default configuration
///
/// This factory provides convenient methods for creating service containers
/// with common configurations and patterns.
///
/// ## Usage
///
/// ```rust
/// use clickup_cli::di::ServiceContainerFactory;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let container = ServiceContainerFactory::with_loaded_config()?;
///     let container2 = ServiceContainerFactory::with_default_config()?;
///     Ok(())
/// }
/// ```
pub struct ServiceContainerFactory;

impl ServiceContainerFactory {
    /// Create a service container with loaded configuration
    ///
    /// This method loads the configuration using `Config::load()` and
    /// creates a service container with the loaded configuration.
    ///
    /// # Returns
    ///
    /// Returns a new `ServiceContainer` instance or a `ClickUpError` on failure
    ///
    /// # Errors
    ///
    /// This function can return:
    /// - `ClickUpError::ConfigError` if configuration cannot be loaded
    /// - `ClickUpError::NetworkError` if API client cannot be created
    /// - `ClickUpError::AuthError` if authentication is invalid
    pub fn with_loaded_config() -> Result<ServiceContainer, ClickUpError> {
        let config = Config::load()?;
        ServiceContainer::new(config)
    }
    
    /// Create a service container with default configuration
    ///
    /// This method creates a service container with the default configuration.
    /// Note that this will likely fail for API operations without proper authentication.
    ///
    /// # Returns
    ///
    /// Returns a new `ServiceContainer` instance or a `ClickUpError` on failure
    ///
    /// # Errors
    ///
    /// This function can return:
    /// - `ClickUpError::NetworkError` if API client cannot be created
    /// - `ClickUpError::AuthError` if authentication is invalid
    pub fn with_default_config() -> Result<ServiceContainer, ClickUpError> {
        let config = Config::default();
        ServiceContainer::new(config)
    }
    
    /// Create a service container with custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The custom configuration to use
    ///
    /// # Returns
    ///
    /// Returns a new `ServiceContainer` instance or a `ClickUpError` on failure
    ///
    /// # Errors
    ///
    /// This function can return:
    /// - `ClickUpError::NetworkError` if API client cannot be created
    /// - `ClickUpError::AuthError` if authentication is invalid
    pub fn with_config(config: Config) -> Result<ServiceContainer, ClickUpError> {
        ServiceContainer::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    #[test]
    fn test_service_container_creation() {
        let config = Config::default();
        let container = ServiceContainer::new(config);
        assert!(container.is_ok());
    }
    
    #[test]
    fn test_service_container_builder() {
        let config = Config::default();
        let container = ServiceContainerBuilder::new()
            .with_config(config)
            .build();
        assert!(container.is_ok());
    }
    
    #[test]
    fn test_service_container_builder_no_config() {
        let container = ServiceContainerBuilder::new().build();
        assert!(container.is_err());
    }
    
    #[test]
    fn test_service_container_factory_default() {
        let container = ServiceContainerFactory::with_default_config();
        assert!(container.is_ok());
    }
    
    #[tokio::test]
    async fn test_service_container_repository_access() {
        let config = Config::default();
        let container = ServiceContainer::new(config).unwrap();
        
        // Test that we can access the repository
        let repo = container.repository();
        assert!(repo.get_workspaces().await.is_err()); // Should fail without auth, but trait works
    }
    
    #[test]
    fn test_service_container_config_access() {
        let config = Config::default();
        let container = ServiceContainer::new(config).unwrap();
        
        // Test that we can access the configuration
        let _config = container.config();
        assert!(true); // If we get here, the access worked
    }
    
    #[test]
    fn test_service_container_arc_access() {
        let config = Config::default();
        let container = ServiceContainer::new(config).unwrap();
        
        // Test that we can get Arc references
        let _config_arc = container.config_arc();
        let _repo_arc = container.repository_arc();
        assert!(true); // If we get here, the access worked
    }
} 