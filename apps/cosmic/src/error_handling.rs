//! 错误处理模块 - DO-178C Level A 标准
//! 
//! 航空航天级别错误处理要求：
//! - 所有错误必须被捕获和记录
//! - 错误信息必须清晰可追溯
//! - 关键错误必须触发安全机制
//! - 错误恢复策略明确

use std::fmt;
use tracing::{error, warn};

/// 应用错误类型
#[derive(Debug, Clone)]
pub enum AppError {
    /// 文件操作错误
    FileOperation {
        operation: String,
        path: String,
        reason: String,
    },
    
    /// 网络错误
    Network {
        endpoint: String,
        reason: String,
    },
    
    /// 配置错误
    Configuration {
        key: String,
        reason: String,
    },
    
    /// 状态错误
    InvalidState {
        expected: String,
        actual: String,
    },
    
    /// 权限错误
    Permission {
        resource: String,
        required: String,
    },
    
    /// 资源不足
    ResourceExhausted {
        resource: String,
        limit: usize,
        requested: usize,
    },
    
    /// 超时错误
    Timeout {
        operation: String,
        duration_ms: u64,
    },
    
    /// 数据验证错误
    Validation {
        field: String,
        reason: String,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::FileOperation { operation, path, reason } => {
                write!(f, "File operation '{}' failed on '{}': {}", operation, path, reason)
            }
            AppError::Network { endpoint, reason } => {
                write!(f, "Network error at '{}': {}", endpoint, reason)
            }
            AppError::Configuration { key, reason } => {
                write!(f, "Configuration error for '{}': {}", key, reason)
            }
            AppError::InvalidState { expected, actual } => {
                write!(f, "Invalid state: expected '{}', got '{}'", expected, actual)
            }
            AppError::Permission { resource, required } => {
                write!(f, "Permission denied for '{}': requires '{}'", resource, required)
            }
            AppError::ResourceExhausted { resource, limit, requested } => {
                write!(f, "Resource '{}' exhausted: limit={}, requested={}", resource, limit, requested)
            }
            AppError::Timeout { operation, duration_ms } => {
                write!(f, "Operation '{}' timed out after {}ms", operation, duration_ms)
            }
            AppError::Validation { field, reason } => {
                write!(f, "Validation failed for '{}': {}", field, reason)
            }
        }
    }
}

impl std::error::Error for AppError {}

/// 错误严重级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// 信息级别 - 不影响功能
    Info,
    /// 警告级别 - 可能影响性能
    Warning,
    /// 错误级别 - 影响功能但可恢复
    Error,
    /// 严重级别 - 影响核心功能
    Critical,
    /// 致命级别 - 需要立即停止
    Fatal,
}

/// 错误处理器
pub struct ErrorHandler {
    /// 错误计数器
    error_count: usize,
    /// 警告计数器
    warning_count: usize,
    /// 最大错误数（触发熔断）
    max_errors: usize,
}

impl ErrorHandler {
    /// 创建新的错误处理器
    pub fn new(max_errors: usize) -> Self {
        Self {
            error_count: 0,
            warning_count: 0,
            max_errors,
        }
    }
    
    /// 处理错误
    pub fn handle_error(&mut self, error: &AppError, severity: ErrorSeverity) -> bool {
        match severity {
            ErrorSeverity::Info => {
                tracing::info!("Info: {}", error);
            }
            ErrorSeverity::Warning => {
                self.warning_count += 1;
                warn!("Warning #{}: {}", self.warning_count, error);
            }
            ErrorSeverity::Error => {
                self.error_count += 1;
                error!("Error #{}: {}", self.error_count, error);
            }
            ErrorSeverity::Critical => {
                self.error_count += 1;
                error!("CRITICAL #{}: {}", self.error_count, error);
            }
            ErrorSeverity::Fatal => {
                error!("FATAL: {}", error);
                return true; // 触发停止
            }
        }
        
        // 检查是否超过错误阈值
        if self.error_count >= self.max_errors {
            error!("Error threshold exceeded: {}/{}", self.error_count, self.max_errors);
            return true; // 触发熔断
        }
        
        false
    }
    
    /// 重置错误计数
    pub fn reset(&mut self) {
        self.error_count = 0;
        self.warning_count = 0;
    }
    
    /// 获取错误统计
    pub fn stats(&self) -> (usize, usize) {
        (self.error_count, self.warning_count)
    }
}

/// 结果类型别名
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_handler() {
        let mut handler = ErrorHandler::new(5);
        
        let error = AppError::FileOperation {
            operation: "read".to_string(),
            path: "/test".to_string(),
            reason: "not found".to_string(),
        };
        
        // 不应触发熔断
        assert!(!handler.handle_error(&error, ErrorSeverity::Warning));
        assert_eq!(handler.stats(), (0, 1));
        
        // 添加错误
        for _ in 0..4 {
            assert!(!handler.handle_error(&error, ErrorSeverity::Error));
        }
        assert_eq!(handler.stats(), (4, 1));
        
        // 第5个错误应触发熔断
        assert!(handler.handle_error(&error, ErrorSeverity::Error));
        assert_eq!(handler.stats(), (5, 1));
    }
    
    #[test]
    fn test_error_display() {
        let error = AppError::Timeout {
            operation: "save_session".to_string(),
            duration_ms: 5000,
        };
        
        assert_eq!(
            error.to_string(),
            "Operation 'save_session' timed out after 5000ms"
        );
    }
}
