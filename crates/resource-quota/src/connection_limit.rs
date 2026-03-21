//! Connection Limit Management
//!
//! DO-178C Level A Compliant Connection Limiting

use {
    crate::{QuotaError, QuotaResult},
    parking_lot::RwLock,
    std::sync::Arc,
};

/// Connection limit configuration
#[derive(Debug, Clone)]
pub struct ConnectionLimitConfig {
    /// Maximum concurrent connections
    pub max_connections: usize,
}

impl Default for ConnectionLimitConfig {
    fn default() -> Self {
        Self {
            max_connections: 1000,
        }
    }
}

/// Connection limiter
///
/// DO-178C §11.10: Connection pool management
#[derive(Debug)]
pub struct ConnectionLimiter {
    config: ConnectionLimitConfig,
    current: Arc<RwLock<usize>>,
}

impl ConnectionLimiter {
    /// Create new connection limiter
    pub fn new(config: ConnectionLimitConfig) -> Self {
        Self {
            config,
            current: Arc::new(RwLock::new(0)),
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(ConnectionLimitConfig::default())
    }

    /// Acquire connection
    ///
    /// DO-178C §11.10: Connection acquisition
    pub fn acquire(&self) -> QuotaResult<ConnectionGuard> {
        let mut current = self.current.write();

        if *current >= self.config.max_connections {
            return Err(QuotaError::ConnectionLimitExceeded {
                current: *current,
                limit: self.config.max_connections,
            });
        }

        *current += 1;

        Ok(ConnectionGuard {
            limiter: Arc::new(self.clone()),
        })
    }

    /// Release connection
    fn release(&self) {
        let mut current = self.current.write();
        *current = current.saturating_sub(1);
    }

    /// Get current connection count
    pub fn get_current(&self) -> usize {
        *self.current.read()
    }

    /// Get available connections
    pub fn get_available(&self) -> usize {
        self.config
            .max_connections
            .saturating_sub(*self.current.read())
    }

    /// Get connection limit
    pub fn get_limit(&self) -> usize {
        self.config.max_connections
    }
}

impl Clone for ConnectionLimiter {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            current: Arc::clone(&self.current),
        }
    }
}

/// Connection guard
///
/// Automatically releases connection when dropped
#[derive(Debug)]
pub struct ConnectionGuard {
    limiter: Arc<ConnectionLimiter>,
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        self.limiter.release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_limiter_acquire() {
        let config = ConnectionLimitConfig { max_connections: 5 };
        let limiter = ConnectionLimiter::new(config);

        let _guard1 = limiter.acquire().unwrap();
        assert_eq!(limiter.get_current(), 1);

        let _guard2 = limiter.acquire().unwrap();
        assert_eq!(limiter.get_current(), 2);
    }

    #[test]
    fn test_connection_limiter_exceeds() {
        let config = ConnectionLimitConfig { max_connections: 2 };
        let limiter = ConnectionLimiter::new(config);

        let _guard1 = limiter.acquire().unwrap();
        let _guard2 = limiter.acquire().unwrap();

        let result = limiter.acquire();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            QuotaError::ConnectionLimitExceeded { .. }
        ));
    }

    #[test]
    fn test_connection_limiter_release() {
        let config = ConnectionLimitConfig { max_connections: 2 };
        let limiter = ConnectionLimiter::new(config);

        {
            let _guard = limiter.acquire().unwrap();
            assert_eq!(limiter.get_current(), 1);
        } // guard dropped here

        assert_eq!(limiter.get_current(), 0);
    }

    #[test]
    fn test_connection_limiter_reuse() {
        let config = ConnectionLimitConfig { max_connections: 1 };
        let limiter = ConnectionLimiter::new(config);

        {
            let _guard = limiter.acquire().unwrap();
            assert!(limiter.acquire().is_err());
        }

        // After release, should be able to acquire again
        assert!(limiter.acquire().is_ok());
    }

    #[test]
    fn test_get_available() {
        let config = ConnectionLimitConfig {
            max_connections: 10,
        };
        let limiter = ConnectionLimiter::new(config);

        assert_eq!(limiter.get_available(), 10);

        let _guard1 = limiter.acquire().unwrap();
        assert_eq!(limiter.get_available(), 9);

        let _guard2 = limiter.acquire().unwrap();
        assert_eq!(limiter.get_available(), 8);
    }
}
