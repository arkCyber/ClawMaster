//! Basic usage example for ClawHub registry.

use {
    clawmaster_clawhub::{
        registry::Registry,
        types::{SecurityStatus, ToolMetadata, ToolType},
    },
    time::OffsetDateTime,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new registry
    let registry = Registry::new("example_clawhub.db").await?;

    println!("✅ Registry created successfully");

    // Create sample tool metadata
    let metadata = ToolMetadata {
        name: "example-calc".to_string(),
        version: "1.0.0".to_string(),
        description: "A simple calculator tool".to_string(),
        readme: Some("# Calculator\n\nA simple calculator for basic math operations.".to_string()),
        author: "Example Author".to_string(),
        author_email: Some("author@example.com".to_string()),
        license: "MIT".to_string(),
        repository: Some("https://github.com/example/calc".to_string()),
        homepage: Some("https://example.com/calc".to_string()),
        keywords: vec!["calculator".to_string(), "math".to_string()],
        categories: vec!["utilities".to_string()],
        tool_type: ToolType::Pure,
        wasm_hash: "abc123def456".to_string(),
        wasm_size: 245760,
        signature: "signature_placeholder".to_string(),
        public_key: "public_key_placeholder".to_string(),
        downloads: 0,
        security_status: SecurityStatus::Pending,
        published_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    };

    // Publish the tool
    println!("📦 Publishing tool: {}@{}", metadata.name, metadata.version);
    registry.publish(metadata.clone()).await?;
    println!("✅ Tool published successfully");

    // Retrieve the tool
    println!("\n🔍 Retrieving tool...");
    let retrieved = registry.get_tool("example-calc", "1.0.0").await?;
    println!("✅ Retrieved tool: {}", retrieved.name);
    println!("   Description: {}", retrieved.description);
    println!("   Author: {}", retrieved.author);
    println!("   License: {}", retrieved.license);
    println!("   Type: {:?}", retrieved.tool_type);
    println!("   Security: {:?}", retrieved.security_status);

    // Increment downloads
    println!("\n📥 Incrementing downloads...");
    registry
        .increment_downloads("example-calc", "1.0.0")
        .await?;

    let updated = registry.get_tool("example-calc", "1.0.0").await?;
    println!("✅ Downloads: {}", updated.downloads);

    println!("\n🎉 Example completed successfully!");

    Ok(())
}
