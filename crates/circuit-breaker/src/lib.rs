//! Circuit breaker pattern for ClawMaster to provide fault isolation.

use parking_lot::RwLock;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tracing::{debug, info, warn};

#[derive(Debug, Error)]
pub enum CircuitBreakerError {
    #[error("Circuit breaker is open")]
    CircuitOpen,
    
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed, requests pass through
    Closed,
    /// Circuit is open, requests are rejected
    Open,
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening the circuit
    pub failure_threshold: usize,
    /// Duration to wait before attempting to close the circuit
    pub timeout: Duration,
    /// Number of successful requests needed to close the circuit from half-open
    pub success_threshold: usize,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout: Duration::from_secs(60),
            success_threshold: 2,
        }
    }
}

/// Internal state of the circuit breaker
#[derive(Debug)]
struct CircuitBreakerState {
    state: CircuitState,
    failure_count: usize,
    success_count: usize,
    last_failure_time: Option<Instant>,
}

impl CircuitBreakerState {
    fn new() -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
        }
    }
}

/// Circuit breaker for fault isolation
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitBreakerState>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker with the given configuration
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CircuitBreakerState::new())),
        }
    }

    /// Get the current state of the circuit breaker
    pub fn state(&self) -> CircuitState {
        self.state.read().state
    }

    /// Execute an operation through the circuit breaker
    pub async fn call<F, Fut, T, E>(&self, operation: F) -> Result<T, CircuitBreakerError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        // Check if circuit should transition from open to half-open
        self.check_timeout();

        let current_state = self.state();
        
        match current_state {
            CircuitState::Open => {
                warn!("Circuit breaker is open, rejecting request");
                Err(CircuitBreakerError::CircuitOpen)
            }
            CircuitState::Closed | CircuitState::HalfOpen => {
                match operation().await {
                    Ok(result) => {
                        self.on_success();
                        Ok(result)
                    }
                    Err(err) => {
                        self.on_failure();
                        Err(CircuitBreakerError::OperationFailed(err.to_string()))
                    }
                }
            }
        }
    }

    /// Check if the circuit should transition from open to half-open
    fn check_timeout(&self) {
        let mut state = self.state.write();
        
        if state.state == CircuitState::Open {
            if let Some(last_failure) = state.last_failure_time {
                if last_failure.elapsed() >= self.config.timeout {
                    info!("Circuit breaker transitioning from Open to HalfOpen");
                    state.state = CircuitState::HalfOpen;
                    state.success_count = 0;
                }
            }
        }
    }

    /// Handle successful operation
    fn on_success(&self) {
        let mut state = self.state.write();
        
        match state.state {
            CircuitState::Closed => {
                state.failure_count = 0;
            }
            CircuitState::HalfOpen => {
                state.success_count += 1;
                
                if state.success_count >= self.config.success_threshold {
                    info!("Circuit breaker transitioning from HalfOpen to Closed");
                    state.state = CircuitState::Closed;
                    state.failure_count = 0;
                    state.success_count = 0;
                    state.last_failure_time = None;
                }
            }
            CircuitState::Open => {}
        }
    }

    /// Handle failed operation
    fn on_failure(&self) {
        let mut state = self.state.write();
        
        match state.state {
            CircuitState::Closed => {
                state.failure_count += 1;
                
                if state.failure_count >= self.config.failure_threshold {
                    warn!("Circuit breaker opening due to {} failures", state.failure_count);
                    state.state = CircuitState::Open;
                    state.last_failure_time = Some(Instant::now());
                }
            }
            CircuitState::HalfOpen => {
                warn!("Circuit breaker transitioning from HalfOpen to Open due to failure");
                state.state = CircuitState::Open;
                state.last_failure_time = Some(Instant::now());
                state.success_count = 0;
            }
            CircuitState::Open => {
                state.last_failure_time = Some(Instant::now());
            }
        }
    }

    /// Reset the circuit breaker to closed state
    pub fn reset(&self) {
        let mut state = self.state.write();
        debug!("Circuit breaker reset to Closed state");
        state.state = CircuitState::Closed;
        state.failure_count = 0;
        state.success_count = 0;
        state.last_failure_time = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_circuit_breaker_closed_state() {
        let config = CircuitBreakerConfig::default();
        let cb = CircuitBreaker::new(config);

        let result = cb.call(|| async { Ok::<_, String>(42) }).await;
        assert!(result.is_ok());
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_breaker_opens_after_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        for _ in 0..3 {
            let _ = cb.call(|| async { Err::<i32, _>("error".to_string()) }).await;
        }

        assert_eq!(cb.state(), CircuitState::Open);
        
        let result = cb.call(|| async { Ok::<_, String>(42) }).await;
        assert!(matches!(result, Err(CircuitBreakerError::CircuitOpen)));
    }

    #[tokio::test]
    async fn test_circuit_breaker_half_open_transition() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            timeout: Duration::from_millis(100),
            success_threshold: 2,
        };
        let cb = CircuitBreaker::new(config);

        // Cause failures to open circuit
        for _ in 0..2 {
            let _ = cb.call(|| async { Err::<i32, _>("error".to_string()) }).await;
        }
        assert_eq!(cb.state(), CircuitState::Open);

        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Next call should transition to half-open
        let _ = cb.call(|| async { Ok::<_, String>(1) }).await;
        assert_eq!(cb.state(), CircuitState::HalfOpen);
    }

    #[tokio::test]
    async fn test_circuit_breaker_closes_after_successes() {
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

        // Wait and succeed
        tokio::time::sleep(Duration::from_millis(150)).await;
        let _ = cb.call(|| async { Ok::<_, String>(1) }).await;
        let _ = cb.call(|| async { Ok::<_, String>(2) }).await;

        assert_eq!(cb.state(), CircuitState::Closed);
    }
}
