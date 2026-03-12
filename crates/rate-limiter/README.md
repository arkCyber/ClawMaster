# clawmaster-rate-limiter

Rate limiting for ClawMaster to prevent API abuse.

## Features
- ✅ Sliding window algorithm
- ✅ Fixed window algorithm
- ✅ Per-key rate limiting
- ✅ Thread-safe
- ✅ High performance

## Usage
```rust
use clawmaster_rate_limiter::{RateLimiter, RateLimiterConfig};
use std::time::Duration;

let config = RateLimiterConfig {
    max_requests: 100,
    window: Duration::from_secs(60),
    sliding_window: true,
};

let limiter = RateLimiter::new(config);

// Check rate limit
if limiter.check_rate_limit("user_123").is_ok() {
    // Process request
}
```
