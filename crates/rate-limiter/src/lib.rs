//! Rate limiting for ClawMaster to prevent API abuse.

use dashmap::DashMap;
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tracing::{debug, warn};

#[derive(Debug, Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded for key: {0}")]
    LimitExceeded(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

pub type Result<T> = std::result::Result<T, RateLimitError>;

/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Maximum number of requests allowed
    pub max_requests: usize,
    /// Time window for rate limiting
    pub window: Duration,
    /// Whether to use sliding window (true) or fixed window (false)
    pub sliding_window: bool,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window: Duration::from_secs(60),
            sliding_window: true,
        }
    }
}

/// Request record for tracking
#[derive(Debug, Clone)]
struct RequestRecord {
    timestamps: Vec<Instant>,
    window_start: Instant,
}

impl RequestRecord {
    fn new() -> Self {
        Self {
            timestamps: Vec::new(),
            window_start: Instant::now(),
        }
    }
}

/// Rate limiter using token bucket or sliding window algorithm
pub struct RateLimiter {
    config: RateLimiterConfig,
    records: Arc<DashMap<String, Arc<RwLock<RequestRecord>>>>,
}

impl RateLimiter {
    /// Create a new rate limiter with the given configuration
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            config,
            records: Arc::new(DashMap::new()),
        }
    }

    /// Check if a request is allowed for the given key
    pub fn check_rate_limit(&self, key: &str) -> Result<()> {
        let now = Instant::now();
        
        let record = self.records
            .entry(key.to_string())
            .or_insert_with(|| Arc::new(RwLock::new(RequestRecord::new())))
            .clone();
        
        let mut record = record.write();
        
        if self.config.sliding_window {
            self.check_sliding_window(&mut record, now, key)
        } else {
            self.check_fixed_window(&mut record, now, key)
        }
    }

    /// Check rate limit using sliding window algorithm
    fn check_sliding_window(&self, record: &mut RequestRecord, now: Instant, key: &str) -> Result<()> {
        // Remove timestamps outside the window
        let cutoff = now - self.config.window;
        record.timestamps.retain(|&ts| ts > cutoff);
        
        if record.timestamps.len() >= self.config.max_requests {
            warn!("Rate limit exceeded for key: {}", key);
            return Err(RateLimitError::LimitExceeded(key.to_string()));
        }
        
        record.timestamps.push(now);
        debug!("Request allowed for key: {} ({}/{})", key, record.timestamps.len(), self.config.max_requests);
        Ok(())
    }

    /// Check rate limit using fixed window algorithm
    fn check_fixed_window(&self, record: &mut RequestRecord, now: Instant, key: &str) -> Result<()> {
        // Reset window if expired
        if now.duration_since(record.window_start) >= self.config.window {
            record.timestamps.clear();
            record.window_start = now;
        }
        
        if record.timestamps.len() >= self.config.max_requests {
            warn!("Rate limit exceeded for key: {}", key);
            return Err(RateLimitError::LimitExceeded(key.to_string()));
        }
        
        record.timestamps.push(now);
        debug!("Request allowed for key: {} ({}/{})", key, record.timestamps.len(), self.config.max_requests);
        Ok(())
    }

    /// Get current usage for a key
    pub fn get_usage(&self, key: &str) -> Option<usize> {
        self.records.get(key).map(|record| {
            let record = record.read();
            record.timestamps.len()
        })
    }

    /// Reset rate limit for a key
    pub fn reset(&self, key: &str) {
        self.records.remove(key);
    }

    /// Clear all rate limit records
    pub fn clear_all(&self) {
        self.records.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_rate_limiter_allows_requests() {
        let config = RateLimiterConfig {
            max_requests: 5,
            window: Duration::from_secs(1),
            sliding_window: true,
        };
        let limiter = RateLimiter::new(config);

        for i in 0..5 {
            assert!(limiter.check_rate_limit("test_key").is_ok(), "Request {} should be allowed", i);
        }
    }

    #[test]
    fn test_rate_limiter_blocks_excess_requests() {
        let config = RateLimiterConfig {
            max_requests: 3,
            window: Duration::from_secs(1),
            sliding_window: true,
        };
        let limiter = RateLimiter::new(config);

        for _ in 0..3 {
            assert!(limiter.check_rate_limit("test_key").is_ok());
        }
        
        assert!(limiter.check_rate_limit("test_key").is_err());
    }

    #[test]
    fn test_rate_limiter_resets_after_window() {
        let config = RateLimiterConfig {
            max_requests: 2,
            window: Duration::from_millis(100),
            sliding_window: true,
        };
        let limiter = RateLimiter::new(config);

        assert!(limiter.check_rate_limit("test_key").is_ok());
        assert!(limiter.check_rate_limit("test_key").is_ok());
        assert!(limiter.check_rate_limit("test_key").is_err());

        thread::sleep(Duration::from_millis(150));
        
        assert!(limiter.check_rate_limit("test_key").is_ok());
    }

    #[test]
    fn test_rate_limiter_per_key() {
        let config = RateLimiterConfig {
            max_requests: 2,
            window: Duration::from_secs(1),
            sliding_window: true,
        };
        let limiter = RateLimiter::new(config);

        assert!(limiter.check_rate_limit("key1").is_ok());
        assert!(limiter.check_rate_limit("key1").is_ok());
        assert!(limiter.check_rate_limit("key1").is_err());

        assert!(limiter.check_rate_limit("key2").is_ok());
        assert!(limiter.check_rate_limit("key2").is_ok());
    }
}
