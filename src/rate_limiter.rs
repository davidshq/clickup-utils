//! # Rate Limiter
//!
//! This module provides rate limiting functionality for the ClickUp API client.
//! It tracks API requests and enforces rate limits to prevent hitting API quotas.
//!
//! ## Features
//!
//! - **Request Tracking**: Tracks requests per minute window
//! - **Automatic Throttling**: Delays requests when approaching limits
//! - **Retry Logic**: Automatically retries rate-limited requests
//! - **Configurable Limits**: Supports different rate limits for different account types
//!
//! ## Usage
//!
//! ```rust
//! use clickup_cli::rate_limiter::RateLimiter;
//! use clickup_cli::config::RateLimitConfig;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = RateLimitConfig::default();
//!     let mut limiter = RateLimiter::new(config);
//!
//!     // Before making an API request
//!     limiter.wait_if_needed().await.unwrap();
//!
//!     // After a rate limit error
//!     limiter.handle_rate_limit(Some(60)).await.unwrap();
//! }
//! ```

use crate::config::RateLimitConfig;
use crate::error::ClickUpError;
use log::{debug, info, warn};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Rate limiter for ClickUp API requests
///
/// This struct tracks API requests and enforces rate limits to prevent
/// hitting API quotas. It maintains a sliding window of requests and
/// automatically throttles when approaching limits.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Rate limiting configuration
    config: RateLimitConfig,

    /// Thread-safe request history
    ///
    /// Stores timestamps of recent requests for sliding window tracking
    request_history: Arc<Mutex<VecDeque<Instant>>>,

    /// Current retry count for the current request
    current_retry_count: Arc<Mutex<u32>>,
}

impl RateLimiter {
    /// Creates a new rate limiter with the specified configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Rate limiting configuration
    ///
    /// # Returns
    ///
    /// Returns a new `RateLimiter` instance.
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            request_history: Arc::new(Mutex::new(VecDeque::new())),
            current_retry_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Checks if we can make a request and waits if necessary
    ///
    /// This function checks the current request count within the last minute
    /// and waits if we're approaching the rate limit. It uses a sliding
    /// window approach to track requests accurately.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when it's safe to make a request, or a `ClickUpError`
    /// if we've exceeded the maximum retry attempts.
    ///
    /// # Errors
    ///
    /// This function can return:
    /// - `ClickUpError::RateLimitError` if we've exceeded max retries
    pub fn wait_if_needed(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<(), ClickUpError>> + Send + '_>> {
        Box::pin(async move {
            let mut consecutive_waits = 0;
            const MAX_CONSECUTIVE_WAITS: u32 = 10; // Prevent infinite loops

            loop {
                let now = Instant::now();
                let window_start = now - Duration::from_secs(60);

                // Get current request history
                let mut history = self.request_history.lock().await;

                // Remove requests older than 1 minute
                while let Some(timestamp) = history.front() {
                    if *timestamp < window_start {
                        history.pop_front();
                    } else {
                        break;
                    }
                }

                let current_requests = history.len() as u32;

                // If we're at the limit, wait until we can make another request
                if current_requests >= self.config.requests_per_minute {
                    consecutive_waits += 1;

                    if consecutive_waits > MAX_CONSECUTIVE_WAITS {
                        warn!("Rate limiter stuck in wait loop. Clearing history and continuing.");
                        history.clear();
                        consecutive_waits = 0;
                        continue;
                    }

                    let oldest_request = history.front().unwrap();
                    let wait_time = Duration::from_secs(60) - now.duration_since(*oldest_request);
                    let wait_time = wait_time + Duration::from_secs(self.config.buffer_seconds);

                    // Cap wait time to prevent extremely long waits
                    let max_wait = Duration::from_secs(120); // Use constant when available
                    let actual_wait = std::cmp::min(wait_time, max_wait);

                    info!(
                        "Rate limit reached ({} requests in last minute). Waiting {} seconds...",
                        current_requests,
                        actual_wait.as_secs()
                    );

                    // Show progress for longer waits
                    if actual_wait.as_secs() > 10 {
                        let start_time = Instant::now();
                        let mut last_progress = 0;

                        while start_time.elapsed() < actual_wait {
                            let elapsed = start_time.elapsed().as_secs();
                            let remaining = actual_wait.as_secs() - elapsed;

                            // Show progress every 5 seconds
                            if elapsed - last_progress >= 5 {
                                info!("Rate limit wait: {remaining} seconds remaining...");
                                last_progress = elapsed;
                            }

                            // Sleep in smaller chunks to allow for progress updates
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    } else {
                        tokio::time::sleep(actual_wait).await;
                    }

                    // Continue the loop to re-check after waiting
                    continue;
                }

                // Add current request to history
                history.push_back(now);
                debug!(
                    "Request allowed. {} requests in last minute.",
                    current_requests + 1
                );
                return Ok(());
            }
        })
    }

    /// Handles a rate limit error by waiting and potentially retrying
    ///
    /// This function is called when an API request returns a 429 rate limit error.
    /// It waits for the appropriate time and manages retry attempts.
    ///
    /// # Arguments
    ///
    /// * `retry_after_seconds` - Optional retry-after header value from the API
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the request should be retried, or a `ClickUpError`
    /// if we've exceeded the maximum retry attempts.
    ///
    /// # Errors
    ///
    /// This function can return:
    /// - `ClickUpError::RateLimitError` if we've exceeded max retries
    pub async fn handle_rate_limit(
        &self,
        retry_after_seconds: Option<u64>,
    ) -> Result<(), ClickUpError> {
        let mut retry_count = self.current_retry_count.lock().await;

        *retry_count += 1;

        if *retry_count > self.config.max_retries {
            warn!(
                "Exceeded maximum retry attempts ({}) for rate-limited request",
                self.config.max_retries
            );
            return Err(ClickUpError::RateLimitError);
        }

        if !self.config.auto_retry {
            warn!("Auto-retry disabled, not retrying rate-limited request");
            return Err(ClickUpError::RateLimitError);
        }

        // Calculate wait time
        let wait_time = if let Some(seconds) = retry_after_seconds {
            Duration::from_secs(seconds) + Duration::from_secs(self.config.buffer_seconds)
        } else {
            // Default to 60 seconds if no retry-after header
            Duration::from_secs(60) + Duration::from_secs(self.config.buffer_seconds)
        };

        info!(
            "Rate limited. Retry attempt {}/{}. Waiting {} seconds...",
            *retry_count,
            self.config.max_retries,
            wait_time.as_secs()
        );

        tokio::time::sleep(wait_time).await;

        // Clear old requests from history to give us a fresh start
        let mut history = self.request_history.lock().await;

        let now = Instant::now();
        let window_start = now - Duration::from_secs(60);

        while let Some(timestamp) = history.front() {
            if *timestamp < window_start {
                history.pop_front();
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Resets the retry count for a new request
    ///
    /// This should be called when starting a new request to reset the retry counter.
    pub async fn reset_retry_count(&self) -> Result<(), ClickUpError> {
        let mut retry_count = self.current_retry_count.lock().await;

        *retry_count = 0;
        Ok(())
    }

    /// Gets the current number of requests in the last minute
    ///
    /// This is useful for debugging and monitoring rate limit usage.
    ///
    /// # Returns
    ///
    /// Returns the number of requests made in the last minute.
    #[allow(dead_code)]
    pub async fn get_current_request_count(&self) -> Result<u32, ClickUpError> {
        let now = Instant::now();
        let window_start = now - Duration::from_secs(60);

        let mut history = self.request_history.lock().await;

        // Remove requests older than 1 minute
        while let Some(timestamp) = history.front() {
            if *timestamp < window_start {
                history.pop_front();
            } else {
                break;
            }
        }

        Ok(history.len() as u32)
    }

    /// Gets the current retry count
    ///
    /// This is useful for debugging retry behavior.
    ///
    /// # Returns
    ///
    /// Returns the current retry count for the ongoing request.
    #[allow(dead_code)]
    pub async fn get_current_retry_count(&self) -> Result<u32, ClickUpError> {
        let retry_count = self.current_retry_count.lock().await;

        Ok(*retry_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_initial_state() {
        let config = RateLimitConfig::default();
        let limiter = RateLimiter::new(config);

        assert_eq!(limiter.get_current_request_count().await.unwrap(), 0);
        assert_eq!(limiter.get_current_retry_count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_rate_limiter_allows_requests() {
        let config = RateLimitConfig::default();
        let limiter = RateLimiter::new(config);

        // Should allow requests initially
        assert!(limiter.wait_if_needed().await.is_ok());
        assert_eq!(limiter.get_current_request_count().await.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_rate_limiter_handles_rate_limit() {
        let config = RateLimitConfig {
            max_retries: 2,
            ..Default::default()
        };
        let limiter = RateLimiter::new(config);

        // First rate limit should be handled
        assert!(limiter.handle_rate_limit(Some(1)).await.is_ok());
        assert_eq!(limiter.get_current_retry_count().await.unwrap(), 1);

        // Reset and try again
        limiter.reset_retry_count().await.unwrap();
        assert_eq!(limiter.get_current_retry_count().await.unwrap(), 0);
    }
}
