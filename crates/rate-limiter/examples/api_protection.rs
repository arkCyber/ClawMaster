//! Example: Using rate limiter for API protection

use clawmaster_rate_limiter::{RateLimiter, RateLimiterConfig};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Configure rate limiter: 100 requests per minute per user
    let config = RateLimiterConfig {
        max_requests: 100,
        window: Duration::from_secs(60),
        sliding_window: true,
    };
    
    let limiter = RateLimiter::new(config);
    
    // Simulate API requests
    for i in 1..=120 {
        let user_id = "user_123";
        
        match limiter.check_rate_limit(user_id) {
            Ok(_) => {
                println!("Request {} allowed for {}", i, user_id);
                // Process request
            }
            Err(e) => {
                println!("Request {} blocked for {}: {}", i, user_id, e);
                // Return 429 Too Many Requests
            }
        }
    }
    
    // Check current usage
    if let Some(usage) = limiter.get_usage("user_123") {
        println!("\nCurrent usage for user_123: {}/100", usage);
    }
}
