//! # Constants Module
//!
//! This module centralizes all configuration constants and magic values used throughout
//! the ClickUp CLI application. This eliminates magic constants scattered throughout
//! the codebase and provides a single source of truth for all configuration values.
//!
//! ## Usage
//!
//! ```rust
//! use clickup_cli::constants::api::DEFAULT_TIMEOUT;
//! use clickup_cli::constants::rate_limiting::DEFAULT_RPM;
//!
//! // Use constants instead of magic values
//! let timeout = DEFAULT_TIMEOUT;
//! let rate_limit = DEFAULT_RPM;
//! ```

use std::time::Duration;

/// API-related constants
pub mod api {
    use super::*;

    /// Default timeout for API requests (30 seconds)
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

    /// Maximum number of retry attempts for failed requests
    pub const MAX_RETRIES: u32 = 3;

    /// Base URL for ClickUp API v2
    pub const BASE_URL: &str = "https://api.clickup.com/api/v2";

    /// Default user agent for API requests
    pub const USER_AGENT: &str = "clickup-cli/1.0";

    /// Maximum request body size in bytes (10MB)
    pub const MAX_REQUEST_SIZE: usize = 10 * 1024 * 1024;

    /// Default page size for paginated requests
    pub const DEFAULT_PAGE_SIZE: u32 = 100;

    /// Maximum page size for paginated requests
    pub const MAX_PAGE_SIZE: u32 = 1000;
}

/// Rate limiting constants
pub mod rate_limiting {
    use super::*;

    /// Default requests per minute for ClickUp free accounts
    pub const DEFAULT_RPM: u32 = 100;

    /// Default buffer time in seconds for rate limiting
    pub const DEFAULT_BUFFER: Duration = Duration::from_secs(5);

    /// Maximum wait time in seconds for rate limit backoff
    pub const MAX_WAIT: Duration = Duration::from_secs(120);

    /// Maximum consecutive wait operations before giving up
    pub const MAX_CONSECUTIVE_WAITS: u32 = 10;

    /// Default auto-retry setting for rate-limited requests
    pub const DEFAULT_AUTO_RETRY: bool = true;

    /// Default maximum retry attempts for rate-limited requests
    pub const DEFAULT_MAX_RETRIES: u32 = 3;

    /// Minimum delay between retry attempts in seconds
    pub const MIN_RETRY_DELAY: Duration = Duration::from_secs(1);

    /// Maximum delay between retry attempts in seconds
    pub const MAX_RETRY_DELAY: Duration = Duration::from_secs(60);
}

/// Cache-related constants
pub mod cache {
    use super::*;

    /// Default cache TTL for workspace data (5 minutes)
    pub const WORKSPACE_CACHE_TTL: Duration = Duration::from_secs(300);

    /// Default cache TTL for space data (5 minutes)
    pub const SPACE_CACHE_TTL: Duration = Duration::from_secs(300);

    /// Default cache TTL for list data (5 minutes)
    pub const LIST_CACHE_TTL: Duration = Duration::from_secs(300);

    /// Default cache TTL for task data (2 minutes)
    pub const TASK_CACHE_TTL: Duration = Duration::from_secs(120);

    /// Default cache TTL for comment data (1 minute)
    pub const COMMENT_CACHE_TTL: Duration = Duration::from_secs(60);

    /// Maximum number of cached items
    pub const MAX_CACHE_SIZE: usize = 1000;
}

/// Connection pooling constants
pub mod connection {
    use super::*;

    /// Maximum number of idle connections per host
    pub const MAX_IDLE_CONNECTIONS: usize = 10;

    /// Idle connection timeout in seconds
    pub const IDLE_TIMEOUT: Duration = Duration::from_secs(90);

    /// Connection timeout in seconds
    pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

    /// Maximum number of connections in the pool
    pub const MAX_CONNECTIONS: usize = 100;
}

/// Batch operation constants
pub mod batch {
    /// Maximum number of items in a single batch request
    pub const MAX_BATCH_SIZE: usize = 10;

    /// Default batch size for task operations
    pub const DEFAULT_TASK_BATCH_SIZE: usize = 5;

    /// Default batch size for comment operations
    pub const DEFAULT_COMMENT_BATCH_SIZE: usize = 10;
}

/// Validation constants
pub mod validation {
    /// Maximum task name length
    pub const MAX_TASK_NAME_LENGTH: usize = 1000;

    /// Maximum task description length
    pub const MAX_TASK_DESCRIPTION_LENGTH: usize = 10000;

    /// Maximum comment content length
    pub const MAX_COMMENT_LENGTH: usize = 10000;

    /// Minimum task priority value
    pub const MIN_TASK_PRIORITY: i64 = 1;

    /// Maximum task priority value
    pub const MAX_TASK_PRIORITY: i64 = 4;

    /// Maximum workspace name length
    pub const MAX_WORKSPACE_NAME_LENGTH: usize = 100;

    /// Maximum space name length
    pub const MAX_SPACE_NAME_LENGTH: usize = 100;

    /// Maximum list name length
    pub const MAX_LIST_NAME_LENGTH: usize = 100;
}

/// Security constants
pub mod security {
    /// Minimum API token length
    pub const MIN_TOKEN_LENGTH: usize = 10;

    /// Maximum API token length
    pub const MAX_TOKEN_LENGTH: usize = 1000;

    /// Token prefix for personal tokens
    pub const PERSONAL_TOKEN_PREFIX: &str = "pk_";

    /// Bearer token prefix for OAuth tokens
    pub const BEARER_TOKEN_PREFIX: &str = "Bearer ";
}

/// Logging constants
pub mod logging {
    /// Default log level for production
    pub const DEFAULT_LOG_LEVEL: &str = "info";

    /// Debug log level
    pub const DEBUG_LOG_LEVEL: &str = "debug";

    /// Maximum log message length
    pub const MAX_LOG_MESSAGE_LENGTH: usize = 1000;
}

/// File system constants
pub mod filesystem {
    /// Default configuration directory name
    pub const CONFIG_DIR_NAME: &str = "clickup-cli";

    /// Default configuration file name
    pub const CONFIG_FILE_NAME: &str = "config.toml";

    /// Default environment file name
    pub const ENV_FILE_NAME: &str = ".env";

    /// Example environment file name
    pub const ENV_EXAMPLE_FILE_NAME: &str = ".env.example";

    /// Maximum configuration file size in bytes (1MB)
    pub const MAX_CONFIG_FILE_SIZE: usize = 1024 * 1024;
}

/// Error handling constants
pub mod error {
    /// Maximum error message length
    pub const MAX_ERROR_MESSAGE_LENGTH: usize = 1000;

    /// Maximum error context length
    pub const MAX_ERROR_CONTEXT_LENGTH: usize = 500;

    /// Default error retry delay in seconds
    pub const DEFAULT_ERROR_RETRY_DELAY: u64 = 1;

    /// Maximum error retry attempts
    pub const MAX_ERROR_RETRY_ATTEMPTS: u32 = 3;
} 