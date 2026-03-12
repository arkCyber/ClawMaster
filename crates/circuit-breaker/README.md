# clawmaster-circuit-breaker

Circuit breaker pattern for ClawMaster to provide fault isolation.

## Features
- ✅ Three states: Closed, Open, HalfOpen
- ✅ Automatic state transitions
- ✅ Configurable thresholds
- ✅ Timeout-based recovery
- ✅ Thread-safe

## Usage
```rust
use clawmaster_circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use std::time::Duration;

let config = CircuitBreakerConfig {
    failure_threshold: 5,
    timeout: Duration::from_secs(60),
    success_threshold: 2,
};

let cb = CircuitBreaker::new(config);

let result = cb.call(|| async {
    // Your async operation here
    Ok::<_, String>(42)
}).await;
```
