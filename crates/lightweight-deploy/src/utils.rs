//! Utility functions for lightweight deployment

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

/// Ensure a directory exists, creating it if necessary
pub fn ensure_directory(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        info!("Creating directory: {}", path.display());
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}

/// Check if a file exists and is readable
pub fn check_file_readable(path: &PathBuf) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }
    
    fs::metadata(path)
        .with_context(|| format!("Failed to read metadata for: {}", path.display()))?;
    
    Ok(true)
}

/// Get the current user's home directory
pub fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))
}

/// Get the default configuration directory
pub fn get_config_dir() -> Result<PathBuf> {
    let home = get_home_dir()?;
    Ok(home.join(".config").join("clawmaster"))
}

/// Get the default data directory
pub fn get_data_dir() -> Result<PathBuf> {
    let home = get_home_dir()?;
    Ok(home.join(".local").join("share").join("clawmaster"))
}

/// Generate a secure random string
pub fn generate_random_string(length: usize) -> String {
    use uuid::Uuid;
    let uuid = Uuid::new_v4().to_string();
    uuid.chars().take(length).collect()
}

/// Check if a port is available
pub fn is_port_available(port: u16) -> Result<bool> {
    use std::net::{TcpListener, SocketAddr};
    
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    match TcpListener::bind(addr) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Find an available port starting from the given port
pub fn find_available_port(start_port: u16) -> Result<u16> {
    for port in start_port..(start_port + 100) {
        if is_port_available(port)? {
            return Ok(port);
        }
    }
    
    Err(anyhow::anyhow!("No available ports found starting from {}", start_port))
}

/// Validate a URL
pub fn validate_url(url: &str) -> Result<()> {
    url::Url::parse(url)
        .with_context(|| format!("Invalid URL: {}", url))?;
    Ok(())
}

/// Format bytes into human readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Format duration into human readable format
pub fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    
    if total_seconds < 60 {
        format!("{}s", total_seconds)
    } else if total_seconds < 3600 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{}m {}s", minutes, seconds)
    } else {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        format!("{}h {}m {}s", hours, minutes, seconds)
    }
}

/// Check system requirements
pub fn check_system_requirements() -> Result<SystemInfo> {
    let mut info = SystemInfo::new();
    
    // Get OS information
    info.os = std::env::consts::OS.to_string();
    info.arch = std::env::consts::ARCH.to_string();
    
    // Get memory information
    #[cfg(unix)]
    {
        if let Ok(memory) = get_memory_info() {
            info.total_memory = memory.total_kb;
            info.available_memory = memory.available_kb;
        }
    }
    
    #[cfg(not(unix))]
    {
        warn!("Memory information not available on this platform");
    }
    
    // Get disk information
    if let Ok(disk) = get_disk_info() {
        info.total_disk = disk.total_bytes;
        info.available_disk = disk.available_bytes;
    }
    
    Ok(info)
}

/// System information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub total_memory: u64,
    pub available_memory: u64,
    pub total_disk: u64,
    pub available_disk: u64,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            os: String::new(),
            arch: String::new(),
            total_memory: 0,
            available_memory: 0,
            total_disk: 0,
            available_disk: 0,
        }
    }
    
    pub fn meets_minimum_requirements(&self) -> bool {
        // Minimum requirements: 1GB RAM, 1GB disk space
        const MIN_MEMORY_KB: u64 = 1024 * 1024; // 1GB
        const MIN_DISK_BYTES: u64 = 1024 * 1024 * 1024; // 1GB
        
        self.available_memory >= MIN_MEMORY_KB && self.available_disk >= MIN_DISK_BYTES
    }
    
    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        const MIN_MEMORY_KB: u64 = 1024 * 1024; // 1GB
        const RECOMMENDED_MEMORY_KB: u64 = 2 * 1024 * 1024; // 2GB
        
        if self.available_memory < MIN_MEMORY_KB {
            recommendations.push(
                "Insufficient memory. At least 1GB RAM is required.".to_string()
            );
        } else if self.available_memory < RECOMMENDED_MEMORY_KB {
            recommendations.push(
                "Limited memory available. 2GB+ RAM is recommended for better performance.".to_string()
            );
        }
        
        const MIN_DISK_BYTES: u64 = 1024 * 1024 * 1024; // 1GB
        const RECOMMENDED_DISK_BYTES: u64 = 5 * 1024 * 1024 * 1024; // 5GB
        
        if self.available_disk < MIN_DISK_BYTES {
            recommendations.push(
                "Insufficient disk space. At least 1GB free space is required.".to_string()
            );
        } else if self.available_disk < RECOMMENDED_DISK_BYTES {
            recommendations.push(
                "Limited disk space available. 5GB+ free space is recommended.".to_string()
            );
        }
        
        if recommendations.is_empty() {
            recommendations.push("System meets all requirements.".to_string());
        }
        
        recommendations
    }
}

/// Get memory information (Unix only)
#[cfg(unix)]
fn get_memory_info() -> Result<MemoryInfo> {
    use std::fs;
    
    let meminfo = fs::read_to_string("/proc/meminfo")
        .context("Failed to read /proc/meminfo")?;
    
    let mut total_kb = 0u64;
    let mut available_kb = 0u64;
    
    for line in meminfo.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            match parts[0] {
                "MemTotal:" => total_kb = parts[1].parse().unwrap_or(0),
                "MemAvailable:" => available_kb = parts[1].parse().unwrap_or(0),
                _ => {}
            }
        }
    }
    
    Ok(MemoryInfo {
        total_kb,
        available_kb,
    })
}

/// Get disk information
fn get_disk_info() -> Result<DiskInfo> {
    use std::fs;
    
    let metadata = fs::metadata(".")
        .context("Failed to get current directory metadata")?;
    
    Ok(DiskInfo {
        total_bytes: 0, // Placeholder - requires platform-specific implementation
        available_bytes: 0, // Placeholder - requires platform-specific implementation
    })
}

#[derive(Debug)]
struct MemoryInfo {
    total_kb: u64,
    available_kb: u64,
}

#[derive(Debug)]
struct DiskInfo {
    total_bytes: u64,
    available_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.0 GB");
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(std::time::Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(std::time::Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(std::time::Duration::from_secs(3661)), "1h 1m 1s");
    }
    
    #[test]
    fn test_generate_random_string() {
        let s1 = generate_random_string(10);
        let s2 = generate_random_string(10);
        
        assert_eq!(s1.len(), 10);
        assert_eq!(s2.len(), 10);
        assert_ne!(s1, s2);
    }
    
    #[test]
    fn test_validate_url() {
        assert!(validate_url("http://localhost:8080").is_ok());
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("invalid-url").is_err());
    }
}
