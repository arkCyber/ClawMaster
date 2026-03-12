//! Example: Using circuit breaker to protect database calls

use clawmaster_circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitState};
use std::time::Duration;
use tokio::time::sleep;

async fn database_query(should_fail: bool) -> Result<String, String> {
    if should_fail {
        Err("Database connection timeout".to_string())
    } else {
        Ok("Query result".to_string())
    }
}

#[tokio::main]
async fn main() {
    println!("=== Circuit Breaker Example ===\n");
    
    // Configure circuit breaker
    let config = CircuitBreakerConfig {
        failure_threshold: 3,           // Open after 3 failures
        timeout: Duration::from_secs(2), // Wait 2s before retry
        success_threshold: 2,            // Close after 2 successes
    };
    
    let cb = CircuitBreaker::new(config);
    
    println!("Initial state: {:?}\n", cb.state());
    
    // Simulate failures
    println!("--- Simulating 3 failures ---");
    for i in 1..=3 {
        let result = cb.call(|| database_query(true)).await;
        println!("Call {}: {:?}, State: {:?}", i, result, cb.state());
    }
    
    // Circuit should be open now
    println!("\n--- Circuit is OPEN, calls rejected ---");
    for i in 1..=3 {
        let result = cb.call(|| database_query(false)).await;
        println!("Call {}: {:?}, State: {:?}", i, result, cb.state());
    }
    
    // Wait for timeout
    println!("\n--- Waiting for timeout... ---");
    sleep(Duration::from_secs(3)).await;
    
    // Try again - should transition to HalfOpen
    println!("\n--- After timeout, trying again ---");
    let result = cb.call(|| database_query(false)).await;
    println!("Call: {:?}, State: {:?}", result, cb.state());
    
    // One more success should close the circuit
    let result = cb.call(|| database_query(false)).await;
    println!("Call: {:?}, State: {:?}", result, cb.state());
    
    println!("\n✅ Circuit is CLOSED, normal operation resumed");
}
