//! # Constants Module
//!
//! This module contains all the magic numbers and constants used throughout the ClickUp CLI application.
//! Centralizing these values makes the code more maintainable and self-documenting.
//!
//! ## Usage
//!
//! ```rust
//! use clickup_cli::constants::rate_limit::DEFAULT_REQUESTS_PER_MINUTE;
//! use clickup_cli::constants::http::HTTP_TIMEOUT_SECONDS;
//!
//! // Use rate limiting constants
//! let max_requests = DEFAULT_REQUESTS_PER_MINUTE;
//!
//! // Use time constants
//! let timeout = HTTP_TIMEOUT_SECONDS;
//! ```

/// Rate Limiting Constants
pub mod rate_limit {
    /// Default requests per minute for ClickUp free accounts
    pub const DEFAULT_REQUESTS_PER_MINUTE: u32 = 100;
    
    /// Default buffer seconds for rate limiting
    pub const DEFAULT_BUFFER_SECONDS: u64 = 5;
    
    /// Default maximum retry attempts for rate-limited requests
    pub const DEFAULT_MAX_RETRIES: u32 = 3;
    
    /// Maximum consecutive waits to prevent infinite loops
    pub const MAX_CONSECUTIVE_WAITS: u32 = 10;
    
    /// Rate limit window duration in seconds (1 minute)
    pub const RATE_LIMIT_WINDOW_SECONDS: u64 = 60;
    
    /// Maximum wait time for rate limiting in seconds (2 minutes)
    pub const MAX_WAIT_TIME_SECONDS: u64 = 120;
    
    /// Progress update interval for long waits in seconds
    pub const PROGRESS_UPDATE_INTERVAL_SECONDS: u64 = 5;
    
    /// Minimum wait time to show progress updates in seconds
    pub const MIN_PROGRESS_WAIT_SECONDS: u64 = 10;
    
    /// Sleep duration for progress updates in seconds
    pub const PROGRESS_SLEEP_SECONDS: u64 = 1;
}

/// HTTP and Network Constants
pub mod http {
    /// HTTP timeout for API requests in seconds
    pub const HTTP_TIMEOUT_SECONDS: u64 = 30;
    
    /// Default sleep duration for test retries in milliseconds
    pub const TEST_RETRY_SLEEP_MS: u64 = 100;
}

/// Display and UI Constants
pub mod display {
    /// Maximum length for comment text in table display
    pub const MAX_COMMENT_DISPLAY_LENGTH: usize = 50;
    
    /// Ellipsis string for truncated text
    pub const ELLIPSIS: &str = "...";
}

/// Time Constants (in milliseconds)
pub mod time {
    /// One hour in milliseconds
    pub const ONE_HOUR_MS: u64 = 3_600_000;
    
    /// Two hours in milliseconds
    pub const TWO_HOURS_MS: u64 = 7_200_000;
    
    /// One day in milliseconds
    pub const ONE_DAY_MS: u64 = 86_400_000;
    
    /// One week in milliseconds
    pub const ONE_WEEK_MS: u64 = 604_800_000;
}

/// Unix Timestamp Constants
pub mod timestamps {
    /// Unix timestamp for 2022-01-01 (used in tests)
    pub const TEST_DATE_2022_01_01: u64 = 1_640_995_200;
    
    /// Unix timestamp for 2024-01-01 (used in tests)
    pub const TEST_DATE_2024_01_01: u64 = 1_704_067_200_000;
}

/// Test Constants
pub mod test {
    /// Test assignee ID used in comment tests
    pub const TEST_ASSIGNEE_ID_1: i64 = 123;
    
    /// Test assignee ID used in comment tests
    pub const TEST_ASSIGNEE_ID_2: i64 = 456;
    
    /// Number of test iterations for rate limiting tests
    pub const RATE_LIMIT_TEST_ITERATIONS: u32 = 10;
    
    /// Long token length for testing (1000 characters)
    pub const LONG_TOKEN_LENGTH: usize = 1000;
}

/// API Constants
pub mod api {
    /// Default ClickUp API base URL
    pub const DEFAULT_API_BASE_URL: &str = "https://api.clickup.com/api/v2";
    
    /// Content type for JSON requests
    pub const JSON_CONTENT_TYPE: &str = "application/json";
    
    /// Authorization header name
    pub const AUTHORIZATION_HEADER: &str = "Authorization";
    
    /// Retry-After header name
    pub const RETRY_AFTER_HEADER: &str = "Retry-After";
    
    /// Personal token prefix
    pub const PERSONAL_TOKEN_PREFIX: &str = "pk_";
    
    /// Bearer token prefix
    pub const BEARER_TOKEN_PREFIX: &str = "Bearer ";
}

/// Error and Status Constants
pub mod status {
    /// HTTP status code for bad request
    pub const HTTP_BAD_REQUEST: u16 = 400;
    
    /// HTTP status code for unauthorized
    pub const HTTP_UNAUTHORIZED: u16 = 401;
    
    /// HTTP status code for forbidden
    pub const HTTP_FORBIDDEN: u16 = 403;
    
    /// HTTP status code for not found
    pub const HTTP_NOT_FOUND: u16 = 404;
    
    /// HTTP status code for conflict
    pub const HTTP_CONFLICT: u16 = 409;
    
    /// HTTP status code for unprocessable entity
    pub const HTTP_UNPROCESSABLE_ENTITY: u16 = 422;
    
    /// HTTP status code for too many requests
    pub const HTTP_TOO_MANY_REQUESTS: u16 = 429;
    
    /// HTTP status code range for server errors
    pub const HTTP_SERVER_ERROR_RANGE_START: u16 = 500;
    pub const HTTP_SERVER_ERROR_RANGE_END: u16 = 599;
}

/// Configuration Constants
pub mod config {
    /// Default configuration file name
    pub const CONFIG_FILE_NAME: &str = "config.toml";
    
    /// Environment variable prefix for ClickUp CLI
    pub const ENV_PREFIX: &str = "CLICKUP";
    
    /// Environment variable separator
    pub const ENV_SEPARATOR: &str = "_";
    
    /// Environment variable for API token
    pub const ENV_API_TOKEN: &str = "CLICKUP_API_TOKEN";
    
    /// Environment variable for test API token
    pub const ENV_API_TOKEN_TEST: &str = "CLICKUP_API_TOKEN_TEST";
    
    /// Environment variable to skip .env file loading
    pub const ENV_SKIP_ENV_FILE: &str = "CLICKUP_SKIP_ENV_FILE";
}

/// Validation Constants
pub mod validation {
    /// Maximum length for API tokens
    pub const MAX_TOKEN_LENGTH: usize = 1000;
    
    /// Minimum length for task names
    pub const MIN_TASK_NAME_LENGTH: usize = 1;
    
    /// Maximum length for task names
    pub const MAX_TASK_NAME_LENGTH: usize = 500;
    
    /// Minimum length for comment text
    pub const MIN_COMMENT_TEXT_LENGTH: usize = 1;
    
    /// Maximum length for comment text
    pub const MAX_COMMENT_TEXT_LENGTH: usize = 10000;
}

/// UUID and ID Constants
pub mod ids {
    /// UUID regex pattern for validation
    pub const UUID_REGEX_PATTERN: &str = r"[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}";
    
    /// Invalid task ID pattern for testing
    pub const INVALID_TASK_ID_PATTERN: &str = "invalid-task-id-12345";
    
    /// Invalid list ID pattern for testing
    pub const INVALID_LIST_ID_PATTERN: &str = "invalid-list-id-12345";
    
    /// Invalid comment ID pattern for testing
    pub const INVALID_COMMENT_ID_PATTERN: &str = "invalid-comment-id-12345";
    
    /// Invalid token pattern for testing
    pub const INVALID_TOKEN_PATTERN: &str = "invalid-token-12345";
} 