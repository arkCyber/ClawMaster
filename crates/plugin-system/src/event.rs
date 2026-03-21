//! Event bus for plugin communication

use {
    anyhow::Result,
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, sync::Arc},
    tokio::sync::RwLock,
};

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    /// Plugin loaded
    PluginLoaded { plugin_id: String },
    /// Plugin enabled
    PluginEnabled { plugin_id: String },
    /// Plugin disabled
    PluginDisabled { plugin_id: String },
    /// Plugin unloaded
    PluginUnloaded { plugin_id: String },
    /// Configuration changed
    ConfigChanged {
        plugin_id: String,
        config: serde_json::Value,
    },
    /// Custom event
    Custom {
        event_type: String,
        data: serde_json::Value,
    },
}

impl Event {
    /// Get event type as string
    pub fn event_type(&self) -> &str {
        match self {
            Event::PluginLoaded { .. } => "plugin_loaded",
            Event::PluginEnabled { .. } => "plugin_enabled",
            Event::PluginDisabled { .. } => "plugin_disabled",
            Event::PluginUnloaded { .. } => "plugin_unloaded",
            Event::ConfigChanged { .. } => "config_changed",
            Event::Custom { event_type, .. } => event_type,
        }
    }
}

/// Event handler function type
pub type EventHandler = Box<dyn Fn(Event) -> Result<()> + Send + Sync>;

/// Event bus for plugin communication
pub struct EventBus {
    subscribers: Arc<RwLock<HashMap<String, Vec<EventHandler>>>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Subscribe to an event type
    pub async fn subscribe(&self, event_type: &str, handler: EventHandler) -> Result<()> {
        let mut subscribers = self.subscribers.write().await;

        subscribers
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(handler);

        tracing::debug!(event_type = %event_type, "event handler subscribed");
        Ok(())
    }

    /// Emit an event
    pub async fn emit(&self, event: Event) -> Result<()> {
        let subscribers = self.subscribers.read().await;
        let event_type = event.event_type();

        tracing::debug!(event_type = %event_type, "emitting event");

        // Get handlers for this event type
        if let Some(handlers) = subscribers.get(event_type) {
            for handler in handlers {
                if let Err(e) = handler(event.clone()) {
                    tracing::error!(
                        event_type = %event_type,
                        error = %e,
                        "event handler failed"
                    );
                }
            }
        }

        // Also notify wildcard subscribers
        if let Some(handlers) = subscribers.get("*") {
            for handler in handlers {
                if let Err(e) = handler(event.clone()) {
                    tracing::error!(
                        event_type = %event_type,
                        error = %e,
                        "wildcard event handler failed"
                    );
                }
            }
        }

        Ok(())
    }

    /// Unsubscribe from all events (clear all handlers)
    pub async fn clear(&self) {
        let mut subscribers = self.subscribers.write().await;
        subscribers.clear();
        tracing::debug!("all event handlers cleared");
    }

    /// Get subscriber count for an event type
    pub async fn subscriber_count(&self, event_type: &str) -> usize {
        let subscribers = self.subscribers.read().await;
        subscribers.get(event_type).map(|v| v.len()).unwrap_or(0)
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::sync::atomic::{AtomicUsize, Ordering},
    };

    #[tokio::test]
    async fn test_event_bus_subscribe_and_emit() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        bus.subscribe(
            "plugin_loaded",
            Box::new(move |_event| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }),
        )
        .await
        .unwrap();

        bus.emit(Event::PluginLoaded {
            plugin_id: "test-plugin".to_string(),
        })
        .await
        .unwrap();

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_event_bus_multiple_subscribers() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..3 {
            let counter_clone = counter.clone();
            bus.subscribe(
                "plugin_enabled",
                Box::new(move |_event| {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }),
            )
            .await
            .unwrap();
        }

        bus.emit(Event::PluginEnabled {
            plugin_id: "test-plugin".to_string(),
        })
        .await
        .unwrap();

        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_event_bus_wildcard() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        bus.subscribe(
            "*",
            Box::new(move |_event| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }),
        )
        .await
        .unwrap();

        bus.emit(Event::PluginLoaded {
            plugin_id: "test1".to_string(),
        })
        .await
        .unwrap();

        bus.emit(Event::PluginEnabled {
            plugin_id: "test2".to_string(),
        })
        .await
        .unwrap();

        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_event_bus_clear() {
        let bus = EventBus::new();

        bus.subscribe("plugin_loaded", Box::new(|_| Ok(())))
            .await
            .unwrap();
        assert_eq!(bus.subscriber_count("plugin_loaded").await, 1);

        bus.clear().await;
        assert_eq!(bus.subscriber_count("plugin_loaded").await, 0);
    }

    #[test]
    fn test_event_type() {
        let event = Event::PluginLoaded {
            plugin_id: "test".to_string(),
        };
        assert_eq!(event.event_type(), "plugin_loaded");

        let event = Event::Custom {
            event_type: "custom_event".to_string(),
            data: serde_json::json!({}),
        };
        assert_eq!(event.event_type(), "custom_event");
    }
}
