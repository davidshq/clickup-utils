use crate::config::Config;
use crate::error::ClickUpError;
use crate::models::*;
use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub struct ClickUpApi {
    client: Client,
    config: Config,
}

impl ClickUpApi {
    pub fn new(config: Config) -> Result<Self, ClickUpError> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .default_headers(headers)
            .build()
            .map_err(|e| ClickUpError::NetworkError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    fn get_auth_header(&self) -> Result<HeaderValue, ClickUpError> {
        let token = self.config.get_api_token()?;
        let auth_value = format!("Bearer {}", token);
        HeaderValue::from_str(&auth_value).map_err(|e| {
            ClickUpError::AuthError(format!("Invalid auth header: {}", e))
        })
    }

    async fn make_request<T>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<Value>,
    ) -> Result<T, ClickUpError>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let url = format!("{}{}", self.config.api_base_url, endpoint);
        let mut request = self.client.request(method, &url);

        // Add authentication header
        let auth_header = self.get_auth_header()?;
        request = request.header(AUTHORIZATION, auth_header);

        // Add body if provided
        if let Some(body) = body {
            request = request.json(&body);
        }

        debug!("Making request to: {}", url);

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

        serde_json::from_str(&response_text).map_err(|e| {
            error!("Failed to parse response: {}", e);
            ClickUpError::DeserializationError(format!("Failed to parse response: {}", e))
        })
    }

    // User endpoints
    pub async fn get_user(&self) -> Result<User, ClickUpError> {
        self.make_request(reqwest::Method::GET, "/user", None).await
    }

    // Workspace endpoints
    pub async fn get_workspaces(&self) -> Result<WorkspacesResponse, ClickUpError> {
        self.make_request(reqwest::Method::GET, "/team", None).await
    }

    // Space endpoints
    pub async fn get_spaces(&self, workspace_id: &str) -> Result<SpacesResponse, ClickUpError> {
        let endpoint = format!("/team/{}/space", workspace_id);
        self.make_request(reqwest::Method::GET, &endpoint, None).await
    }

    // List endpoints
    pub async fn get_lists(&self, space_id: &str) -> Result<ListsResponse, ClickUpError> {
        let endpoint = format!("/space/{}/list", space_id);
        self.make_request(reqwest::Method::GET, &endpoint, None).await
    }

    // Task endpoints
    pub async fn get_tasks(&self, list_id: &str) -> Result<TasksResponse, ClickUpError> {
        let endpoint = format!("/list/{}/task", list_id);
        self.make_request(reqwest::Method::GET, &endpoint, None).await
    }

    pub async fn get_task(&self, task_id: &str) -> Result<Task, ClickUpError> {
        let endpoint = format!("/task/{}", task_id);
        self.make_request(reqwest::Method::GET, &endpoint, None).await
    }

    pub async fn create_task(&self, list_id: &str, task_data: CreateTaskRequest) -> Result<Task, ClickUpError> {
        let endpoint = format!("/list/{}/task", list_id);
        let body = serde_json::to_value(task_data).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize task data: {}", e))
        })?;
        self.make_request(reqwest::Method::POST, &endpoint, Some(body)).await
    }

    pub async fn update_task(&self, task_id: &str, task_data: UpdateTaskRequest) -> Result<Task, ClickUpError> {
        let endpoint = format!("/task/{}", task_id);
        let body = serde_json::to_value(task_data).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize task data: {}", e))
        })?;
        self.make_request(reqwest::Method::PUT, &endpoint, Some(body)).await
    }

    pub async fn delete_task(&self, task_id: &str) -> Result<(), ClickUpError> {
        let endpoint = format!("/task/{}", task_id);
        let _: Value = self.make_request(reqwest::Method::DELETE, &endpoint, None).await?;
        Ok(())
    }

    // Comment endpoints
    pub async fn get_comments(&self, task_id: &str) -> Result<CommentsResponse, ClickUpError> {
        let endpoint = format!("/task/{}/comment", task_id);
        self.make_request(reqwest::Method::GET, &endpoint, None).await
    }

    pub async fn create_comment(&self, task_id: &str, comment_data: CreateCommentRequest) -> Result<Comment, ClickUpError> {
        let endpoint = format!("/task/{}/comment", task_id);
        let body = serde_json::to_value(comment_data).map_err(|e| {
            ClickUpError::SerializationError(format!("Failed to serialize comment data: {}", e))
        })?;
        self.make_request(reqwest::Method::POST, &endpoint, Some(body)).await
    }
} 