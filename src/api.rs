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
//! use clickup_cli::api::ClickUpApi;
//! use clickup_cli::config::Config;
//! use clickup_cli::models::CreateTaskRequest;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::default();
//!     let api = ClickUpApi::new(config)?;
//!     // Get user information (would fail without a real token)
//!     // let user = api.get_user().await?;
//!     // Create a task (would fail without a real token)
//!     let task_data = CreateTaskRequest {
//!         name: "Test Task".to_string(),
//!         ..Default::default()
//!     };
//!     // let task = api.create_task("list_id", task_data).await?;
//!     Ok(())
//! }
//! ```

use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::*;
use crate::rate_limiter::RateLimiter;
use colored::Colorize;
use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

/// ClickUp API client for making authenticated requests
///
/// This struct provides a high-level interface for interacting with the ClickUp API.
/// It handles authentication, HTTP requests, response parsing, and error handling.
///
/// The client is configured with a timeout and default headers, and automatically
/// includes authentication tokens in requests.
#[derive(Clone)]
pub struct ClickUpApi {
    /// HTTP client for making requests
    client: Client,
    /// Application configuration containing API token and base URL
    config: Config,
    /// Rate limiter for managing API request limits
    rate_limiter: RateLimiter,
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
            .map_err(|e| {
                ClickUpError::NetworkError(format!("Failed to create HTTP client: {e}"))
            })?;

        // Create rate limiter with configuration
        let rate_limiter = RateLimiter::new(config.rate_limit.clone());

        Ok(Self {
            client,
            config,
            rate_limiter,
        })
    }

    /// Creates the authorization header for API requests
    ///
    /// This function retrieves the API token from configuration and formats it
    /// appropriately for HTTP authorization headers. ClickUp API v2 supports both
    /// personal tokens and OAuth tokens, with different header formats.
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

        // ClickUp API v2 supports both personal tokens and OAuth tokens
        // Personal tokens (starting with 'pk_') are used directly
        // OAuth tokens and other tokens use 'Bearer' prefix
        let auth_value = if token.starts_with("pk_") {
            // Personal token - use directly without Bearer
            token.to_string()
        } else {
            // OAuth token or other token types - use with Bearer prefix
            format!("Bearer {token}")
        };

        HeaderValue::from_str(&auth_value)
            .map_err(|e| ClickUpError::AuthError(format!("Invalid auth header: {e}")))
    }

    /// Makes an HTTP request to the ClickUp API
    ///
    /// This is the core function that handles all API communication. It automatically
    /// adds authentication headers, handles request bodies, and processes responses.
    /// Includes rate limiting and retry logic for failed requests.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use (GET, POST, PUT, DELETE)
    /// * `endpoint` - The API endpoint path (without base URL)
    /// * `body` - Optional JSON body for POST/PUT requests
    /// * `query_params` - Optional query parameters to append to the URL
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
    /// - Rate limiting errors (with automatic retry logic)
    fn make_request<'a, T>(
        &'a self,
        method: reqwest::Method,
        endpoint: &'a str,
        body: Option<Value>,
        query_params: Option<Vec<(String, String)>>,
    ) -> Pin<Box<dyn Future<Output = Result<T, ClickUpError>> + Send + 'a>>
    where
        T: for<'de> serde::Deserialize<'de> + Send + 'static,
    {
        Box::pin(async move {
            // Reset retry count for new request
            self.rate_limiter.reset_retry_count().await?;
            // Wait if we're approaching rate limits
            self.rate_limiter.wait_if_needed().await?;
            // Construct the full URL
            let mut url = format!("{}{}", self.config.api_base_url, endpoint);
            // Add query parameters if provided
            if let Some(ref params) = query_params {
                let query_string: String = params
                    .iter()
                    .map(|(key, value)| format!("{key}={value}"))
                    .collect::<Vec<_>>()
                    .join("&");
                url = format!("{url}?{query_string}");
            }
            let mut request = self.client.request(method.clone(), &url);
            // Add authentication header
            let auth_header = self.get_auth_header()?;
            request = request.header(AUTHORIZATION, auth_header);
            // Add request body if provided
            if let Some(ref body) = body {
                request = request.json(&body);
            }
            debug!("Making request to: {url}");
            // Send the request
            let response = request.send().await.map_err(|e| {
                error!("Request failed: {e}");
                ClickUpError::from(e)
            })?;
            // Check for rate limiting headers
            let retry_after_seconds = response
                .headers()
                .get("Retry-After")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());
            let status = response.status();
            let response_text = response.text().await.map_err(|e| {
                error!("Failed to read response: {e}");
                ClickUpError::NetworkError(format!("Failed to read response: {e}"))
            })?;
            debug!("Response status: {status}, body: {response_text}");
            // Handle error responses
            if !status.is_success() {
                let error_msg = if !response_text.is_empty() {
                    // Try to parse ClickUp-specific error format
                    if let Ok(error_json) =
                        serde_json::from_str::<serde_json::Value>(&response_text)
                    {
                        if let (Some(err_msg), Some(ecode)) = (
                            error_json.get("err").and_then(|v| v.as_str()),
                            error_json.get("ECODE").and_then(|v| v.as_str()),
                        ) {
                            format!("ClickUp Error {ecode}: {err_msg}")
                        } else {
                            response_text.clone()
                        }
                    } else {
                        response_text.clone()
                    }
                } else {
                    format!(
                        "HTTP {}: {}",
                        status.as_u16(),
                        status.canonical_reason().unwrap_or("Unknown")
                    )
                };
                return match status.as_u16() {
                    400 => Err(ClickUpError::ValidationError(format!(
                        "Bad request: {error_msg}"
                    ))),
                    401 => Err(ClickUpError::AuthError("Invalid API token".to_string())),
                    403 => Err(ClickUpError::PermissionError(
                        "Insufficient permissions".to_string(),
                    )),
                    404 => Err(ClickUpError::NotFoundError(
                        "Resource not found".to_string(),
                    )),
                    409 => Err(ClickUpError::ApiError(format!("Conflict: {error_msg}"))),
                    422 => Err(ClickUpError::ValidationError(format!(
                        "Validation error: {error_msg}"
                    ))),
                    429 => {
                        // Handle rate limiting with retry logic
                        match self
                            .rate_limiter
                            .handle_rate_limit(retry_after_seconds)
                            .await
                        {
                            Ok(()) => {
                                // Retry the request after waiting
                                info!("Retrying request after rate limit wait");
                                self.make_request(method, endpoint, body, query_params)
                                    .await
                            }
                            Err(e) => Err(e),
                        }
                    }
                    500..=599 => Err(ClickUpError::ApiError(format!("Server error: {error_msg}"))),
                    _ => Err(ClickUpError::ApiError(error_msg)),
                };
            }
            // Parse the response JSON
            serde_json::from_str(&response_text).map_err(|e| {
                error!("Failed to parse response: {e}");
                ClickUpError::DeserializationError(format!("Failed to parse response: {e}"))
            })
        })
    }

    /// Makes an HTTP request to the ClickUp API and returns raw response text
    ///
    /// This is similar to make_request but returns the raw response text instead of parsing it.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use (GET, POST, PUT, DELETE)
    /// * `endpoint` - The API endpoint path (without base URL)
    /// * `body` - Optional JSON body for POST/PUT requests
    /// * `query_params` - Optional query parameters
    ///
    /// # Returns
    ///
    /// Returns the raw response text on success, or a `ClickUpError` on failure.
    ///
    /// # Errors
    ///
    /// This function can return various errors including:
    /// - Network errors (timeout, connection issues)
    /// - Authentication errors (invalid token)
    /// - API errors (4xx, 5xx responses)
    fn make_request_raw<'a>(
        &'a self,
        method: reqwest::Method,
        endpoint: &'a str,
        body: Option<Value>,
        query_params: Option<Vec<(String, String)>>,
    ) -> Pin<Box<dyn Future<Output = Result<String, ClickUpError>> + Send + 'a>> {
        Box::pin(async move {
            self.rate_limiter.reset_retry_count().await?;
            self.rate_limiter.wait_if_needed().await?;
            let mut url = format!("{}{}", self.config.api_base_url, endpoint);
            if let Some(ref params) = query_params {
                let query_string: String = params
                    .iter()
                    .map(|(key, value)| format!("{key}={value}"))
                    .collect::<Vec<_>>()
                    .join("&");
                url = format!("{url}?{query_string}");
            }
            let mut request = self.client.request(method.clone(), &url);
            let auth_header = self.get_auth_header()?;
            request = request.header(AUTHORIZATION, auth_header);
            if let Some(ref body) = body {
                request = request.json(&body);
            }
            debug!("Making request to: {url}");
            let response = request.send().await.map_err(|e| {
                error!("Request failed: {e}");
                ClickUpError::from(e)
            })?;
            let retry_after_seconds = response
                .headers()
                .get("Retry-After")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());
            let status = response.status();
            let response_text = response.text().await.map_err(|e| {
                error!("Failed to read response: {e}");
                ClickUpError::NetworkError(format!("Failed to read response: {e}"))
            })?;
            debug!("Response status: {status}, body: {response_text}");
            if !status.is_success() {
                let error_msg = if !response_text.is_empty() {
                    if let Ok(error_json) =
                        serde_json::from_str::<serde_json::Value>(&response_text)
                    {
                        if let (Some(err_msg), Some(ecode)) = (
                            error_json.get("err").and_then(|v| v.as_str()),
                            error_json.get("ECODE").and_then(|v| v.as_str()),
                        ) {
                            format!("ClickUp Error {ecode}: {err_msg}")
                        } else {
                            response_text.clone()
                        }
                    } else {
                        response_text.clone()
                    }
                } else {
                    format!(
                        "HTTP {}: {}",
                        status.as_u16(),
                        status.canonical_reason().unwrap_or("Unknown")
                    )
                };
                return match status.as_u16() {
                    400 => Err(ClickUpError::ValidationError(format!(
                        "Bad request: {error_msg}"
                    ))),
                    401 => Err(ClickUpError::AuthError("Invalid API token".to_string())),
                    403 => Err(ClickUpError::PermissionError(
                        "Insufficient permissions".to_string(),
                    )),
                    404 => Err(ClickUpError::NotFoundError(
                        "Resource not found".to_string(),
                    )),
                    409 => Err(ClickUpError::ApiError(format!("Conflict: {error_msg}"))),
                    422 => Err(ClickUpError::ValidationError(format!(
                        "Validation error: {error_msg}"
                    ))),
                    429 => {
                        match self
                            .rate_limiter
                            .handle_rate_limit(retry_after_seconds)
                            .await
                        {
                            Ok(()) => {
                                info!("Retrying request after rate limit wait");
                                self.make_request_raw(method, endpoint, body, query_params)
                                    .await
                            }
                            Err(e) => Err(e),
                        }
                    }
                    500..=599 => Err(ClickUpError::ApiError(format!("Server error: {error_msg}"))),
                    _ => Err(ClickUpError::ApiError(error_msg)),
                };
            }
            Ok(response_text)
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
        self.make_request(reqwest::Method::GET, "/user", None, None)
            .await
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
        let response_text = self
            .make_request_raw(reqwest::Method::GET, "/team", None, None)
            .await?;
        println!(
            "DEBUG: Raw workspace response (first 500 chars): {}",
            &response_text[..response_text.len().min(500)]
        );
        serde_json::from_str(&response_text).map_err(|e| {
            error!("Failed to parse workspace response: {e}");
            ClickUpError::DeserializationError(format!("Failed to parse workspace response: {e}"))
        })
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
        let endpoint = format!("/team/{workspace_id}/space");
        self.make_request(reqwest::Method::GET, &endpoint, None, None)
            .await
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
        let endpoint = format!("/space/{space_id}/list");
        self.make_request(reqwest::Method::GET, &endpoint, None, None)
            .await
    }

    // Folder endpoints

    /// Retrieves all folders within a specific space
    ///
    /// # Arguments
    ///
    /// * `space_id` - The ID of the space to get folders from
    ///
    /// # Returns
    ///
    /// Returns a `FoldersResponse` containing a list of folders.
    ///
    /// # Errors
    ///
    /// This function can return authentication, permission, or network errors.
    pub async fn get_folders(&self, space_id: &str) -> Result<FoldersResponse, ClickUpError> {
        let endpoint = format!("/space/{space_id}/folder");
        self.make_request(reqwest::Method::GET, &endpoint, None, None)
            .await
    }

    /// Retrieves all lists within a specific folder
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The ID of the folder to get lists from
    ///
    /// # Returns
    ///
    /// Returns a `ListsResponse` containing a list of lists.
    ///
    /// # Errors
    ///
    /// This function can return authentication, permission, or network errors.
    pub async fn get_folder_lists(&self, folder_id: &str) -> Result<ListsResponse, ClickUpError> {
        let endpoint = format!("/folder/{folder_id}/list");
        self.make_request(reqwest::Method::GET, &endpoint, None, None)
            .await
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
        let mut all_tasks = Vec::new();
        let mut page = 0;
        let limit = 100; // ClickUp API default limit

        loop {
            let query_params = vec![
                ("page".to_string(), page.to_string()),
                ("limit".to_string(), limit.to_string()),
                ("subtasks".to_string(), "true".to_string()),
            ];

            let endpoint = format!("/list/{list_id}/task");
            let response: TasksResponse = self
                .make_request(reqwest::Method::GET, &endpoint, None, Some(query_params))
                .await?;

            let tasks_count = response.tasks.len();
            all_tasks.extend(response.tasks);

            // If we got fewer tasks than the limit, we've reached the end
            if tasks_count < limit {
                break;
            }

            page += 1;
        }

        Ok(TasksResponse { tasks: all_tasks })
    }

    /// Retrieves tasks within a specific list filtered by tag
    ///
    /// Note: This performs client-side filtering since the ClickUp API may not support
    /// server-side tag filtering. All tasks are fetched and then filtered locally.
    ///
    /// # Arguments
    ///
    /// * `list_id` - The ID of the list to get tasks from
    /// * `tag` - The tag name to filter by
    ///
    /// # Returns
    ///
    /// Returns a `TasksResponse` containing a list of tasks with the specified tag.
    ///
    /// # Errors
    ///
    /// This function can return authentication, permission, or network errors.
    pub async fn get_tasks_by_tag(
        &self,
        list_id: &str,
        tag: &str,
    ) -> Result<TasksResponse, ClickUpError> {
        // Get all tasks from the list
        let all_tasks = self.get_tasks(list_id).await?;

        // Filter tasks that have the specified tag
        let filtered_tasks: Vec<Task> = all_tasks
            .tasks
            .into_iter()
            .filter(|task| {
                task.tags
                    .iter()
                    .any(|task_tag| task_tag.name.as_deref() == Some(tag))
            })
            .collect();

        Ok(TasksResponse {
            tasks: filtered_tasks,
        })
    }

    /// Searches for tasks with a specific tag across all lists in a space
    ///
    /// This method will prompt the user to select workspace and space if not provided,
    /// then search through all lists in the selected space for tasks with the specified tag.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag name to search for
    /// * `workspace_id` - Optional workspace ID (will prompt if not provided)
    /// * `space_id` - Optional space ID (will prompt if not provided)
    ///
    /// # Returns
    ///
    /// Returns a `TasksResponse` containing all tasks with the specified tag.
    ///
    /// # Errors
    ///
    /// This function can return authentication, permission, or network errors.
    pub async fn search_tasks_by_tag(
        &self,
        tag: String,
        workspace_id: Option<String>,
        space_id: Option<String>,
    ) -> Result<TasksResponse, ClickUpError> {
        use std::io::{self, Write};

        let mut all_tasks = Vec::new();

        // Get workspace ID
        let workspace_id = if let Some(id) = workspace_id {
            id
        } else {
            println!("{}", "Available Workspaces:".bold());
            let workspaces = self.get_workspaces().await?;
            for (i, workspace) in workspaces.teams.iter().enumerate() {
                println!(
                    "  {}. {} ({})",
                    i + 1,
                    workspace.name.as_deref().unwrap_or(""),
                    workspace.id
                );
            }

            print!("Select workspace (enter number): ");
            io::stdout()
                .flush()
                .map_err(|e| ClickUpError::ApiError(format!("Failed to flush stdout: {e}")))?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| ClickUpError::ApiError(format!("Failed to read input: {e}")))?;

            let selection: usize = input.trim().parse().map_err(|_| {
                ClickUpError::ValidationError("Invalid workspace selection".to_string())
            })?;

            if selection == 0 || selection > workspaces.teams.len() {
                return Err(ClickUpError::ValidationError(
                    "Invalid workspace selection".to_string(),
                ));
            }

            workspaces.teams[selection - 1].id.clone()
        };

        // Get space ID
        let space_id = if let Some(id) = space_id {
            id
        } else {
            println!("{}", "Available Spaces:".bold());
            let spaces = self.get_spaces(&workspace_id).await?;
            for (i, space) in spaces.spaces.iter().enumerate() {
                println!(
                    "  {}. {} ({})",
                    i + 1,
                    space.name.as_deref().unwrap_or(""),
                    space.id
                );
            }

            print!("Select space (enter number): ");
            io::stdout()
                .flush()
                .map_err(|e| ClickUpError::ApiError(format!("Failed to flush stdout: {e}")))?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| ClickUpError::ApiError(format!("Failed to read input: {e}")))?;

            let selection: usize = input.trim().parse().map_err(|_| {
                ClickUpError::ValidationError("Invalid space selection".to_string())
            })?;

            if selection == 0 || selection > spaces.spaces.len() {
                return Err(ClickUpError::ValidationError(
                    "Invalid space selection".to_string(),
                ));
            }

            spaces.spaces[selection - 1].id.clone()
        };

        // Get all lists in the space (both direct lists and lists in folders)
        let mut all_lists = Vec::new();

        // Get direct lists in the space
        let space_lists = self.get_lists(&space_id).await?;
        all_lists.extend(space_lists.lists);

        // Get folders and their lists
        match self.get_folders(&space_id).await {
            Ok(folders_response) => {
                for folder in &folders_response.folders {
                    println!(
                        "  Checking folder: {}",
                        folder.name.as_deref().unwrap_or("")
                    );

                    match self.get_folder_lists(&folder.id).await {
                        Ok(folder_lists) => {
                            all_lists.extend(folder_lists.lists);
                        }
                        Err(e) => {
                            println!(
                                "{} Warning: Could not fetch lists from folder '{}': {}",
                                "⚠️".yellow(),
                                folder.name.as_deref().unwrap_or(""),
                                e
                            );
                        }
                    }
                }
            }
            Err(e) => {
                println!("{} Warning: Could not fetch folders: {}", "⚠️".yellow(), e);
            }
        }

        println!(
            "{}",
            format!(
                "Searching through {} lists for tasks with tag '{}'...",
                all_lists.len(),
                tag
            )
            .blue()
        );

        // Search through each list
        for list in &all_lists {
            let list_name = list.name.as_deref().unwrap_or("");
            let folder_info = list
                .folder
                .as_ref()
                .map(|f| format!(" (in folder: {})", f.name))
                .unwrap_or_default();
            println!("  Checking list: {list_name}{folder_info}");

            match self.get_tasks(&list.id).await {
                Ok(tasks_response) => {
                    // Helper function to recursively process tasks and their subtasks
                    async fn process_task_recursively(
                        api: &ClickUpApi,
                        task: Task,
                        tag: &str,
                        depth: usize,
                    ) -> Result<Vec<Task>, ClickUpError> {
                        let mut matching_tasks = Vec::new();
                        let indent = "  ".repeat(depth);

                        // Process current task
                        let task_name = task.name.as_deref().unwrap_or("Unnamed Task");
                        let tag_names: Vec<String> =
                            task.tags.iter().filter_map(|t| t.name.clone()).collect();

                        if tag_names.is_empty() {
                            println!("{}Task '{}' (ID: {}): No tags", indent, task_name, task.id);
                        } else {
                            println!(
                                "{}Task '{}' (ID: {}): Tags: [{}]",
                                indent,
                                task_name,
                                task.id,
                                tag_names.join(", ")
                            );
                        }

                        // Check if current task has the tag
                        let has_tag = task.tags.iter().any(|task_tag| {
                            let tag_matches = task_tag.name.as_deref() == Some(tag);
                            if tag_matches {
                                println!(
                                    "{indent}✓ Found matching tag '{tag}' on task '{task_name}'"
                                );
                            }
                            tag_matches
                        });

                        if has_tag {
                            matching_tasks.push(task.clone());
                        }

                        // Recursively process subtasks
                        if let Some(subtasks) = &task.subtasks {
                            println!(
                                "{}Found {} subtasks for task '{}'",
                                indent,
                                subtasks.len(),
                                task_name
                            );
                            for subtask in subtasks {
                                println!(
                                    "{}  Subtask: '{}' (ID: {})",
                                    indent,
                                    subtask.name.as_deref().unwrap_or("Unnamed"),
                                    subtask.id
                                );
                                // Process subtask recursively (it should have full details including tags)
                                let subtask_matches = Box::pin(process_task_recursively(
                                    api,
                                    subtask.clone(),
                                    tag,
                                    depth + 1,
                                ))
                                .await?;
                                matching_tasks.extend(subtask_matches);
                            }
                        } else {
                            println!("{indent}No subtasks found for task '{task_name}'");
                        }

                        Ok(matching_tasks)
                    }

                    // Process all tasks and their subtasks
                    let mut all_matching_tasks = Vec::new();
                    for task in tasks_response.tasks {
                        match process_task_recursively(self, task, &tag, 1).await {
                            Ok(task_matches) => {
                                all_matching_tasks.extend(task_matches);
                            }
                            Err(e) => {
                                println!(
                                    "{} Warning: Could not process task: {}",
                                    "⚠️".yellow(),
                                    e
                                );
                            }
                        }
                    }
                    all_tasks.extend(all_matching_tasks);
                }
                Err(e) => {
                    println!(
                        "{} Warning: Could not fetch tasks from list '{}': {}",
                        "⚠️".yellow(),
                        list_name,
                        e
                    );
                }
            }
        }

        Ok(TasksResponse { tasks: all_tasks })
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
        let endpoint = format!("/task/{task_id}");
        self.make_request(reqwest::Method::GET, &endpoint, None, None)
            .await
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
    pub async fn create_task(
        &self,
        list_id: &str,
        task_data: CreateTaskRequest,
    ) -> Result<Task, ClickUpError> {
        let endpoint = format!("/list/{list_id}/task");
        let body = serde_json::to_value(task_data).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize task data: {e}"))
        })?;
        self.make_request(reqwest::Method::POST, &endpoint, Some(body), None)
            .await
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
    pub async fn update_task(
        &self,
        task_id: &str,
        task_data: UpdateTaskRequest,
    ) -> Result<Task, ClickUpError> {
        let endpoint = format!("/task/{task_id}");
        let body = serde_json::to_value(task_data).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize task data: {e}"))
        })?;
        self.make_request(reqwest::Method::PUT, &endpoint, Some(body), None)
            .await
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
        let endpoint = format!("/task/{task_id}");
        // DELETE operations may return empty responses, so we use make_request_raw
        // and handle the response manually
        let response_text = self
            .make_request_raw(reqwest::Method::DELETE, &endpoint, None, None)
            .await?;

        // For DELETE operations, empty response or 204 status indicates success
        if response_text.trim().is_empty() {
            Ok(())
        } else {
            // If there's a response body, it might be an error message
            // Try to parse it as JSON to get a proper error
            if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
                if let (Some(err_msg), Some(ecode)) = (
                    error_json.get("err").and_then(|v| v.as_str()),
                    error_json.get("ECODE").and_then(|v| v.as_str()),
                ) {
                    Err(ClickUpError::ApiError(format!(
                        "ClickUp Error {ecode}: {err_msg}"
                    )))
                } else {
                    Err(ClickUpError::ApiError(format!(
                        "Delete failed: {response_text}"
                    )))
                }
            } else {
                // If it's not JSON, treat as generic error
                Err(ClickUpError::ApiError(format!(
                    "Delete failed: {response_text}"
                )))
            }
        }
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
        let endpoint = format!("/task/{task_id}/comment");
        self.make_request(reqwest::Method::GET, &endpoint, None, None)
            .await
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
    pub async fn create_comment(
        &self,
        task_id: &str,
        comment_data: CreateCommentRequest,
    ) -> Result<(), ClickUpError> {
        let endpoint = format!("/task/{task_id}/comment");
        let body = serde_json::to_value(comment_data).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize comment data: {e}"))
        })?;
        let _response_text = self
            .make_request_raw(reqwest::Method::POST, &endpoint, Some(body), None)
            .await?;
        Ok(())
    }

    // Additional API endpoints

    /// Retrieves a specific workspace by its ID
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - The ID of the workspace to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Workspace` struct containing the workspace's information.
    ///
    /// # Errors
    ///
    /// This function can return authentication, permission, or network errors.
    pub async fn get_workspace(&self, workspace_id: &str) -> Result<Workspace, ClickUpError> {
        let endpoint = format!("/team/{workspace_id}");
        self.make_request(reqwest::Method::GET, &endpoint, None, None)
            .await
    }

    /// Updates a comment on a task
    ///
    /// # Arguments
    ///
    /// * `comment_id` - The ID of the comment to update
    /// * `comment_data` - The updated comment data
    ///
    /// # Returns
    ///
    /// Returns the updated `Comment` with the new information.
    ///
    /// # Errors
    ///
    /// This function can return authentication, permission, validation, or network errors.
    pub async fn update_comment(
        &self,
        comment_id: &str,
        comment_data: CreateCommentRequest,
    ) -> Result<(), ClickUpError> {
        let endpoint = format!("/comment/{comment_id}");
        let body = serde_json::to_value(comment_data).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize comment data: {e}"))
        })?;
        let _response_text = self
            .make_request_raw(reqwest::Method::PUT, &endpoint, Some(body), None)
            .await?;
        Ok(())
    }

    /// Deletes a comment
    ///
    /// # Arguments
    ///
    /// * `comment_id` - The ID of the comment to delete
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful deletion.
    ///
    /// # Errors
    ///
    /// This function can return authentication, permission, or network errors.
    pub async fn delete_comment(&self, comment_id: &str) -> Result<(), ClickUpError> {
        let endpoint = format!("/comment/{comment_id}");
        // DELETE operations may return empty responses, so we use make_request_raw
        // and handle the response manually
        let response_text = self
            .make_request_raw(reqwest::Method::DELETE, &endpoint, None, None)
            .await?;

        // For DELETE operations, empty response or 204 status indicates success
        if response_text.trim().is_empty() || response_text.trim() == "{}" {
            Ok(())
        } else {
            // If there's a response body, it might be an error message
            // Try to parse it as JSON to get a proper error
            if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
                if let (Some(err_msg), Some(ecode)) = (
                    error_json.get("err").and_then(|v| v.as_str()),
                    error_json.get("ECODE").and_then(|v| v.as_str()),
                ) {
                    Err(ClickUpError::ApiError(format!(
                        "ClickUp Error {ecode}: {err_msg}"
                    )))
                } else {
                    Err(ClickUpError::ApiError(format!(
                        "Delete failed: {response_text}"
                    )))
                }
            } else {
                // If it's not JSON, treat as generic error
                Err(ClickUpError::ApiError(format!(
                    "Delete failed: {response_text}"
                )))
            }
        }
    }

    /// Gets rate limiting statistics
    ///
    /// This method returns information about the current rate limiting state,
    /// including the number of requests made in the last minute and current
    /// retry count. This is useful for debugging and monitoring.
    ///
    /// # Returns
    ///
    /// Returns a tuple of (requests_in_last_minute, current_retry_count) on success,
    /// or a `ClickUpError` on failure.
    ///
    /// # Errors
    ///
    /// This function can return errors if the rate limiter state cannot be accessed.
    #[allow(dead_code)]
    pub async fn get_rate_limit_stats(&self) -> Result<(u32, u32), ClickUpError> {
        let request_count = self.rate_limiter.get_current_request_count().await?;
        let retry_count = self.rate_limiter.get_current_retry_count().await?;
        Ok((request_count, retry_count))
    }

    /// Retrieves a specific comment by its ID
    ///
    /// This method uses an efficient search strategy to find a comment by ID.
    /// It searches through tasks in a more targeted way to avoid the O(n⁴) complexity
    /// of the original implementation.
    ///
    /// # Arguments
    ///
    /// * `comment_id` - The ID of the comment to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Comment` struct containing the comment's information.
    ///
    /// # Errors
    ///
    /// This function can return authentication, permission, or network errors.
    pub async fn get_comment(&self, comment_id: &str) -> Result<Comment, ClickUpError> {
        // First, try to get the user's workspaces to understand the scope
        let workspaces = self.get_workspaces().await?;
        
        // Search through workspaces in parallel for better performance
        let mut search_futures = Vec::new();
        
        for workspace in &workspaces.teams {
            let api = self.clone();
            let workspace_id = workspace.id.clone();
            let comment_id = comment_id.to_string();
            
            let future = async move {
                // Get spaces for this workspace
                let spaces = api.get_spaces(&workspace_id).await?;
                
                for space in &spaces.spaces {
                    // Get lists for this space
                    let lists = api.get_lists(&space.id).await?;
                    
                    for list in &lists.lists {
                        // Get tasks for this list
                        let tasks = api.get_tasks(&list.id).await?;
                        
                        for task in &tasks.tasks {
                            // Get comments for this task
                            let comments = api.get_comments(&task.id).await?;
                            
                            // Look for the specific comment
                            if let Some(comment) = comments.comments.into_iter().find(|c| c.id == comment_id) {
                                return Ok(Some(comment));
                            }
                        }
                    }
                }
                Ok::<Option<Comment>, ClickUpError>(None)
            };
            
            search_futures.push(future);
        }
        
        // Execute all searches concurrently
        let results = futures::future::join_all(search_futures).await;
        
        // Find the first successful result
        for result in results {
            match result {
                Ok(Some(comment)) => return Ok(comment),
                Ok(None) => continue,
                Err(e) => {
                    // Log the error but continue searching other workspaces
                    eprintln!("Warning: Error searching workspace: {}", e);
                    continue;
                }
            }
        }
        
        // If we get here, the comment wasn't found
        Err(ClickUpError::NotFoundError(format!("Comment with ID '{}' not found", comment_id)))
    }
}
