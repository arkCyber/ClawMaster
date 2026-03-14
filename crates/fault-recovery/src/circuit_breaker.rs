//! Circuit Breaker Pattern
//!
//! DO-178C Level A Compliant Circuit Breaker

use crate::{FaultError, FaultResult};
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed, requests pass through
    Closed,
    
    /// Circuit is open, requests are blocked
    Open,
    
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Failure threshold to open circuit
    pub failure_threshold: usize,
    
    /// Success threshold to close circuit from half-open
    pub success_threshold: usize,
    
    /// Timeout before attempting to close circuit
    pub timeout: Duration,
    
    /// Time window for counting failures
    pub window: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            window: Duration::from_secs(60),
        }
    }
}

/// Circuit breaker
///
/// DO-178C §6.3.3: Circuit breaker pattern
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitBreakerState>>,
}

#[derive(Debug)]
struct CircuitBreakerState {
    current_state: CircuitState,
    failure_count: usize,
    success_count: usize,
    last_failure_time: Option<Instant>,
    last_state_change: Instant,
    failure_times: Vec<Instant>,
}

impl CircuitBreaker {
    /// Create new circuit breaker
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CircuitBreakerState {
                current_state: CircuitState::Closed,
                failure_count: 0,
                success_count: 0,
                last_failure_time: None,
                last_state_change: Instant::now(),
                failure_times: Vec::new(),
            })),
        }
    }

    /// Execute operation with circuit breaker protection
    ///
    /// DO-178C §6.3.3: Protected execution
    pub async fn call<F, T, E>(&self, operation: F) -> FaultResult<T>
    where
        F: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        // Check if circuit allows request
        self.check_state()?;

        // Execute operation
        match operation.await {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                self.on_failure();
                Err(FaultError::OperationFailed(e.to_string()))
            }
        }
    }

    /// Check current circuit state
    fn check_state(&self) -> FaultResult<()> {
        let mut state = self.state.write();

        match state.current_state {
            CircuitState::Closed => Ok(()),
            CircuitState::Open => {
                // Check if timeout has elapsed
                if state.last_state_change.elapsed() >= self.config.timeout {
                    state.current_state = CircuitState::HalfOpen;
                    state.success_count = 0;
                    state.last_state_change = Instant::now();
                    tracing::info!("Circuit breaker: Open -> HalfOpen");
                    Ok(())
                } else {
                    Err(FaultError::CircuitBreakerOpen(
                        "Circuit is open, requests blocked".to_string(),
                    ))
                }
            }
            CircuitState::HalfOpen => Ok(()),
        }
    }

    /// Handle successful operation
    fn on_success(&self) {
        let mut state = self.state.write();

        match state.current_state {
            CircuitState::Closed => {
                // Reset failure count on success
                state.failure_count = 0;
                state.failure_times.clear();
            }
            CircuitState::HalfOpen => {
                state.success_count += 1;
                if state.success_count >= self.config.success_threshold {
                    state.current_state = CircuitState::Closed;
                    state.failure_count = 0;
                    state.success_count = 0;
                    state.failure_times.clear();
                    state.last_state_change = Instant::now();
                    tracing::info!("Circuit breaker: HalfOpen -> Closed");
                }
            }
            CircuitState::Open => {}
        }
    }

    /// Handle failed operation
    fn on_failure(&self) {
        let mut state = self.state.write();
        let now = Instant::now();

        state.last_failure_time = Some(now);
        state.failure_times.push(now);

        // Remove old failures outside the window
        let cutoff = now - self.config.window;
        state.failure_times.retain(|&t| t > cutoff);

        state.failure_count = state.failure_times.len();

        match state.current_state {
            CircuitState::Closed => {
                if state.failure_count >= self.config.failure_threshold {
                    state.current_state = CircuitState::Open;
                    state.last_state_change = Instant::now();
                    tracing::warn!("Circuit breaker: Closed -> Open");
                }
            }
            CircuitState::HalfOpen => {
                state.current_state = CircuitState::Open;
                state.success_count = 0;
                state.last_state_change = Instant::now();
                tracing::warn!("Circuit breaker: HalfOpen -> Open");
            }
            CircuitState::Open => {}
        }
    }

    /// Get current state
    pub fn get_state(&self) -> CircuitState {
        self.state.read().current_state
    }

    /// Get failure count
    pub fn get_failure_count(&self) -> usize {
        self.state.read().failure_count
    }

    /// Reset circuit breaker
    pub fn reset(&self) {
        let mut state = self.state.write();
        state.current_state = CircuitState::Closed;
        state.failure_count = 0;
        state.success_count = 0;
        state.failure_times.clear();
        state.last_state_change = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_circuit_breaker_closed_state() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_secs(1),
            window: Duration::from_secs(10),
        };
        let cb = CircuitBreaker::new(config);

        assert_eq!(cb.get_state(), CircuitState::Closed);

        // Successful operation
        let result = cb.call(async { Ok::<_, String>("success") }).await;
        assert!(result.is_ok());
        assert_eq!(cb.get_state(), CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_breaker_opens_on_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_secs(1),
            window: Duration::from_secs(10),
        };
        let cb = CircuitBreaker::new(config);

        // Trigger failures
        for _ in 0..3 {
            let _ = cb.call(async { Err::<(), _>("error") }).await;
        }

        assert_eq!(cb.get_state(), CircuitState::Open);
    }

    #[tokio::test]
    async fn test_circuit_breaker_blocks_when_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 2,
            timeout: Duration::from_secs(10),
            window: Duration::from_secs(10),
        };
        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            let _ = cb.call(async { Err::<(), _>("error") }).await;
        }

        assert_eq!(cb.get_state(), CircuitState::Open);

        // Next call should be blocked
        let result = cb.call(async { Ok::<_, String>("success") }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_circuit_breaker_half_open_transition() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            window: Duration::from_secs(10),
        };
        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            let _ = cb.call(async { Err::<(), _>("error") }).await;
        }

        assert_eq!(cb.get_state(), CircuitState::Open);

        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Next call should transition to half-open
        let result = cb.call(async { Ok::<_, String>("success") }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_circuit_breaker_closes_from_half_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            window: Duration::from_secs(10),
        };
        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            let _ = cb.call(async { Err::<(), _>("error") }).await;
        }

        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Successful calls to close circuit
        for _ in 0..2 {
            let _ = cb.call(async { Ok::<_, String>("success") }).await;
        }

        assert_eq!(cb.get_state(), CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_breaker_reset() {
        let config = CircuitBreakerConfig::default();
        let cb = CircuitBreaker::new(config);

        // Trigger failures
        for _ in 0..5 {
            let _ = cb.call(async { Err::<(), _>("error") }).await;
        }

        assert_eq!(cb.get_state(), CircuitState::Open);

        // Reset
        cb.reset();

        assert_eq!(cb.get_state(), CircuitState::Closed);
        assert_eq!(cb.get_failure_count(), 0);
    }
}
