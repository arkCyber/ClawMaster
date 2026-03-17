//! Voice capabilities for ClawMaster: Text-to-Speech (TTS) and Speech-to-Text (STT).
//!
//! This crate provides provider-agnostic abstractions for voice services,
//! with implementations for popular providers like ElevenLabs, OpenAI, and Whisper.
//!
//! # Features
//!
//! - **TTS**: 5 providers (ElevenLabs, OpenAI, Google, Piper, Coqui)
//! - **STT**: 9 providers (Whisper, Groq, Deepgram, Google, Mistral, Voxtral, etc.)
//! - **Streaming**: Low-latency streaming TTS support
//! - **VAD**: Voice Activity Detection for audio processing
//! - **Audio Utils**: Comprehensive audio processing utilities

pub mod audio_utils;
pub mod config;
pub mod stt;
pub mod tts;
pub mod vad;

pub use {
    audio_utils::{
        AudioMetadata, AudioQuality, apply_fade_in, apply_fade_out, calculate_duration,
        convert_format, detect_format, mono_to_stereo, normalize_volume, resample, stereo_to_mono,
    },
    config::{
        CoquiTtsConfig, DeepgramConfig, ElevenLabsConfig, ElevenLabsSttConfig, GoogleSttConfig,
        GoogleTtsConfig, GroqSttConfig, MistralSttConfig, OpenAiTtsConfig, PiperTtsConfig,
        SherpaOnnxConfig, SttConfig, SttProviderId, TtsAutoMode, TtsConfig, TtsProviderId,
        VoiceConfig, VoxtralLocalConfig, WhisperCliConfig, WhisperConfig,
    },
    stt::{
        DeepgramStt, ElevenLabsStt, GoogleStt, GroqStt, MistralStt, SherpaOnnxStt, SttProvider,
        TranscribeRequest, Transcript, VoxtralLocalStt, WhisperCliStt, WhisperStt,
    },
    tts::{
        AudioFormat, AudioOutput, CoquiTts, ElevenLabsTts, GoogleTts, OpenAiTts, PiperTts,
        SynthesizeRequest, TtsProvider, Voice, contains_ssml, sanitize_text_for_tts,
        strip_ssml_tags,
    },
    vad::{EnergyVad, VadConfig, VadResult, bytes_to_samples, trim_silence},
};
