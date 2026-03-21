#!/bin/bash
# Debug test to see actual error message

cd /Users/arksong/ClawMaster

cat > /tmp/test_read_error.rs << 'EOF'
use tempfile::TempDir;
use serde_json::json;

#[tokio::main]
async fn main() {
    let temp_dir = TempDir::new().unwrap();
    
    // Simulate the test
    let result = std::fs::read_to_string(temp_dir.path().join("nonexistent.txt"));
    
    match result {
        Ok(_) => println!("File exists (unexpected)"),
        Err(e) => {
            println!("Error: {}", e);
            println!("Error debug: {:?}", e);
            println!("Contains 'does not exist': {}", e.to_string().contains("does not exist"));
            println!("Contains 'No such file': {}", e.to_string().contains("No such file"));
        }
    }
}
EOF

rustc --edition 2021 /tmp/test_read_error.rs -o /tmp/test_read_error \
    --extern tempfile=/Users/arksong/ClawMaster/target/debug/deps/libtempfile-*.rlib \
    --extern tokio=/Users/arksong/ClawMaster/target/debug/deps/libtokio-*.rlib \
    --extern serde_json=/Users/arksong/ClawMaster/target/debug/deps/libserde_json-*.rlib \
    2>/dev/null || echo "Compile failed, trying simpler test..."

# Simpler test
cat > /tmp/simple_test.rs << 'EOF'
fn main() {
    let result = std::fs::read_to_string("/tmp/nonexistent_file_12345.txt");
    match result {
        Ok(_) => println!("File exists"),
        Err(e) => {
            println!("Error message: {}", e);
            println!("Contains 'does not exist': {}", e.to_string().contains("does not exist"));
            println!("Contains 'No such file': {}", e.to_string().contains("No such file"));
            println!("Contains 'not found': {}", e.to_string().to_lowercase().contains("not found"));
        }
    }
}
EOF

rustc /tmp/simple_test.rs -o /tmp/simple_test && /tmp/simple_test
