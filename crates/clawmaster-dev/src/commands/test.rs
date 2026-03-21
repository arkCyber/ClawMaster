//! Test command

use {anyhow::Result, colored::Colorize, std::process::Command};

pub async fn execute(test_name: Option<&str>) -> Result<()> {
    println!("{} Running tests", "🧪".bright_green());

    let mut cmd = Command::new("cargo");
    cmd.arg("test");

    if let Some(name) = test_name {
        cmd.arg(name);
        println!(
            "  {} Test filter: {}",
            "🔍".bright_yellow(),
            name.bright_cyan()
        );
    }

    let output = cmd.output()?;

    if output.status.success() {
        println!("{} All tests passed!", "✅".bright_green());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{} Tests failed:\n{}", "❌".bright_red(), stderr);
        anyhow::bail!("Tests failed");
    }

    Ok(())
}
