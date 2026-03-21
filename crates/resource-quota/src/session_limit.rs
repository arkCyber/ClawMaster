//! Session Limit Management
//!
//! DO-178C Level A Compliant Session Limiting

use {
    crate::{QuotaError, QuotaResult},
    dashmap::DashMap,
    parking_lot::RwLock,
    std::sync::Arc,
};

/// Session limit configuration
#[derive(Debug, Clone)]
pub struct SessionLimitConfig {
    /// Maximum concurrent sessions per user
    pub max_sessions_per_user: usize,

    /// Maximum total sessions
    pub max_total_sessions: usize,
}

impl Default for SessionLimitConfig {
    fn default() -> Self {
        Self {
            max_sessions_per_user: 10,
            max_total_sessions: 10000,
        }
    }
}

/// Session limiter
///
/// DO-178C §11.10: Session management
#[derive(Debug)]
pub struct SessionLimiter {
    config: SessionLimitConfig,
    user_sessions: Arc<DashMap<String, RwLock<usize>>>,
    total_sessions: Arc<RwLock<usize>>,
}

impl SessionLimiter {
    /// Create new session limiter
    pub fn new(config: SessionLimitConfig) -> Self {
        Self {
            config,
            user_sessions: Arc::new(DashMap::new()),
            total_sessions: Arc::new(RwLock::new(0)),
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(SessionLimitConfig::default())
    }

    /// Acquire session for user
    ///
    /// DO-178C §11.10: Session acquisition
    pub fn acquire(&self, user_id: &str) -> QuotaResult<SessionGuard> {
        // Check total sessions limit
        {
            let total = self.total_sessions.read();
            if *total >= self.config.max_total_sessions {
                return Err(QuotaError::SessionLimitExceeded {
                    current: *total,
                    limit: self.config.max_total_sessions,
                });
            }
        }

        // Get or create user session count
        let user_count_ref = self
            .user_sessions
            .entry(user_id.to_string())
            .or_insert_with(|| RwLock::new(0));

        let mut user_count = user_count_ref.write();

        // Check per-user limit
        if *user_count >= self.config.max_sessions_per_user {
            return Err(QuotaError::SessionLimitExceeded {
                current: *user_count,
                limit: self.config.max_sessions_per_user,
            });
        }

        // Increment counters
        *user_count += 1;
        *self.total_sessions.write() += 1;

        Ok(SessionGuard {
            limiter: Arc::new(self.clone()),
            user_id: user_id.to_string(),
        })
    }

    /// Release session for user
    fn release(&self, user_id: &str) {
        if let Some(user_count_ref) = self.user_sessions.get(user_id) {
            let mut user_count = user_count_ref.write();
            *user_count = user_count.saturating_sub(1);
        }

        let mut total = self.total_sessions.write();
        *total = total.saturating_sub(1);
    }

    /// Get current session count for user
    pub fn get_user_sessions(&self, user_id: &str) -> usize {
        self.user_sessions
            .get(user_id)
            .map(|r| *r.read())
            .unwrap_or(0)
    }

    /// Get total session count
    pub fn get_total_sessions(&self) -> usize {
        *self.total_sessions.read()
    }

    /// Get available sessions for user
    pub fn get_user_available(&self, user_id: &str) -> usize {
        let current = self.get_user_sessions(user_id);
        self.config.max_sessions_per_user.saturating_sub(current)
    }

    /// Get total available sessions
    pub fn get_total_available(&self) -> usize {
        let current = *self.total_sessions.read();
        self.config.max_total_sessions.saturating_sub(current)
    }
}

impl Clone for SessionLimiter {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            user_sessions: Arc::clone(&self.user_sessions),
            total_sessions: Arc::clone(&self.total_sessions),
        }
    }
}

/// Session guard
///
/// Automatically releases session when dropped
#[derive(Debug)]
pub struct SessionGuard {
    limiter: Arc<SessionLimiter>,
    user_id: String,
}

impl Drop for SessionGuard {
    fn drop(&mut self) {
        self.limiter.release(&self.user_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_limiter_acquire() {
        let config = SessionLimitConfig {
            max_sessions_per_user: 5,
            max_total_sessions: 100,
        };
        let limiter = SessionLimiter::new(config);

        let _guard = limiter.acquire("user1").unwrap();
        assert_eq!(limiter.get_user_sessions("user1"), 1);
        assert_eq!(limiter.get_total_sessions(), 1);
    }

    #[test]
    fn test_session_limiter_per_user_limit() {
        let config = SessionLimitConfig {
            max_sessions_per_user: 2,
            max_total_sessions: 100,
        };
        let limiter = SessionLimiter::new(config);

        let _guard1 = limiter.acquire("user1").unwrap();
        let _guard2 = limiter.acquire("user1").unwrap();

        let result = limiter.acquire("user1");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            QuotaError::SessionLimitExceeded { .. }
        ));
    }

    #[test]
    fn test_session_limiter_total_limit() {
        let config = SessionLimitConfig {
            max_sessions_per_user: 10,
            max_total_sessions: 2,
        };
        let limiter = SessionLimiter::new(config);

        let _guard1 = limiter.acquire("user1").unwrap();
        let _guard2 = limiter.acquire("user2").unwrap();

        let result = limiter.acquire("user3");
        assert!(result.is_err());
    }

    #[test]
    fn test_session_limiter_release() {
        let config = SessionLimitConfig {
            max_sessions_per_user: 2,
            max_total_sessions: 100,
        };
        let limiter = SessionLimiter::new(config);

        {
            let _guard = limiter.acquire("user1").unwrap();
            assert_eq!(limiter.get_user_sessions("user1"), 1);
        } // guard dropped here

        assert_eq!(limiter.get_user_sessions("user1"), 0);
        assert_eq!(limiter.get_total_sessions(), 0);
    }

    #[test]
    fn test_session_limiter_multiple_users() {
        let config = SessionLimitConfig {
            max_sessions_per_user: 2,
            max_total_sessions: 100,
        };
        let limiter = SessionLimiter::new(config);

        let _guard1 = limiter.acquire("user1").unwrap();
        let _guard2 = limiter.acquire("user1").unwrap();
        let _guard3 = limiter.acquire("user2").unwrap();

        assert_eq!(limiter.get_user_sessions("user1"), 2);
        assert_eq!(limiter.get_user_sessions("user2"), 1);
        assert_eq!(limiter.get_total_sessions(), 3);
    }

    #[test]
    fn test_get_available() {
        let config = SessionLimitConfig {
            max_sessions_per_user: 5,
            max_total_sessions: 100,
        };
        let limiter = SessionLimiter::new(config);

        assert_eq!(limiter.get_user_available("user1"), 5);
        assert_eq!(limiter.get_total_available(), 100);

        let _guard = limiter.acquire("user1").unwrap();

        assert_eq!(limiter.get_user_available("user1"), 4);
        assert_eq!(limiter.get_total_available(), 99);
    }
}
