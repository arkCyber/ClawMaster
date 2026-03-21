//! Data models for the ClawMaster cosmic client

use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    uuid::Uuid,
};

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: u32,
    pub is_active: bool,
    pub channel: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub session_id: String,
    pub role: MessageRole,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Message role
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Tool,
}

/// System status information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemStatus {
    pub connection_status: ConnectionStatus,
    pub active_sessions: u32,
    pub total_sessions: u32,
    pub available_models: u32,
    pub memory_usage: MemoryUsage,
    pub uptime_seconds: u64,
    pub version: String,
}

/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Connecting,
    Error,
}

/// Memory usage information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryUsage {
    pub used_mb: u64,
    pub total_mb: u64,
    pub free_mb: u64,
    pub process_mb: u64,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub model_type: ModelType,
    pub is_available: bool,
    pub context_length: Option<u32>,
    pub pricing: Option<ModelPricing>,
}

/// Model type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelType {
    Chat,
    Completion,
    Embedding,
    Image,
    Audio,
}

/// Model pricing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    pub input_price_per_1k: f64,
    pub output_price_per_1k: f64,
    pub currency: String,
}

/// Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ui: UiConfig,
    pub security: SecurityConfig,
    pub providers: HashMap<String, ProviderConfig>,
    pub channels: HashMap<String, ChannelConfig>,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub language: String,
    pub web_enabled: bool,
    pub web_port: u16,
    pub cosmic_enabled: bool,
    pub auto_start: bool,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub emergency_stop_enabled: bool,
    pub approval_required: bool,
    pub session_timeout_minutes: u32,
    pub max_concurrent_sessions: u32,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub enabled: bool,
    pub api_key_set: bool,
    pub models: Vec<String>,
    pub settings: HashMap<String, serde_json::Value>,
}

/// Channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub name: String,
    pub enabled: bool,
    pub channel_type: String,
    pub settings: HashMap<String, serde_json::Value>,
}

/// Activity log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub activity_type: ActivityType,
    pub description: String,
    pub session_id: Option<String>,
    pub user: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Activity type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActivityType {
    SessionCreated,
    MessageSent,
    EmergencyStop,
    ConfigUpdated,
    ProviderAdded,
    ChannelConnected,
    Error,
    Warning,
    Info,
}

/// Quick action for the dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAction {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub action_type: QuickActionType,
    pub target: Option<String>,
}

/// Quick action type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuickActionType {
    CreateSession,
    OpenSettings,
    OpenSecurity,
    ViewLogs,
    Navigate,
    Command,
}

/// Dashboard statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_sessions: u32,
    pub active_sessions: u32,
    pub total_messages: u32,
    pub messages_today: u32,
    pub uptime_hours: f64,
    pub error_count_24h: u32,
    pub last_activity: Option<DateTime<Utc>>,
}

/// Theme information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub is_dark: bool,
    pub colors: ThemeColors,
}

/// Theme color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub surface: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub accent: String,
    pub error: String,
    pub warning: String,
    pub success: String,
}

/// Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub timestamp: DateTime<Utc>,
    pub read: bool,
    pub actions: Vec<NotificationAction>,
}

/// Notification type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
    Emergency,
}

/// Notification action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationAction {
    pub id: String,
    pub label: String,
    pub action: String,
    pub style: NotificationActionStyle,
}

/// Notification action style
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationActionStyle {
    Primary,
    Secondary,
    Danger,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}

impl Session {
    /// Create a new session
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            created_at: now,
            updated_at: now,
            message_count: 0,
            is_active: true,
            channel: None,
            metadata: HashMap::new(),
        }
    }

    /// Get formatted duration since creation
    pub fn duration(&self) -> chrono::Duration {
        Utc::now() - self.created_at
    }

    /// Get formatted duration as string
    pub fn duration_string(&self) -> String {
        let duration = self.duration();
        let total_minutes = duration.num_minutes();

        if total_minutes < 60 {
            format!("{}m", total_minutes)
        } else if total_minutes < 1440 {
            format!("{}h {}m", total_minutes / 60, total_minutes % 60)
        } else {
            format!("{}d {}h", total_minutes / 1440, (total_minutes % 1440) / 60)
        }
    }
}

impl Message {
    /// Create a new message
    pub fn new(session_id: String, role: MessageRole, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            role,
            content,
            created_at: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Get formatted timestamp
    pub fn timestamp(&self) -> String {
        self.created_at.format("%H:%M:%S").to_string()
    }
}

impl MemoryUsage {
    /// Get memory usage percentage
    pub fn usage_percentage(&self) -> f64 {
        if self.total_mb == 0 {
            0.0
        } else {
            (self.used_mb as f64 / self.total_mb as f64) * 100.0
        }
    }
}

impl SystemStatus {
    /// Check if system is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.connection_status, ConnectionStatus::Connected)
            && self.memory_usage.usage_percentage() < 90.0
    }

    /// Get uptime as formatted string
    pub fn uptime_string(&self) -> String {
        let hours = self.uptime_seconds / 3600;
        let minutes = (self.uptime_seconds % 3600) / 60;

        if hours < 24 {
            format!("{}h {}m", hours, minutes)
        } else {
            let days = hours / 24;
            let remaining_hours = hours % 24;
            format!("{}d {}h", days, remaining_hours)
        }
    }
}
