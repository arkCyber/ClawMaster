//! Integration tests for voice features.

use clawmaster_voice::{
    AudioQuality, EnergyVad, VadConfig,
    apply_fade_in, apply_fade_out, bytes_to_samples,
    calculate_duration, mono_to_stereo, normalize_volume,
    resample, stereo_to_mono, trim_silence,
    tts::streaming::{AudioChunk, StreamingConfig, StreamingMetrics},
};
use bytes::Bytes;

#[test]
fn test_complete_vad_workflow() {
    // Create VAD with custom config
    let config = VadConfig {
        min_speech_duration: 0.2,
        max_silence_duration: 0.5,
        energy_threshold: 0.015,
        sample_rate: 16000,
    };
    
    let vad = EnergyVad::new(config.clone());
    
    // Generate test audio: speech-silence-speech pattern
    let mut samples = Vec::new();
    
    // Speech segment 1 (0.5s)
    for i in 0..8000 {
        samples.push((1000.0 * (i as f32 * 0.1).sin()) as i16);
    }
    
    // Silence (0.6s)
    for _ in 0..9600 {
        samples.push(0);
    }
    
    // Speech segment 2 (0.5s)
    for i in 0..8000 {
        samples.push((1000.0 * (i as f32 * 0.1).sin()) as i16);
    }
    
    // Detect speech
    let result = vad.detect(&samples, 16000).unwrap();
    assert!(result.has_speech);
    assert!(result.confidence > 0.0);
    
    // Detect segments
    let segments = vad.detect_segments(&samples, 16000).unwrap();
    assert!(segments.len() >= 1);
    
    // Trim silence
    let trimmed = trim_silence(&samples, 16000, &config).unwrap();
    // Trimmed length should be less than or equal to original (may be equal if all speech)
    assert!(trimmed.len() <= samples.len());
    assert!(trimmed.len() > 0);
}

#[test]
fn test_audio_processing_pipeline() {
    // Start with stereo audio
    let stereo_samples = vec![100i16, 150, 200, 250, 300, 350, 400, 450];
    
    // Convert to mono
    let mono_samples = stereo_to_mono(&stereo_samples).unwrap();
    assert_eq!(mono_samples.len(), 4);
    assert_eq!(mono_samples[0], 125); // (100 + 150) / 2
    
    // Normalize volume
    let mut normalized = mono_samples.clone();
    normalize_volume(&mut normalized, 0.8);
    
    // Apply fade in and out
    let mut processed = normalized.clone();
    apply_fade_in(&mut processed, 2);
    apply_fade_out(&mut processed, 2);
    
    // Verify fade effects
    assert!(processed[0] < normalized[0]);
    assert!(processed[processed.len() - 1] < normalized[normalized.len() - 1]);
}

#[test]
fn test_resampling_accuracy() {
    // Create a simple signal
    let original: Vec<i16> = (0..100).map(|i| (i * 10) as i16).collect();
    
    // Upsample 8kHz -> 16kHz (should double the samples)
    let upsampled = resample(&original, 8000, 16000).unwrap();
    assert!(upsampled.len() > original.len());
    assert!(upsampled.len() <= original.len() * 2 + 1);
    
    // Downsample 16kHz -> 8kHz (should halve the samples)
    let downsampled = resample(&original, 16000, 8000).unwrap();
    assert!(downsampled.len() < original.len());
    assert!(downsampled.len() >= original.len() / 2 - 1);
    
    // Same rate should return identical length
    let same_rate = resample(&original, 16000, 16000).unwrap();
    assert_eq!(same_rate.len(), original.len());
}

#[test]
fn test_streaming_metrics_tracking() {
    let mut metrics = StreamingMetrics::new();
    
    // Simulate receiving chunks
    for i in 0..5 {
        let chunk = AudioChunk {
            data: Bytes::from(vec![0u8; 1024 * (i + 1)]),
            sequence: i as u64,
            is_final: i == 4,
            duration_ms: Some(100),
        };
        metrics.record_chunk(&chunk);
    }
    
    metrics.set_time_to_first_chunk(50);
    
    assert_eq!(metrics.total_chunks, 5);
    assert_eq!(metrics.time_to_first_chunk_ms, Some(50));
    assert_eq!(metrics.total_duration_ms, Some(500));
    assert!(metrics.avg_chunk_size > 0);
}

#[test]
fn test_audio_quality_consistency() {
    let qualities = [
        AudioQuality::Low,
        AudioQuality::Medium,
        AudioQuality::High,
        AudioQuality::Studio,
    ];
    
    for quality in &qualities {
        // Sample rate should increase with quality
        assert!(quality.sample_rate() >= 8000);
        assert!(quality.sample_rate() <= 48000);
        
        // Bitrate should increase with quality
        assert!(quality.bitrate_kbps() >= 64);
        assert!(quality.bitrate_kbps() <= 320);
        
        // Channels should be 1 or 2
        assert!(quality.channels() == 1 || quality.channels() == 2);
    }
    
    // Verify quality ordering
    assert!(AudioQuality::Low.sample_rate() < AudioQuality::Medium.sample_rate());
    assert!(AudioQuality::Medium.sample_rate() < AudioQuality::High.sample_rate());
    assert!(AudioQuality::High.sample_rate() < AudioQuality::Studio.sample_rate());
}

#[test]
fn test_bytes_to_samples_conversion() {
    // Create test audio data (16-bit little-endian)
    let audio_bytes = Bytes::from(vec![
        0x00, 0x01, // 256
        0xFF, 0x7F, // 32767 (max positive)
        0x00, 0x80, // -32768 (max negative)
        0xFF, 0xFF, // -1
    ]);
    
    let samples = bytes_to_samples(&audio_bytes).unwrap();
    assert_eq!(samples.len(), 4);
    assert_eq!(samples[0], 256);
    assert_eq!(samples[1], 32767);
    assert_eq!(samples[2], -32768);
    assert_eq!(samples[3], -1);
}

#[test]
fn test_duration_calculation() {
    // Test various sample rates and counts
    assert_eq!(calculate_duration(16000, 16000), 1.0);
    assert_eq!(calculate_duration(8000, 16000), 0.5);
    assert_eq!(calculate_duration(48000, 16000), 3.0);
    assert_eq!(calculate_duration(0, 16000), 0.0);
}

#[test]
fn test_channel_conversion_roundtrip() {
    let original_mono = vec![100i16, 200, 300, 400];
    
    // Mono -> Stereo
    let stereo = mono_to_stereo(&original_mono);
    assert_eq!(stereo.len(), original_mono.len() * 2);
    
    // Stereo -> Mono (should get back original)
    let back_to_mono = stereo_to_mono(&stereo).unwrap();
    assert_eq!(back_to_mono.len(), original_mono.len());
    assert_eq!(back_to_mono, original_mono);
}

#[test]
fn test_fade_effects_symmetry() {
    let original = vec![1000i16; 20];
    
    // Test fade in
    let mut fade_in = original.clone();
    apply_fade_in(&mut fade_in, 10);
    assert_eq!(fade_in[0], 0);
    assert!(fade_in[5] > 0 && fade_in[5] < 1000);
    assert_eq!(fade_in[19], 1000);
    
    // Test fade out
    let mut fade_out = original.clone();
    apply_fade_out(&mut fade_out, 10);
    assert_eq!(fade_out[0], 1000);
    assert!(fade_out[15] > 0 && fade_out[15] < 1000);
    assert_eq!(fade_out[19], 0);
}

#[test]
fn test_normalize_volume_preserves_waveform() {
    // Create a simple waveform
    let mut samples = vec![100i16, -200, 300, -400];
    let original_ratios: Vec<f32> = samples
        .windows(2)
        .map(|w| w[1] as f32 / w[0] as f32)
        .collect();
    
    normalize_volume(&mut samples, 0.5);
    
    let normalized_ratios: Vec<f32> = samples
        .windows(2)
        .map(|w| w[1] as f32 / w[0] as f32)
        .collect();
    
    // Ratios should be approximately the same (waveform shape preserved)
    for (orig, norm) in original_ratios.iter().zip(normalized_ratios.iter()) {
        let diff = (orig - norm).abs();
        assert!(diff < 0.01, "Waveform shape not preserved");
    }
}

#[test]
fn test_streaming_config_defaults() {
    let config = StreamingConfig::default();
    assert_eq!(config.chunk_size, 4096);
    assert_eq!(config.buffer_size, 8);
    assert!(config.optimize_chunks);
}

#[test]
fn test_vad_config_validation() {
    let config = VadConfig::default();
    
    // Ensure sensible defaults
    assert!(config.min_speech_duration > 0.0);
    assert!(config.max_silence_duration > config.min_speech_duration);
    assert!(config.energy_threshold > 0.0 && config.energy_threshold < 1.0);
    assert!(config.sample_rate >= 8000);
}

#[test]
fn test_empty_audio_handling() {
    let vad = EnergyVad::new(VadConfig::default());
    
    // Empty samples should not crash
    let result = vad.detect(&[], 16000).unwrap();
    assert!(!result.has_speech);
    assert_eq!(result.confidence, 0.0);
    
    let segments = vad.detect_segments(&[], 16000).unwrap();
    assert_eq!(segments.len(), 0);
}

#[test]
fn test_audio_processing_edge_cases() {
    // Test with single sample
    let single = vec![100i16];
    let mut normalized = single.clone();
    normalize_volume(&mut normalized, 0.5);
    assert_eq!(normalized.len(), 1);
    
    // Test with two samples
    let pair = vec![100i16, 200];
    let resampled = resample(&pair, 16000, 16000).unwrap();
    assert_eq!(resampled.len(), 2);
}
