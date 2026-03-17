//! Initialize new project command

use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::templates;
use crate::utils;

pub async fn execute(name: &str, project_type: &str) -> Result<()> {
    println!("{} Initializing new {} project: {}", 
        "✨".bright_green(), 
        project_type.bright_yellow(),
        name.bright_cyan()
    );

    // Validate project name
    if !utils::is_valid_name(name) {
        anyhow::bail!("Invalid project name. Use only alphanumeric characters, hyphens, and underscores.");
    }

    // Check if directory already exists
    if Path::new(name).exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }

    // Create project directory
    fs::create_dir_all(name)?;
    println!("  {} Created directory: {}", "✓".bright_green(), name);

    // Generate project structure based on type
    match project_type {
        "plugin" => create_plugin_project(name).await?,
        "skill" => create_skill_project(name).await?,
        "tool" => create_tool_project(name).await?,
        _ => anyhow::bail!("Unknown project type: {}", project_type),
    }

    println!("\n{} Project initialized successfully!", "🎉".bright_green());
    println!("\nNext steps:");
    println!("  cd {}", name.bright_cyan());
    println!("  clawmaster-dev serve");

    Ok(())
}

async fn create_plugin_project(name: &str) -> Result<()> {
    let base_path = Path::new(name);

    // Create directory structure
    fs::create_dir_all(base_path.join("src"))?;
    fs::create_dir_all(base_path.join("tests"))?;
    fs::create_dir_all(base_path.join("examples"))?;

    // Generate plugin.toml
    let plugin_toml = templates::plugin::generate_manifest(name);
    fs::write(base_path.join("plugin.toml"), plugin_toml)?;
    println!("  {} Created plugin.toml", "✓".bright_green());

    // Generate Cargo.toml
    let cargo_toml = templates::plugin::generate_cargo_toml(name);
    fs::write(base_path.join("Cargo.toml"), cargo_toml)?;
    println!("  {} Created Cargo.toml", "✓".bright_green());

    // Generate src/lib.rs
    let lib_rs = templates::plugin::generate_lib_rs(name);
    fs::write(base_path.join("src/lib.rs"), lib_rs)?;
    println!("  {} Created src/lib.rs", "✓".bright_green());

    // Generate README.md
    let readme = templates::common::generate_readme(name, "plugin");
    fs::write(base_path.join("README.md"), readme)?;
    println!("  {} Created README.md", "✓".bright_green());

    // Generate .gitignore
    let gitignore = templates::common::generate_gitignore();
    fs::write(base_path.join(".gitignore"), gitignore)?;
    println!("  {} Created .gitignore", "✓".bright_green());

    Ok(())
}

async fn create_skill_project(name: &str) -> Result<()> {
    let base_path = Path::new(name);

    // Create directory structure
    fs::create_dir_all(base_path)?;

    // Generate SKILL.md
    let skill_md = templates::skill::generate_skill_md(name);
    fs::write(base_path.join("SKILL.md"), skill_md)?;
    println!("  {} Created SKILL.md", "✓".bright_green());

    // Generate README.md
    let readme = templates::common::generate_readme(name, "skill");
    fs::write(base_path.join("README.md"), readme)?;
    println!("  {} Created README.md", "✓".bright_green());

    // Generate LICENSE
    let license = templates::common::generate_license();
    fs::write(base_path.join("LICENSE"), license)?;
    println!("  {} Created LICENSE", "✓".bright_green());

    Ok(())
}

async fn create_tool_project(name: &str) -> Result<()> {
    let base_path = Path::new(name);

    // Create directory structure
    fs::create_dir_all(base_path.join("src"))?;
    fs::create_dir_all(base_path.join("tests"))?;

    // Generate Cargo.toml
    let cargo_toml = templates::tool::generate_cargo_toml(name);
    fs::write(base_path.join("Cargo.toml"), cargo_toml)?;
    println!("  {} Created Cargo.toml", "✓".bright_green());

    // Generate src/lib.rs
    let lib_rs = templates::tool::generate_lib_rs(name);
    fs::write(base_path.join("src/lib.rs"), lib_rs)?;
    println!("  {} Created src/lib.rs", "✓".bright_green());

    // Generate README.md
    let readme = templates::common::generate_readme(name, "tool");
    fs::write(base_path.join("README.md"), readme)?;
    println!("  {} Created README.md", "✓".bright_green());

    Ok(())
}
