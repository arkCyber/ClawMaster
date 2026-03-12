//! Search command implementation.

use anyhow::Result;
use colored::Colorize;
use clawmaster_clawhub::types::{SearchQuery, SortOrder};
use reqwest::Client;

const API_BASE: &str = "https://api.clawhub.io";

pub async fn execute(query: &str, category: Option<&str>) -> Result<()> {
    println!("{} {}", "Searching for:".bright_blue().bold(), query);
    
    let client = Client::new();
    
    let search_query = SearchQuery {
        query: Some(query.to_string()),
        category: category.map(String::from),
        tool_type: None,
        security_status: None,
        sort: SortOrder::Downloads,
        page: 0,
        page_size: 20,
    };
    
    let response = client
        .get(format!("{}/search", API_BASE))
        .query(&search_query)
        .send()
        .await?;
    
    if !response.status().is_success() {
        anyhow::bail!("Search failed: {}", response.status());
    }
    
    let tools: Vec<clawmaster_clawhub::types::ToolMetadata> = response.json().await?;
    
    if tools.is_empty() {
        println!("{}", "No tools found.".yellow());
        return Ok(());
    }
    
    println!("\n{} {} tools found:\n", "✓".green().bold(), tools.len());
    
    for tool in tools {
        println!("{} {}", "●".bright_cyan(), tool.name.bright_white().bold());
        println!("  {} {}", "Version:".dimmed(), tool.version);
        println!("  {} {}", "Description:".dimmed(), tool.description);
        println!("  {} {}", "Downloads:".dimmed(), tool.downloads);
        println!("  {} {:?}", "Type:".dimmed(), tool.tool_type);
        println!();
    }
    
    Ok(())
}
