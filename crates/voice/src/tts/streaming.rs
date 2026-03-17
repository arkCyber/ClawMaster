//! Streaming TTS support for lower latency audio delivery.

use {anyhow::Result, async_trait::async_trait, bytes::Bytes, futures::Stream, std::pin::Pin};

use super::SynthesizeRequest;

/// Streaming audio chunk.
#[derive(Debug, Clone)]
pub struct AudioChunk {
    /// Audio data for this chunk.
    pub data: Bytes,
    /// Chunk sequence number (0-indexed).
    pub sequence: u64,
    /// Whether this is the final chunk.
    pub is_final: bool,
    /// Estimated duration of this chunk in milliseconds.
    pub duration_ms: Option<u64>,
}

/// Stream of audio chunks.
pub type AudioStream = Pin<Box<dyn Stream<Item = Result<AudioChunk>> + Send>>;

/// Streaming Text-to-Speech provider trait.
///
/// Providers that support streaming can deliver audio in chunks as it's
/// generated, reducing time-to-first-byte and perceived latency.
#[async_trait]
pub trait StreamingTtsProvider: Send + Sync {
    /// Provider identifier.
    fn id(&self) -> &'static str;

    /// Human-readable provider name.
    fn name(&self) -> &'static str;

    /// Check if the provider is configured and ready.
    fn is_configured(&self) -> bool;

    /// Whether this provider supports streaming synthesis.
    fn supports_streaming(&self) -> bool {
        true
    }

    /// Convert text to speech with streaming output.
    ///
    /// Returns a stream of audio chunks that can be played as they arrive.
    /// The stream should be consumed sequentially.
    async fn synthesize_stream(&self, request: SynthesizeRequest) -> Result<AudioStream>;
}

/// Configuration for streaming TTS.
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    /// Target chunk size in bytes (approximate).
    pub chunk_size: usize,
    /// Buffer size for the stream.
    pub buffer_size: usize,
    /// Enable chunk optimization (merge small chunks).
    pub optimize_chunks: bool,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            chunk_size: 4096,      // 4KB chunks
            buffer_size: 8,        // Buffer up to 8 chunks
            optimize_chunks: true, // Merge small chunks by default
        }
    }
}

/// Metrics for streaming TTS performance.
#[derive(Debug, Clone, Default)]
pub struct StreamingMetrics {
    /// Time to first chunk in milliseconds.
    pub time_to_first_chunk_ms: Option<u64>,
    /// Total chunks received.
    pub total_chunks: u64,
    /// Total bytes received.
    pub total_bytes: u64,
    /// Average chunk size in bytes.
    pub avg_chunk_size: u64,
    /// Total duration in milliseconds.
    pub total_duration_ms: Option<u64>,
}

impl StreamingMetrics {
    /// Create new metrics tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a chunk.
    pub fn record_chunk(&mut self, chunk: &AudioChunk) {
        self.total_chunks += 1;
        self.total_bytes += chunk.data.len() as u64;

        if self.total_chunks > 0 {
            self.avg_chunk_size = self.total_bytes / self.total_chunks;
        }

        if let Some(duration) = chunk.duration_ms {
            self.total_duration_ms = Some(self.total_duration_ms.unwrap_or(0) + duration);
        }
    }

    /// Set time to first chunk.
    pub fn set_time_to_first_chunk(&mut self, ms: u64) {
        self.time_to_first_chunk_ms = Some(ms);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_config_defaults() {
        let config = StreamingConfig::default();
        assert_eq!(config.chunk_size, 4096);
        assert_eq!(config.buffer_size, 8);
        assert!(config.optimize_chunks);
    }

    #[test]
    fn test_streaming_metrics() {
        let mut metrics = StreamingMetrics::new();

        let chunk1 = AudioChunk {
            data: Bytes::from(vec![0u8; 1024]),
            sequence: 0,
            is_final: false,
            duration_ms: Some(100),
        };

        let chunk2 = AudioChunk {
            data: Bytes::from(vec![0u8; 2048]),
            sequence: 1,
            is_final: true,
            duration_ms: Some(200),
        };

        metrics.record_chunk(&chunk1);
        metrics.record_chunk(&chunk2);
        metrics.set_time_to_first_chunk(50);

        assert_eq!(metrics.total_chunks, 2);
        assert_eq!(metrics.total_bytes, 3072);
        assert_eq!(metrics.avg_chunk_size, 1536);
        assert_eq!(metrics.time_to_first_chunk_ms, Some(50));
        assert_eq!(metrics.total_duration_ms, Some(300));
    }

    #[test]
    fn test_audio_chunk() {
        let chunk = AudioChunk {
            data: Bytes::from("test audio data"),
            sequence: 0,
            is_final: false,
            duration_ms: Some(500),
        };

        assert_eq!(chunk.sequence, 0);
        assert!(!chunk.is_final);
        assert_eq!(chunk.duration_ms, Some(500));
        assert_eq!(chunk.data.len(), 15);
    }
}
