//! Retry with Exponential Backoff
//!
//! DO-178C Level A Compliant Retry Mechanism

use {
    crate::{FaultError, FaultResult},
    std::time::Duration,
};

/// Retry policy configuration
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_attempts: usize,

    /// Initial backoff duration
    pub initial_backoff: Duration,

    /// Maximum backoff duration
    pub max_backoff: Duration,

    /// Backoff multiplier
    pub multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

/// Retry executor
///
/// DO-178C §6.3.3: Retry with exponential backoff
pub struct RetryExecutor {
    policy: RetryPolicy,
}

impl RetryExecutor {
    /// Create new retry executor
    pub fn new(policy: RetryPolicy) -> Self {
        Self { policy }
    }

    /// Execute operation with retry
    ///
    /// DO-178C §6.3.3: Retry execution
    pub async fn execute<F, Fut, T, E>(&self, mut operation: F) -> FaultResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut attempt = 0;
        let mut backoff = self.policy.initial_backoff;

        loop {
            attempt += 1;

            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        tracing::info!("Operation succeeded after {} attempts", attempt);
                    }
                    return Ok(result);
                },
                Err(e) => {
                    if attempt >= self.policy.max_attempts {
                        tracing::error!("Operation failed after {} attempts: {}", attempt, e);
                        return Err(FaultError::MaxRetriesExceeded(format!(
                            "Failed after {} attempts: {}",
                            attempt, e
                        )));
                    }

                    tracing::warn!(
                        "Attempt {}/{} failed: {}. Retrying in {:?}",
                        attempt,
                        self.policy.max_attempts,
                        e,
                        backoff
                    );

                    // Wait before retry
                    tokio::time::sleep(backoff).await;

                    // Calculate next backoff
                    backoff = self.calculate_next_backoff(backoff);
                },
            }
        }
    }

    /// Calculate next backoff duration
    fn calculate_next_backoff(&self, current: Duration) -> Duration {
        let next = current.mul_f64(self.policy.multiplier);
        next.min(self.policy.max_backoff)
    }
}

impl Default for RetryExecutor {
    fn default() -> Self {
        Self::new(RetryPolicy::default())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
    };

    #[tokio::test]
    async fn test_retry_success_first_attempt() {
        let policy = RetryPolicy::default();
        let executor = RetryExecutor::new(policy);

        let result = executor
            .execute(|| async { Ok::<_, String>("success") })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_retry_success_after_failures() {
        let policy = RetryPolicy {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_secs(1),
            multiplier: 2.0,
        };
        let executor = RetryExecutor::new(policy);

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        let result = executor
            .execute(move || {
                let count = counter_clone.fetch_add(1, Ordering::SeqCst);
                async move {
                    if count < 2 {
                        Err("temporary error")
                    } else {
                        Ok("success")
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_max_attempts_exceeded() {
        let policy = RetryPolicy {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_secs(1),
            multiplier: 2.0,
        };
        let executor = RetryExecutor::new(policy);

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        let result = executor
            .execute(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                async move { Err::<(), _>("persistent error") }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_calculate_next_backoff() {
        let policy = RetryPolicy {
            max_attempts: 5,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(10),
            multiplier: 2.0,
        };
        let executor = RetryExecutor::new(policy);

        let backoff1 = Duration::from_millis(100);
        let backoff2 = executor.calculate_next_backoff(backoff1);
        assert_eq!(backoff2, Duration::from_millis(200));

        let backoff3 = executor.calculate_next_backoff(backoff2);
        assert_eq!(backoff3, Duration::from_millis(400));

        // Test max backoff
        let large_backoff = Duration::from_secs(20);
        let capped = executor.calculate_next_backoff(large_backoff);
        assert_eq!(capped, Duration::from_secs(10));
    }
}
