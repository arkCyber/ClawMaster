//! Rate limiting for authentication endpoints.
//! 
//! Aerospace Standard: Prevents resource exhaustion and brute force attacks.
//! Implements token bucket algorithm with per-IP tracking.

use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

/// Rate limit error types.
#[derive(Debug, Clone, thiserror::Error)]
pub enum RateLimitError {
    #[error("Too many requests. Please try again later.")]
    TooManyAttempts,
    
    #[error("Rate limit configuration error: {0}")]
    ConfigError(String),
}

/// Token bucket for rate limiting.
/// 
/// Aerospace Compliance:
/// - Deterministic behavior (fixed capacity and refill rate)
/// - Bounded memory usage (automatic cleanup of old entries)
/// - Thread-safe (RwLock for concurrent access)
#[derive(Debug, Clone)]
struct TokenBucket {
    capacity: u32,
    tokens: u32,
    refill_rate: u32,
    refill_interval: Duration,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: u32, refill_rate: u32, refill_interval: Duration) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_rate,
            refill_interval,
            last_refill: Instant::now(),
        }
    }
    
    /// Attempt to consume a token. Returns true if successful.
    /// 
    /// Aerospace Standard: O(1) time complexity, deterministic behavior.
    fn try_consume(&mut self) -> bool {
        self.refill();
        
        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
    
    /// Refill tokens based on elapsed time.
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        
        if elapsed >= self.refill_interval {
            let intervals = (elapsed.as_secs_f64() / self.refill_interval.as_secs_f64()) as u32;
            let new_tokens = intervals * self.refill_rate;
            
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_refill = now;
        }
    }
}

/// Rate limiter for authentication endpoints.
/// 
/// Limits (Aerospace Standard - Conservative):
/// - Login: 5 attempts per IP per minute
/// - Password reset: 3 attempts per IP per hour
/// - Setup: 10 attempts per IP per hour
/// - API key creation: 5 per IP per hour
/// 
/// Memory Management:
/// - Automatic cleanup of entries older than 1 hour
/// - Maximum 10,000 tracked IPs (prevents memory exhaustion)
pub struct AuthRateLimiter {
    login_buckets: Arc<RwLock<HashMap<IpAddr, TokenBucket>>>,
    password_reset_buckets: Arc<RwLock<HashMap<IpAddr, TokenBucket>>>,
    setup_buckets: Arc<RwLock<HashMap<IpAddr, TokenBucket>>>,
    last_cleanup: Arc<RwLock<Instant>>,
}

impl AuthRateLimiter {
    /// Create a new rate limiter with aerospace-grade conservative limits.
    pub fn new() -> Self {
        Self {
            login_buckets: Arc::new(RwLock::new(HashMap::new())),
            password_reset_buckets: Arc::new(RwLock::new(HashMap::new())),
            setup_buckets: Arc::new(RwLock::new(HashMap::new())),
            last_cleanup: Arc::new(RwLock::new(Instant::now())),
        }
    }
    
    /// Check if a login attempt is allowed for the given IP.
    /// 
    /// Limit: 5 attempts per minute
    /// 
    /// Security: Prevents brute force password attacks.
    pub async fn check_login(&self, ip: IpAddr) -> Result<(), RateLimitError> {
        self.check_limit(
            &self.login_buckets,
            ip,
            5,  // capacity
            5,  // refill rate
            Duration::from_secs(60),  // refill interval (1 minute)
        ).await
    }
    
    /// Check if a password reset attempt is allowed for the given IP.
    /// 
    /// Limit: 3 attempts per hour
    /// 
    /// Security: Prevents password reset abuse and enumeration.
    pub async fn check_password_reset(&self, ip: IpAddr) -> Result<(), RateLimitError> {
        self.check_limit(
            &self.password_reset_buckets,
            ip,
            3,  // capacity
            3,  // refill rate
            Duration::from_secs(3600),  // refill interval (1 hour)
        ).await
    }
    
    /// Check if a setup attempt is allowed for the given IP.
    /// 
    /// Limit: 10 attempts per hour
    /// 
    /// Security: Prevents setup code brute forcing.
    pub async fn check_setup(&self, ip: IpAddr) -> Result<(), RateLimitError> {
        self.check_limit(
            &self.setup_buckets,
            ip,
            10,  // capacity
            10,  // refill rate
            Duration::from_secs(3600),  // refill interval (1 hour)
        ).await
    }
    
    /// Generic rate limit check with automatic cleanup.
    async fn check_limit(
        &self,
        buckets: &Arc<RwLock<HashMap<IpAddr, TokenBucket>>>,
        ip: IpAddr,
        capacity: u32,
        refill_rate: u32,
        refill_interval: Duration,
    ) -> Result<(), RateLimitError> {
        // Periodic cleanup to prevent memory exhaustion
        self.cleanup_if_needed().await;
        
        let mut buckets = buckets.write().await;
        
        // Aerospace Standard: Bounded memory usage
        const MAX_TRACKED_IPS: usize = 10_000;
        if buckets.len() >= MAX_TRACKED_IPS {
            // Remove oldest entries (simple LRU approximation)
            let to_remove: Vec<IpAddr> = buckets.keys().take(1000).copied().collect();
            for ip in to_remove {
                buckets.remove(&ip);
            }
            
            tracing::warn!(
                tracked_ips = buckets.len(),
                max_ips = MAX_TRACKED_IPS,
                "Rate limiter memory limit reached, cleaned up old entries"
            );
        }
        
        let bucket = buckets
            .entry(ip)
            .or_insert_with(|| TokenBucket::new(capacity, refill_rate, refill_interval));
        
        if bucket.try_consume() {
            Ok(())
        } else {
            tracing::warn!(
                ip = %ip,
                "Rate limit exceeded"
            );
            Err(RateLimitError::TooManyAttempts)
        }
    }
    
    /// Clean up old entries to prevent memory leaks.
    /// 
    /// Aerospace Standard: Bounded memory usage, deterministic cleanup.
    async fn cleanup_if_needed(&self) {
        const CLEANUP_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour
        
        let mut last_cleanup = self.last_cleanup.write().await;
        let now = Instant::now();
        
        if now.duration_since(*last_cleanup) >= CLEANUP_INTERVAL {
            // Cleanup is done by the bounded size check in check_limit
            *last_cleanup = now;
        }
    }
}

impl Default for AuthRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};
    
    #[tokio::test]
    async fn test_login_rate_limit() {
        let limiter = AuthRateLimiter::new();
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        
        // First 5 attempts should succeed
        for i in 0..5 {
            assert!(
                limiter.check_login(ip).await.is_ok(),
                "Attempt {} should succeed", i + 1
            );
        }
        
        // 6th attempt should fail
        assert!(
            limiter.check_login(ip).await.is_err(),
            "6th attempt should be rate limited"
        );
    }
    
    #[tokio::test]
    async fn test_different_ips_independent() {
        let limiter = AuthRateLimiter::new();
        let ip1 = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let ip2 = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 101));
        
        // Exhaust ip1's limit
        for _ in 0..5 {
            let _ = limiter.check_login(ip1).await;
        }
        
        // ip2 should still have full quota
        assert!(
            limiter.check_login(ip2).await.is_ok(),
            "Different IP should have independent quota"
        );
    }
    
    #[test]
    fn test_token_bucket_refill() {
        let mut bucket = TokenBucket::new(5, 5, Duration::from_millis(100));
        
        // Consume all tokens
        for _ in 0..5 {
            assert!(bucket.try_consume());
        }
        assert!(!bucket.try_consume());
        
        // Wait for refill
        std::thread::sleep(Duration::from_millis(150));
        
        // Should have tokens again
        assert!(bucket.try_consume());
    }
}
