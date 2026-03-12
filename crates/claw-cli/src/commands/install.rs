//! Install command implementation.

use anyhow::Result;
use colored::Colorize;
use reqwest::Client;
use std::fs;
use std::path::PathBuf;

const API_BASE: &str = "https://api.clawhub.io";

pub async fn execute(name: &str, version: Option<&str>) -> Result<()> {
    let version = version.unwrap_or("latest");
    
    println!(
        "{} {}@{}",
        "Installing:".bright_blue().bold(),
        name,
        version
    );
    
    let client = Client::new();
    
    // Get tool metadata
    let url = if version == "latest" {
        format!("{}/tools/{}", API_BASE, name)
    } else {
        format!("{}/tools/{}/{}", API_BASE, name, version)
    };
    
    let response = client.get(&url).send().await?;
    
    if !response.status().is_success() {
        anyhow::bail!("Tool not found: {}@{}", name, version);
    }
    
    let metadata: clawmaster_clawhub::types::ToolMetadata = response.json().await?;
    
    println!("{} Downloading...", "→".bright_cyan());
    
    // Download Wasm file
    let download_url = format!(
        "{}/tools/{}/{}/download",
        API_BASE, metadata.name, metadata.version
    );
    
    let wasm_bytes = client
        .get(&download_url)
        .send()
        .await?
        .bytes()
        .await?;
    
    // Save to local directory
    let install_dir = get_install_dir()?;
    fs::create_dir_all(&install_dir)?;
    
    let tool_path = install_dir.join(format!("{}.wasm", metadata.name));
    fs::write(&tool_path, wasm_bytes)?;
    
    println!(
        "{} Installed {}@{} to {}",
        "✓".green().bold(),
        metadata.name,
        metadata.version,
        tool_path.display()
    );
    
    Ok(())
}

fn get_install_dir() -> Result<PathBuf> {
    let home = dirs_next::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
    Ok(home.join(".clawhub").join("tools"))
}
