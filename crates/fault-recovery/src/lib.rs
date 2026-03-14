//! Fault Detection and Auto-Recovery
//!
//! DO-178C Level A Compliant Fault Recovery System
//!
//! This module provides comprehensive fault detection and recovery including:
//! - Circuit Breaker pattern
//! - Exponential Backoff retry mechanism
//! - Graceful Degradation
//! - Fault Isolation
//! - Deadlock Detection
//!
//! Compliance: DO-178C §6.3.3 - Fault detection and recovery

pub mod circuit_breaker;
pub mod retry;
pub mod degradation;
pub mod isolation;
pub mod deadlock;

pub use circuit_breaker::*;
pub use retry::*;
pub use degradation::*;
pub use isolation::*;
pub use deadlock::*;

use thiserror::Error;

/// Fault recovery error
///
/// DO-178C §6.3.2: Clear error reporting
#[derive(Debug, Error)]
pub enum FaultError {
    #[error("Circuit breaker open: {0}")]
    CircuitBreakerOpen(String),
    
    #[error("Max retries exceeded: {0}")]
    MaxRetriesExceeded(String),
    
    #[error("Service degraded: {0}")]
    ServiceDegraded(String),
    
    #[error("Fault isolated: {0}")]
    FaultIsolated(String),
    
    #[error("Deadlock detected: {0}")]
    DeadlockDetected(String),
    
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

/// Fault recovery result
pub type FaultResult<T = ()> = Result<T, FaultError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fault_error_display() {
        let err = FaultError::CircuitBreakerOpen("service unavailable".to_string());
        assert_eq!(err.to_string(), "Circuit breaker open: service unavailable");

        let err = FaultError::DeadlockDetected("thread A and B".to_string());
        assert_eq!(err.to_string(), "Deadlock detected: thread A and B");
    }
}
