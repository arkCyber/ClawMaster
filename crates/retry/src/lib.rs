//! Retry mechanism for ClawMaster to improve reliability.

use std::future::Future;
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, warn};

#[derive(Debug, Error)]
pub enum RetryError<E> {
    #[error("All retry attempts failed")]
    AllAttemptsFailed,
    
    #[error("Operation failed: {0}")]
    OperationFailed(E),
}

/// Backoff strategy for retries
#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    /// Fixed delay between retries
    Fixed(Duration),
    /// Exponential backoff with base delay
    Exponential { base: Duration, max: Duration },
    /// Linear backoff with increment
    Linear { base: Duration, increment: Duration, max: Duration },
}

impl BackoffStrategy {
    /// Calculate delay for the given attempt number (0-indexed)
    pub fn delay(&self, attempt: usize) -> Duration {
        match self {
            BackoffStrategy::Fixed(duration) => *duration,
            BackoffStrategy::Exponential { base, max } => {
                let delay = base.as_millis() * 2_u128.pow(attempt as u32);
                Duration::from_millis(delay.min(max.as_millis()) as u64)
            }
            BackoffStrategy::Linear { base, increment, max } => {
                let delay = base.as_millis() + increment.as_millis() * attempt as u128;
                Duration::from_millis(delay.min(max.as_millis()) as u64)
            }
        }
    }
}

/// Retry policy configuration
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    /// Backoff strategy
    pub backoff: BackoffStrategy,
    /// Whether to add jitter to backoff
    pub jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            backoff: BackoffStrategy::Exponential {
                base: Duration::from_millis(100),
                max: Duration::from_secs(10),
            },
            jitter: true,
        }
    }
}

impl RetryPolicy {
    /// Create a new retry policy with exponential backoff
    pub fn exponential(max_attempts: usize, base: Duration, max: Duration) -> Self {
        Self {
            max_attempts,
            backoff: BackoffStrategy::Exponential { base, max },
            jitter: true,
        }
    }

    /// Create a new retry policy with fixed delay
    pub fn fixed(max_attempts: usize, delay: Duration) -> Self {
        Self {
            max_attempts,
            backoff: BackoffStrategy::Fixed(delay),
            jitter: false,
        }
    }

    /// Retry an async operation with this policy
    pub async fn retry<F, Fut, T, E>(&self, mut operation: F) -> Result<T, RetryError<E>>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut attempt = 0;
        
        loop {
            match operation().await {
                Ok(result) => {
                    if attempt > 0 {
                        debug!("Operation succeeded after {} retries", attempt);
                    }
                    return Ok(result);
                }
                Err(err) => {
                    attempt += 1;
                    
                    if attempt >= self.max_attempts {
                        warn!("All {} retry attempts failed", self.max_attempts);
                        return Err(RetryError::OperationFailed(err));
                    }
                    
                    let mut delay = self.backoff.delay(attempt - 1);
                    
                    if self.jitter {
                        let jitter = rand::random::<f64>() * 0.3; // ±30% jitter
                        delay = Duration::from_millis(
                            (delay.as_millis() as f64 * (1.0 + jitter - 0.15)) as u64
                        );
                    }
                    
                    warn!("Operation failed (attempt {}/{}): {}. Retrying in {:?}", 
                          attempt, self.max_attempts, err, delay);
                    
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}

/// Retry an operation with a custom policy
pub async fn retry_with_policy<F, Fut, T, E>(
    policy: &RetryPolicy,
    operation: F,
) -> Result<T, RetryError<E>>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    policy.retry(operation).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_succeeds_first_attempt() {
        let policy = RetryPolicy::fixed(3, Duration::from_millis(10));
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let result = policy.retry(|| async {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok::<_, String>(42)
        }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_succeeds_after_failures() {
        let policy = RetryPolicy::fixed(3, Duration::from_millis(10));
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let result = policy.retry(|| async {
            let count = counter_clone.fetch_add(1, Ordering::SeqCst);
            if count < 2 {
                Err("Temporary failure".to_string())
            } else {
                Ok(42)
            }
        }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_fails_after_max_attempts() {
        let policy = RetryPolicy::fixed(3, Duration::from_millis(10));
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let result = policy.retry(|| async {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            Err::<i32, _>("Permanent failure".to_string())
        }).await;

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_exponential_backoff() {
        let backoff = BackoffStrategy::Exponential {
            base: Duration::from_millis(100),
            max: Duration::from_secs(10),
        };

        assert_eq!(backoff.delay(0), Duration::from_millis(100));
        assert_eq!(backoff.delay(1), Duration::from_millis(200));
        assert_eq!(backoff.delay(2), Duration::from_millis(400));
        assert_eq!(backoff.delay(3), Duration::from_millis(800));
    }
}
