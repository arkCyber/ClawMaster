//! Integration tests for retry mechanism

use clawmaster_retry::{BackoffStrategy, RetryPolicy};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_exponential_backoff_timing() {
    let policy = RetryPolicy {
        max_attempts: 4,
        backoff: BackoffStrategy::Exponential {
            base: Duration::from_millis(50),
            max: Duration::from_secs(1),
        },
        jitter: false,
    };

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();
    let start = Instant::now();

    let _ = policy.retry(|| async {
        let count = counter_clone.fetch_add(1, Ordering::SeqCst);
        if count < 3 {
            Err("fail".to_string())
        } else {
            Ok(())
        }
    }).await;

    let elapsed = start.elapsed();
    
    // Expected delays: 50ms + 100ms + 200ms = 350ms minimum
    assert!(elapsed >= Duration::from_millis(300));
    assert_eq!(counter.load(Ordering::SeqCst), 4);
}

#[tokio::test]
async fn test_linear_backoff() {
    let policy = RetryPolicy {
        max_attempts: 4,
        backoff: BackoffStrategy::Linear {
            base: Duration::from_millis(50),
            increment: Duration::from_millis(25),
            max: Duration::from_secs(1),
        },
        jitter: false,
    };

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let _ = policy.retry(|| async {
        let count = counter_clone.fetch_add(1, Ordering::SeqCst);
        if count < 3 {
            Err("fail".to_string())
        } else {
            Ok(())
        }
    }).await;

    assert_eq!(counter.load(Ordering::SeqCst), 4);
}

#[tokio::test]
async fn test_retry_with_different_errors() {
    let policy = RetryPolicy::fixed(3, Duration::from_millis(10));
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let result = policy.retry(|| async {
        let count = counter_clone.fetch_add(1, Ordering::SeqCst);
        match count {
            0 => Err("network error".to_string()),
            1 => Err("timeout error".to_string()),
            _ => Ok(42),
        }
    }).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_max_delay_cap() {
    let policy = RetryPolicy {
        max_attempts: 10,
        backoff: BackoffStrategy::Exponential {
            base: Duration::from_millis(100),
            max: Duration::from_millis(500),
        },
        jitter: false,
    };

    // Test that delay is capped at max
    for attempt in 0..10 {
        let delay = policy.backoff.delay(attempt);
        assert!(delay <= Duration::from_millis(500));
    }
}

#[tokio::test]
async fn test_immediate_success_no_retry() {
    let policy = RetryPolicy::fixed(5, Duration::from_millis(100));
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();
    let start = Instant::now();

    let result = policy.retry(|| async {
        counter_clone.fetch_add(1, Ordering::SeqCst);
        Ok::<_, String>(42)
    }).await;

    let elapsed = start.elapsed();

    assert!(result.is_ok());
    assert_eq!(counter.load(Ordering::SeqCst), 1);
    // Should complete almost immediately
    assert!(elapsed < Duration::from_millis(50));
}
