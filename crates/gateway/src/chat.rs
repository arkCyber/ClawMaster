//! Chat execution engine — re-exported from [`clawmaster_chat`] with the gateway
//! runtime adapter.

pub use clawmaster_chat::*;

use std::sync::Arc;

use {async_trait::async_trait, serde_json::Value};

use {clawmaster_channels::ChannelReplyTarget, clawmaster_tools::sandbox::SandboxRouter};

use crate::{
    event_streams::{LlmEvent, LogLevel, SystemEvent, TokenUsage, ToolEvent, ToolStatus},
    state::GatewayState,
};

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

fn structured_stream_events(
    state: &GatewayState,
    topic: &str,
    payload: &Value,
) -> Vec<(&'static str, Value)> {
    if topic != "chat" {
        return Vec::new();
    }

    let Some(chat_state) = payload.get("state").and_then(Value::as_str) else {
        return Vec::new();
    };

    let timestamp = chrono::Utc::now().timestamp_millis();

    match chat_state {
        "tool_call_start" => {
            let event = ToolEvent {
                tool_name: payload
                    .get("toolName")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown")
                    .to_string(),
                status: ToolStatus::Started,
                arguments: payload.get("arguments").cloned(),
                result: None,
                error: None,
                duration_ms: payload.get("durationMs").and_then(Value::as_u64),
                timestamp,
            };
            state.event_router.emit_tool(event.clone());
            serde_json::to_value(event)
                .ok()
                .map(|value| vec![("stream.tool", value)])
                .unwrap_or_default()
        },
        "tool_call_end" => {
            let success = payload
                .get("success")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let event = ToolEvent {
                tool_name: payload
                    .get("toolName")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown")
                    .to_string(),
                status: if success {
                    ToolStatus::Completed
                } else {
                    ToolStatus::Failed
                },
                arguments: None,
                result: payload.get("result").cloned(),
                error: payload.get("error").map(value_to_string),
                duration_ms: payload.get("durationMs").and_then(Value::as_u64),
                timestamp,
            };
            state.event_router.emit_tool(event.clone());
            serde_json::to_value(event)
                .ok()
                .map(|value| vec![("stream.tool", value)])
                .unwrap_or_default()
        },
        "delta" => {
            let event = LlmEvent {
                content: payload
                    .get("text")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                is_final: false,
                finish_reason: None,
                token_usage: None,
                timestamp,
            };
            state.event_router.emit_llm(event.clone());
            serde_json::to_value(event)
                .ok()
                .map(|value| vec![("stream.llm", value)])
                .unwrap_or_default()
        },
        "final" => {
            let token_usage = match (
                payload.get("inputTokens").and_then(Value::as_u64),
                payload.get("outputTokens").and_then(Value::as_u64),
            ) {
                (Some(prompt_tokens), Some(completion_tokens)) => Some(TokenUsage {
                    prompt_tokens: prompt_tokens as u32,
                    completion_tokens: completion_tokens as u32,
                    total_tokens: prompt_tokens.saturating_add(completion_tokens) as u32,
                }),
                _ => None,
            };
            let event = LlmEvent {
                content: payload
                    .get("text")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                is_final: true,
                finish_reason: Some("stop".to_string()),
                token_usage,
                timestamp,
            };
            state.event_router.emit_llm(event.clone());
            serde_json::to_value(event)
                .ok()
                .map(|value| vec![("stream.llm", value)])
                .unwrap_or_default()
        },
        "thinking" | "retrying" | "error" => {
            let event = SystemEvent {
                level: match chat_state {
                    "thinking" => LogLevel::Debug,
                    "retrying" => LogLevel::Warning,
                    "error" => LogLevel::Error,
                    _ => LogLevel::Info,
                },
                message: payload
                    .get("error")
                    .map(value_to_string)
                    .unwrap_or_else(|| chat_state.to_string()),
                context: Some(payload.clone()),
                timestamp,
            };
            state.event_router.emit_system(event.clone());
            serde_json::to_value(event)
                .ok()
                .map(|value| vec![("stream.system", value)])
                .unwrap_or_default()
        },
        _ => Vec::new(),
    }
}

// ── GatewayChatRuntime ──────────────────────────────────────────────────────

/// Adapts [`GatewayState`] to the [`ChatRuntime`] trait expected by
/// `moltis-chat`.
pub struct GatewayChatRuntime {
    state: Arc<GatewayState>,
}

impl GatewayChatRuntime {
    pub fn from_state(state: Arc<GatewayState>) -> Arc<dyn ChatRuntime> {
        Arc::new(Self { state })
    }
}

#[async_trait]
impl ChatRuntime for GatewayChatRuntime {
    // ── Broadcasting ────────────────────────────────────────────────────────

    async fn broadcast(&self, topic: &str, payload: Value) {
        let structured_events = structured_stream_events(&self.state, topic, &payload);
        crate::broadcast::broadcast(
            &self.state,
            topic,
            payload,
            crate::broadcast::BroadcastOpts::default(),
        )
        .await;
        for (event, payload) in structured_events {
            crate::broadcast::broadcast(
                &self.state,
                event,
                payload,
                crate::broadcast::BroadcastOpts::default(),
            )
            .await;
        }
    }

    // ── Channel reply queue ─────────────────────────────────────────────────

    async fn push_channel_reply(&self, session_key: &str, target: ChannelReplyTarget) {
        self.state.push_channel_reply(session_key, target).await;
    }

    async fn drain_channel_replies(&self, session_key: &str) -> Vec<ChannelReplyTarget> {
        self.state.drain_channel_replies(session_key).await
    }

    async fn peek_channel_replies(&self, session_key: &str) -> Vec<ChannelReplyTarget> {
        self.state.peek_channel_replies(session_key).await
    }

    // ── Channel status log ──────────────────────────────────────────────────

    async fn push_channel_status_log(&self, session_key: &str, message: String) {
        self.state
            .push_channel_status_log(session_key, message)
            .await;
    }

    async fn drain_channel_status_log(&self, session_key: &str) -> Vec<String> {
        self.state.drain_channel_status_log(session_key).await
    }

    // ── Run error tracking ──────────────────────────────────────────────────

    async fn set_run_error(&self, run_id: &str, error: String) {
        self.state.set_run_error(run_id, error).await;
    }

    // ── Connection → session/project mapping ────────────────────────────────

    async fn active_session_key(&self, conn_id: &str) -> Option<String> {
        self.state
            .inner
            .read()
            .await
            .active_sessions
            .get(conn_id)
            .cloned()
    }

    async fn active_project_id(&self, conn_id: &str) -> Option<String> {
        self.state
            .inner
            .read()
            .await
            .active_projects
            .get(conn_id)
            .cloned()
    }

    // ── Immutable accessors ─────────────────────────────────────────────────

    fn hostname(&self) -> &str {
        &self.state.hostname
    }

    fn sandbox_router(&self) -> Option<&Arc<SandboxRouter>> {
        self.state.sandbox_router.as_ref()
    }

    fn memory_manager(&self) -> Option<&Arc<clawmaster_memory::manager::MemoryManager>> {
        self.state.memory_manager.as_ref()
    }

    // ── Cached location ─────────────────────────────────────────────────────

    async fn cached_location(&self) -> Option<clawmaster_config::GeoLocation> {
        self.state.inner.read().await.cached_location.clone()
    }

    // ── TTS overrides ───────────────────────────────────────────────────────

    async fn tts_overrides(
        &self,
        session_key: &str,
        channel_key: &str,
    ) -> (Option<TtsOverride>, Option<TtsOverride>) {
        let inner = self.state.inner.read().await;
        let channel = inner
            .tts_channel_overrides
            .get(channel_key)
            .map(|o| TtsOverride {
                provider: o.provider.clone(),
                voice_id: o.voice_id.clone(),
                model: o.model.clone(),
            });
        let session = inner
            .tts_session_overrides
            .get(session_key)
            .map(|o| TtsOverride {
                provider: o.provider.clone(),
                voice_id: o.voice_id.clone(),
                model: o.model.clone(),
            });
        (channel, session)
    }

    // ── Services ────────────────────────────────────────────────────────────

    fn channel_outbound(&self) -> Option<Arc<dyn clawmaster_channels::ChannelOutbound>> {
        self.state.services.channel_outbound_arc()
    }

    fn channel_stream_outbound(
        &self,
    ) -> Option<Arc<dyn clawmaster_channels::ChannelStreamOutbound>> {
        self.state.services.channel_stream_outbound_arc()
    }

    fn tts_service(&self) -> Arc<dyn clawmaster_service_traits::TtsService> {
        Arc::clone(&self.state.services.tts)
    }

    fn project_service(&self) -> &dyn clawmaster_service_traits::ProjectService {
        &*self.state.services.project
    }

    fn mcp_service(&self) -> &dyn clawmaster_service_traits::McpService {
        &*self.state.services.mcp
    }

    async fn chat_service(&self) -> Arc<dyn clawmaster_service_traits::ChatService> {
        self.state.chat().await
    }

    async fn last_run_error(&self, run_id: &str) -> Option<String> {
        self.state.last_run_error(run_id).await
    }

    // ── Push notifications ──────────────────────────────────────────────────

    async fn send_push_notification(
        &self,
        title: &str,
        body: &str,
        url: Option<&str>,
        session_key: Option<&str>,
    ) -> error::Result<usize> {
        #[cfg(feature = "push-notifications")]
        {
            if let Some(push_service) = self.state.get_push_service().await {
                return crate::push_routes::send_push_notification(
                    &push_service,
                    title,
                    body,
                    url,
                    session_key,
                )
                .await
                .map_err(|source| error::Error::message(source.to_string()));
            }
        }
        let _ = (title, body, url, session_key);
        Ok(0)
    }

    // ── Local LLM ───────────────────────────────────────────────────────────

    async fn ensure_local_model_cached(&self, model_id: &str) -> error::Result<bool> {
        #[cfg(feature = "local-llm")]
        {
            return crate::local_llm_setup::ensure_local_model_cached(model_id, &self.state)
                .await
                .map_err(error::Error::message);
        }
        #[cfg(not(feature = "local-llm"))]
        {
            let _ = model_id;
            Ok(false)
        }
    }

    // ── Remote nodes ────────────────────────────────────────────────────────

    async fn connected_nodes(&self) -> Vec<runtime::ConnectedNodeSummary> {
        let inner = self.state.inner.read().await;
        inner
            .nodes
            .list()
            .iter()
            .map(|n| runtime::ConnectedNodeSummary {
                node_id: n.node_id.clone(),
                display_name: n.display_name.clone(),
                platform: n.platform.clone(),
                capabilities: n.capabilities.clone(),
                cpu_count: n.cpu_count,
                cpu_usage: n.cpu_usage,
                mem_total: n.mem_total,
                mem_available: n.mem_available,
                telemetry_stale: n
                    .last_telemetry
                    .is_some_and(|t| t.elapsed() > std::time::Duration::from_secs(120)),
                disk_total: n.disk_total,
                disk_available: n.disk_available,
                runtimes: n.runtimes.clone(),
                providers: n
                    .providers
                    .iter()
                    .map(|p| (p.provider.clone(), p.models.clone()))
                    .collect(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        auth::{AuthMode, ResolvedAuth},
        services::GatewayServices,
    };

    fn test_state() -> Arc<GatewayState> {
        GatewayState::new(
            ResolvedAuth {
                mode: AuthMode::Token,
                token: None,
                password: None,
            },
            (*GatewayServices::noop()).clone(),
        )
    }

    #[test]
    fn structured_stream_events_maps_tool_call_start() {
        let state = test_state();
        let events = structured_stream_events(
            &state,
            "chat",
            &serde_json::json!({
                "state": "tool_call_start",
                "toolName": "exec",
                "arguments": { "command": "pwd" }
            }),
        );
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0, "stream.tool");
        assert_eq!(events[0].1["tool_name"], "exec");
        assert_eq!(events[0].1["status"], "started");
    }

    #[test]
    fn structured_stream_events_maps_delta() {
        let state = test_state();
        let events = structured_stream_events(
            &state,
            "chat",
            &serde_json::json!({
                "state": "delta",
                "text": "hello"
            }),
        );
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0, "stream.llm");
        assert_eq!(events[0].1["content"], "hello");
        assert_eq!(events[0].1["is_final"], false);
    }
}
