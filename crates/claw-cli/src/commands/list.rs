//! List command implementation.

use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub async fn execute() -> Result<()> {
    println!("{}", "Installed tools:".bright_blue().bold());
    
    let install_dir = get_install_dir()?;
    
    if !install_dir.exists() {
        println!("{}", "No tools installed yet.".yellow());
        return Ok(());
    }
    
    let entries = fs::read_dir(&install_dir)?;
    let mut count = 0;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            let size = entry.metadata()?.len();
            
            println!(
                "{} {} {}",
                "●".bright_cyan(),
                name.bright_white().bold(),
                format!("({} bytes)", size).dimmed()
            );
            
            count += 1;
        }
    }
    
    if count == 0 {
        println!("{}", "No tools installed yet.".yellow());
    } else {
        println!("\n{} {} tools installed", "✓".green().bold(), count);
    }
    
    Ok(())
}

fn get_install_dir() -> Result<PathBuf> {
    let home = dirs_next::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
    Ok(home.join(".clawhub").join("tools"))
}
