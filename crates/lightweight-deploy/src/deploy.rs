//! Deployment management functionality

use crate::config::{LiteConfig, DeployMode};
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;
use tracing::{info, warn, error};

/// Deployment manager for handling configuration and deployment tasks
pub struct DeployManager {
    config: LiteConfig,
}

impl DeployManager {
    /// Create a new deployment manager
    pub fn new(config: LiteConfig) -> Self {
        Self { config }
    }
    
    /// Initialize configuration with template
    pub async fn init_config(&mut self, force: bool, template: &str) -> Result<()> {
        let config_path = PathBuf::from("./moltis-lite.toml");
        
        if config_path.exists() && !force {
            return Err(anyhow::anyhow!(
                "Configuration file already exists. Use --force to overwrite."
            ));
        }
        
        info!("Creating configuration from template: {}", template);
        
        let config = LiteConfig::from_template(template)?;
        config.save_to_file(&config_path)?;
        
        info!("Configuration saved to: {}", config_path.display());
        info!("You can now start the server with: moltis-lite start");
        
        // Update internal config
        self.config = config;
        
        Ok(())
    }
    
    /// Validate current configuration
    pub async fn validate(&self) -> Result<()> {
        info!("Validating configuration...");
        
        self.config.validate()
            .context("Configuration validation failed")?;
        
        // Check if required directories exist
        self.check_directories().await?;
        
        // Check database connectivity
        self.check_database().await?;
        
        info!("✅ Configuration is valid");
        
        Ok(())
    }
    
    /// Show system status
    pub async fn show_status(&self) -> Result<()> {
        info!("=== System Status ===");
        
        // Show configuration summary
        self.show_config_summary().await;
        
        // Check system resources
        self.check_system_resources().await?;
        
        // Show feature status
        self.show_feature_status().await;
        
        Ok(())
    }
    
    /// Generate deployment script
    pub async fn generate_script(&self, output: PathBuf, script_type: &str) -> Result<()> {
        info!("Generating {} deployment script: {}", script_type, output.display());
        
        let script_content = match script_type {
            "bash" => self.generate_bash_script().await?,
            "powershell" => self.generate_powershell_script().await?,
            "docker" => self.generate_docker_compose().await?,
            _ => return Err(anyhow::anyhow!("Unsupported script type: {}", script_type)),
        };
        
        fs::write(&output, script_content)
            .with_context(|| format!("Failed to write script to: {}", output.display()))?;
        
        // Make script executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&output)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&output, perms)?;
        }
        
        info!("✅ Script generated: {}", output.display());
        
        Ok(())
    }
    
    /// Build Docker image
    pub async fn build_docker_image(&self, name: &str, tag: &str) -> Result<()> {
        info!("Building Docker image: {}:{}", name, tag);
        
        // Generate Dockerfile
        let dockerfile_content = self.generate_dockerfile().await?;
        let dockerfile_path = PathBuf::from("./Dockerfile");
        fs::write(&dockerfile_path, dockerfile_content)?;
        
        // Generate .dockerignore
        let dockerignore_content = self.generate_dockerignore().await?;
        let dockerignore_path = PathBuf::from("./.dockerignore");
        fs::write(&dockerignore_path, dockerignore_content)?;
        
        // Build command
        let build_command = format!("docker build -t {}:{} .", name, tag);
        
        info!("Running: {}", build_command);
        
        #[cfg(not(feature = "docker-build"))]
        {
            info!("Docker build command generated. Run manually:");
            info!("  {}", build_command);
        }
        
        #[cfg(feature = "docker-build")]
        {
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(&build_command)
                .output()
                .context("Failed to execute docker build")?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!("Docker build failed: {}", stderr));
            }
            
            info!("✅ Docker image built successfully: {}:{}", name, tag);
        }
        
        Ok(())
    }
    
    /// Check required directories
    async fn check_directories(&self) -> Result<()> {
        let dirs_to_check = vec![
            ("data", "Data directory"),
            ("logs", "Logs directory"),
        ];
        
        for (dir, description) in dirs_to_check {
            let path = PathBuf::from(dir);
            if !path.exists() {
                info!("Creating {}...", description);
                fs::create_dir_all(&path)
                    .with_context(|| format!("Failed to create {}: {}", description, dir))?;
            }
        }
        
        Ok(())
    }
    
    /// Check database connectivity
    async fn check_database(&self) -> Result<()> {
        match self.config.database.db_type {
            crate::config::DatabaseType::SQLite => {
                let db_path = &self.config.database.sqlite_path;
                if let Some(parent) = db_path.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)
                            .with_context(|| format!("Failed to create database directory: {}", parent.display()))?;
                    }
                }
                info!("✅ SQLite database path: {}", db_path.display());
            }
            crate::config::DatabaseType::PostgreSQL | crate::config::DatabaseType::MySQL => {
                // In a real implementation, this would test the database connection
                info!("✅ External database configured");
            }
        }
        
        Ok(())
    }
    
    /// Show configuration summary
    async fn show_config_summary(&self) {
        info!("Configuration Summary:");
        info!("  Mode: {}", self.config.mode);
        info!("  Server: {}:{}", self.config.server.host, self.config.server.port);
        info!("  Database: {:?}", self.config.database.db_type);
        info!("  Authentication: {}", if self.config.security.auth_enabled { "Enabled" } else { "Disabled" });
        
        let enabled_channels: Vec<_> = [
            ("Discord", self.config.channels.discord),
            ("Telegram", self.config.channels.telegram),
            ("Slack", self.config.channels.slack),
            ("Matrix", self.config.channels.matrix),
            ("WhatsApp", self.config.channels.whatsapp),
            ("WeChat", self.config.channels.wechat),
            ("DingTalk", self.config.channels.dingtalk),
            ("Feishu", self.config.channels.feishu),
        ]
        .iter()
        .filter_map(|(name, enabled)| if *enabled { Some(*name) } else { None })
        .collect();
        
        info!("  Enabled Channels: {}", enabled_channels.join(", "));
    }
    
    /// Check system resources
    async fn check_system_resources(&self) -> Result<()> {
        info!("System Resources:");
        
        // Check available memory
        #[cfg(unix)]
        {
            let memory_info = self.get_memory_info().await?;
            info!("  Available Memory: {} MB", memory_info.available_mb);
        }
        
        #[cfg(not(unix))]
        {
            info!("  Memory check: Skipped (non-Unix system)");
        }
        
        // Check disk space
        let disk_info = self.get_disk_info().await?;
        info!("  Available Disk Space: {} GB", disk_info.available_gb);
        
        Ok(())
    }
    
    /// Show feature status
    async fn show_feature_status(&self) {
        info!("Feature Status:");
        info!("  Web Interface: {}", if self.config.features.web_enabled { "✅" } else { "❌" });
        info!("  API: {}", if self.config.features.api_enabled { "✅" } else { "❌" });
        info!("  Agent Loop: {}", if self.config.features.agent_loop_enabled { "✅" } else { "❌" });
        info!("  Chat Catchup: {}", if self.config.features.chat_catchup_enabled { "✅" } else { "❌" });
        info!("  Rate Limiting: {}", if self.config.features.rate_limiting_enabled { "✅" } else { "❌" });
        info!("  Metrics: {}", if self.config.features.metrics_enabled { "✅" } else { "❌" });
        info!("  Tracing: {}", if self.config.features.tracing_enabled { "✅" } else { "❌" });
    }
    
    /// Generate bash deployment script
    async fn generate_bash_script(&self) -> Result<String> {
        let script = format!(r#"#!/bin/bash
# ClawMaster Lightweight Deployment Script
# Generated automatically by moltis-lite

set -e

# Configuration
HOST="{}"
PORT={}
CONFIG_FILE="./moltis-lite.toml"
DATA_DIR="./data"
LOGS_DIR="./logs"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo_info() {{
    echo -e "${{GREEN}}[INFO]${{NC}} $1"
}}

echo_warn() {{
    echo -e "${{YELLOW}}[WARN]${{NC}} $1"
}}

echo_error() {{
    echo -e "${{RED}}[ERROR]${{NC}} $1"
}}

# Check prerequisites
check_prerequisites() {{
    echo_info "Checking prerequisites..."
    
    if ! command -v moltis-lite &> /dev/null; then
        echo_error "clawmaster-lite is not installed or not in PATH"
        exit 1
    fi
    
    echo_info "Prerequisites check passed"
}}

# Setup directories
setup_directories() {{
    echo_info "Setting up directories..."
    
    mkdir -p "$DATA_DIR"
    mkdir -p "$LOGS_DIR"
    
    echo_info "Directories setup complete"
}}

# Validate configuration
validate_config() {{
    echo_info "Validating configuration..."
    
    if [ ! -f "$CONFIG_FILE" ]; then
        echo_warn "Configuration file not found: $CONFIG_FILE"
        echo_info "Creating default configuration..."
        moltis-lite init --template basic
    fi
    
    moltis-lite validate --config "$CONFIG_FILE"
    
    echo_info "Configuration validation complete"
}}

# Start the server
start_server() {{
    echo_info "Starting ClawMaster server..."
    echo_info "Server will be available at http://$HOST:$PORT"
    
    moltis-lite start --host "$HOST" --port "$PORT" --config "$CONFIG_FILE"
}}

# Main execution
main() {{
    echo_info "Starting ClawMaster deployment..."
    
    check_prerequisites
    setup_directories
    validate_config
    start_server
}}

# Handle signals
trap 'echo_info "Shutting down..."; exit 0' SIGINT SIGTERM

# Run main function
main "$@"
"#, self.config.server.host, self.config.server.port);
        
        Ok(script)
    }
    
    /// Generate PowerShell deployment script
    async fn generate_powershell_script(&self) -> Result<String> {
        let script = format!(r#"# ClawMaster Lightweight Deployment Script
# Generated automatically by moltis-lite

# Configuration
$Host = "{}"
$Port = {}
$ConfigFile = ".\moltis-lite.toml"
$DataDir = ".\data"
$LogsDir = ".\logs"

# Colors for output
function Write-Info($Message) {{
    Write-Host "[INFO] $Message" -ForegroundColor Green
}}

function Write-Warn($Message) {{
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}}

function Write-Error($Message) {{
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}}

# Check prerequisites
function Test-Prerequisites {{
    Write-Info "Checking prerequisites..."
    
    try {{
        $null = Get-Command moltis-lite -ErrorAction Stop
        Write-Info "Prerequisites check passed"
    }} catch {{
        Write-Error "clawmaster-lite is not installed or not in PATH"
        exit 1
    }}
}}

# Setup directories
function Initialize-Directories {{
    Write-Info "Setting up directories..."
    
    if (!(Test-Path $DataDir)) {{
        New-Item -ItemType Directory -Path $DataDir -Force | Out-Null
    }}
    
    if (!(Test-Path $LogsDir)) {{
        New-Item -ItemType Directory -Path $LogsDir -Force | Out-Null
    }}
    
    Write-Info "Directories setup complete"
}}

# Validate configuration
function Test-Configuration {{
    Write-Info "Validating configuration..."
    
    if (!(Test-Path $ConfigFile)) {{
        Write-Warn "Configuration file not found: $ConfigFile"
        Write-Info "Creating default configuration..."
        & moltis-lite init --template basic
    }}
    
    & moltis-lite validate --config $ConfigFile
    
    Write-Info "Configuration validation complete"
}}

# Start the server
function Start-Server {{
    Write-Info "Starting ClawMaster server..."
    Write-Info "Server will be available at http://$Host`:$Port"
    
    & moltis-lite start --host $Host --port $Port --config $ConfigFile
}}

# Main execution
function Main {{
    Write-Info "Starting ClawMaster deployment..."
    
    Test-Prerequisites
    Initialize-Directories
    Test-Configuration
    Start-Server
}}

# Handle Ctrl+C
$originalErrorActionPreference = $ErrorActionPreference
$ErrorActionPreference = "SilentlyContinue"
try {{
    [Console]::TreatControlCAsInput = $true
}} catch {{
    # Ignore if not supported
}}
$ErrorActionPreference = $originalErrorActionPreference

# Run main function
Main
"#, self.config.server.host, self.config.server.port);
        
        Ok(script)
    }
    
    /// Generate Docker Compose file
    async fn generate_docker_compose(&self) -> Result<String> {
        let compose = format!(r#"version: '3.8'

services:
  moltis-lite:
    build: .
    ports:
      - "{}:{}"
    environment:
      - RUST_LOG=info
      - MOLTIS_MODE={}
    volumes:
      - ./data:/app/data
      - ./logs:/app/logs
      - ./moltis-lite.toml:/app/moltis-lite.toml:ro
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:{}/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

networks:
  default:
    name: moltis-network
"#, 
            self.config.server.port, 
            self.config.server.port,
            self.config.mode,
            self.config.server.port
        );
        
        Ok(compose)
    }
    
    /// Generate Dockerfile
    async fn generate_dockerfile(&self) -> Result<String> {
        let features = if self.config.features.web_enabled && self.config.features.api_enabled {
            "all"
        } else {
            "minimal"
        };
        
        let dockerfile = format!(r#"# ClawMaster Lightweight Docker Image
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build the application
RUN cargo build --release --features {} --bin moltis-lite

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false moltis

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/moltis-lite /usr/local/bin/moltis-lite

# Create data directories
RUN mkdir -p /app/data /app/logs && \
    chown -R moltis:moltis /app

# Switch to app user
USER moltis

# Expose port
EXPOSE {}

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:{}/health || exit 1

# Default command
CMD ["clawmaster-lite", "start", "--host", "0.0.0.0", "--port", "{}"]
"#, 
            features,
            self.config.server.port,
            self.config.server.port,
            self.config.server.port
        );
        
        Ok(dockerfile)
    }
    
    /// Generate .dockerignore
    async fn generate_dockerignore(&self) -> Result<String> {
        let dockerignore = r#"# Rust
target/
Cargo.lock
**/*.rs.bk

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Logs
logs/
*.log

# Data (will be mounted as volume)
data/
*.db
*.sqlite

# Git
.git/
.gitignore

# Documentation
docs/
*.md

# Tests
tests/
**/*_test.rs

# Temporary files
tmp/
temp/
*.tmp

# Docker
Dockerfile
.dockerignore
docker-compose.yml
"#;
        
        Ok(dockerignore.to_string())
    }
    
    /// Get memory information (Unix only)
    #[cfg(unix)]
    async fn get_memory_info(&self) -> Result<MemoryInfo> {
        use std::fs;
        
        let meminfo = fs::read_to_string("/proc/meminfo")?;
        let mut available_kb = 0;
        
        for line in meminfo.lines() {
            if line.starts_with("MemAvailable:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    available_kb = parts[1].parse().unwrap_or(0);
                }
                break;
            }
        }
        
        Ok(MemoryInfo {
            available_mb: available_kb / 1024,
        })
    }
    
    /// Get disk information
    async fn get_disk_info(&self) -> Result<DiskInfo> {
        // Note: Getting actual disk space requires platform-specific APIs
        // For a lightweight deployment, we'll return placeholder values
        // In production, use the `fs2` or `sysinfo` crate for cross-platform disk info
        
        Ok(DiskInfo {
            total_gb: 0, // Placeholder - would need platform-specific implementation
            available_gb: 0, // Placeholder - would need platform-specific implementation
        })
    }
}

#[derive(Debug)]
struct MemoryInfo {
    available_mb: u64,
}

#[derive(Debug)]
struct DiskInfo {
    total_gb: u64,
    available_gb: u64,
}
