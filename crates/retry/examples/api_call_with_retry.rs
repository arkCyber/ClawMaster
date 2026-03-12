//! Example: Retry external API calls with exponential backoff

use clawmaster_retry::RetryPolicy;
use std::time::Duration;

async fn unreliable_api_call(attempt: &mut usize) -> Result<String, String> {
    *attempt += 1;
    println!("API call attempt {}", attempt);
    
    // Simulate failures for first 2 attempts
    if *attempt < 3 {
        Err(format!("Temporary failure (attempt {})", attempt))
    } else {
        Ok("Success!".to_string())
    }
}

#[tokio::main]
async fn main() {
    println!("=== Retry Example ===\n");
    
    // Configure retry policy with exponential backoff
    let policy = RetryPolicy::exponential(
        5,                              // max 5 attempts
        Duration::from_millis(100),     // start with 100ms
        Duration::from_secs(10),        // cap at 10s
    );
    
    let mut attempt = 0;
    
    match policy.retry(|| unreliable_api_call(&mut attempt)).await {
        Ok(result) => {
            println!("\n✅ Success after {} attempts: {}", attempt, result);
        }
        Err(e) => {
            println!("\n❌ Failed after {} attempts: {:?}", attempt, e);
        }
    }
}
