//! Upload Size Limit
//!
//! DO-178C Level A Compliant Upload Size Limiting

use crate::{QuotaError, QuotaResult};

/// Upload limit configuration
#[derive(Debug, Clone)]
pub struct UploadLimitConfig {
    /// Maximum file size in bytes
    pub max_file_size: usize,

    /// Maximum total upload size per request
    pub max_total_size: usize,
}

impl Default for UploadLimitConfig {
    fn default() -> Self {
        Self {
            max_file_size: 100 * 1024 * 1024,  // 100MB per file
            max_total_size: 500 * 1024 * 1024, // 500MB total
        }
    }
}

/// Upload limiter
///
/// DO-178C §11.10: Upload size management
pub struct UploadLimiter {
    config: UploadLimitConfig,
}

impl UploadLimiter {
    /// Create new upload limiter
    pub fn new(config: UploadLimitConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(UploadLimitConfig::default())
    }

    /// Check if file size is allowed
    ///
    /// DO-178C §11.10: File size validation
    pub fn check_file_size(&self, size: usize) -> QuotaResult<()> {
        if size > self.config.max_file_size {
            return Err(QuotaError::UploadSizeExceeded {
                size,
                limit: self.config.max_file_size,
            });
        }
        Ok(())
    }

    /// Check if total upload size is allowed
    ///
    /// DO-178C §11.10: Total upload validation
    pub fn check_total_size(&self, size: usize) -> QuotaResult<()> {
        if size > self.config.max_total_size {
            return Err(QuotaError::UploadSizeExceeded {
                size,
                limit: self.config.max_total_size,
            });
        }
        Ok(())
    }

    /// Check multiple files
    pub fn check_files(&self, file_sizes: &[usize]) -> QuotaResult<()> {
        // Check each file individually
        for &size in file_sizes {
            self.check_file_size(size)?;
        }

        // Check total size
        let total: usize = file_sizes.iter().sum();
        self.check_total_size(total)?;

        Ok(())
    }

    /// Get maximum file size
    pub fn get_max_file_size(&self) -> usize {
        self.config.max_file_size
    }

    /// Get maximum total size
    pub fn get_max_total_size(&self) -> usize {
        self.config.max_total_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_limiter_file_size_ok() {
        let config = UploadLimitConfig {
            max_file_size: 1000,
            max_total_size: 5000,
        };
        let limiter = UploadLimiter::new(config);

        assert!(limiter.check_file_size(500).is_ok());
        assert!(limiter.check_file_size(1000).is_ok());
    }

    #[test]
    fn test_upload_limiter_file_size_exceeds() {
        let config = UploadLimitConfig {
            max_file_size: 1000,
            max_total_size: 5000,
        };
        let limiter = UploadLimiter::new(config);

        let result = limiter.check_file_size(1500);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            QuotaError::UploadSizeExceeded { .. }
        ));
    }

    #[test]
    fn test_upload_limiter_total_size_ok() {
        let config = UploadLimitConfig {
            max_file_size: 1000,
            max_total_size: 5000,
        };
        let limiter = UploadLimiter::new(config);

        assert!(limiter.check_total_size(3000).is_ok());
        assert!(limiter.check_total_size(5000).is_ok());
    }

    #[test]
    fn test_upload_limiter_total_size_exceeds() {
        let config = UploadLimitConfig {
            max_file_size: 1000,
            max_total_size: 5000,
        };
        let limiter = UploadLimiter::new(config);

        let result = limiter.check_total_size(6000);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            QuotaError::UploadSizeExceeded { .. }
        ));
    }

    #[test]
    fn test_upload_limiter_check_files() {
        let config = UploadLimitConfig {
            max_file_size: 1000,
            max_total_size: 3000,
        };
        let limiter = UploadLimiter::new(config);

        // All files within limits
        assert!(limiter.check_files(&[500, 800, 700]).is_ok());
    }

    #[test]
    fn test_upload_limiter_check_files_single_exceeds() {
        let config = UploadLimitConfig {
            max_file_size: 1000,
            max_total_size: 5000,
        };
        let limiter = UploadLimiter::new(config);

        // One file too large
        let result = limiter.check_files(&[500, 1500, 700]);
        assert!(result.is_err());
    }

    #[test]
    fn test_upload_limiter_check_files_total_exceeds() {
        let config = UploadLimitConfig {
            max_file_size: 1000,
            max_total_size: 2000,
        };
        let limiter = UploadLimiter::new(config);

        // Individual files ok, but total too large
        let result = limiter.check_files(&[800, 800, 800]);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_limits() {
        let config = UploadLimitConfig {
            max_file_size: 1000,
            max_total_size: 5000,
        };
        let limiter = UploadLimiter::new(config);

        assert_eq!(limiter.get_max_file_size(), 1000);
        assert_eq!(limiter.get_max_total_size(), 5000);
    }
}
