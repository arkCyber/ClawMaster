//! Integration tests for rate limiter

use clawmaster_rate_limiter::{RateLimiter, RateLimiterConfig};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_concurrent_requests() {
    let config = RateLimiterConfig {
        max_requests: 10,
        window: Duration::from_secs(1),
        sliding_window: true,
    };
    let limiter = RateLimiter::new(config);

    let mut handles = vec![];
    for i in 0..15 {
        let limiter = limiter.clone();
        let handle = tokio::spawn(async move {
            limiter.check_rate_limit("concurrent_key")
        });
        handles.push(handle);
    }

    let mut success_count = 0;
    let mut failure_count = 0;
    
    for handle in handles {
        match handle.await.unwrap() {
            Ok(_) => success_count += 1,
            Err(_) => failure_count += 1,
        }
    }

    assert_eq!(success_count, 10);
    assert_eq!(failure_count, 5);
}

#[tokio::test]
async fn test_multiple_keys_isolation() {
    let config = RateLimiterConfig {
        max_requests: 3,
        window: Duration::from_secs(1),
        sliding_window: true,
    };
    let limiter = RateLimiter::new(config);

    // Fill up key1
    for _ in 0..3 {
        assert!(limiter.check_rate_limit("key1").is_ok());
    }
    assert!(limiter.check_rate_limit("key1").is_err());

    // key2 should still work
    for _ in 0..3 {
        assert!(limiter.check_rate_limit("key2").is_ok());
    }
    assert!(limiter.check_rate_limit("key2").is_err());

    // key3 should still work
    assert!(limiter.check_rate_limit("key3").is_ok());
}

#[tokio::test]
async fn test_usage_tracking() {
    let config = RateLimiterConfig {
        max_requests: 5,
        window: Duration::from_secs(1),
        sliding_window: true,
    };
    let limiter = RateLimiter::new(config);

    assert_eq!(limiter.get_usage("test_key"), None);

    limiter.check_rate_limit("test_key").unwrap();
    assert_eq!(limiter.get_usage("test_key"), Some(1));

    limiter.check_rate_limit("test_key").unwrap();
    assert_eq!(limiter.get_usage("test_key"), Some(2));

    limiter.reset("test_key");
    assert_eq!(limiter.get_usage("test_key"), None);
}

#[tokio::test]
async fn test_sliding_window_cleanup() {
    let config = RateLimiterConfig {
        max_requests: 3,
        window: Duration::from_millis(200),
        sliding_window: true,
    };
    let limiter = RateLimiter::new(config);

    // Fill up the window
    for _ in 0..3 {
        assert!(limiter.check_rate_limit("cleanup_key").is_ok());
    }
    assert!(limiter.check_rate_limit("cleanup_key").is_err());

    // Wait for half the window
    sleep(Duration::from_millis(100)).await;
    
    // Still blocked
    assert!(limiter.check_rate_limit("cleanup_key").is_err());

    // Wait for full window
    sleep(Duration::from_millis(150)).await;
    
    // Should work now
    assert!(limiter.check_rate_limit("cleanup_key").is_ok());
}
