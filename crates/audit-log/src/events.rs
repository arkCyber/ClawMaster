//! Audit Event Definitions
//!
//! DO-178C Level A Compliant Event Types

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// Event severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventSeverity {
    /// Critical security event
    Critical,
    
    /// High priority event
    High,
    
    /// Medium priority event
    Medium,
    
    /// Low priority event
    Low,
    
    /// Informational event
    Info,
}

/// Authentication event type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthEvent {
    /// User login attempt
    LoginAttempt {
        username: String,
        success: bool,
        ip_address: Option<String>,
        user_agent: Option<String>,
    },
    
    /// User logout
    Logout {
        username: String,
        session_id: String,
    },
    
    /// Password change
    PasswordChange {
        username: String,
        success: bool,
    },
    
    /// Failed authentication
    AuthenticationFailed {
        username: String,
        reason: String,
        ip_address: Option<String>,
    },
    
    /// Session expired
    SessionExpired {
        username: String,
        session_id: String,
    },
}

/// Authorization event type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthzEvent {
    /// Access granted
    AccessGranted {
        username: String,
        resource: String,
        action: String,
    },
    
    /// Access denied
    AccessDenied {
        username: String,
        resource: String,
        action: String,
        reason: String,
    },
    
    /// Permission change
    PermissionChange {
        username: String,
        resource: String,
        old_permissions: Vec<String>,
        new_permissions: Vec<String>,
    },
    
    /// Role assignment
    RoleAssignment {
        username: String,
        role: String,
        assigned_by: String,
    },
}

/// Configuration change event
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigChangeEvent {
    /// Configuration key
    pub key: String,
    
    /// Old value (redacted if sensitive)
    pub old_value: Option<String>,
    
    /// New value (redacted if sensitive)
    pub new_value: Option<String>,
    
    /// User who made the change
    pub changed_by: String,
    
    /// Change reason
    pub reason: Option<String>,
}

/// Security event type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityEvent {
    /// Suspicious activity detected
    SuspiciousActivity {
        description: String,
        source_ip: Option<String>,
        user: Option<String>,
    },
    
    /// Rate limit exceeded
    RateLimitExceeded {
        user: String,
        endpoint: String,
        limit: usize,
    },
    
    /// Invalid input detected
    InvalidInput {
        input_type: String,
        threat_type: String,
        user: Option<String>,
    },
    
    /// Security policy violation
    PolicyViolation {
        policy: String,
        user: String,
        details: String,
    },
    
    /// Encryption key rotation
    KeyRotation {
        key_type: String,
        rotated_by: String,
    },
}

/// System event type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemEvent {
    /// Service started
    ServiceStarted {
        service_name: String,
        version: String,
    },
    
    /// Service stopped
    ServiceStopped {
        service_name: String,
        reason: String,
    },
    
    /// Health check failed
    HealthCheckFailed {
        check_name: String,
        details: String,
    },
    
    /// Resource quota exceeded
    QuotaExceeded {
        resource_type: String,
        current: usize,
        limit: usize,
    },
    
    /// Database migration
    DatabaseMigration {
        version: String,
        success: bool,
    },
    
    /// Backup completed
    BackupCompleted {
        backup_id: String,
        size_bytes: usize,
    },
}

/// Audit event
///
/// DO-178C §11.9: Comprehensive audit event
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique event ID
    pub id: Uuid,
    
    /// Event timestamp
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    
    /// Event severity
    pub severity: EventSeverity,
    
    /// Event category
    pub category: EventCategory,
    
    /// Event details
    pub details: EventDetails,
    
    /// Additional metadata
    pub metadata: serde_json::Value,
}

/// Event category
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventCategory {
    Authentication,
    Authorization,
    Configuration,
    Security,
    System,
}

/// Event details
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventDetails {
    Auth(AuthEvent),
    Authz(AuthzEvent),
    ConfigChange(ConfigChangeEvent),
    Security(SecurityEvent),
    System(SystemEvent),
}

impl AuditEvent {
    /// Create new audit event
    pub fn new(severity: EventSeverity, category: EventCategory, details: EventDetails) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: OffsetDateTime::now_utc(),
            severity,
            category,
            details,
            metadata: serde_json::json!({}),
        }
    }
    
    /// Create authentication event
    pub fn auth(severity: EventSeverity, event: AuthEvent) -> Self {
        Self::new(severity, EventCategory::Authentication, EventDetails::Auth(event))
    }
    
    /// Create authorization event
    pub fn authz(severity: EventSeverity, event: AuthzEvent) -> Self {
        Self::new(severity, EventCategory::Authorization, EventDetails::Authz(event))
    }
    
    /// Create configuration change event
    pub fn config_change(severity: EventSeverity, event: ConfigChangeEvent) -> Self {
        Self::new(severity, EventCategory::Configuration, EventDetails::ConfigChange(event))
    }
    
    /// Create security event
    pub fn security(severity: EventSeverity, event: SecurityEvent) -> Self {
        Self::new(severity, EventCategory::Security, EventDetails::Security(event))
    }
    
    /// Create system event
    pub fn system(severity: EventSeverity, event: SystemEvent) -> Self {
        Self::new(severity, EventCategory::System, EventDetails::System(event))
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        if let Some(obj) = self.metadata.as_object_mut() {
            obj.insert(key.to_string(), value);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_event_creation() {
        let event = AuditEvent::auth(
            EventSeverity::High,
            AuthEvent::LoginAttempt {
                username: "user1".to_string(),
                success: true,
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: None,
            },
        );

        assert_eq!(event.severity, EventSeverity::High);
        assert_eq!(event.category, EventCategory::Authentication);
    }

    #[test]
    fn test_security_event_creation() {
        let event = AuditEvent::security(
            EventSeverity::Critical,
            SecurityEvent::RateLimitExceeded {
                user: "user1".to_string(),
                endpoint: "/api/data".to_string(),
                limit: 100,
            },
        );

        assert_eq!(event.severity, EventSeverity::Critical);
        assert_eq!(event.category, EventCategory::Security);
    }

    #[test]
    fn test_event_serialization() {
        let event = AuditEvent::system(
            EventSeverity::Info,
            SystemEvent::ServiceStarted {
                service_name: "gateway".to_string(),
                version: "0.10.18".to_string(),
            },
        );

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("ServiceStarted"));
        assert!(json.contains("gateway"));
    }

    #[test]
    fn test_event_with_metadata() {
        let event = AuditEvent::auth(
            EventSeverity::Medium,
            AuthEvent::Logout {
                username: "user1".to_string(),
                session_id: "session123".to_string(),
            },
        )
        .with_metadata("client_version", serde_json::json!("1.0.0"));

        assert!(event.metadata.get("client_version").is_some());
    }
}
