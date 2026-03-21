//! Rate Limiter
//!
//! DO-178C Level A Compliant Rate Limiting

use {
    crate::{QuotaError, QuotaResult},
    dashmap::DashMap,
    parking_lot::RwLock,
    std::{
        sync::Arc,
        time::{Duration, Instant},
    },
};

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: usize,

    /// Time window duration
    pub window_duration: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window_duration: Duration::from_secs(60),
        }
    }
}

/// Request record
#[derive(Debug, Clone)]
struct RequestRecord {
    timestamps: Vec<Instant>,
}

impl RequestRecord {
    fn new() -> Self {
        Self {
            timestamps: Vec::new(),
        }
    }

    fn add_request(&mut self, now: Instant) {
        self.timestamps.push(now);
    }

    fn cleanup_old(&mut self, now: Instant, window: Duration) {
        let cutoff = now - window;
        self.timestamps.retain(|&ts| ts > cutoff);
    }

    fn count(&self) -> usize {
        self.timestamps.len()
    }
}

/// Rate limiter
///
/// DO-178C §11.10: Request rate limiting
pub struct RateLimiter {
    config: RateLimitConfig,
    records: Arc<DashMap<String, RwLock<RequestRecord>>>,
}

impl RateLimiter {
    /// Create new rate limiter
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            records: Arc::new(DashMap::new()),
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(RateLimitConfig::default())
    }

    /// Check if request is allowed
    ///
    /// DO-178C §11.10: Rate limit enforcement
    pub fn check_rate_limit(&self, key: &str) -> QuotaResult<()> {
        let now = Instant::now();

        // Get or create record
        let record_ref = self
            .records
            .entry(key.to_string())
            .or_insert_with(|| RwLock::new(RequestRecord::new()));

        let mut record = record_ref.write();

        // Cleanup old requests
        record.cleanup_old(now, self.config.window_duration);

        // Check limit
        if record.count() >= self.config.max_requests {
            return Err(QuotaError::RateLimitExceeded(format!(
                "{} requests in {:?}",
                record.count(),
                self.config.window_duration
            )));
        }

        // Add new request
        record.add_request(now);

        Ok(())
    }

    /// Get current request count for key
    pub fn get_count(&self, key: &str) -> usize {
        let now = Instant::now();

        if let Some(record_ref) = self.records.get(key) {
            let mut record = record_ref.write();
            record.cleanup_old(now, self.config.window_duration);
            record.count()
        } else {
            0
        }
    }

    /// Reset rate limit for key
    pub fn reset(&self, key: &str) {
        self.records.remove(key);
    }

    /// Clear all rate limits
    pub fn clear_all(&self) {
        self.records.clear();
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::thread};

    #[test]
    fn test_rate_limiter_allows_requests() {
        let config = RateLimitConfig {
            max_requests: 5,
            window_duration: Duration::from_secs(1),
        };
        let limiter = RateLimiter::new(config);

        // First 5 requests should succeed
        for _ in 0..5 {
            assert!(limiter.check_rate_limit("user1").is_ok());
        }
    }

    #[test]
    fn test_rate_limiter_blocks_excess() {
        let config = RateLimitConfig {
            max_requests: 3,
            window_duration: Duration::from_secs(1),
        };
        let limiter = RateLimiter::new(config);

        // First 3 requests succeed
        for _ in 0..3 {
            assert!(limiter.check_rate_limit("user1").is_ok());
        }

        // 4th request should fail
        let result = limiter.check_rate_limit("user1");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            QuotaError::RateLimitExceeded(_)
        ));
    }

    #[test]
    fn test_rate_limiter_window_reset() {
        let config = RateLimitConfig {
            max_requests: 2,
            window_duration: Duration::from_millis(100),
        };
        let limiter = RateLimiter::new(config);

        // Use up quota
        assert!(limiter.check_rate_limit("user1").is_ok());
        assert!(limiter.check_rate_limit("user1").is_ok());
        assert!(limiter.check_rate_limit("user1").is_err());

        // Wait for window to expire
        thread::sleep(Duration::from_millis(150));

        // Should work again
        assert!(limiter.check_rate_limit("user1").is_ok());
    }

    #[test]
    fn test_rate_limiter_per_user() {
        let config = RateLimitConfig {
            max_requests: 2,
            window_duration: Duration::from_secs(1),
        };
        let limiter = RateLimiter::new(config);

        // User1 uses quota
        assert!(limiter.check_rate_limit("user1").is_ok());
        assert!(limiter.check_rate_limit("user1").is_ok());
        assert!(limiter.check_rate_limit("user1").is_err());

        // User2 should still have quota
        assert!(limiter.check_rate_limit("user2").is_ok());
        assert!(limiter.check_rate_limit("user2").is_ok());
    }

    #[test]
    fn test_get_count() {
        let limiter = RateLimiter::default();

        assert_eq!(limiter.get_count("user1"), 0);

        limiter.check_rate_limit("user1").unwrap();
        limiter.check_rate_limit("user1").unwrap();

        assert_eq!(limiter.get_count("user1"), 2);
    }

    #[test]
    fn test_reset() {
        let limiter = RateLimiter::default();

        limiter.check_rate_limit("user1").unwrap();
        assert_eq!(limiter.get_count("user1"), 1);

        limiter.reset("user1");
        assert_eq!(limiter.get_count("user1"), 0);
    }
}
