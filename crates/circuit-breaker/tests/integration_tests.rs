//! Integration tests for circuit breaker

use clawmaster_circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitState};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_circuit_breaker_full_cycle() {
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        timeout: Duration::from_millis(100),
        success_threshold: 2,
    };
    let cb = CircuitBreaker::new(config);

    // Start in Closed state
    assert_eq!(cb.state(), CircuitState::Closed);

    // Cause failures to open circuit
    for _ in 0..3 {
        let _ = cb.call(|| async { Err::<i32, _>("error".to_string()) }).await;
    }
    assert_eq!(cb.state(), CircuitState::Open);

    // Wait for timeout
    sleep(Duration::from_millis(150)).await;

    // Transition to HalfOpen
    let _ = cb.call(|| async { Ok::<_, String>(1) }).await;
    assert_eq!(cb.state(), CircuitState::HalfOpen);

    // Success should close circuit
    let _ = cb.call(|| async { Ok::<_, String>(2) }).await;
    assert_eq!(cb.state(), CircuitState::Closed);
}

#[tokio::test]
async fn test_circuit_breaker_prevents_cascading_failures() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_secs(1),
        success_threshold: 1,
    };
    let cb = CircuitBreaker::new(config);
    let call_count = Arc::new(AtomicUsize::new(0));

    // Cause failures
    for _ in 0..2 {
        let count_clone = call_count.clone();
        let _ = cb.call(|| async move {
            count_clone.fetch_add(1, Ordering::SeqCst);
            Err::<i32, _>("error".to_string())
        }).await;
    }

    // Circuit should be open
    assert_eq!(cb.state(), CircuitState::Open);

    // Try 10 more calls - they should be rejected without executing
    for _ in 0..10 {
        let count_clone = call_count.clone();
        let result = cb.call(|| async move {
            count_clone.fetch_add(1, Ordering::SeqCst);
            Ok::<_, String>(1)
        }).await;
        assert!(result.is_err());
    }

    // Only 2 calls should have been executed
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn test_half_open_failure_reopens_circuit() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_millis(100),
        success_threshold: 2,
    };
    let cb = CircuitBreaker::new(config);

    // Open circuit
    for _ in 0..2 {
        let _ = cb.call(|| async { Err::<i32, _>("error".to_string()) }).await;
    }
    assert_eq!(cb.state(), CircuitState::Open);

    // Wait for timeout
    sleep(Duration::from_millis(150)).await;

    // Transition to HalfOpen
    let _ = cb.call(|| async { Ok::<_, String>(1) }).await;
    assert_eq!(cb.state(), CircuitState::HalfOpen);

    // Failure in HalfOpen should reopen circuit
    let _ = cb.call(|| async { Err::<i32, _>("error".to_string()) }).await;
    assert_eq!(cb.state(), CircuitState::Open);
}

#[tokio::test]
async fn test_circuit_breaker_reset() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        timeout: Duration::from_secs(10),
        success_threshold: 1,
    };
    let cb = CircuitBreaker::new(config);

    // Open circuit
    for _ in 0..2 {
        let _ = cb.call(|| async { Err::<i32, _>("error".to_string()) }).await;
    }
    assert_eq!(cb.state(), CircuitState::Open);

    // Manual reset
    cb.reset();
    assert_eq!(cb.state(), CircuitState::Closed);

    // Should work now
    let result = cb.call(|| async { Ok::<_, String>(42) }).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_concurrent_calls_in_closed_state() {
    let config = CircuitBreakerConfig::default();
    let cb = Arc::new(CircuitBreaker::new(config));
    let success_count = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];
    for _ in 0..10 {
        let cb_clone = cb.clone();
        let count_clone = success_count.clone();
        let handle = tokio::spawn(async move {
            cb_clone.call(|| async move {
                count_clone.fetch_add(1, Ordering::SeqCst);
                Ok::<_, String>(())
            }).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    assert_eq!(success_count.load(Ordering::SeqCst), 10);
    assert_eq!(cb.state(), CircuitState::Closed);
}
