//! Validate project command

use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub async fn execute() -> Result<()> {
    println!("{} Validating project", "🔍".bright_green());

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check for required files
    if !Path::new("Cargo.toml").exists() && !Path::new("plugin.toml").exists() && !Path::new("SKILL.md").exists() {
        errors.push("No project manifest found (Cargo.toml, plugin.toml, or SKILL.md)");
    }

    // Check for README
    if !Path::new("README.md").exists() {
        warnings.push("README.md not found");
    }

    // Check for LICENSE
    if !Path::new("LICENSE").exists() && !Path::new("LICENSE.md").exists() {
        warnings.push("LICENSE file not found");
    }

    // Print results
    if !warnings.is_empty() {
        println!("\n{} Warnings:", "⚠️".bright_yellow());
        for warning in &warnings {
            println!("  - {}", warning.yellow());
        }
    }

    if !errors.is_empty() {
        println!("\n{} Errors:", "❌".bright_red());
        for error in &errors {
            println!("  - {}", error.red());
        }
        anyhow::bail!("Validation failed with {} error(s)", errors.len());
    }

    println!("\n{} Validation passed!", "✅".bright_green());
    Ok(())
}
