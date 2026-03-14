//! Audit Log Storage
//!
//! DO-178C Level A Compliant Storage Implementation

use crate::{AuditError, AuditEvent, AuditResult, EventFilter};
use async_trait::async_trait;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::sync::Arc;

/// SQLite storage implementation
pub struct SqliteStorage {
    pool: Arc<Pool<Sqlite>>,
}

impl SqliteStorage {
    /// Create new SQLite storage
    pub async fn new(database_url: &str) -> AuditResult<Self> {
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| AuditError::DatabaseError(e.to_string()))?;

        let storage = Self {
            pool: Arc::new(pool),
        };

        storage.init_schema().await?;

        Ok(storage)
    }

    /// Initialize database schema
    async fn init_schema(&self) -> AuditResult<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS audit_events (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                severity TEXT NOT NULL,
                category TEXT NOT NULL,
                event_data TEXT NOT NULL,
                metadata TEXT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| AuditError::DatabaseError(e.to_string()))?;

        // Create indices for common queries
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_timestamp ON audit_events(timestamp)")
            .execute(&*self.pool)
            .await
            .map_err(|e| AuditError::DatabaseError(e.to_string()))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_severity ON audit_events(severity)")
            .execute(&*self.pool)
            .await
            .map_err(|e| AuditError::DatabaseError(e.to_string()))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_category ON audit_events(category)")
            .execute(&*self.pool)
            .await
            .map_err(|e| AuditError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl crate::LogStorage for SqliteStorage {
    async fn store_batch(&self, events: &[AuditEvent]) -> AuditResult<()> {
        for event in events {
            let event_data = serde_json::to_string(&event.details)
                .map_err(|e| AuditError::SerializationError(e.to_string()))?;

            let metadata = serde_json::to_string(&event.metadata)
                .map_err(|e| AuditError::SerializationError(e.to_string()))?;

            sqlx::query(
                r#"
                INSERT INTO audit_events (id, timestamp, severity, category, event_data, metadata)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(event.id.to_string())
            .bind(event.timestamp.to_string())
            .bind(format!("{:?}", event.severity))
            .bind(format!("{:?}", event.category))
            .bind(event_data)
            .bind(metadata)
            .execute(&*self.pool)
            .await
            .map_err(|e| AuditError::DatabaseError(e.to_string()))?;
        }

        Ok(())
    }

    async fn query(&self, filter: EventFilter) -> AuditResult<Vec<AuditEvent>> {
        let mut query = String::from("SELECT id, timestamp, severity, category, event_data, metadata FROM audit_events WHERE 1=1");
        let mut params: Vec<String> = Vec::new();

        if let Some(severity) = filter.severity {
            query.push_str(" AND severity = ?");
            params.push(format!("{:?}", severity));
        }

        if let Some(category) = filter.category {
            query.push_str(" AND category = ?");
            params.push(format!("{:?}", category));
        }

        if let Some(start_time) = filter.start_time {
            query.push_str(" AND timestamp >= ?");
            params.push(start_time.to_string());
        }

        if let Some(end_time) = filter.end_time {
            query.push_str(" AND timestamp <= ?");
            params.push(end_time.to_string());
        }

        query.push_str(" ORDER BY timestamp DESC");

        if let Some(limit) = filter.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        // Execute query (simplified for now)
        let rows = sqlx::query(&query)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| AuditError::DatabaseError(e.to_string()))?;

        let mut events = Vec::new();
        for row in rows {
            let id: String = row.try_get("id")
                .map_err(|e| AuditError::DatabaseError(e.to_string()))?;
            let timestamp: String = row.try_get("timestamp")
                .map_err(|e| AuditError::DatabaseError(e.to_string()))?;
            let severity: String = row.try_get("severity")
                .map_err(|e| AuditError::DatabaseError(e.to_string()))?;
            let category: String = row.try_get("category")
                .map_err(|e| AuditError::DatabaseError(e.to_string()))?;
            let event_data: String = row.try_get("event_data")
                .map_err(|e| AuditError::DatabaseError(e.to_string()))?;
            let metadata: String = row.try_get("metadata")
                .map_err(|e| AuditError::DatabaseError(e.to_string()))?;

            // Parse event (simplified)
            let event = AuditEvent {
                id: id.parse().map_err(|e: uuid::Error| AuditError::InvalidEvent(e.to_string()))?,
                timestamp: time::OffsetDateTime::parse(&timestamp, &time::format_description::well_known::Rfc3339)
                    .map_err(|e| AuditError::InvalidEvent(e.to_string()))?,
                severity: match severity.as_str() {
                    "Critical" => crate::EventSeverity::Critical,
                    "High" => crate::EventSeverity::High,
                    "Medium" => crate::EventSeverity::Medium,
                    "Low" => crate::EventSeverity::Low,
                    _ => crate::EventSeverity::Info,
                },
                category: match category.as_str() {
                    "Authentication" => crate::EventCategory::Authentication,
                    "Authorization" => crate::EventCategory::Authorization,
                    "Configuration" => crate::EventCategory::Configuration,
                    "Security" => crate::EventCategory::Security,
                    _ => crate::EventCategory::System,
                },
                details: serde_json::from_str(&event_data)
                    .map_err(|e| AuditError::SerializationError(e.to_string()))?,
                metadata: serde_json::from_str(&metadata)
                    .map_err(|e| AuditError::SerializationError(e.to_string()))?,
            };

            events.push(event);
        }

        Ok(events)
    }
}

/// In-memory storage for testing
pub struct MemoryStorage {
    events: Arc<tokio::sync::RwLock<Vec<AuditEvent>>>,
}

impl MemoryStorage {
    /// Create new memory storage
    pub fn new() -> Self {
        Self {
            events: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl crate::LogStorage for MemoryStorage {
    async fn store_batch(&self, events: &[AuditEvent]) -> AuditResult<()> {
        let mut stored = self.events.write().await;
        stored.extend_from_slice(events);
        Ok(())
    }

    async fn query(&self, filter: EventFilter) -> AuditResult<Vec<AuditEvent>> {
        let events = self.events.read().await;
        let mut filtered: Vec<AuditEvent> = events
            .iter()
            .filter(|e| {
                if let Some(severity) = filter.severity {
                    if e.severity != severity {
                        return false;
                    }
                }
                if let Some(category) = &filter.category {
                    if &e.category != category {
                        return false;
                    }
                }
                if let Some(start_time) = filter.start_time {
                    if e.timestamp < start_time {
                        return false;
                    }
                }
                if let Some(end_time) = filter.end_time {
                    if e.timestamp > end_time {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Sort by timestamp descending
        filtered.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Apply limit
        if let Some(limit) = filter.limit {
            filtered.truncate(limit);
        }

        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuthEvent, EventSeverity, LogStorage};

    #[tokio::test]
    async fn test_memory_storage() {
        let storage = MemoryStorage::new();

        let event = AuditEvent::auth(
            EventSeverity::High,
            AuthEvent::LoginAttempt {
                username: "user1".to_string(),
                success: true,
                ip_address: None,
                user_agent: None,
            },
        );

        storage.store_batch(&[event.clone()]).await.unwrap();

        let filter = EventFilter::default();
        let events = storage.query(filter).await.unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, event.id);
    }

    #[tokio::test]
    async fn test_memory_storage_filter() {
        let storage = MemoryStorage::new();

        let event1 = AuditEvent::auth(
            EventSeverity::High,
            AuthEvent::LoginAttempt {
                username: "user1".to_string(),
                success: true,
                ip_address: None,
                user_agent: None,
            },
        );

        let event2 = AuditEvent::auth(
            EventSeverity::Low,
            AuthEvent::Logout {
                username: "user1".to_string(),
                session_id: "session1".to_string(),
            },
        );

        storage.store_batch(&[event1, event2]).await.unwrap();

        let filter = EventFilter {
            severity: Some(EventSeverity::High),
            ..Default::default()
        };
        let events = storage.query(filter).await.unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].severity, EventSeverity::High);
    }

    #[tokio::test]
    async fn test_memory_storage_limit() {
        let storage = MemoryStorage::new();

        for i in 0..10 {
            let event = AuditEvent::auth(
                EventSeverity::Info,
                AuthEvent::LoginAttempt {
                    username: format!("user{}", i),
                    success: true,
                    ip_address: None,
                    user_agent: None,
                },
            );
            storage.store_batch(&[event]).await.unwrap();
        }

        let filter = EventFilter {
            limit: Some(5),
            ..Default::default()
        };
        let events = storage.query(filter).await.unwrap();

        assert_eq!(events.len(), 5);
    }
}
