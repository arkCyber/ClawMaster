//! Configuration management for the cosmic client

use {
    anyhow::{Context, Result},
    serde::{Deserialize, Serialize},
    std::path::PathBuf,
    tracing::{debug, info, warn},
};

/// Configuration for the cosmic UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicConfig {
    /// Gateway URL for RPC communication
    pub gateway_url: String,
    /// UI settings
    pub ui: UiSettings,
    /// Theme settings
    pub theme: ThemeSettings,
    /// Window settings
    pub window: WindowSettings,
    /// Network settings
    pub network: NetworkSettings,
}

/// UI-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    /// Language code (e.g., "en", "zh")
    pub language: String,
    /// Auto-scroll in chat
    pub auto_scroll: bool,
    /// Show timestamps
    pub show_timestamps: bool,
    /// Show message word count
    pub show_word_count: bool,
    /// Messages per page in chat
    pub messages_per_page: u32,
    /// Auto-refresh interval in seconds
    pub auto_refresh_interval: u32,
    /// Show system tray icon
    pub show_system_tray: bool,
    /// Minimize to system tray
    pub minimize_to_tray: bool,
    /// Start minimized
    pub start_minimized: bool,
}

/// Theme settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    /// Theme name
    pub name: String,
    /// Custom theme path (if any)
    pub custom_theme_path: Option<PathBuf>,
    /// Font family
    pub font_family: String,
    /// Font size in points
    pub font_size: f32,
    /// Enable animations
    pub enable_animations: bool,
    /// Animation speed multiplier
    pub animation_speed: f32,
    /// Accent color (hex)
    pub accent_color: Option<String>,
    /// Use system theme
    pub use_system_theme: bool,
}

/// Window settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
    /// Default window width
    pub default_width: u32,
    /// Default window height
    pub default_height: u32,
    /// Remember window size
    pub remember_size: bool,
    /// Remember window position
    pub remember_position: bool,
    /// Start maximized
    pub start_maximized: bool,
    /// Always on top
    pub always_on_top: bool,
    /// Show in taskbar
    pub show_in_taskbar: bool,
}

/// Network settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    /// Connection timeout in seconds
    pub connection_timeout: u32,
    /// Read timeout in seconds
    pub read_timeout: u32,
    /// Reconnect interval in seconds
    pub reconnect_interval: u32,
    /// Max reconnection attempts
    pub max_reconnect_attempts: u32,
    /// Enable keep-alive
    pub enable_keep_alive: bool,
    /// Keep-alive interval in seconds
    pub keep_alive_interval: u32,
}

impl Default for CosmicConfig {
    fn default() -> Self {
        Self {
            gateway_url: "http://localhost:59233".to_string(),
            ui: UiSettings::default(),
            theme: ThemeSettings::default(),
            window: WindowSettings::default(),
            network: NetworkSettings::default(),
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            auto_scroll: true,
            show_timestamps: true,
            show_word_count: false,
            messages_per_page: 50,
            auto_refresh_interval: 5,
            show_system_tray: true,
            minimize_to_tray: false,
            start_minimized: false,
        }
    }
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            name: "dark".to_string(),
            custom_theme_path: None,
            font_family: "System".to_string(),
            font_size: 14.0,
            enable_animations: true,
            animation_speed: 1.0,
            accent_color: None,
            use_system_theme: true,
        }
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            default_width: 1200,
            default_height: 800,
            remember_size: true,
            remember_position: true,
            start_maximized: false,
            always_on_top: false,
            show_in_taskbar: true,
        }
    }
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            connection_timeout: 30,
            read_timeout: 60,
            reconnect_interval: 5,
            max_reconnect_attempts: 10,
            enable_keep_alive: true,
            keep_alive_interval: 30,
        }
    }
}

impl CosmicConfig {
    /// Load configuration from file
    pub async fn load() -> Result<Self> {
        let config_path = Self::config_path();

        if !config_path.exists() {
            debug!("Config file not found, creating default config");
            let default_config = Self::default();
            default_config.save().await?;
            return Ok(default_config);
        }

        debug!("Loading config from: {:?}", config_path);

        let content = tokio::fs::read_to_string(&config_path)
            .await
            .context("Failed to read config file")?;

        let config: Self = toml::from_str(&content).context("Failed to parse config file")?;

        info!("Configuration loaded successfully");
        Ok(config)
    }

    /// Save configuration to file
    pub async fn save(&self) -> Result<()> {
        let config_path = Self::config_path();

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .context("Failed to create config directory")?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        tokio::fs::write(&config_path, content)
            .await
            .context("Failed to write config file")?;

        debug!("Configuration saved to: {:?}", config_path);
        Ok(())
    }

    /// Get the configuration file path
    pub fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join("clawmaster");

        config_dir.join("cosmic.toml")
    }

    /// Get the data directory
    pub fn data_dir() -> PathBuf {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from(".local/share"))
            .join("clawmaster");

        data_dir
    }

    /// Get the cache directory
    pub fn cache_dir() -> PathBuf {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from(".cache"))
            .join("clawmaster");

        cache_dir
    }

    /// Update gateway URL
    pub fn set_gateway_url(&mut self, url: String) {
        self.gateway_url = url;
    }

    /// Get current theme name
    pub fn theme_name(&self) -> &str {
        &self.theme.name
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: String) {
        self.theme.name = theme;
    }

    /// Get font size
    pub fn font_size(&self) -> f32 {
        self.theme.font_size
    }

    /// Set font size
    pub fn set_font_size(&mut self, size: f32) {
        self.theme.font_size = size.clamp(8.0, 32.0);
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate gateway URL
        if self.gateway_url.is_empty() {
            return Err(anyhow::anyhow!("Gateway URL cannot be empty"));
        }

        if let Err(e) = url::Url::parse(&self.gateway_url) {
            warn!("Invalid gateway URL: {}", e);
            return Err(anyhow::anyhow!("Invalid gateway URL: {}", e));
        }

        // Validate UI settings
        if self.ui.messages_per_page == 0 {
            return Err(anyhow::anyhow!("Messages per page must be greater than 0"));
        }

        if self.ui.auto_refresh_interval == 0 {
            return Err(anyhow::anyhow!(
                "Auto refresh interval must be greater than 0"
            ));
        }

        // Validate theme settings
        if self.theme.font_size <= 0.0 {
            return Err(anyhow::anyhow!("Font size must be greater than 0"));
        }

        // Validate window settings
        if self.window.default_width < 400 {
            return Err(anyhow::anyhow!("Window width must be at least 400"));
        }

        if self.window.default_height < 300 {
            return Err(anyhow::anyhow!("Window height must be at least 300"));
        }

        // Validate network settings
        if self.network.connection_timeout == 0 {
            return Err(anyhow::anyhow!("Connection timeout must be greater than 0"));
        }

        Ok(())
    }

    /// Reset to default values
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }

    /// Merge with another configuration
    pub fn merge(&mut self, other: &Self) {
        if other.gateway_url != Self::default().gateway_url {
            self.gateway_url = other.gateway_url.clone();
        }

        // Merge UI settings
        if other.ui.language != Self::default().ui.language {
            self.ui.language = other.ui.language.clone();
        }
        if other.ui.auto_scroll != Self::default().ui.auto_scroll {
            self.ui.auto_scroll = other.ui.auto_scroll;
        }
        if other.ui.show_timestamps != Self::default().ui.show_timestamps {
            self.ui.show_timestamps = other.ui.show_timestamps;
        }
        if other.ui.messages_per_page != Self::default().ui.messages_per_page {
            self.ui.messages_per_page = other.ui.messages_per_page;
        }

        // Merge theme settings
        if other.theme.name != Self::default().theme.name {
            self.theme.name = other.theme.name.clone();
        }
        if other.theme.font_size != Self::default().theme.font_size {
            self.theme.font_size = other.theme.font_size;
        }

        // Merge window settings
        if other.window.default_width != Self::default().window.default_width {
            self.window.default_width = other.window.default_width;
        }
        if other.window.default_height != Self::default().window.default_height {
            self.window.default_height = other.window.default_height;
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::TempDir};

    #[tokio::test]
    async fn test_default_config() {
        let config = CosmicConfig::default();
        assert!(config.validate().is_ok());
    }

    #[tokio::test]
    async fn test_config_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        let original_config = CosmicConfig {
            gateway_url: "http://localhost:8080".to_string(),
            ui: UiSettings {
                language: "zh".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        // Save config
        let content = toml::to_string_pretty(&original_config).unwrap();
        tokio::fs::write(&config_path, content).await.unwrap();

        // Load config
        let loaded_content: String = tokio::fs::read_to_string(&config_path).await.unwrap();
        let loaded_config: CosmicConfig = toml::from_str(&loaded_content).unwrap();

        assert_eq!(loaded_config.gateway_url, "http://localhost:8080");
        assert_eq!(loaded_config.ui.language, "zh");
    }

    #[test]
    fn test_config_validation() {
        let mut config = CosmicConfig::default();

        // Valid config should pass
        assert!(config.validate().is_ok());

        // Empty gateway URL should fail
        config.gateway_url = String::new();
        assert!(config.validate().is_err());

        // Invalid URL should fail
        config.gateway_url = "not-a-url".to_string();
        assert!(config.validate().is_err());
    }
}
