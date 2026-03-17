//! Voice features demonstration.
//!
//! This example shows how to use the voice crate for TTS and STT.
//!
//! Run with:
//! ```bash
//! cargo run --example voice_demo --features voice
//! ```

use clawmaster_voice::{
    AudioQuality, EnergyVad, VadConfig, VadResult, apply_fade_in, apply_fade_out,
    calculate_duration, normalize_volume, resample, stereo_to_mono,
};

fn main() {
    println!("🎤 ClawMaster Voice Features Demo\n");

    // Demo 1: Audio Quality Settings
    demo_audio_quality();

    // Demo 2: Voice Activity Detection
    demo_vad();

    // Demo 3: Audio Processing
    demo_audio_processing();

    println!("\n✅ Demo completed!");
}

fn demo_audio_quality() {
    println!("📊 Audio Quality Settings:");
    println!("─────────────────────────");

    for quality in &[
        AudioQuality::Low,
        AudioQuality::Medium,
        AudioQuality::High,
        AudioQuality::Studio,
    ] {
        println!(
            "{:?}: {}Hz, {} channels, {}kbps",
            quality,
            quality.sample_rate(),
            quality.channels(),
            quality.bitrate_kbps()
        );
    }
    println!();
}

fn demo_vad() {
    println!("🔊 Voice Activity Detection:");
    println!("─────────────────────────");

    let config = VadConfig {
        min_speech_duration: 0.3,
        max_silence_duration: 0.8,
        energy_threshold: 0.02,
        sample_rate: 16000,
    };

    let vad = EnergyVad::new(config);

    // Simulate speech: sine wave
    let mut speech_samples = Vec::new();
    for i in 0..8000 {
        let sample = (1000.0 * (i as f32 * 0.1).sin()) as i16;
        speech_samples.push(sample);
    }

    // Simulate silence
    let silence_samples = vec![0i16; 8000];

    match vad.detect(&speech_samples, 16000) {
        Ok(result) => {
            println!("Speech detected: {}", result.has_speech);
            println!("Confidence: {:.2}", result.confidence);
            if let Some(duration) = result.duration {
                println!("Duration: {:.2}s", duration);
            }
        },
        Err(e) => println!("Error: {}", e),
    }

    match vad.detect(&silence_samples, 16000) {
        Ok(result) => {
            println!("\nSilence detected: {}", !result.has_speech);
        },
        Err(e) => println!("Error: {}", e),
    }

    println!();
}

fn demo_audio_processing() {
    println!("🎛️  Audio Processing:");
    println!("─────────────────────────");

    // Create test audio samples
    let mut samples = vec![100i16, 200, 300, 400, 500];
    println!("Original samples: {:?}", samples);

    // Normalize volume
    normalize_volume(&mut samples, 0.5);
    println!("After normalization: {:?}", samples);

    // Apply fade in
    let mut fade_samples = vec![1000i16; 10];
    apply_fade_in(&mut fade_samples, 5);
    println!("Fade-in applied: {:?}", &fade_samples[..5]);

    // Apply fade out
    let mut fade_samples = vec![1000i16; 10];
    apply_fade_out(&mut fade_samples, 5);
    println!("Fade-out applied: {:?}", &fade_samples[5..]);

    // Stereo to mono conversion
    let stereo = vec![100i16, 200, 300, 400];
    match stereo_to_mono(&stereo) {
        Ok(mono) => println!("Stereo to mono: {:?} -> {:?}", stereo, mono),
        Err(e) => println!("Error: {}", e),
    }

    // Resampling
    let original = vec![100i16, 200, 300, 400];
    match resample(&original, 8000, 16000) {
        Ok(resampled) => {
            println!(
                "Resampled 8kHz->16kHz: {} samples -> {} samples",
                original.len(),
                resampled.len()
            );
        },
        Err(e) => println!("Error: {}", e),
    }

    // Duration calculation
    let duration = calculate_duration(16000, 16000);
    println!("Duration of 16000 samples at 16kHz: {:.2}s", duration);

    println!();
}
