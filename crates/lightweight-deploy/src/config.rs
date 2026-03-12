//! Lightweight configuration management

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::{Context, Result};
use std::fs;

/// Lightweight configuration for easy deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteConfig {
    /// Basic server configuration
    pub server: ServerConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Channel configuration (simplified)
    pub channels: ChannelsConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Deployment mode
    pub mode: DeployMode,
    /// Feature flags
    pub features: FeatureFlags,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Base URL
    pub base_url: String,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database type
    pub db_type: DatabaseType,
    /// Database URL (for PostgreSQL/MySQL)
    pub url: Option<String>,
    /// SQLite file path
    pub sqlite_path: PathBuf,
    /// Maximum connections
    pub max_connections: usize,
}

/// Database types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    SQLite,
    PostgreSQL,
    MySQL,
}

/// Simplified channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsConfig {
    /// Enable Discord
    pub discord: bool,
    /// Enable Telegram
    pub telegram: bool,
    /// Enable Slack
    pub slack: bool,
    /// Enable Matrix
    pub matrix: bool,
    /// Enable WhatsApp (Enterprise)
    pub whatsapp: bool,
    /// Enable WeChat (Enterprise)
    pub wechat: bool,
    /// Enable DingTalk (Enterprise)
    pub dingtalk: bool,
    /// Enable Feishu (Enterprise)
    pub feishu: bool,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable authentication
    pub auth_enabled: bool,
    /// JWT secret (auto-generated if empty)
    pub jwt_secret: Option<String>,
    /// Session timeout in hours
    pub session_timeout_hours: u64,
    /// Enable CORS
    pub cors_enabled: bool,
    /// Allowed origins for CORS
    pub cors_origins: Vec<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log to file
    pub file_enabled: bool,
    /// Log file path
    pub file_path: Option<PathBuf>,
    /// Log to console
    pub console_enabled: bool,
    /// Log format
    pub format: LogFormat,
}

/// Log formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Pretty,
    Compact,
}

/// Deployment modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeployMode {
    /// Auto-detect and configure
    Auto,
    /// Development mode
    Development,
    /// Production mode
    Production,
    /// Minimal mode (basic features only)
    Minimal,
    /// Enterprise mode (all features)
    Enterprise,
}

/// Feature flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    /// Enable web interface
    pub web_enabled: bool,
    /// Enable API
    pub api_enabled: bool,
    /// Enable agent loop
    pub agent_loop_enabled: bool,
    /// Enable chat catchup
    pub chat_catchup_enabled: bool,
    /// Enable rate limiting
    pub rate_limiting_enabled: bool,
    /// Enable metrics
    pub metrics_enabled: bool,
    /// Enable tracing
    pub tracing_enabled: bool,
}

impl Default for LiteConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            channels: ChannelsConfig::default(),
            security: SecurityConfig::default(),
            logging: LoggingConfig::default(),
            mode: DeployMode::Auto,
            features: FeatureFlags::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            base_url: "http://localhost:8080".to_string(),
            max_connections: 1000,
            timeout_seconds: 30,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_type: DatabaseType::SQLite,
            url: None,
            sqlite_path: PathBuf::from("./data/moltis.db"),
            max_connections: 10,
        }
    }
}

impl Default for ChannelsConfig {
    fn default() -> Self {
        Self {
            discord: true,
            telegram: true,
            slack: false,
            matrix: false,
            whatsapp: false,
            wechat: false,
            dingtalk: false,
            feishu: false,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth_enabled: true,
            jwt_secret: None, // Will be auto-generated
            session_timeout_hours: 24,
            cors_enabled: true,
            cors_origins: vec!["*".to_string()],
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file_enabled: false,
            file_path: None,
            console_enabled: true,
            format: LogFormat::Pretty,
        }
    }
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            web_enabled: true,
            api_enabled: true,
            agent_loop_enabled: true,
            chat_catchup_enabled: true,
            rate_limiting_enabled: true,
            metrics_enabled: false,
            tracing_enabled: true,
        }
    }
}

impl LiteConfig {
    /// Load configuration from file
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        
        let config: LiteConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
        
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }
        
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize configuration")?;
        
        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        
        Ok(())
    }
    
    /// Generate configuration for specific template
    pub fn from_template(template: &str) -> Result<Self> {
        let mut config = match template {
            "basic" => Self::basic_template(),
            "development" => Self::development_template(),
            "production" => Self::production_template(),
            "minimal" => Self::minimal_template(),
            "enterprise" => Self::enterprise_template(),
            _ => return Err(anyhow::anyhow!("Unknown template: {}", template)),
        };
        
        // Auto-generate JWT secret if needed
        if config.security.jwt_secret.is_none() {
            config.security.jwt_secret = Some(generate_jwt_secret());
        }
        
        Ok(config)
    }
    
    /// Basic template for quick start
    pub fn basic_template() -> Self {
        let mut config = Self::default();
        config.mode = DeployMode::Auto;
        config.channels = ChannelsConfig {
            discord: true,
            telegram: true,
            ..Default::default()
        };
        config
    }
    
    /// Development template
    pub fn development_template() -> Self {
        let mut config = Self::default();
        config.mode = DeployMode::Development;
        config.server.host = "127.0.0.1".to_string();
        config.server.port = 3000;
        config.logging.level = "debug".to_string();
        config.logging.file_enabled = true;
        config.logging.file_path = Some(PathBuf::from("./logs/moltis.log"));
        config.features.metrics_enabled = true;
        config
    }
    
    /// Production template
    pub fn production_template() -> Self {
        let mut config = Self::default();
        config.mode = DeployMode::Production;
        config.server.max_connections = 10000;
        config.database.db_type = DatabaseType::PostgreSQL;
        config.security.cors_origins = vec![]; // No wildcard in production
        config.logging.level = "warn".to_string();
        config.logging.file_enabled = true;
        config.logging.console_enabled = false;
        config.features.metrics_enabled = true;
        config
    }
    
    /// Minimal template
    pub fn minimal_template() -> Self {
        let mut config = Self::default();
        config.mode = DeployMode::Minimal;
        config.channels = ChannelsConfig {
            discord: true,
            ..Default::default()
        };
        config.features = FeatureFlags {
            web_enabled: true,
            api_enabled: true,
            agent_loop_enabled: false,
            chat_catchup_enabled: false,
            rate_limiting_enabled: false,
            metrics_enabled: false,
            tracing_enabled: false,
        };
        config
    }
    
    /// Enterprise template
    pub fn enterprise_template() -> Self {
        let mut config = Self::default();
        config.mode = DeployMode::Enterprise;
        config.channels = ChannelsConfig {
            discord: true,
            telegram: true,
            slack: true,
            matrix: true,
            whatsapp: true,
            wechat: true,
            dingtalk: true,
            feishu: true,
        };
        config.server.max_connections = 50000;
        config.database.db_type = DatabaseType::PostgreSQL;
        config.features.metrics_enabled = true;
        config
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate server configuration
        if self.server.port == 0 {
            return Err(anyhow::anyhow!("Server port cannot be 0"));
        }
        
        // Validate database configuration
        match self.database.db_type {
            DatabaseType::PostgreSQL | DatabaseType::MySQL => {
                if self.database.url.is_none() || self.database.url.as_ref().unwrap().is_empty() {
                    return Err(anyhow::anyhow!("Database URL is required for PostgreSQL/MySQL"));
                }
            }
            DatabaseType::SQLite => {
                // SQLite path will be created if it doesn't exist
            }
        }
        
        // Validate security configuration
        if self.security.auth_enabled && self.security.jwt_secret.is_none() {
            return Err(anyhow::anyhow!("JWT secret is required when authentication is enabled"));
        }
        
        // Validate logging configuration
        if self.logging.file_enabled && self.logging.file_path.is_none() {
            return Err(anyhow::anyhow!("Log file path is required when file logging is enabled"));
        }
        
        Ok(())
    }
}

/// Generate a random JWT secret
fn generate_jwt_secret() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

impl std::str::FromStr for DeployMode {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(DeployMode::Auto),
            "development" | "dev" => Ok(DeployMode::Development),
            "production" | "prod" => Ok(DeployMode::Production),
            "minimal" => Ok(DeployMode::Minimal),
            "enterprise" | "ent" => Ok(DeployMode::Enterprise),
            _ => Err(anyhow::anyhow!("Invalid deploy mode: {}", s)),
        }
    }
}

impl std::fmt::Display for DeployMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeployMode::Auto => write!(f, "auto"),
            DeployMode::Development => write!(f, "development"),
            DeployMode::Production => write!(f, "production"),
            DeployMode::Minimal => write!(f, "minimal"),
            DeployMode::Enterprise => write!(f, "enterprise"),
        }
    }
}
