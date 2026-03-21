//! Local LLM provider with pluggable backends.
//!
//! Supports multiple inference backends:
//! - GGUF (llama.cpp) - Cross-platform, CPU + GPU
//! - MLX - Apple Silicon optimized (macOS only)
//!
//! The provider automatically selects the best backend based on the platform
//! and available hardware.

pub mod backend;
pub mod models;
pub mod response_parser;
pub mod system_info;

use std::{path::PathBuf, pin::Pin};

use {
    anyhow::Result,
    async_trait::async_trait,
    std::sync::{Arc, LazyLock, Weak},
    tokio::sync::{Mutex, RwLock},
    tokio_stream::Stream,
    tracing::info,
};

use clawmaster_agents::model::{ChatMessage, CompletionResponse, LlmProvider, StreamEvent};

pub use {
    backend::{BackendType, LocalBackend},
    models::{LocalModelDef, ModelFormat},
};

/// Total bytes currently held by loaded llama.cpp tensors for local GGUF
/// backends. This is updated when models are loaded/unloaded.
#[must_use]
pub fn loaded_llama_model_bytes() -> u64 {
    backend::loaded_llama_model_bytes()
}

struct ActiveGgufProvider {
    model_id: String,
    inner: Weak<RwLock<Option<Arc<dyn LocalBackend>>>>,
    selected_backend: Weak<RwLock<Option<BackendType>>>,
}

static ACTIVE_GGUF_PROVIDER: LazyLock<Mutex<Option<ActiveGgufProvider>>> =
    LazyLock::new(|| Mutex::new(None));

/// Configuration for the local LLM provider.
#[derive(Debug, Clone)]
pub struct LocalLlmConfig {
    /// Model ID from the registry.
    pub model_id: String,
    /// Direct path to a model file (skips auto-download).
    pub model_path: Option<PathBuf>,
    /// Preferred backend (auto-detected if None).
    pub backend: Option<BackendType>,
    /// Context size in tokens (default: from model definition).
    pub context_size: Option<u32>,
    /// Number of layers to offload to GPU (GGUF only, 0 = CPU only).
    pub gpu_layers: u32,
    /// Sampling temperature.
    pub temperature: f32,
    /// Directory for caching downloaded models.
    pub cache_dir: PathBuf,
}

impl Default for LocalLlmConfig {
    fn default() -> Self {
        Self {
            model_id: String::new(),
            model_path: None,
            backend: None,
            context_size: None,
            gpu_layers: 0,
            temperature: 0.7,
            cache_dir: models::default_models_dir(),
        }
    }
}

/// Local LLM provider with lazy loading.
///
/// Automatically selects the best backend for the current platform and
/// loads the model on first use.
pub struct LocalLlmProvider {
    config: LocalLlmConfig,
    inner: Arc<RwLock<Option<Arc<dyn LocalBackend>>>>,
    selected_backend: Arc<RwLock<Option<BackendType>>>,
}

impl LocalLlmProvider {
    /// Create a new lazy-loading local LLM provider.
    pub fn new(config: LocalLlmConfig) -> Self {
        Self {
            config,
            inner: Arc::new(RwLock::new(None)),
            selected_backend: Arc::new(RwLock::new(None)),
        }
    }

    async fn prepare_for_backend_load(&self, backend_type: BackendType) {
        if backend_type != BackendType::Gguf {
            return;
        }

        let mut active = ACTIVE_GGUF_PROVIDER.lock().await;
        let Some(current) = active.as_ref() else {
            return;
        };

        if current.model_id == self.config.model_id {
            return;
        }

        if let Some(previous_inner) = current.inner.upgrade() {
            let mut guard = previous_inner.write().await;
            *guard = None;
        }
        if let Some(previous_backend) = current.selected_backend.upgrade() {
            let mut guard = previous_backend.write().await;
            *guard = None;
        }

        *active = None;
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }

    async fn mark_backend_loaded(&self, backend_type: BackendType) {
        let mut selected = self.selected_backend.write().await;
        *selected = Some(backend_type);
        drop(selected);

        if backend_type == BackendType::Gguf {
            let mut active = ACTIVE_GGUF_PROVIDER.lock().await;
            *active = Some(ActiveGgufProvider {
                model_id: self.config.model_id.clone(),
                inner: Arc::downgrade(&self.inner),
                selected_backend: Arc::downgrade(&self.selected_backend),
            });
        }
    }

    /// Get the backend type that will be (or was) used.
    pub async fn backend_type(&self) -> BackendType {
        if let Some(bt) = *self.selected_backend.read().await {
            return bt;
        }
        self.config
            .backend
            .unwrap_or_else(backend::detect_best_backend)
    }

    /// Ensure the backend is loaded.
    async fn ensure_loaded(&self) -> Result<()> {
        // Fast path: check if already loaded
        {
            let guard = self.inner.read().await;
            if guard.is_some() {
                return Ok(());
            }
        }

        // Slow path: load the backend
        let mut guard = self.inner.write().await;

        // Double-check after acquiring write lock
        if guard.is_some() {
            return Ok(());
        }

        let backend_type = self
            .config
            .backend
            .unwrap_or_else(|| backend::detect_backend_for_model(&self.config.model_id));
        info!(
            model = %self.config.model_id,
            backend = ?backend_type,
            "loading local LLM model"
        );

        self.prepare_for_backend_load(backend_type).await;

        let backend: Arc<dyn LocalBackend> =
            Arc::from(backend::create_backend(backend_type, &self.config).await?);
        *guard = Some(Arc::clone(&backend));
        drop(guard);
        self.mark_backend_loaded(backend_type).await;

        Ok(())
    }
}

#[async_trait]
impl LlmProvider for LocalLlmProvider {
    fn name(&self) -> &str {
        "local-llm"
    }

    fn id(&self) -> &str {
        &self.config.model_id
    }

    fn context_window(&self) -> u32 {
        self.config
            .context_size
            .or_else(|| models::find_model(&self.config.model_id).map(|m| m.context_window))
            .unwrap_or(8192)
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn tool_mode(&self) -> Option<clawmaster_config::ToolMode> {
        Some(clawmaster_config::ToolMode::Text)
    }

    async fn complete(
        &self,
        messages: &[ChatMessage],
        tools: &[serde_json::Value],
    ) -> Result<CompletionResponse> {
        self.ensure_loaded().await?;

        let guard = self.inner.read().await;
        let backend = guard
            .as_ref()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("backend should be loaded after ensure_loaded"))?;
        drop(guard);
        if tools.is_empty() {
            backend.complete(messages).await
        } else {
            backend.complete_with_tools(messages, tools).await
        }
    }

    fn stream(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Pin<Box<dyn Stream<Item = StreamEvent> + Send + '_>> {
        Box::pin(async_stream::stream! {
            if let Err(e) = self.ensure_loaded().await {
                yield StreamEvent::Error(format!("failed to load model: {e}"));
                return;
            }

            let guard = self.inner.read().await;
            let Some(backend) = guard.as_ref().cloned() else {
                yield StreamEvent::Error("backend should be loaded after ensure_loaded".into());
                return;
            };
            drop(guard);

            let mut stream = backend.stream(&messages);
            while let Some(event) = futures::StreamExt::next(&mut stream).await {
                yield event;
            }
        })
    }
}

/// Log system info and backend availability.
pub fn log_system_info() {
    let sys = system_info::SystemInfo::detect();
    let tier = sys.memory_tier();
    let best_backend = backend::detect_best_backend();

    info!(
        total_ram_gb = sys.total_ram_gb(),
        available_ram_gb = sys.available_ram_gb(),
        has_metal = sys.has_metal,
        has_cuda = sys.has_cuda,
        is_apple_silicon = sys.is_apple_silicon,
        tier = %tier,
        best_backend = ?best_backend,
        "local-llm system info"
    );

    // Log available backends
    let available_backends = backend::available_backends();
    info!(backends = ?available_backends, "available local LLM backends");

    // Suggest models
    if let Some(suggested) = models::suggest_model(tier, best_backend) {
        info!(
            model = suggested.id,
            display_name = suggested.display_name,
            backend = ?suggested.format.backend_type(),
            "suggested local model for your system"
        );
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        clawmaster_agents::model::{LlmProvider, Usage},
        std::sync::Arc,
    };

    struct FakeBackend {
        model_id: String,
        backend_type: BackendType,
    }

    #[async_trait]
    impl LocalBackend for FakeBackend {
        fn backend_type(&self) -> BackendType {
            self.backend_type
        }

        fn model_id(&self) -> &str {
            &self.model_id
        }

        fn context_window(&self) -> u32 {
            1024
        }

        async fn complete(&self, _messages: &[ChatMessage]) -> Result<CompletionResponse> {
            Ok(CompletionResponse {
                text: Some("ok".into()),
                tool_calls: Vec::new(),
                usage: Usage::default(),
            })
        }

        fn stream<'a>(
            &'a self,
            _messages: &'a [ChatMessage],
        ) -> Pin<Box<dyn Stream<Item = StreamEvent> + Send + 'a>> {
            Box::pin(tokio_stream::iter(vec![
                StreamEvent::Done(Usage::default()),
            ]))
        }
    }

    #[test]
    fn test_default_config() {
        let config = LocalLlmConfig::default();
        assert!(config.model_id.is_empty());
        assert!(config.model_path.is_none());
        assert!(config.backend.is_none());
        assert_eq!(config.gpu_layers, 0);
    }

    #[tokio::test]
    async fn test_backend_detection() {
        let backend = backend::detect_best_backend();
        // Should always return something
        assert!(matches!(backend, BackendType::Gguf | BackendType::Mlx));
    }

    #[test]
    fn test_available_backends() {
        let backends = backend::available_backends();
        // GGUF should always be available when compiled with local-llm feature
        assert!(backends.contains(&BackendType::Gguf));
    }

    #[test]
    fn local_llm_provider_uses_text_tool_mode() {
        let provider = LocalLlmProvider::new(LocalLlmConfig::default());
        assert_eq!(
            provider.tool_mode(),
            Some(clawmaster_config::ToolMode::Text)
        );
        assert!(provider.supports_tools());
    }

    #[tokio::test]
    async fn gguf_switch_clears_previous_loaded_provider() {
        let first = LocalLlmProvider::new(LocalLlmConfig {
            model_id: "model-a".into(),
            ..Default::default()
        });
        let second = LocalLlmProvider::new(LocalLlmConfig {
            model_id: "model-b".into(),
            ..Default::default()
        });

        {
            let mut guard = first.inner.write().await;
            *guard = Some(Arc::new(FakeBackend {
                model_id: "model-a".into(),
                backend_type: BackendType::Gguf,
            }));
        }
        {
            let mut guard = first.selected_backend.write().await;
            *guard = Some(BackendType::Gguf);
        }

        first.mark_backend_loaded(BackendType::Gguf).await;
        second.prepare_for_backend_load(BackendType::Gguf).await;

        assert!(first.inner.read().await.is_none());
        assert!(first.selected_backend.read().await.is_none());
    }

    #[tokio::test]
    async fn non_gguf_switch_does_not_clear_previous_gguf_provider() {
        let first = LocalLlmProvider::new(LocalLlmConfig {
            model_id: "model-a".into(),
            ..Default::default()
        });
        let second = LocalLlmProvider::new(LocalLlmConfig {
            model_id: "model-b".into(),
            ..Default::default()
        });

        {
            let mut guard = first.inner.write().await;
            *guard = Some(Arc::new(FakeBackend {
                model_id: "model-a".into(),
                backend_type: BackendType::Gguf,
            }));
        }
        {
            let mut guard = first.selected_backend.write().await;
            *guard = Some(BackendType::Gguf);
        }

        first.mark_backend_loaded(BackendType::Gguf).await;
        second.prepare_for_backend_load(BackendType::Mlx).await;

        assert!(first.inner.read().await.is_some());
        assert_eq!(
            *first.selected_backend.read().await,
            Some(BackendType::Gguf)
        );
    }
}
