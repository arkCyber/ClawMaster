//! Validation command

use {
    anyhow::Result,
    colored::Colorize,
    std::{path::Path, process::Command},
};

pub async fn execute() -> Result<()> {
    println!("{} Validating project", "🔍".bright_green());

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // 1. Check project structure
    println!("\n  {} Checking project structure...", "📁".bright_yellow());
    match check_project_structure() {
        Ok(msgs) => {
            for msg in msgs {
                println!("    {} {}", "✓".bright_green(), msg);
            }
        },
        Err(e) => {
            errors.push(format!("Project structure: {}", e));
            println!("    {} {}", "✗".bright_red(), e);
        },
    }

    // 2. Check dependencies
    println!("\n  {} Checking dependencies...", "📦".bright_yellow());
    match check_dependencies() {
        Ok(msgs) => {
            for msg in msgs {
                println!("    {} {}", "✓".bright_green(), msg);
            }
        },
        Err(e) => {
            warnings.push(format!("Dependencies: {}", e));
            println!("    {} {}", "⚠".bright_yellow(), e);
        },
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

fn check_project_structure() -> Result<Vec<String>> {
    let mut messages = Vec::new();

    let required_files = vec![
        ("Cargo.toml", "Cargo manifest"),
        ("README.md", "README file"),
    ];

    for (file, desc) in required_files {
        if Path::new(file).exists() {
            messages.push(format!("{} exists", desc));
        } else {
            anyhow::bail!("Missing {}", desc);
        }
    }

    if Path::new("plugin.toml").exists() {
        messages.push("Plugin manifest exists".to_string());
    } else if Path::new("SKILL.md").exists() {
        messages.push("Skill manifest exists".to_string());
    } else {
        anyhow::bail!("Missing plugin.toml or SKILL.md");
    }

    if Path::new("src").exists() {
        messages.push("Source directory exists".to_string());
    } else {
        anyhow::bail!("Missing src directory");
    }

    Ok(messages)
}

fn check_dependencies() -> Result<Vec<String>> {
    let output = Command::new("cargo").arg("tree").output()?;

    if !output.status.success() {
        anyhow::bail!("Failed to check dependencies");
    }

    Ok(vec!["Dependencies are valid".to_string()])
}
