//! Info command implementation.

use anyhow::Result;
use colored::Colorize;
use reqwest::Client;

const API_BASE: &str = "https://api.clawhub.io";

pub async fn execute(name: &str, version: Option<&str>) -> Result<()> {
    let version = version.unwrap_or("latest");
    
    println!(
        "{} {}@{}",
        "Fetching info for:".bright_blue().bold(),
        name,
        version
    );
    
    let client = Client::new();
    
    let url = if version == "latest" {
        format!("{}/tools/{}", API_BASE, name)
    } else {
        format!("{}/tools/{}/{}", API_BASE, name, version)
    };
    
    let response = client.get(&url).send().await?;
    
    if !response.status().is_success() {
        anyhow::bail!("Tool not found: {}@{}", name, version);
    }
    
    let tool: clawmaster_clawhub::types::ToolMetadata = response.json().await?;
    
    println!("\n{}", "Tool Information".bright_white().bold().underline());
    println!();
    println!("{} {}", "Name:".bright_cyan().bold(), tool.name);
    println!("{} {}", "Version:".bright_cyan().bold(), tool.version);
    println!("{} {}", "Description:".bright_cyan().bold(), tool.description);
    println!("{} {}", "Author:".bright_cyan().bold(), tool.author);
    
    if let Some(email) = &tool.author_email {
        println!("{} {}", "Email:".bright_cyan().bold(), email);
    }
    
    println!("{} {}", "License:".bright_cyan().bold(), tool.license);
    
    if let Some(repo) = &tool.repository {
        println!("{} {}", "Repository:".bright_cyan().bold(), repo);
    }
    
    if let Some(homepage) = &tool.homepage {
        println!("{} {}", "Homepage:".bright_cyan().bold(), homepage);
    }
    
    println!("{} {:?}", "Type:".bright_cyan().bold(), tool.tool_type);
    println!("{} {:?}", "Security:".bright_cyan().bold(), tool.security_status);
    println!("{} {}", "Downloads:".bright_cyan().bold(), tool.downloads);
    println!("{} {} bytes", "Size:".bright_cyan().bold(), tool.wasm_size);
    
    if !tool.keywords.is_empty() {
        println!("{} {}", "Keywords:".bright_cyan().bold(), tool.keywords.join(", "));
    }
    
    if !tool.categories.is_empty() {
        println!("{} {}", "Categories:".bright_cyan().bold(), tool.categories.join(", "));
    }
    
    if let Some(readme) = &tool.readme {
        println!("\n{}", "README".bright_white().bold().underline());
        println!("{}", readme);
    }
    
    Ok(())
}
