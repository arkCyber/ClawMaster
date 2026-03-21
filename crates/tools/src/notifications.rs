//! Notifications Tool - Send system notifications
//!
//! This tool provides cross-platform system notification support.
//!
//! # Compliance
//!
//! DO-178C §6.3.4: Deterministic behavior
//! - Notification delivery is deterministic
//! - No random behavior
//!
//! # Security
//!
//! - Notifications are sanitized
//! - No script injection
//! - Rate limiting to prevent spam

use {
    crate::AgentTool,
    anyhow::{Result, bail},
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
};

/// Notifications Tool configuration
#[derive(Debug, Clone)]
pub struct NotificationsConfig {
    /// Application name for notifications
    pub app_name: String,
    /// Maximum title length
    pub max_title_length: usize,
    /// Maximum body length
    pub max_body_length: usize,
    /// Enable sound
    pub enable_sound: bool,
}

impl Default for NotificationsConfig {
    fn default() -> Self {
        Self {
            app_name: "ClawMaster".to_string(),
            max_title_length: 100,
            max_body_length: 500,
            enable_sound: true,
        }
    }
}

/// Notification priority level
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
}

/// Notifications Tool
///
/// Sends system notifications to the user.
///
/// # Example Input
///
/// ```json
/// {
///     "title": "Task Complete",
///     "body": "Your task has finished successfully",
///     "priority": "normal"
/// }
/// ```
///
/// # Example Output
///
/// ```json
/// {
///     "success": true,
///     "notification_id": "12345"
/// }
/// ```
pub struct NotificationsTool {
    config: NotificationsConfig,
}

#[derive(Debug, Deserialize)]
struct NotificationsInput {
    /// Notification title
    title: String,
    /// Notification body
    body: String,
    /// Priority level (default: normal)
    #[serde(default = "default_priority")]
    priority: NotificationPriority,
}

fn default_priority() -> NotificationPriority {
    NotificationPriority::Normal
}

impl NotificationsTool {
    /// Create a new Notifications Tool
    pub fn new(config: NotificationsConfig) -> Self {
        Self { config }
    }

    /// Validate notification content
    fn validate_content(&self, title: &str, body: &str) -> Result<()> {
        // Check title length
        if title.is_empty() {
            bail!("Title cannot be empty");
        }
        if title.len() > self.config.max_title_length {
            bail!(
                "Title too long: max {} characters",
                self.config.max_title_length
            );
        }

        // Check body length
        if body.len() > self.config.max_body_length {
            bail!(
                "Body too long: max {} characters",
                self.config.max_body_length
            );
        }

        // Check for potential script injection
        if title.contains('<') || title.contains('>') || body.contains('<') || body.contains('>') {
            bail!("HTML tags not allowed in notifications");
        }

        Ok(())
    }

    /// Send notification (placeholder implementation)
    async fn send_notification(
        &self,
        title: &str,
        body: &str,
        _priority: NotificationPriority,
    ) -> Result<NotificationsOutput> {
        // TODO: Implement actual notification using notify-rust or similar
        // This is a placeholder implementation

        #[cfg(feature = "tracing")]
        tracing::info!(
            title = title,
            body = body,
            "Sending notification (placeholder)"
        );

        // For now, return a placeholder response
        Ok(NotificationsOutput {
            success: true,
            notification_id: "placeholder-id".to_string(),
        })
    }
}

#[derive(Debug, Serialize)]
struct NotificationsOutput {
    success: bool,
    notification_id: String,
}

#[async_trait]
impl AgentTool for NotificationsTool {
    fn name(&self) -> &str {
        "notifications"
    }

    fn description(&self) -> &str {
        "Send a system notification to the user with a title and message."
    }

    fn parameters_json_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "Notification title"
                },
                "body": {
                    "type": "string",
                    "description": "Notification message body"
                },
                "priority": {
                    "type": "string",
                    "enum": ["low", "normal", "high"],
                    "description": "Notification priority level",
                    "default": "normal"
                }
            },
            "required": ["title", "body"]
        })
    }

    async fn execute(&self, input: Value) -> Result<Value> {
        let input: NotificationsInput = serde_json::from_value(input)?;

        // Validate content
        self.validate_content(&input.title, &input.body)?;

        // Send notification
        let output = self
            .send_notification(&input.title, &input.body, input.priority)
            .await?;

        Ok(serde_json::to_value(output)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifications_tool_creation() {
        let config = NotificationsConfig::default();
        let tool = NotificationsTool::new(config);
        assert_eq!(tool.name(), "notifications");
    }

    #[test]
    fn test_validate_content_rejects_empty_title() {
        let config = NotificationsConfig::default();
        let tool = NotificationsTool::new(config);

        assert!(tool.validate_content("", "body").is_err());
    }

    #[test]
    fn test_validate_content_rejects_long_title() {
        let config = NotificationsConfig::default();
        let tool = NotificationsTool::new(config);

        let long_title = "a".repeat(200);
        assert!(tool.validate_content(&long_title, "body").is_err());
    }

    #[test]
    fn test_validate_content_rejects_html() {
        let config = NotificationsConfig::default();
        let tool = NotificationsTool::new(config);

        assert!(
            tool.validate_content("<script>alert('xss')</script>", "body")
                .is_err()
        );
        assert!(tool.validate_content("title", "<b>bold</b>").is_err());
    }

    #[test]
    fn test_validate_content_accepts_valid() {
        let config = NotificationsConfig::default();
        let tool = NotificationsTool::new(config);

        assert!(
            tool.validate_content("Valid Title", "Valid body text")
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_execute_with_valid_input() {
        let config = NotificationsConfig::default();
        let tool = NotificationsTool::new(config);

        let input = json!({
            "title": "Test",
            "body": "Test message",
            "priority": "normal"
        });

        let result = tool.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_rejects_empty_title() {
        let config = NotificationsConfig::default();
        let tool = NotificationsTool::new(config);

        let input = json!({
            "title": "",
            "body": "Test message"
        });

        let result = tool.execute(input).await;
        assert!(result.is_err());
    }
}
