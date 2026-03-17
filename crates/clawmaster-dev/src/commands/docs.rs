//! Generate documentation command

use anyhow::Result;
use colored::Colorize;
use std::process::Command;

pub async fn execute(open: bool) -> Result<()> {
    println!("{} Generating documentation", "📚".bright_green());

    let mut cmd = Command::new("cargo");
    cmd.arg("doc");
    cmd.arg("--no-deps");

    if open {
        cmd.arg("--open");
    }

    let output = cmd.output()?;

    if output.status.success() {
        println!("{} Documentation generated successfully!", "✅".bright_green());
        
        if !open {
            println!("\nView documentation at: {}", 
                "target/doc/index.html".bright_cyan()
            );
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{} Documentation generation failed:\n{}", "❌".bright_red(), stderr);
        anyhow::bail!("Documentation generation failed");
    }

    Ok(())
}
