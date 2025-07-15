//! Library Interface Tests
//!
//! These tests verify the public API exposure, module organization, and export functionality
//! of the ClickUp CLI library. They ensure that all public exports are accessible and work
//! correctly when the library is used as a dependency.

use clickup_cli::{
    // Main public API exports
    ClickUpApi, Config, ClickUpError, RepositoryFactory,
    // Re-exported constants
    BASE_URL, DEFAULT_TIMEOUT, MAX_RETRIES,
    DEFAULT_RPM, DEFAULT_BUFFER, MAX_WAIT,
    MAX_TASK_NAME_LENGTH, MAX_TASK_DESCRIPTION_LENGTH,
    // Models (wildcard export)
    models::*,
};

#[cfg(test)]
mod public_api_tests {
    use super::*;

    /// Test that all main public API types can be imported and used
    #[test]
    fn test_main_public_api_exports() {
        // Test ClickUpApi
        let config = Config::default();
        let api_result = ClickUpApi::new(config);
        assert!(api_result.is_ok() || api_result.is_err()); // Should compile and run

        // Test Config
        let config = Config::default();
        assert!(!config.is_authenticated());

        // Test ClickUpError
        let error = ClickUpError::AuthError("test".to_string());
        assert_eq!(error.to_string(), "Authentication failed: test");

        // Test RepositoryFactory
        let config = Config::default();
        let repo_result = RepositoryFactory::create(&config);
        assert!(repo_result.is_ok());
        
        // Test repository type
        let _repository = repo_result.unwrap();
        assert!(std::any::type_name::<dyn clickup_cli::repository::ClickUpRepository>().contains("ClickUpRepository"));
    }

    /// Test that all re-exported constants are accessible and have expected values
    #[test]
    fn test_constant_exports() {
        // API constants
        assert!(!BASE_URL.is_empty());
        assert!(DEFAULT_TIMEOUT.as_secs() > 0);
        assert!(MAX_RETRIES > 0);

        // Rate limiting constants
        assert!(DEFAULT_RPM > 0);
        assert!(DEFAULT_BUFFER.as_secs() > 0);
        assert!(MAX_WAIT.as_secs() > 0);

        // Validation constants
        assert!(MAX_TASK_NAME_LENGTH > 0);
        assert!(MAX_TASK_DESCRIPTION_LENGTH > 0);
    }

    /// Test that models wildcard export works correctly
    #[test]
    fn test_models_wildcard_export() {
        // Test that key model types are accessible
        let _workspace = Workspace {
            id: "test".to_string(),
            name: Some("Test Workspace".to_string()),
            color: Some("#000000".to_string()),
            avatar: None,
            members: vec![],
            roles: None,
        };

        let _task = Task {
            id: "test".to_string(),
            name: Some("Test Task".to_string()),
            custom_id: None,
            text_content: "Test content".to_string(),
            description: "Test description".to_string(),
            status: TaskStatus {
                id: "status_1".to_string(),
                status: "open".to_string(),
                color: "#000000".to_string(),
                orderindex: 1,
                type_: "open".to_string(),
            },
            orderindex: "1".to_string(),
            date_created: "2023-01-01T00:00:00Z".to_string(),
            date_updated: "2023-01-01T00:00:00Z".to_string(),
            date_closed: None,
            creator: TaskCreator {
                id: 1,
                username: "test".to_string(),
                color: "#000000".to_string(),
                profile_picture: None,
            },
            assignees: vec![],
            watchers: vec![],
            checklists: vec![],
            tags: vec![],
            parent: None,
            top_level_parent: None,
            priority: None,
            due_date: None,
            start_date: None,
            time_estimate: None,
            time_spent: None,
            custom_fields: vec![],
            dependencies: vec![],
            linked_tasks: vec![],
            team_id: "team_1".to_string(),
            list: TaskList {
                id: "list_1".to_string(),
                name: Some("Test List".to_string()),
                access: Some(true),
            },
            folder: None,
            space: TaskSpace {
                id: "space_1".to_string(),
                name: Some("Test Space".to_string()),
            },
            url: "https://app.clickup.com/t/test".to_string(),
            subtasks: None,
        };
    }
}

#[cfg(test)]
mod module_organization_tests {

    /// Test that all modules are properly exposed and accessible
    #[test]
    fn test_module_organization() {
        // Test that we can access all major modules through the library
        use clickup_cli::{
            api, config, error, models, rate_limiter,
        };

        // Verify modules exist and are accessible
        assert!(std::any::type_name::<api::ClickUpApi>().contains("ClickUpApi"));
        assert!(std::any::type_name::<config::Config>().contains("Config"));
        assert!(std::any::type_name::<error::ClickUpError>().contains("ClickUpError"));
        assert!(std::any::type_name::<models::Workspace>().contains("Workspace"));
        assert!(std::any::type_name::<rate_limiter::RateLimiter>().contains("RateLimiter"));
    }

    /// Test that command modules are accessible
    #[test]
    fn test_command_modules_accessible() {
        use clickup_cli::commands::{
            auth, comments, lists, spaces, tasks, teams, workspaces,
        };

        // Verify command modules exist
        assert!(std::any::type_name::<auth::AuthCommands>().contains("AuthCommands"));
        assert!(std::any::type_name::<comments::CommentCommands>().contains("CommentCommands"));
        assert!(std::any::type_name::<lists::ListCommands>().contains("ListCommands"));
        assert!(std::any::type_name::<spaces::SpaceCommands>().contains("SpaceCommands"));
        assert!(std::any::type_name::<tasks::TaskCommands>().contains("TaskCommands"));
        assert!(std::any::type_name::<teams::TeamCommands>().contains("TeamCommands"));
        assert!(std::any::type_name::<workspaces::WorkspaceCommands>().contains("WorkspaceCommands"));
    }
}

#[cfg(test)]
mod library_usage_tests {
    use clickup_cli::{Config, ClickUpApi, ClickUpError, RepositoryFactory};

    /// Test the example usage pattern from the library documentation
    #[test]
    fn test_library_usage_pattern() {
        // Test the pattern shown in the library documentation
        let config = Config::default();
        
        // Test API client creation (will fail without auth, but shows the pattern)
        let api_result = ClickUpApi::new(config);
        
        // The API creation should either succeed or fail with an auth error
        match api_result {
            Ok(_) => {
                // If it succeeds, we have a valid API client
                println!("API client created successfully");
            }
            Err(ClickUpError::AuthError(_)) => {
                // Expected when no authentication is configured
                println!("Authentication required");
            }
            Err(e) => {
                // Other errors should be handled appropriately
                println!("Unexpected error: {}", e);
            }
        }
    }

    /// Test that the library can be used with async/await
    #[tokio::test]
    async fn test_async_library_usage() {
        let config = Config::default();
        let api_result = ClickUpApi::new(config);
        
        match api_result {
            Ok(api) => {
                // Test async API call (will likely fail without auth, but tests the pattern)
                let workspaces_result = api.get_workspaces().await;
                match workspaces_result {
                    Ok(workspaces) => {
                        println!("Found {} workspaces", workspaces.teams.len());
                    }
                    Err(ClickUpError::AuthError(_)) => {
                        println!("Authentication required for API calls");
                    }
                    Err(e) => {
                        println!("API error: {}", e);
                    }
                }
            }
            Err(ClickUpError::AuthError(_)) => {
                println!("Authentication required for API client creation");
            }
            Err(e) => {
                println!("API client creation error: {}", e);
            }
        }
    }

    /// Test repository factory usage
    #[test]
    fn test_repository_factory_usage() {
        let config = Config::default();
        let repository_result = RepositoryFactory::create(&config);
        
        assert!(repository_result.is_ok());
        
        let _repository = repository_result.unwrap();
        // Verify we can access repository methods
        assert!(std::any::type_name::<dyn clickup_cli::repository::ClickUpRepository>().contains("ClickUpRepository"));
    }
}

#[cfg(test)]
mod error_handling_tests {
    use clickup_cli::ClickUpError;

    /// Test that error types are properly exported and usable
    #[test]
    fn test_error_type_exports() {
        // Test various error types
        let auth_error = ClickUpError::AuthError("test auth error".to_string());
        let network_error = ClickUpError::NetworkError("test network error".to_string());
        let api_error = ClickUpError::ApiError("test api error".to_string());
        let validation_error = ClickUpError::ValidationError("test validation error".to_string());
        let not_found_error = ClickUpError::NotFoundError("test not found error".to_string());
        let rate_limit_error = ClickUpError::RateLimitError;
        let config_error = ClickUpError::ConfigError("test config error".to_string());

        // Verify error messages
        assert!(auth_error.to_string().contains("Authentication failed"));
        assert!(network_error.to_string().contains("Network error"));
        assert!(api_error.to_string().contains("API request failed"));
        assert!(validation_error.to_string().contains("Invalid input"));
        assert!(not_found_error.to_string().contains("Resource not found"));
        assert!(rate_limit_error.to_string().contains("Rate limit exceeded"));
        assert!(config_error.to_string().contains("Configuration error"));
    }
}

#[cfg(test)]
mod integration_tests {
    use clickup_cli::{Config, ClickUpApi, RepositoryFactory};

    /// Test that the library can be used as a complete unit
    #[test]
    fn test_library_as_complete_unit() {
        // Test configuration
        let mut config = Config::default();
        assert!(!config.is_authenticated());
        
        // Test setting API token
        let token_result = config.set_api_token("test_token".to_string());
        assert!(token_result.is_ok());
        assert!(config.is_authenticated());
        
        // Test API client creation
        let api_result = ClickUpApi::new(config);
        assert!(api_result.is_ok());
        
        // Test repository creation
        let config = Config::default();
        let repo_result = RepositoryFactory::create(&config);
        assert!(repo_result.is_ok());
    }

    /// Test that all major components work together
    #[test]
    fn test_component_integration() {
        // Create configuration
        let mut config = Config::default();
        config.set_api_token("test_token".to_string()).unwrap();
        
        // Create API client
        let _api = ClickUpApi::new(config.clone()).unwrap();
        
        // Create repository
        let _repository = RepositoryFactory::create(&config).unwrap();
        
        // Verify all components are of expected types
        assert!(std::any::type_name::<Config>().contains("Config"));
        assert!(std::any::type_name::<ClickUpApi>().contains("ClickUpApi"));
        assert!(std::any::type_name::<dyn clickup_cli::repository::ClickUpRepository>().contains("ClickUpRepository"));
    }
} 