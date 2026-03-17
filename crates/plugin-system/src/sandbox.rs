//! Plugin sandboxing for security isolation

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Maximum memory usage in MB
    pub max_memory_mb: usize,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: u8,
    /// Execution timeout
    pub timeout: Duration,
    /// Enable network access
    pub allow_network: bool,
    /// Enable file system access
    pub allow_filesystem: bool,
    /// Allowed file paths
    pub allowed_paths: Vec<String>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            max_cpu_percent: 80,
            timeout: Duration::from_secs(30),
            allow_network: false,
            allow_filesystem: false,
            allowed_paths: vec![],
        }
    }
}

/// Plugin sandbox
pub struct PluginSandbox {
    config: SandboxConfig,
}

impl PluginSandbox {
    /// Create a new sandbox
    pub fn new(config: SandboxConfig) -> Result<Self> {
        Ok(Self { config })
    }

    /// Execute code in sandbox
    pub async fn execute<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        // Spawn in a separate task with timeout
        let timeout = self.config.timeout;
        
        let handle = tokio::task::spawn_blocking(f);
        
        match tokio::time::timeout(timeout, handle).await {
            Ok(Ok(result)) => result,
            Ok(Err(e)) => anyhow::bail!("sandbox task panicked: {:?}", e),
            Err(_) => anyhow::bail!("sandbox execution timeout"),
        }
    }

    /// Check if path is allowed
    pub fn is_path_allowed(&self, path: &str) -> bool {
        if !self.config.allow_filesystem {
            return false;
        }

        if self.config.allowed_paths.is_empty() {
            return true;
        }

        self.config.allowed_paths.iter().any(|allowed| {
            path.starts_with(allowed)
        })
    }

    /// Check if network access is allowed
    pub fn is_network_allowed(&self) -> bool {
        self.config.allow_network
    }

    /// Get memory limit
    pub fn memory_limit_mb(&self) -> usize {
        self.config.max_memory_mb
    }

    /// Get CPU limit
    pub fn cpu_limit_percent(&self) -> u8 {
        self.config.max_cpu_percent
    }

    /// Get timeout
    pub fn timeout(&self) -> Duration {
        self.config.timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sandbox_execute_success() {
        let sandbox = PluginSandbox::new(SandboxConfig::default()).unwrap();
        
        let result = sandbox.execute(|| Ok(42)).await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_sandbox_execute_timeout() {
        let mut config = SandboxConfig::default();
        config.timeout = Duration::from_millis(100);
        
        let sandbox = PluginSandbox::new(config).unwrap();
        
        let result = sandbox.execute(|| {
            std::thread::sleep(Duration::from_secs(1));
            Ok(42)
        }).await;
        
        assert!(result.is_err());
    }

    #[test]
    fn test_sandbox_path_allowed() {
        let mut config = SandboxConfig::default();
        config.allow_filesystem = true;
        config.allowed_paths = vec!["/tmp".to_string()];
        
        let sandbox = PluginSandbox::new(config).unwrap();
        
        assert!(sandbox.is_path_allowed("/tmp/test.txt"));
        assert!(!sandbox.is_path_allowed("/etc/passwd"));
    }

    #[test]
    fn test_sandbox_network_allowed() {
        let mut config = SandboxConfig::default();
        config.allow_network = true;
        
        let sandbox = PluginSandbox::new(config).unwrap();
        assert!(sandbox.is_network_allowed());
    }

    #[test]
    fn test_sandbox_limits() {
        let config = SandboxConfig {
            max_memory_mb: 256,
            max_cpu_percent: 50,
            timeout: Duration::from_secs(10),
            allow_network: false,
            allow_filesystem: false,
            allowed_paths: vec![],
        };
        
        let sandbox = PluginSandbox::new(config).unwrap();
        
        assert_eq!(sandbox.memory_limit_mb(), 256);
        assert_eq!(sandbox.cpu_limit_percent(), 50);
        assert_eq!(sandbox.timeout(), Duration::from_secs(10));
    }
}
