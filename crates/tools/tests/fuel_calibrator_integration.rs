//! Integration tests for Fuel Calibrator.
//!
//! These tests verify the Fuel Calibrator works correctly in realistic scenarios.
//!
//! # Compliance
//! DO-178C §11.10: Resource management
//! - Tests actual resource usage patterns
//! - Validates calibration accuracy
//! - Ensures thread safety under load

#[cfg(feature = "wasm")]
use clawmaster_tools::fuel_calibrator::{FuelCalibrator, FuelStats};
#[cfg(feature = "wasm")]
use std::sync::Arc;
#[cfg(feature = "wasm")]
use std::thread;

#[test]
#[cfg(feature = "wasm")]
fn test_calibrator_realistic_usage() {
    let calibrator = FuelCalibrator::default();

    // Simulate realistic tool execution patterns
    let tool_executions = vec![
        ("calc", vec![45_000, 52_000, 48_000, 51_000, 49_000]),
        ("web_fetch", vec![
            2_500_000, 2_800_000, 2_600_000, 2_700_000,
        ]),
        ("web_search", vec![
            3_000_000, 3_200_000, 3_100_000, 3_150_000,
        ]),
    ];

    // Record measurements
    for (tool, measurements) in &tool_executions {
        for &fuel in measurements {
            calibrator.record(tool, fuel);
        }
    }

    // Verify recommendations
    for (tool, measurements) in &tool_executions {
        let recommended = calibrator.recommend(tool, None);
        assert!(
            recommended.is_some(),
            "Should have recommendation for {}",
            tool
        );

        let limit = recommended.unwrap();
        let max = *measurements.iter().max().unwrap();

        // Recommendation should be higher than max (with buffer)
        assert!(limit > max, "Recommended limit should exceed max usage");

        // But not too much higher (reasonable buffer)
        assert!(limit < max * 2, "Recommended limit should be reasonable");
    }
}

#[test]
#[cfg(feature = "wasm")]
fn test_calibrator_concurrent_access() {
    let calibrator = Arc::new(FuelCalibrator::default());
    let mut handles = vec![];

    // Spawn multiple threads recording measurements
    for thread_id in 0..10 {
        let calibrator = Arc::clone(&calibrator);
        let handle = thread::spawn(move || {
            for i in 0..100 {
                let fuel = (thread_id * 1000 + i) as u64;
                calibrator.record("concurrent_tool", fuel);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all measurements were recorded
    let stats = calibrator.stats("concurrent_tool").unwrap();
    assert_eq!(stats.count, 1000, "Should have 1000 measurements");
}

#[test]
#[cfg(feature = "wasm")]
fn test_calibrator_stats_accuracy() {
    let calibrator = FuelCalibrator::default();

    // Record known values
    for i in 1..=100 {
        calibrator.record("test_tool", i);
    }

    let stats = calibrator.stats("test_tool").unwrap();

    // Verify statistics
    assert_eq!(stats.count, 100);
    assert_eq!(stats.min, 1);
    assert_eq!(stats.max, 100);
    assert_eq!(stats.median, 50);
    assert_eq!(stats.mean, 50); // (1+100)*100/2 / 100 = 50
    assert_eq!(stats.p95, 95);
    assert_eq!(stats.p99, 99);
}

#[test]
#[cfg(feature = "wasm")]
fn test_calibrator_buffer_percentages() {
    let calibrator = FuelCalibrator::default();

    // Record measurements
    for i in 1..=100 {
        calibrator.record("test_tool", i * 1000);
    }

    // Test different buffer percentages
    let buffers = vec![0.10, 0.20, 0.30, 0.50];

    for buffer in buffers {
        let recommended = calibrator.recommend("test_tool", Some(buffer)).unwrap();

        // P95 is 95000
        let expected = (95_000.0 * (1.0 + buffer)) as u64;
        assert_eq!(
            recommended, expected,
            "Buffer {} should give {}",
            buffer, expected
        );
    }
}

#[test]
#[cfg(feature = "wasm")]
fn test_calibrator_history_limit() {
    let calibrator = FuelCalibrator::new(10); // Only keep 10 measurements

    // Record 20 measurements
    for i in 1..=20 {
        calibrator.record("test_tool", i);
    }

    let stats = calibrator.stats("test_tool").unwrap();

    // Should only have last 10
    assert_eq!(stats.count, 10);
    assert_eq!(stats.min, 11); // First 10 were dropped
    assert_eq!(stats.max, 20);
}

#[test]
#[cfg(feature = "wasm")]
fn test_calibrator_clear_operations() {
    let calibrator = FuelCalibrator::default();

    // Record for multiple tools
    calibrator.record("tool1", 1000);
    calibrator.record("tool2", 2000);
    calibrator.record("tool3", 3000);

    // Clear one tool
    calibrator.clear("tool1");
    assert!(calibrator.stats("tool1").is_none());
    assert!(calibrator.stats("tool2").is_some());
    assert!(calibrator.stats("tool3").is_some());

    // Clear all
    calibrator.clear_all();
    assert!(calibrator.stats("tool2").is_none());
    assert!(calibrator.stats("tool3").is_none());
}

#[test]
#[cfg(feature = "wasm")]
fn test_calibrator_edge_cases() {
    let calibrator = FuelCalibrator::default();

    // No data
    assert!(calibrator.recommend("nonexistent", None).is_none());
    assert!(calibrator.stats("nonexistent").is_none());

    // Insufficient data (< 10 samples)
    for i in 1..=5 {
        calibrator.record("few_samples", i);
    }
    assert!(calibrator.recommend("few_samples", None).is_none());

    // Exactly 10 samples (minimum)
    for i in 6..=10 {
        calibrator.record("few_samples", i);
    }
    assert!(calibrator.recommend("few_samples", None).is_some());
}

#[test]
#[cfg(feature = "wasm")]
fn test_calibrator_realistic_workflow() {
    let calibrator = FuelCalibrator::default();

    // Simulate a realistic workflow:
    // 1. Tool executes multiple times
    // 2. We get recommendations
    // 3. We adjust limits
    // 4. We continue monitoring

    // Phase 1: Initial executions
    for i in 1..=20 {
        calibrator.record("production_tool", 50_000 + i * 100);
    }

    let initial_recommendation = calibrator.recommend("production_tool", None).unwrap();

    // Phase 2: More executions with similar pattern
    for i in 21..=40 {
        calibrator.record("production_tool", 50_000 + i * 100);
    }

    let updated_recommendation = calibrator.recommend("production_tool", None).unwrap();

    // Recommendations should be stable for similar patterns
    let diff = (updated_recommendation as i64 - initial_recommendation as i64).abs();
    let tolerance = initial_recommendation / 10; // 10% tolerance
    assert!(diff < tolerance as i64, "Recommendations should be stable");
}
