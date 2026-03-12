# clawmaster-retry

Retry mechanism for ClawMaster to improve reliability.

## Features
- ✅ Exponential backoff
- ✅ Fixed delay
- ✅ Linear backoff
- ✅ Jitter support
- ✅ Configurable max attempts

## Usage
```rust
use clawmaster_retry::{RetryPolicy, BackoffStrategy};
use std::time::Duration;

let policy = RetryPolicy::exponential(
    5, // max attempts
    Duration::from_millis(100), // base delay
    Duration::from_secs(10), // max delay
);

let result = policy.retry(|| async {
    // Your async operation here
    Ok::<_, String>(42)
}).await;
```
