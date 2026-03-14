//! Memory Quota Management
//!
//! DO-178C Level A Compliant Memory Quota

use crate::{QuotaError, QuotaResult};
use parking_lot::RwLock;
use std::sync::Arc;

/// Memory quota configuration
#[derive(Debug, Clone)]
pub struct MemoryQuotaConfig {
    /// Maximum memory in bytes
    pub max_memory: usize,
}

impl Default for MemoryQuotaConfig {
    fn default() -> Self {
        Self {
            max_memory: 1024 * 1024 * 1024, // 1GB
        }
    }
}

/// Memory quota tracker
///
/// DO-178C §11.10: Memory quota management
pub struct MemoryQuota {
    config: MemoryQuotaConfig,
    used: Arc<RwLock<usize>>,
}

impl MemoryQuota {
    /// Create new memory quota
    pub fn new(config: MemoryQuotaConfig) -> Self {
        Self {
            config,
            used: Arc::new(RwLock::new(0)),
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(MemoryQuotaConfig::default())
    }

    /// Allocate memory
    ///
    /// DO-178C §11.10: Memory allocation tracking
    pub fn allocate(&self, size: usize) -> QuotaResult<()> {
        let mut used = self.used.write();

        let new_used = *used + size;

        if new_used > self.config.max_memory {
            return Err(QuotaError::MemoryQuotaExceeded {
                used: new_used,
                limit: self.config.max_memory,
            });
        }

        *used = new_used;
        Ok(())
    }

    /// Deallocate memory
    pub fn deallocate(&self, size: usize) {
        let mut used = self.used.write();
        *used = used.saturating_sub(size);
    }

    /// Get current memory usage
    pub fn get_used(&self) -> usize {
        *self.used.read()
    }

    /// Get available memory
    pub fn get_available(&self) -> usize {
        self.config.max_memory.saturating_sub(*self.used.read())
    }

    /// Get memory limit
    pub fn get_limit(&self) -> usize {
        self.config.max_memory
    }

    /// Reset memory usage
    pub fn reset(&self) {
        *self.used.write() = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_quota_allocate() {
        let config = MemoryQuotaConfig {
            max_memory: 1000,
        };
        let quota = MemoryQuota::new(config);

        assert!(quota.allocate(500).is_ok());
        assert_eq!(quota.get_used(), 500);
        assert_eq!(quota.get_available(), 500);
    }

    #[test]
    fn test_memory_quota_exceeds() {
        let config = MemoryQuotaConfig {
            max_memory: 1000,
        };
        let quota = MemoryQuota::new(config);

        quota.allocate(800).unwrap();

        let result = quota.allocate(300);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), QuotaError::MemoryQuotaExceeded { .. }));
    }

    #[test]
    fn test_memory_quota_deallocate() {
        let config = MemoryQuotaConfig {
            max_memory: 1000,
        };
        let quota = MemoryQuota::new(config);

        quota.allocate(500).unwrap();
        assert_eq!(quota.get_used(), 500);

        quota.deallocate(200);
        assert_eq!(quota.get_used(), 300);
        assert_eq!(quota.get_available(), 700);
    }

    #[test]
    fn test_memory_quota_reset() {
        let quota = MemoryQuota::default();

        quota.allocate(500).unwrap();
        assert_eq!(quota.get_used(), 500);

        quota.reset();
        assert_eq!(quota.get_used(), 0);
    }
}
