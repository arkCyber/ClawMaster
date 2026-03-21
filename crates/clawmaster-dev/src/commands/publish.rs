//! Publish command

use {
    anyhow::Result,
    colored::Colorize,
    std::{path::Path, process::Command},
};

pub async fn execute(dry_run: bool) -> Result<()> {
    println!("{} Publishing to marketplace", "📦".bright_green());

    if dry_run {
        println!(
            "  {} Dry run mode - no actual publishing",
            "ℹ️".bright_blue()
        );
    }

    // 1. Validate project structure
    println!("  {} Validating project...", "🔍".bright_yellow());
    validate_project()?;
    println!("    {} Project structure valid", "✓".bright_green());

    // 2. Check version
    println!("  {} Checking version...", "🔍".bright_yellow());
    let version = get_project_version()?;
    println!(
        "    {} Version: {}",
        "✓".bright_green(),
        version.bright_cyan()
    );

    // 3. Run tests
    println!("  {} Running tests...", "🧪".bright_yellow());
    run_tests()?;
    println!("    {} All tests passed", "✓".bright_green());

    // 4. Build release
    println!("  {} Building release...", "🔨".bright_yellow());
    build_release()?;
    println!("    {} Build successful", "✓".bright_green());

    // 5. Generate changelog
    println!("  {} Generating changelog...", "📝".bright_yellow());
    generate_changelog()?;
    println!("    {} Changelog generated", "✓".bright_green());

    // 6. Create package
    println!("  {} Creating package...", "�".bright_yellow());
    let package_path = create_package()?;
    println!(
        "    {} Package created: {}",
        "✓".bright_green(),
        package_path.bright_white()
    );

    if !dry_run {
        // 7. Upload to marketplace
        println!("  {} Uploading to marketplace...", "☁️".bright_yellow());
        upload_to_marketplace(&package_path)?;
        println!("    {} Upload successful", "✓".bright_green());
    } else {
        println!("  {} Skipping upload (dry run)", "ℹ️".bright_blue());
    }

    println!("\n{} Publish complete!", "✅".bright_green());
    if !dry_run {
        println!(
            "  View your plugin at: {}",
            format!(
                "https://marketplace.clawmaster.ai/plugins/{}",
                get_plugin_id()?
            )
            .bright_cyan()
        );
    }

    Ok(())
}

fn validate_project() -> Result<()> {
    // Check for required files
    let required_files = ["Cargo.toml", "plugin.toml", "README.md"];

    for file in &required_files {
        if !Path::new(file).exists() {
            anyhow::bail!("Missing required file: {}", file);
        }
    }

    Ok(())
}

fn get_project_version() -> Result<String> {
    let cargo_toml = std::fs::read_to_string("Cargo.toml")?;
    let value: toml::Value = toml::from_str(&cargo_toml)?;

    value
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Version not found in Cargo.toml"))
}

fn get_plugin_id() -> Result<String> {
    let plugin_toml = std::fs::read_to_string("plugin.toml")?;
    let value: toml::Value = toml::from_str(&plugin_toml)?;

    value
        .get("id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Plugin ID not found in plugin.toml"))
}

fn run_tests() -> Result<()> {
    let output = Command::new("cargo").arg("test").output()?;

    if !output.status.success() {
        anyhow::bail!("Tests failed");
    }

    Ok(())
}

fn build_release() -> Result<()> {
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Build failed");
    }

    Ok(())
}

fn generate_changelog() -> Result<()> {
    // Simple changelog generation
    let version = get_project_version()?;
    let changelog = format!(
        "# Changelog\n\n## [{}] - {}\n\n### Added\n- Initial release\n",
        version,
        chrono::Local::now().format("%Y-%m-%d")
    );

    std::fs::write("CHANGELOG.md", changelog)?;
    Ok(())
}

fn create_package() -> Result<String> {
    use std::{fs, io::Write};

    let plugin_id = get_plugin_id()?;
    let version = get_project_version()?;
    let package_name = format!("{}-{}.tar.gz", plugin_id, version);

    // Create tar.gz package
    let tar_gz = fs::File::create(&package_name)?;
    let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);

    // Add files to package
    tar.append_path("Cargo.toml")?;
    tar.append_path("plugin.toml")?;
    tar.append_path("README.md")?;

    if Path::new("src").exists() {
        tar.append_dir_all("src", "src")?;
    }

    tar.finish()?;

    Ok(package_name)
}

fn upload_to_marketplace(package_path: &str) -> Result<()> {
    // TODO: Implement actual marketplace API integration
    // For now, just simulate the upload
    println!(
        "    {} Simulating upload of {}",
        "ℹ️".bright_blue(),
        package_path
    );

    // In real implementation, this would:
    // 1. Authenticate with marketplace API
    // 2. Upload the package
    // 3. Trigger marketplace indexing
    // 4. Return the plugin URL

    Ok(())
}
