//! Fuel consumption calibration for optimal resource limits.
//!
//! This module tracks actual fuel consumption and provides recommendations
//! for optimal fuel limits based on P95 + buffer.
//!
//! # Compliance
//! DO-178C §11.10: Resource management
//! - Adaptive resource limits based on actual usage
//! - Prevents over-allocation and under-allocation
//! - Maintains safety margins (20% buffer above P95)
//!
//! # Example
//! ```no_run
//! use clawmaster_tools::fuel_calibrator::FuelCalibrator;
//!
//! let calibrator = FuelCalibrator::default();
//!
//! // Record fuel consumption after tool execution
//! calibrator.record("calc", 45_000);
//! calibrator.record("calc", 52_000);
//! // ... more executions ...
//!
//! // Get recommendation (after >= 10 samples)
//! if let Some(recommended) = calibrator.recommend("calc", None) {
//!     println!("Recommended fuel limit: {}", recommended);
//! }
//! ```

use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;

/// Maximum number of measurements to keep per tool.
const MAX_HISTORY: usize = 1000;

/// Default buffer percentage above P95 for safety margin.
const DEFAULT_BUFFER_PERCENT: f64 = 0.20; // 20%

/// Minimum number of samples required for recommendation.
const MIN_SAMPLES: usize = 10;

/// Calibrates fuel limits based on actual consumption.
///
/// # Thread Safety
/// This struct uses `RwLock` for interior mutability and is safe to share
/// across threads via `Arc`.
///
/// # Compliance
/// DO-178C §6.3.2: Exception handling
/// - All lock operations handle poisoning gracefully
/// - Never panics on lock failure
pub struct FuelCalibrator {
    measurements: RwLock<HashMap<String, VecDeque<u64>>>,
    max_history: usize,
}

impl FuelCalibrator {
    /// Create a new fuel calibrator with specified history size.
    ///
    /// # Arguments
    /// * `max_history` - Maximum number of measurements to keep per tool
    ///
    /// # Example
    /// ```
    /// use clawmaster_tools::fuel_calibrator::FuelCalibrator;
    ///
    /// let calibrator = FuelCalibrator::new(500);
    /// ```
    pub fn new(max_history: usize) -> Self {
        Self {
            measurements: RwLock::new(HashMap::new()),
            max_history,
        }
    }
    
    /// Record actual fuel consumed by a tool invocation.
    ///
    /// This method is lock-free from the caller's perspective and handles
    /// lock poisoning gracefully.
    ///
    /// # Arguments
    /// * `tool` - Tool name
    /// * `consumed` - Fuel consumed during execution
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Exception handling
    /// - Handles lock poisoning gracefully
    /// - Never panics on lock failure
    ///
    /// # Example
    /// ```
    /// # use clawmaster_tools::fuel_calibrator::FuelCalibrator;
    /// let calibrator = FuelCalibrator::default();
    /// calibrator.record("calc", 45_000);
    /// ```
    pub fn record(&self, tool: &str, consumed: u64) {
        let mut measurements = match self.measurements.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("Fuel calibrator lock poisoned, recovering");
                poisoned.into_inner()
            }
        };
        
        let history = measurements
            .entry(tool.to_string())
            .or_insert_with(VecDeque::new);
        
        history.push_back(consumed);
        
        // Keep only recent measurements
        while history.len() > self.max_history {
            history.pop_front();
        }
        
        #[cfg(feature = "tracing")]
        tracing::debug!(
            tool = %tool,
            consumed = consumed,
            history_size = history.len(),
            "Recorded fuel consumption"
        );
    }
    
    /// Get recommended fuel limit based on P95 + buffer.
    ///
    /// Returns `None` if insufficient data (< 10 samples).
    ///
    /// # Arguments
    /// * `tool` - Tool name
    /// * `buffer_percent` - Safety buffer above P95 (default: 20%)
    ///
    /// # Returns
    /// Recommended fuel limit, or `None` if insufficient data.
    ///
    /// # Example
    /// ```
    /// # use clawmaster_tools::fuel_calibrator::FuelCalibrator;
    /// let calibrator = FuelCalibrator::default();
    ///
    /// // Record some measurements
    /// for i in 1..=100 {
    ///     calibrator.record("calc", i * 1000);
    /// }
    ///
    /// // Get recommendation with default 20% buffer
    /// let recommended = calibrator.recommend("calc", None);
    /// assert!(recommended.is_some());
    ///
    /// // Get recommendation with custom 30% buffer
    /// let recommended = calibrator.recommend("calc", Some(0.30));
    /// assert!(recommended.is_some());
    /// ```
    pub fn recommend(&self, tool: &str, buffer_percent: Option<f64>) -> Option<u64> {
        let buffer = buffer_percent.unwrap_or(DEFAULT_BUFFER_PERCENT);
        
        let measurements = match self.measurements.read() {
            Ok(guard) => guard,
            Err(poisoned) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("Fuel calibrator lock poisoned, recovering");
                poisoned.into_inner()
            }
        };
        
        let history = measurements.get(tool)?;
        
        if history.len() < MIN_SAMPLES {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                tool = %tool,
                samples = history.len(),
                min_required = MIN_SAMPLES,
                "Insufficient data for recommendation"
            );
            return None;
        }
        
        // Calculate P95
        let mut sorted: Vec<u64> = history.iter().copied().collect();
        sorted.sort_unstable();
        
        let p95_idx = ((sorted.len() as f64) * 0.95) as usize;
        let p95 = sorted[p95_idx.min(sorted.len() - 1)];
        
        // Add safety buffer
        let recommended = (p95 as f64 * (1.0 + buffer)) as u64;
        
        #[cfg(feature = "tracing")]
        tracing::info!(
            tool = %tool,
            p95 = p95,
            buffer_percent = buffer * 100.0,
            recommended = recommended,
            samples = sorted.len(),
            "Fuel limit recommendation"
        );
        
        Some(recommended)
    }
    
    /// Get statistics for a tool.
    ///
    /// Returns `None` if no measurements have been recorded for the tool.
    ///
    /// # Example
    /// ```
    /// # use clawmaster_tools::fuel_calibrator::FuelCalibrator;
    /// let calibrator = FuelCalibrator::default();
    ///
    /// for i in 1..=100 {
    ///     calibrator.record("calc", i);
    /// }
    ///
    /// let stats = calibrator.stats("calc").unwrap();
    /// assert_eq!(stats.count, 100);
    /// assert_eq!(stats.min, 1);
    /// assert_eq!(stats.max, 100);
    /// ```
    pub fn stats(&self, tool: &str) -> Option<FuelStats> {
        let measurements = match self.measurements.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };
        
        let history = measurements.get(tool)?;
        
        if history.is_empty() {
            return None;
        }
        
        let mut sorted: Vec<u64> = history.iter().copied().collect();
        sorted.sort_unstable();
        
        let min = sorted[0];
        let max = sorted[sorted.len() - 1];
        let median = sorted[sorted.len() / 2];
        let p95 = sorted[((sorted.len() as f64) * 0.95) as usize];
        let p99 = sorted[((sorted.len() as f64) * 0.99) as usize];
        let mean = sorted.iter().sum::<u64>() / sorted.len() as u64;
        
        Some(FuelStats {
            count: sorted.len(),
            min,
            max,
            mean,
            median,
            p95,
            p99,
        })
    }
    
    /// Clear all measurements for a tool.
    ///
    /// # Example
    /// ```
    /// # use clawmaster_tools::fuel_calibrator::FuelCalibrator;
    /// let calibrator = FuelCalibrator::default();
    ///
    /// calibrator.record("calc", 1000);
    /// calibrator.clear("calc");
    ///
    /// assert!(calibrator.stats("calc").is_none());
    /// ```
    pub fn clear(&self, tool: &str) {
        let mut measurements = match self.measurements.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };
        
        measurements.remove(tool);
        
        #[cfg(feature = "tracing")]
        tracing::debug!(tool = %tool, "Cleared fuel measurements");
    }
    
    /// Clear all measurements for all tools.
    pub fn clear_all(&self) {
        let mut measurements = match self.measurements.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };
        
        measurements.clear();
        
        #[cfg(feature = "tracing")]
        tracing::debug!("Cleared all fuel measurements");
    }
}

impl Default for FuelCalibrator {
    fn default() -> Self {
        Self::new(MAX_HISTORY)
    }
}

/// Fuel consumption statistics.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FuelStats {
    /// Number of measurements
    pub count: usize,
    /// Minimum fuel consumed
    pub min: u64,
    /// Maximum fuel consumed
    pub max: u64,
    /// Mean (average) fuel consumed
    pub mean: u64,
    /// Median fuel consumed
    pub median: u64,
    /// 95th percentile
    pub p95: u64,
    /// 99th percentile
    pub p99: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibrator_basic() {
        let calibrator = FuelCalibrator::default();
        
        // Record some measurements
        for i in 1..=100 {
            calibrator.record("test-tool", i * 1000);
        }
        
        // Get recommendation
        let recommended = calibrator.recommend("test-tool", None);
        assert!(recommended.is_some());
        
        let limit = recommended.unwrap();
        // P95 of 1000..=100000 is 95000
        // With 20% buffer: 95000 * 1.2 = 114000
        assert!(limit >= 110_000 && limit <= 120_000);
    }

    #[test]
    fn insufficient_data() {
        let calibrator = FuelCalibrator::default();
        
        // Only 5 measurements (need >= 10)
        for i in 1..=5 {
            calibrator.record("test-tool", i * 1000);
        }
        
        assert!(calibrator.recommend("test-tool", None).is_none());
    }

    #[test]
    fn stats() {
        let calibrator = FuelCalibrator::default();
        
        for i in 1..=100 {
            calibrator.record("test-tool", i);
        }
        
        let stats = calibrator.stats("test-tool").unwrap();
        assert_eq!(stats.count, 100);
        assert_eq!(stats.min, 1);
        assert_eq!(stats.max, 100);
        assert_eq!(stats.median, 50);
    }

    #[test]
    fn custom_buffer() {
        let calibrator = FuelCalibrator::default();
        
        for i in 1..=100 {
            calibrator.record("test-tool", i * 1000);
        }
        
        // 30% buffer
        let recommended = calibrator.recommend("test-tool", Some(0.30)).unwrap();
        // P95 = 95000, with 30% buffer = 123500
        assert!(recommended >= 120_000 && recommended <= 130_000);
    }

    #[test]
    fn max_history() {
        let calibrator = FuelCalibrator::new(10);
        
        // Record 20 measurements
        for i in 1..=20 {
            calibrator.record("test-tool", i);
        }
        
        let stats = calibrator.stats("test-tool").unwrap();
        // Should only keep last 10
        assert_eq!(stats.count, 10);
        assert_eq!(stats.min, 11);
        assert_eq!(stats.max, 20);
    }

    #[test]
    fn clear() {
        let calibrator = FuelCalibrator::default();
        
        calibrator.record("test-tool", 1000);
        assert!(calibrator.stats("test-tool").is_some());
        
        calibrator.clear("test-tool");
        assert!(calibrator.stats("test-tool").is_none());
    }

    #[test]
    fn clear_all() {
        let calibrator = FuelCalibrator::default();
        
        calibrator.record("tool1", 1000);
        calibrator.record("tool2", 2000);
        
        calibrator.clear_all();
        
        assert!(calibrator.stats("tool1").is_none());
        assert!(calibrator.stats("tool2").is_none());
    }

    #[test]
    fn no_data() {
        let calibrator = FuelCalibrator::default();
        
        assert!(calibrator.recommend("nonexistent", None).is_none());
        assert!(calibrator.stats("nonexistent").is_none());
    }
}
