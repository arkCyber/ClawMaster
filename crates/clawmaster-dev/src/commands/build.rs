//! Build project command

use anyhow::Result;
use colored::Colorize;
use std::process::Command;

pub async fn execute(release: bool) -> Result<()> {
    let mode = if release { "release" } else { "debug" };
    
    println!("{} Building project in {} mode", 
        "🔨".bright_green(), 
        mode.bright_cyan()
    );

    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    
    if release {
        cmd.arg("--release");
    }

    let output = cmd.output()?;

    if output.status.success() {
        println!("{} Build completed successfully!", "✅".bright_green());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{} Build failed:\n{}", "❌".bright_red(), stderr);
        anyhow::bail!("Build failed");
    }

    Ok(())
}
