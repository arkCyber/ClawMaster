//! Publish command implementation.

use anyhow::Result;
use colored::Colorize;
use clawmaster_clawhub::types::{PublishRequest, ToolMetadata};
use reqwest::Client;
use std::fs;

const API_BASE: &str = "https://api.clawhub.io";

pub async fn execute(wasm_file: &str, metadata_file: &str) -> Result<()> {
    println!("{} {}", "Publishing:".bright_blue().bold(), wasm_file);
    
    // Read Wasm file
    let wasm_bytes = fs::read(wasm_file)?;
    println!("{} Read {} bytes", "→".bright_cyan(), wasm_bytes.len());
    
    // Read metadata
    let metadata_json = fs::read_to_string(metadata_file)?;
    let metadata: ToolMetadata = serde_json::from_str(&metadata_json)?;
    
    println!("{} Parsed metadata for {}@{}", "→".bright_cyan(), metadata.name, metadata.version);
    
    // Encode Wasm bytes to base64
    let wasm_base64 = base64::encode(&wasm_bytes);
    
    let request = PublishRequest {
        metadata,
        wasm_bytes: wasm_base64,
    };
    
    // Send publish request
    let client = Client::new();
    let response = client
        .post(format!("{}/tools", API_BASE))
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error = response.text().await?;
        anyhow::bail!("Publish failed: {}", error);
    }
    
    let publish_response: clawmaster_clawhub::types::PublishResponse = response.json().await?;
    
    println!(
        "{} {}",
        "✓".green().bold(),
        publish_response.message.bright_white()
    );
    println!(
        "{} {}",
        "Download URL:".dimmed(),
        publish_response.download_url
    );
    
    Ok(())
}
