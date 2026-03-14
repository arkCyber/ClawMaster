//! Voice Activity Detection (VAD) for audio processing.
//!
//! Detects speech segments in audio streams to improve transcription
//! efficiency and reduce API costs.

use {
    anyhow::{anyhow, Result},
    bytes::Bytes,
    serde::{Deserialize, Serialize},
};

/// Voice activity detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VadResult {
    /// Whether speech was detected.
    pub has_speech: bool,
    /// Confidence score (0.0 - 1.0).
    pub confidence: f32,
    /// Start time of speech in seconds.
    pub start_time: Option<f32>,
    /// End time of speech in seconds.
    pub end_time: Option<f32>,
    /// Duration of speech in seconds.
    pub duration: Option<f32>,
}

/// Voice activity detection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VadConfig {
    /// Minimum speech duration in seconds to consider valid.
    pub min_speech_duration: f32,
    /// Maximum silence duration in seconds before splitting.
    pub max_silence_duration: f32,
    /// Energy threshold for speech detection (0.0 - 1.0).
    pub energy_threshold: f32,
    /// Sample rate of the audio.
    pub sample_rate: u32,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            min_speech_duration: 0.3,  // 300ms minimum
            max_silence_duration: 0.8,  // 800ms max silence
            energy_threshold: 0.02,     // 2% energy threshold
            sample_rate: 16000,         // 16kHz default
        }
    }
}

/// Simple energy-based VAD implementation.
///
/// This is a basic implementation that calculates audio energy.
/// For production use, consider using more sophisticated models like
/// Silero VAD or WebRTC VAD.
pub struct EnergyVad {
    config: VadConfig,
}

impl EnergyVad {
    /// Create a new energy-based VAD.
    pub fn new(config: VadConfig) -> Self {
        Self { config }
    }

    /// Detect voice activity in audio samples.
    ///
    /// # Arguments
    /// * `samples` - Audio samples (16-bit PCM, mono)
    /// * `sample_rate` - Sample rate in Hz
    pub fn detect(&self, samples: &[i16], sample_rate: u32) -> Result<VadResult> {
        if samples.is_empty() {
            return Ok(VadResult {
                has_speech: false,
                confidence: 0.0,
                start_time: None,
                end_time: None,
                duration: None,
            });
        }

        // Calculate RMS energy
        let energy = self.calculate_rms_energy(samples);
        let has_speech = energy > self.config.energy_threshold;

        // Calculate confidence based on energy level
        let confidence = if has_speech {
            (energy / self.config.energy_threshold).min(1.0)
        } else {
            0.0
        };

        // Calculate duration
        let duration = samples.len() as f32 / sample_rate as f32;

        Ok(VadResult {
            has_speech,
            confidence,
            start_time: if has_speech { Some(0.0) } else { None },
            end_time: if has_speech { Some(duration) } else { None },
            duration: if has_speech { Some(duration) } else { None },
        })
    }

    /// Calculate RMS (Root Mean Square) energy of audio samples.
    fn calculate_rms_energy(&self, samples: &[i16]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }

        let sum_squares: f64 = samples
            .iter()
            .map(|&s| {
                let normalized = s as f64 / i16::MAX as f64;
                normalized * normalized
            })
            .sum();

        let mean_square = sum_squares / samples.len() as f64;
        mean_square.sqrt() as f32
    }

    /// Detect speech segments with timestamps.
    ///
    /// Returns a list of (start_time, end_time) tuples for each speech segment.
    pub fn detect_segments(
        &self,
        samples: &[i16],
        sample_rate: u32,
    ) -> Result<Vec<(f32, f32)>> {
        if samples.is_empty() {
            return Ok(Vec::new());
        }

        let window_size = (sample_rate as f32 * 0.02) as usize; // 20ms windows
        let hop_size = window_size / 2; // 50% overlap

        let mut segments = Vec::new();
        let mut in_speech = false;
        let mut speech_start = 0.0;
        let mut silence_duration = 0.0;

        for (i, window) in samples.chunks(hop_size).enumerate() {
            if window.len() < window_size / 2 {
                break;
            }

            let energy = self.calculate_rms_energy(window);
            let current_time = (i * hop_size) as f32 / sample_rate as f32;
            let is_speech = energy > self.config.energy_threshold;

            if is_speech {
                if !in_speech {
                    // Start of new speech segment
                    speech_start = current_time;
                    in_speech = true;
                }
                silence_duration = 0.0;
            } else if in_speech {
                // In speech but current frame is silence
                silence_duration += hop_size as f32 / sample_rate as f32;

                if silence_duration > self.config.max_silence_duration {
                    // End of speech segment
                    let speech_duration = current_time - speech_start;
                    if speech_duration >= self.config.min_speech_duration {
                        segments.push((speech_start, current_time - silence_duration));
                    }
                    in_speech = false;
                    silence_duration = 0.0;
                }
            }
        }

        // Handle final segment
        if in_speech {
            let final_time = samples.len() as f32 / sample_rate as f32;
            let speech_duration = final_time - speech_start;
            if speech_duration >= self.config.min_speech_duration {
                segments.push((speech_start, final_time));
            }
        }

        Ok(segments)
    }
}

/// Convert raw audio bytes to i16 samples.
///
/// This function is re-exported from the vad module for convenience.
pub fn bytes_to_samples(audio: &Bytes) -> Result<Vec<i16>> {
    if audio.len() % 2 != 0 {
        return Err(anyhow!("Audio data length must be even for 16-bit samples"));
    }

    let samples: Vec<i16> = audio
        .chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    Ok(samples)
}

/// Trim silence from the beginning and end of audio samples.
pub fn trim_silence(
    samples: &[i16],
    sample_rate: u32,
    config: &VadConfig,
) -> Result<Vec<i16>> {
    let vad = EnergyVad::new(config.clone());
    let segments = vad.detect_segments(samples, sample_rate)?;

    if segments.is_empty() {
        return Ok(Vec::new());
    }

    // Get the first and last speech segments
    let start_time = segments.first().map(|(s, _)| *s).unwrap_or(0.0);
    let end_time = segments.last().map(|(_, e)| *e).unwrap_or(0.0);

    let start_sample = (start_time * sample_rate as f32) as usize;
    let end_sample = (end_time * sample_rate as f32) as usize;

    let trimmed = samples
        .get(start_sample..end_sample.min(samples.len()))
        .unwrap_or(&[])
        .to_vec();

    Ok(trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vad_config_defaults() {
        let config = VadConfig::default();
        assert_eq!(config.min_speech_duration, 0.3);
        assert_eq!(config.max_silence_duration, 0.8);
        assert_eq!(config.energy_threshold, 0.02);
        assert_eq!(config.sample_rate, 16000);
    }

    #[test]
    fn test_energy_vad_empty_samples() {
        let vad = EnergyVad::new(VadConfig::default());
        let result = vad.detect(&[], 16000).unwrap();
        assert!(!result.has_speech);
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_energy_vad_silence() {
        let vad = EnergyVad::new(VadConfig::default());
        let samples = vec![0i16; 1600]; // 100ms of silence at 16kHz
        let result = vad.detect(&samples, 16000).unwrap();
        assert!(!result.has_speech);
    }

    #[test]
    fn test_energy_vad_speech() {
        let vad = EnergyVad::new(VadConfig::default());
        // Generate a simple sine wave to simulate speech
        let mut samples = Vec::new();
        for i in 0..1600 {
            let sample = (1000.0 * (i as f32 * 0.1).sin()) as i16;
            samples.push(sample);
        }
        let result = vad.detect(&samples, 16000).unwrap();
        assert!(result.has_speech);
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_rms_energy_calculation() {
        let vad = EnergyVad::new(VadConfig::default());
        let samples = vec![1000i16, -1000, 1000, -1000];
        let energy = vad.calculate_rms_energy(&samples);
        assert!(energy > 0.0);
        assert!(energy < 1.0);
    }

    #[test]
    fn test_bytes_to_samples() {
        let audio = Bytes::from(vec![0x00, 0x01, 0xFF, 0x7F]);
        let samples = bytes_to_samples(&audio).unwrap();
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0], 0x0100);
        assert_eq!(samples[1], 0x7FFF);
    }

    #[test]
    fn test_bytes_to_samples_odd_length() {
        let audio = Bytes::from(vec![0x00, 0x01, 0xFF]);
        let result = bytes_to_samples(&audio);
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_segments() {
        let vad = EnergyVad::new(VadConfig::default());
        
        // Create audio with speech-silence-speech pattern
        let mut samples = Vec::new();
        
        // Speech segment 1 (0.5s)
        for i in 0..8000 {
            samples.push((1000.0 * (i as f32 * 0.1).sin()) as i16);
        }
        
        // Silence (1.0s)
        for _ in 0..16000 {
            samples.push(0);
        }
        
        // Speech segment 2 (0.5s)
        for i in 0..8000 {
            samples.push((1000.0 * (i as f32 * 0.1).sin()) as i16);
        }
        
        let segments = vad.detect_segments(&samples, 16000).unwrap();
        assert!(segments.len() >= 1); // Should detect at least one segment
    }

    #[test]
    fn test_trim_silence() {
        let config = VadConfig::default();
        
        // Create audio with silence-speech-silence
        let mut samples = Vec::new();
        
        // Leading silence (0.2s)
        for _ in 0..3200 {
            samples.push(0);
        }
        
        // Speech (0.5s)
        for i in 0..8000 {
            samples.push((1000.0 * (i as f32 * 0.1).sin()) as i16);
        }
        
        // Trailing silence (0.2s)
        for _ in 0..3200 {
            samples.push(0);
        }
        
        let trimmed = trim_silence(&samples, 16000, &config).unwrap();
        assert!(trimmed.len() < samples.len());
        assert!(trimmed.len() > 0);
    }
}
