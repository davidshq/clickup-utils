//! # ClickUp API Client
//! 
//! This module provides a high-level client for interacting with the ClickUp API.
//! It handles authentication, HTTP requests, response parsing, and error handling.
//! 
//! The `ClickUpApi` struct encapsulates all API operations and provides a clean
//! interface for making authenticated requests to various ClickUp endpoints.
//! 
//! ## Features
//! 
//! - **Authentication**: Automatic Bearer token handling
//! - **Error Handling**: Comprehensive error types and mapping
//! - **Request/Response**: JSON serialization and deserialization
//! - **Rate Limiting**: Built-in rate limit detection
//! - **Timeout Handling**: Configurable request timeouts
//! 
//! ## Usage
//! 
//! ```rust
//! use crate::api::ClickUpApi;
//! use crate::config::Config;
//! 
//! let config = Config::load()?;
//! let api = ClickUpApi::new(config)?;
//! 
//! // Get user information
//! let user = api.get_user().await?;
//! 
//! // Create a task
//! let task = api.create_task("list_id", task_data).await?;
//! ```

use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::*;
use log::{debug, error};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

/// ClickUp API client for making authenticated requests
/// 
/// This struct provides a high-level interface for interacting with the ClickUp API.
/// It handles authentication, HTTP requests, response parsing, and error handling.
/// 
/// The client is configured with a timeout and default headers, and automatically
/// includes authentication tokens in requests.
pub struct ClickUpApi {
    /// HTTP client for making requests
    client: Client,
    /// Application configuration containing API token and base URL
    config: Config,
}

impl ClickUpApi {
    /// Creates a new ClickUp API client
    /// 
    /// This function initializes the HTTP client with appropriate timeouts and
    /// default headers for JSON communication with the ClickUp API.
    /// 
    /// # Arguments
    /// 
    /// * `config` - The application configuration containing API settings
    /// 
    /// # Returns
    /// 
    /// Returns a new `ClickUpApi` instance on success, or a `ClickUpError` on failure.
    /// 
    /// # Errors
    /// 
    /// This function can return:
    /// - `ClickUpError::NetworkError` if the HTTP client cannot be created
    pub fn new(config: Config) -> Result<Self, ClickUpError> {
        // Set up default headers for JSON communication
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Create HTTP client with timeout and default headers
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .default_headers(headers)
            .build()
            .map_err(|e| ClickUpError::NetworkError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    /// Creates the authorization header for API requests
    /// 
    /// This function retrieves the API token from configuration and formats it
    /// appropriately for HTTP authorization headers. Personal tokens (starting with 'pk_')
    /// are used without 'Bearer' prefix, while OAuth tokens use 'Bearer' prefix.
    /// 
    /// # Returns
    /// 
    /// Returns a properly formatted `HeaderValue` for authorization, or a `ClickUpError`
    /// if the token is invalid or missing.
    /// 
    /// # Errors
    /// 
    /// This function can return:
    /// - `ClickUpError::AuthError` if the API token is missing or invalid
    fn get_auth_header(&self) -> Result<HeaderValue, ClickUpError> {
        let token = self.config.get_api_token()?;
        
        // Personal tokens (starting with 'pk_') should be used without 'Bearer'
        // OAuth tokens should use 'Bearer' prefix
        let auth_value = if token.starts_with("pk_") {
            // Personal token - use without Bearer
            token.to_string()
        } else {
            // OAuth token - use with Bearer
            format!("Bearer {}", token)
        };
        
        HeaderValue::from_str(&auth_value).map_err(|e| {
            ClickUpError::AuthError(format!("Invalid auth header: {}", e))
        })
    }

    /// Makes an HTTP request to the ClickUp API
    /// 
    /// This is the core function that handles all API communication. It automatically
    /// adds authentication headers, handles request bodies, and processes responses.
    /// 
    /// # Arguments
    /// 
    /// * `method` - The HTTP method to use (GET, POST, PUT, DELETE)
    /// * `endpoint` - The API endpoint path (without base URL)
    /// * `body` - Optional JSON body for POST/PUT requests
    /// 
    /// # Returns
    /// 
    /// Returns the deserialized response data on success, or a `ClickUpError` on failure.
    /// 
    /// # Errors
    /// 
    /// This function can return various errors including:
    /// - Network errors (timeout, connection issues)
    /// - Authentication errors (invalid token)
    /// - API errors (4xx, 5xx responses)
    /// - Serialization errors (invalid JSON)
    async fn make_request<T>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<Value>,
    ) -> Result<T, ClickUpError>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        // Construct the full URL
        let url = format!("{}{}", self.config.api_base_url, endpoint);
        let mut request = self.client.request(method, &url);

        // Add authentication header
        let auth_header = self.get_auth_header()?;
        request = request.header(AUTHORIZATION, auth_header);

        // Add request body if provided
        if let Some(body) = body {
            request = request.json(&body);
        }

        debug!("Making request to: {}", url);

        // Send the request
        let response = request.send().await.map_err(|e| {
            error!("Request failed: {}", e);
            ClickUpError::from(e)
        })?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| {
            error!("Failed to read response: {}", e);
            ClickUpError::NetworkError(format!("Failed to read response: {}", e))
        })?;

        debug!("Response status: {}, body: {}", status, response_text);

        // Handle error responses
        if !status.is_success() {
            let error_msg = if !response_text.is_empty() {
                response_text
            } else {
                format!("HTTP {}: {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown"))
            };

            return match status.as_u16() {
                401 => Err(ClickUpError::AuthError("Invalid API token".to_string())),
                403 => Err(ClickUpError::PermissionError("Insufficient permissions".to_string())),
                404 => Err(ClickUpError::NotFoundError("Resource not found".to_string())),
                429 => Err(ClickUpError::RateLimitError),
                _ => Err(ClickUpError::ApiError(error_msg)),
            };
        }

        // Parse the response JSON
        serde_json::from_str(&response_text).map_err(|e| {
            error!("Failed to parse response: {}", e);
            ClickUpError::DeserializationError(format!("Failed to parse response: {}", e))
        })
    }

    // User endpoints

    /// Retrieves the current user's information
    /// 
    /// This endpoint returns details about the authenticated user including
    /// their ID, username, email, and profile information.
    /// 
    /// # Returns
    /// 
    /// Returns a `User` struct containing the user's information.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication or network errors.
    pub async fn get_user(&self) -> Result<User, ClickUpError> {
        self.make_request(reqwest::Method::GET, "/user", None).await
    }

    // Workspace endpoints

    /// Retrieves all workspaces accessible to the authenticated user
    /// 
    /// This endpoint returns a list of all workspaces (teams) that the
    /// authenticated user has access to.
    /// 
    /// # Returns
    /// 
    /// Returns a `WorkspacesResponse` containing a list of workspaces.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication, permission, or network errors.
    pub async fn get_workspaces(&self) -> Result<WorkspacesResponse, ClickUpError> {
        self.make_request(reqwest::Method::GET, "/team", None).await
    }

    // Space endpoints

    /// Retrieves all spaces within a specific workspace
    /// 
    /// # Arguments
    /// 
    /// * `workspace_id` - The ID of the workspace to get spaces from
    /// 
    /// # Returns
    /// 
    /// Returns a `SpacesResponse` containing a list of spaces.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication, permission, or network errors.
    pub async fn get_spaces(&self, workspace_id: &str) -> Result<SpacesResponse, ClickUpError> {
        let endpoint = format!("/team/{}/space", workspace_id);
        self.make_request(reqwest::Method::GET, &endpoint, None).await
    }

    // List endpoints

    /// Retrieves all lists within a specific space
    /// 
    /// # Arguments
    /// 
    /// * `space_id` - The ID of the space to get lists from
    /// 
    /// # Returns
    /// 
    /// Returns a `ListsResponse` containing a list of lists.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication, permission, or network errors.
    pub async fn get_lists(&self, space_id: &str) -> Result<ListsResponse, ClickUpError> {
        let endpoint = format!("/space/{}/list", space_id);
        self.make_request(reqwest::Method::GET, &endpoint, None).await
    }

    // Task endpoints

    /// Retrieves all tasks within a specific list
    /// 
    /// # Arguments
    /// 
    /// * `list_id` - The ID of the list to get tasks from
    /// 
    /// # Returns
    /// 
    /// Returns a `TasksResponse` containing a list of tasks.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication, permission, or network errors.
    pub async fn get_tasks(&self, list_id: &str) -> Result<TasksResponse, ClickUpError> {
        let endpoint = format!("/list/{}/task", list_id);
        self.make_request(reqwest::Method::GET, &endpoint, None).await
    }

    /// Retrieves a specific task by its ID
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to retrieve
    /// 
    /// # Returns
    /// 
    /// Returns a `Task` struct containing the task's information.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication, permission, or network errors.
    pub async fn get_task(&self, task_id: &str) -> Result<Task, ClickUpError> {
        let endpoint = format!("/task/{}", task_id);
        self.make_request(reqwest::Method::GET, &endpoint, None).await
    }

    /// Creates a new task in a specific list
    /// 
    /// # Arguments
    /// 
    /// * `list_id` - The ID of the list to create the task in
    /// * `task_data` - The task data to create
    /// 
    /// # Returns
    /// 
    /// Returns the created `Task` with its assigned ID and metadata.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication, permission, validation, or network errors.
    pub async fn create_task(&self, list_id: &str, task_data: CreateTaskRequest) -> Result<Task, ClickUpError> {
        let endpoint = format!("/list/{}/task", list_id);
        let body = serde_json::to_value(task_data).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize task data: {}", e))
        })?;
        self.make_request(reqwest::Method::POST, &endpoint, Some(body)).await
    }

    /// Updates an existing task
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to update
    /// * `task_data` - The updated task data
    /// 
    /// # Returns
    /// 
    /// Returns the updated `Task` with the new information.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication, permission, validation, or network errors.
    pub async fn update_task(&self, task_id: &str, task_data: UpdateTaskRequest) -> Result<Task, ClickUpError> {
        let endpoint = format!("/task/{}", task_id);
        let body = serde_json::to_value(task_data).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize task data: {}", e))
        })?;
        self.make_request(reqwest::Method::PUT, &endpoint, Some(body)).await
    }

    /// Deletes a task
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to delete
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on successful deletion.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication, permission, or network errors.
    pub async fn delete_task(&self, task_id: &str) -> Result<(), ClickUpError> {
        let endpoint = format!("/task/{}", task_id);
        let _: Value = self.make_request(reqwest::Method::DELETE, &endpoint, None).await?;
        Ok(())
    }

    // Comment endpoints

    /// Retrieves all comments for a specific task
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to get comments from
    /// 
    /// # Returns
    /// 
    /// Returns a `CommentsResponse` containing a list of comments.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication, permission, or network errors.
    pub async fn get_comments(&self, task_id: &str) -> Result<CommentsResponse, ClickUpError> {
        let endpoint = format!("/task/{}/comment", task_id);
        self.make_request(reqwest::Method::GET, &endpoint, None).await
    }

    /// Creates a new comment on a task
    /// 
    /// # Arguments
    /// 
    /// * `task_id` - The ID of the task to comment on
    /// * `comment_data` - The comment data to create
    /// 
    /// # Returns
    /// 
    /// Returns the created `Comment` with its assigned ID and metadata.
    /// 
    /// # Errors
    /// 
    /// This function can return authentication, permission, validation, or network errors.
    pub async fn create_comment(&self, task_id: &str, comment_data: CreateCommentRequest) -> Result<Comment, ClickUpError> {
        let endpoint = format!("/task/{}/comment", task_id);
        let body = serde_json::to_value(comment_data).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize comment data: {}", e))
        })?;
        self.make_request(reqwest::Method::POST, &endpoint, Some(body)).await
    }
} 