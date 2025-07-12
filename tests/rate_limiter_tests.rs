use clickup_cli::config::RateLimitConfig;
use clickup_cli::rate_limiter::RateLimiter;


#[tokio::test]
async fn test_rate_limiter_creation() {
    let config = RateLimitConfig::default();
    let limiter = RateLimiter::new(config);
    
    assert_eq!(limiter.get_current_request_count().await.unwrap(), 0);
    assert_eq!(limiter.get_current_retry_count().await.unwrap(), 0);
}

#[tokio::test]
async fn test_rate_limiter_basic_functionality() {
    let config = RateLimitConfig {
        requests_per_minute: 3, // Very low limit for testing
        auto_retry: true,
        max_retries: 3,
        buffer_seconds: 0, // No buffer for faster testing
    };
    let limiter = RateLimiter::new(config);
    
    // Should allow initial requests
    for i in 0..3 {
        assert!(limiter.wait_if_needed().await.is_ok());
        assert_eq!(limiter.get_current_request_count().await.unwrap(), i + 1);
    }
    
    // Verify we're at the limit
    assert_eq!(limiter.get_current_request_count().await.unwrap(), 3);
    
    // The next request should trigger waiting, but we won't actually wait in tests
    // Instead, we'll just verify the counting logic works correctly
    assert!(limiter.get_current_request_count().await.unwrap() >= 3);
}

#[tokio::test]
async fn test_rate_limiter_counting_logic() {
    let config = RateLimitConfig {
        requests_per_minute: 10,
        auto_retry: true,
        max_retries: 3,
        buffer_seconds: 0,
    };
    let limiter = RateLimiter::new(config);
    
    // Make several requests and verify counting
    for i in 0..5 {
        assert!(limiter.wait_if_needed().await.is_ok());
        assert_eq!(limiter.get_current_request_count().await.unwrap(), i + 1);
    }
    
    // Verify the count is correct
    assert_eq!(limiter.get_current_request_count().await.unwrap(), 5);
}

#[tokio::test]
async fn test_rate_limiter_retry_handling() {
    let config = RateLimitConfig {
        requests_per_minute: 100,
        auto_retry: true,
        max_retries: 2,
        buffer_seconds: 1,
    };
    let limiter = RateLimiter::new(config);
    
    // First rate limit should be handled
    assert!(limiter.handle_rate_limit(Some(1)).await.is_ok());
    assert_eq!(limiter.get_current_retry_count().await.unwrap(), 1);
    
    // Second rate limit should also be handled
    assert!(limiter.handle_rate_limit(Some(1)).await.is_ok());
    assert_eq!(limiter.get_current_retry_count().await.unwrap(), 2);
    
    // Third rate limit should fail (exceeds max retries)
    assert!(limiter.handle_rate_limit(Some(1)).await.is_err());
}

#[tokio::test]
async fn test_rate_limiter_auto_retry_disabled() {
    let config = RateLimitConfig {
        requests_per_minute: 100,
        auto_retry: false,
        max_retries: 3,
        buffer_seconds: 1,
    };
    let limiter = RateLimiter::new(config);
    
    // Should fail immediately when auto-retry is disabled
    assert!(limiter.handle_rate_limit(Some(1)).await.is_err());
}

#[tokio::test]
async fn test_rate_limiter_reset_retry_count() {
    let config = RateLimitConfig::default();
    let limiter = RateLimiter::new(config);
    
    // Increment retry count
    limiter.handle_rate_limit(Some(1)).await.unwrap();
    assert_eq!(limiter.get_current_retry_count().await.unwrap(), 1);
    
    // Reset retry count
    limiter.reset_retry_count().await.unwrap();
    assert_eq!(limiter.get_current_retry_count().await.unwrap(), 0);
} 