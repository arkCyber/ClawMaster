//! Audit Logger
//!
//! DO-178C Level A Compliant Audit Logger

use crate::{AuditEvent, AuditResult, EventSeverity};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Audit logger configuration
#[derive(Debug, Clone)]
pub struct AuditLoggerConfig {
    /// Minimum severity to log
    pub min_severity: EventSeverity,
    
    /// Enable log signing
    pub enable_signing: bool,
    
    /// Buffer size before flush
    pub buffer_size: usize,
}

impl Default for AuditLoggerConfig {
    fn default() -> Self {
        Self {
            min_severity: EventSeverity::Info,
            enable_signing: true,
            buffer_size: 100,
        }
    }
}

/// Audit logger
///
/// DO-178C §11.9: Audit logging system
pub struct AuditLogger {
    config: AuditLoggerConfig,
    storage: Arc<dyn LogStorage>,
    signer: Option<Arc<dyn LogSigner>>,
    buffer: Arc<RwLock<Vec<AuditEvent>>>,
}

impl AuditLogger {
    /// Create new audit logger
    pub fn new(
        config: AuditLoggerConfig,
        storage: Arc<dyn LogStorage>,
        signer: Option<Arc<dyn LogSigner>>,
    ) -> Self {
        Self {
            config,
            storage,
            signer,
            buffer: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Log an audit event
    ///
    /// DO-178C §11.9: Event logging
    pub async fn log(&self, event: AuditEvent) -> AuditResult<()> {
        // Check severity threshold
        if !self.should_log(&event) {
            return Ok(());
        }

        // Sign event if enabled
        let signed_event = if self.config.enable_signing {
            if let Some(signer) = &self.signer {
                let signature = signer.sign(&event).await?;
                let mut event = event;
                event.metadata.as_object_mut()
                    .unwrap()
                    .insert("signature".to_string(), serde_json::json!(signature));
                event
            } else {
                event
            }
        } else {
            event
        };

        // Add to buffer
        let mut buffer = self.buffer.write().await;
        buffer.push(signed_event);

        // Flush if buffer is full
        if buffer.len() >= self.config.buffer_size {
            self.flush_buffer(&mut buffer).await?;
        }

        Ok(())
    }

    /// Flush buffered events to storage
    pub async fn flush(&self) -> AuditResult<()> {
        let mut buffer = self.buffer.write().await;
        self.flush_buffer(&mut buffer).await
    }

    /// Check if event should be logged
    fn should_log(&self, event: &AuditEvent) -> bool {
        self.severity_level(event.severity) >= self.severity_level(self.config.min_severity)
    }

    /// Get severity level as number
    fn severity_level(&self, severity: EventSeverity) -> u8 {
        match severity {
            EventSeverity::Critical => 5,
            EventSeverity::High => 4,
            EventSeverity::Medium => 3,
            EventSeverity::Low => 2,
            EventSeverity::Info => 1,
        }
    }

    /// Flush buffer to storage
    async fn flush_buffer(&self, buffer: &mut Vec<AuditEvent>) -> AuditResult<()> {
        if buffer.is_empty() {
            return Ok(());
        }

        // Store events
        self.storage.store_batch(buffer).await?;

        // Clear buffer
        buffer.clear();

        Ok(())
    }

    /// Query events
    pub async fn query(&self, filter: EventFilter) -> AuditResult<Vec<AuditEvent>> {
        self.storage.query(filter).await
    }

    /// Verify event signature
    pub async fn verify(&self, event: &AuditEvent) -> AuditResult<bool> {
        if let Some(signer) = &self.signer {
            signer.verify(event).await
        } else {
            Ok(true) // No signer, consider valid
        }
    }
}

/// Event filter for queries
#[derive(Debug, Clone, Default)]
pub struct EventFilter {
    /// Filter by severity
    pub severity: Option<EventSeverity>,
    
    /// Filter by category
    pub category: Option<crate::EventCategory>,
    
    /// Filter by username
    pub username: Option<String>,
    
    /// Start time
    pub start_time: Option<time::OffsetDateTime>,
    
    /// End time
    pub end_time: Option<time::OffsetDateTime>,
    
    /// Maximum results
    pub limit: Option<usize>,
}

/// Log storage trait
#[async_trait]
pub trait LogStorage: Send + Sync {
    /// Store a batch of events
    async fn store_batch(&self, events: &[AuditEvent]) -> AuditResult<()>;
    
    /// Query events
    async fn query(&self, filter: EventFilter) -> AuditResult<Vec<AuditEvent>>;
}

/// Log signer trait
#[async_trait]
pub trait LogSigner: Send + Sync {
    /// Sign an event
    async fn sign(&self, event: &AuditEvent) -> AuditResult<String>;
    
    /// Verify event signature
    async fn verify(&self, event: &AuditEvent) -> AuditResult<bool>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuthEvent, EventCategory};

    struct MockStorage {
        events: Arc<RwLock<Vec<AuditEvent>>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                events: Arc::new(RwLock::new(Vec::new())),
            }
        }
    }

    #[async_trait]
    impl LogStorage for MockStorage {
        async fn store_batch(&self, events: &[AuditEvent]) -> AuditResult<()> {
            let mut stored = self.events.write().await;
            stored.extend_from_slice(events);
            Ok(())
        }

        async fn query(&self, _filter: EventFilter) -> AuditResult<Vec<AuditEvent>> {
            Ok(self.events.read().await.clone())
        }
    }

    #[tokio::test]
    async fn test_logger_basic() {
        let storage = Arc::new(MockStorage::new());
        let config = AuditLoggerConfig {
            min_severity: EventSeverity::Info,
            enable_signing: false,
            buffer_size: 10,
        };
        let logger = AuditLogger::new(config, storage.clone(), None);

        let event = AuditEvent::auth(
            EventSeverity::High,
            AuthEvent::LoginAttempt {
                username: "user1".to_string(),
                success: true,
                ip_address: None,
                user_agent: None,
            },
        );

        logger.log(event).await.unwrap();
        logger.flush().await.unwrap();

        let stored = storage.events.read().await;
        assert_eq!(stored.len(), 1);
    }

    #[tokio::test]
    async fn test_logger_severity_filter() {
        let storage = Arc::new(MockStorage::new());
        let config = AuditLoggerConfig {
            min_severity: EventSeverity::High,
            enable_signing: false,
            buffer_size: 10,
        };
        let logger = AuditLogger::new(config, storage.clone(), None);

        // This should be logged
        let event1 = AuditEvent::auth(
            EventSeverity::Critical,
            AuthEvent::LoginAttempt {
                username: "user1".to_string(),
                success: true,
                ip_address: None,
                user_agent: None,
            },
        );

        // This should be filtered out
        let event2 = AuditEvent::auth(
            EventSeverity::Info,
            AuthEvent::Logout {
                username: "user1".to_string(),
                session_id: "session1".to_string(),
            },
        );

        logger.log(event1).await.unwrap();
        logger.log(event2).await.unwrap();
        logger.flush().await.unwrap();

        let stored = storage.events.read().await;
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].severity, EventSeverity::Critical);
    }

    #[tokio::test]
    async fn test_logger_buffer_flush() {
        let storage = Arc::new(MockStorage::new());
        let config = AuditLoggerConfig {
            min_severity: EventSeverity::Info,
            enable_signing: false,
            buffer_size: 2,
        };
        let logger = AuditLogger::new(config, storage.clone(), None);

        // Add 3 events, should auto-flush after 2
        for i in 0..3 {
            let event = AuditEvent::auth(
                EventSeverity::Info,
                AuthEvent::LoginAttempt {
                    username: format!("user{}", i),
                    success: true,
                    ip_address: None,
                    user_agent: None,
                },
            );
            logger.log(event).await.unwrap();
        }

        // Should have flushed 2, 1 still in buffer
        let stored = storage.events.read().await;
        assert_eq!(stored.len(), 2);

        drop(stored);
        logger.flush().await.unwrap();

        let stored = storage.events.read().await;
        assert_eq!(stored.len(), 3);
    }
}
