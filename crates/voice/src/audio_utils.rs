//! Audio processing utilities for voice features.

use {
    anyhow::{Result, anyhow},
    bytes::Bytes,
};

use crate::tts::AudioFormat;

/// Audio metadata information.
#[derive(Debug, Clone)]
pub struct AudioMetadata {
    /// Sample rate in Hz.
    pub sample_rate: u32,
    /// Number of channels (1 = mono, 2 = stereo).
    pub channels: u16,
    /// Bits per sample (8, 16, 24, 32).
    pub bits_per_sample: u16,
    /// Duration in seconds.
    pub duration_seconds: f32,
    /// Total number of samples.
    pub total_samples: u64,
    /// Audio format.
    pub format: AudioFormat,
}

/// Audio quality settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioQuality {
    /// Low quality (8kHz, mono, 64kbps).
    Low,
    /// Medium quality (16kHz, mono, 128kbps).
    Medium,
    /// High quality (24kHz, mono, 192kbps).
    High,
    /// Studio quality (48kHz, stereo, 320kbps).
    Studio,
}

impl AudioQuality {
    /// Get sample rate for this quality level.
    pub fn sample_rate(&self) -> u32 {
        match self {
            Self::Low => 8000,
            Self::Medium => 16000,
            Self::High => 24000,
            Self::Studio => 48000,
        }
    }

    /// Get number of channels for this quality level.
    pub fn channels(&self) -> u16 {
        match self {
            Self::Low | Self::Medium | Self::High => 1,
            Self::Studio => 2,
        }
    }

    /// Get bitrate for this quality level (in kbps).
    pub fn bitrate_kbps(&self) -> u32 {
        match self {
            Self::Low => 64,
            Self::Medium => 128,
            Self::High => 192,
            Self::Studio => 320,
        }
    }
}

/// Convert audio format to another format.
///
/// Note: This is a placeholder. For production use, integrate with
/// FFmpeg or similar audio processing library.
pub fn convert_format(
    audio: &Bytes,
    from_format: AudioFormat,
    to_format: AudioFormat,
) -> Result<Bytes> {
    // If formats are the same, return as-is
    if from_format == to_format {
        return Ok(audio.clone());
    }

    // For now, return error indicating conversion is not implemented
    // In production, this would use FFmpeg or similar
    Err(anyhow!(
        "Audio format conversion from {:?} to {:?} requires FFmpeg integration",
        from_format,
        to_format
    ))
}

/// Normalize audio volume to target level.
///
/// This is a simple peak normalization. For production use, consider
/// using loudness normalization (LUFS/EBU R128).
pub fn normalize_volume(samples: &mut [i16], target_peak: f32) {
    if samples.is_empty() {
        return;
    }

    // Find current peak
    let current_peak = samples.iter().map(|&s| s.abs()).max().unwrap_or(0) as f32;

    if current_peak == 0.0 {
        return;
    }

    // Calculate gain
    let target_peak_value = target_peak * i16::MAX as f32;
    let gain = target_peak_value / current_peak;

    // Apply gain to all samples
    for sample in samples.iter_mut() {
        let normalized = (*sample as f32 * gain).clamp(i16::MIN as f32, i16::MAX as f32);
        *sample = normalized as i16;
    }
}

/// Resample audio to a different sample rate.
///
/// Note: This is a simple linear interpolation. For production use,
/// consider using a proper resampling library like libsamplerate.
pub fn resample(samples: &[i16], from_rate: u32, to_rate: u32) -> Result<Vec<i16>> {
    if from_rate == to_rate {
        return Ok(samples.to_vec());
    }

    if samples.is_empty() {
        return Ok(Vec::new());
    }

    let ratio = to_rate as f64 / from_rate as f64;
    let output_len = (samples.len() as f64 * ratio) as usize;
    let mut output = Vec::with_capacity(output_len);

    for i in 0..output_len {
        let src_pos = i as f64 / ratio;
        let src_idx = src_pos.floor() as usize;
        let frac = src_pos - src_idx as f64;

        if src_idx + 1 < samples.len() {
            // Linear interpolation
            let sample1 = samples[src_idx] as f64;
            let sample2 = samples[src_idx + 1] as f64;
            let interpolated = sample1 + (sample2 - sample1) * frac;
            output.push(interpolated as i16);
        } else if src_idx < samples.len() {
            output.push(samples[src_idx]);
        }
    }

    Ok(output)
}

/// Convert stereo audio to mono by averaging channels.
pub fn stereo_to_mono(stereo_samples: &[i16]) -> Result<Vec<i16>> {
    if stereo_samples.len() % 2 != 0 {
        return Err(anyhow!("Stereo audio must have even number of samples"));
    }

    let mono_samples: Vec<i16> = stereo_samples
        .chunks_exact(2)
        .map(|chunk| {
            let left = chunk[0] as i32;
            let right = chunk[1] as i32;
            ((left + right) / 2) as i16
        })
        .collect();

    Ok(mono_samples)
}

/// Convert mono audio to stereo by duplicating the channel.
pub fn mono_to_stereo(mono_samples: &[i16]) -> Vec<i16> {
    let mut stereo_samples = Vec::with_capacity(mono_samples.len() * 2);
    for &sample in mono_samples {
        stereo_samples.push(sample);
        stereo_samples.push(sample);
    }
    stereo_samples
}

/// Calculate audio duration from sample count and rate.
pub fn calculate_duration(sample_count: usize, sample_rate: u32) -> f32 {
    if sample_rate == 0 {
        return 0.0;
    }
    sample_count as f32 / sample_rate as f32
}

/// Detect audio format from file header (magic bytes).
pub fn detect_format(data: &[u8]) -> Option<AudioFormat> {
    if data.len() < 4 {
        return None;
    }

    // Check for common audio format signatures
    match &data[0..4] {
        // MP3 (ID3v2 or MPEG frame sync)
        [0x49, 0x44, 0x33, _] | [0xFF, 0xFB, _, _] | [0xFF, 0xF3, _, _] | [0xFF, 0xF2, _, _] => {
            Some(AudioFormat::Mp3)
        },
        // OGG
        [0x4F, 0x67, 0x67, 0x53] => Some(AudioFormat::Opus),
        // WebM
        [0x1A, 0x45, 0xDF, 0xA3] => Some(AudioFormat::Webm),
        // WAV/PCM (RIFF)
        [0x52, 0x49, 0x46, 0x46] => Some(AudioFormat::Pcm),
        // M4A/AAC (ftyp)
        _ if data.len() >= 8 && &data[4..8] == b"ftyp" => Some(AudioFormat::Aac),
        _ => None,
    }
}

/// Apply fade-in effect to audio samples.
pub fn apply_fade_in(samples: &mut [i16], duration_samples: usize) {
    let fade_len = duration_samples.min(samples.len());
    for (i, sample) in samples.iter_mut().take(fade_len).enumerate() {
        let gain = i as f32 / fade_len as f32;
        *sample = (*sample as f32 * gain) as i16;
    }
}

/// Apply fade-out effect to audio samples.
pub fn apply_fade_out(samples: &mut [i16], duration_samples: usize) {
    let fade_len = duration_samples.min(samples.len());
    let start_idx = samples.len().saturating_sub(fade_len);

    for (i, sample) in samples.iter_mut().skip(start_idx).enumerate() {
        // Ensure the last sample is exactly 0
        let gain = if i + 1 >= fade_len {
            0.0
        } else {
            1.0 - ((i + 1) as f32 / fade_len as f32)
        };
        *sample = (*sample as f32 * gain) as i16;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_quality_settings() {
        assert_eq!(AudioQuality::Low.sample_rate(), 8000);
        assert_eq!(AudioQuality::Medium.sample_rate(), 16000);
        assert_eq!(AudioQuality::High.sample_rate(), 24000);
        assert_eq!(AudioQuality::Studio.sample_rate(), 48000);

        assert_eq!(AudioQuality::Low.channels(), 1);
        assert_eq!(AudioQuality::Studio.channels(), 2);

        assert_eq!(AudioQuality::Low.bitrate_kbps(), 64);
        assert_eq!(AudioQuality::Studio.bitrate_kbps(), 320);
    }

    #[test]
    fn test_normalize_volume() {
        let mut samples = vec![100i16, -200, 300, -400];
        normalize_volume(&mut samples, 0.5);

        // Peak should be around 0.5 * i16::MAX
        let max_abs = samples.iter().map(|&s| s.abs()).max().unwrap();
        assert!(max_abs > 15000 && max_abs < 17000); // ~0.5 * 32767
    }

    #[test]
    fn test_normalize_volume_empty() {
        let mut samples: Vec<i16> = Vec::new();
        normalize_volume(&mut samples, 0.5);
        assert!(samples.is_empty());
    }

    #[test]
    fn test_stereo_to_mono() {
        let stereo = vec![100i16, 200, 300, 400];
        let mono = stereo_to_mono(&stereo).unwrap();
        assert_eq!(mono.len(), 2);
        assert_eq!(mono[0], 150); // (100 + 200) / 2
        assert_eq!(mono[1], 350); // (300 + 400) / 2
    }

    #[test]
    fn test_stereo_to_mono_odd_samples() {
        let stereo = vec![100i16, 200, 300];
        let result = stereo_to_mono(&stereo);
        assert!(result.is_err());
    }

    #[test]
    fn test_mono_to_stereo() {
        let mono = vec![100i16, 200];
        let stereo = mono_to_stereo(&mono);
        assert_eq!(stereo.len(), 4);
        assert_eq!(stereo, vec![100, 100, 200, 200]);
    }

    #[test]
    fn test_calculate_duration() {
        let duration = calculate_duration(16000, 16000);
        assert_eq!(duration, 1.0);

        let duration = calculate_duration(8000, 16000);
        assert_eq!(duration, 0.5);
    }

    #[test]
    fn test_detect_format_mp3() {
        let mp3_header = vec![0x49, 0x44, 0x33, 0x04];
        assert_eq!(detect_format(&mp3_header), Some(AudioFormat::Mp3));
    }

    #[test]
    fn test_detect_format_ogg() {
        let ogg_header = vec![0x4F, 0x67, 0x67, 0x53];
        assert_eq!(detect_format(&ogg_header), Some(AudioFormat::Opus));
    }

    #[test]
    fn test_detect_format_wav() {
        let wav_header = vec![0x52, 0x49, 0x46, 0x46];
        assert_eq!(detect_format(&wav_header), Some(AudioFormat::Pcm));
    }

    #[test]
    fn test_detect_format_unknown() {
        let unknown = vec![0x00, 0x00, 0x00, 0x00];
        assert_eq!(detect_format(&unknown), None);
    }

    #[test]
    fn test_resample_same_rate() {
        let samples = vec![100i16, 200, 300];
        let resampled = resample(&samples, 16000, 16000).unwrap();
        assert_eq!(resampled, samples);
    }

    #[test]
    fn test_resample_upsample() {
        let samples = vec![100i16, 200];
        let resampled = resample(&samples, 8000, 16000).unwrap();
        assert_eq!(resampled.len(), 4);
    }

    #[test]
    fn test_resample_downsample() {
        let samples = vec![100i16, 200, 300, 400];
        let resampled = resample(&samples, 16000, 8000).unwrap();
        assert_eq!(resampled.len(), 2);
    }

    #[test]
    fn test_apply_fade_in() {
        let mut samples = vec![1000i16; 10];
        apply_fade_in(&mut samples, 5);

        assert_eq!(samples[0], 0);
        assert!(samples[4] > 0 && samples[4] < 1000);
        assert_eq!(samples[9], 1000);
    }

    #[test]
    fn test_apply_fade_out() {
        let mut samples = vec![1000i16; 10];
        apply_fade_out(&mut samples, 5);

        assert_eq!(samples[0], 1000);
        assert!(samples[7] > 0 && samples[7] < 1000);
        assert_eq!(samples[9], 0);
    }

    #[test]
    fn test_convert_format_same() {
        let audio = Bytes::from("test audio");
        let result = convert_format(&audio, AudioFormat::Mp3, AudioFormat::Mp3).unwrap();
        assert_eq!(result, audio);
    }

    #[test]
    fn test_convert_format_different() {
        let audio = Bytes::from("test audio");
        let result = convert_format(&audio, AudioFormat::Mp3, AudioFormat::Opus);
        assert!(result.is_err());
    }
}
