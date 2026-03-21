//! Create new component command

use {
    anyhow::Result,
    colored::Colorize,
    std::{fs, path::Path},
};

use crate::{templates, utils};

pub async fn execute(component_type: &str, name: &str) -> Result<()> {
    println!(
        "{} Creating new {}: {}",
        "✨".bright_green(),
        component_type.bright_yellow(),
        name.bright_cyan()
    );

    // Validate name
    if !utils::is_valid_name(name) {
        anyhow::bail!(
            "Invalid component name. Use only alphanumeric characters, hyphens, and underscores."
        );
    }

    match component_type {
        "skill" => create_skill(name).await?,
        "tool" => create_tool(name).await?,
        "plugin" => create_plugin(name).await?,
        _ => anyhow::bail!(
            "Unknown component type: {}. Use 'skill', 'tool', or 'plugin'.",
            component_type
        ),
    }

    println!("\n{} Component created successfully!", "🎉".bright_green());
    Ok(())
}

async fn create_skill(name: &str) -> Result<()> {
    let skill_path = Path::new("skills").join(name);

    if skill_path.exists() {
        anyhow::bail!("Skill directory already exists: {:?}", skill_path);
    }

    fs::create_dir_all(&skill_path)?;

    let skill_md = templates::skill::generate_skill_md(name);
    fs::write(skill_path.join("SKILL.md"), skill_md)?;
    println!("  {} Created skills/{}/SKILL.md", "✓".bright_green(), name);

    Ok(())
}

async fn create_tool(name: &str) -> Result<()> {
    let tool_path = Path::new("src").join(format!("{}_tool.rs", name.replace("-", "_")));

    if tool_path.exists() {
        anyhow::bail!("Tool file already exists: {:?}", tool_path);
    }

    let tool_code = templates::tool::generate_tool_code(name);
    fs::write(&tool_path, tool_code)?;
    println!("  {} Created {:?}", "✓".bright_green(), tool_path);

    Ok(())
}

async fn create_plugin(name: &str) -> Result<()> {
    let plugin_path = Path::new("plugins").join(name);

    if plugin_path.exists() {
        anyhow::bail!("Plugin directory already exists: {:?}", plugin_path);
    }

    fs::create_dir_all(plugin_path.join("src"))?;

    let plugin_toml = templates::plugin::generate_manifest(name);
    fs::write(plugin_path.join("plugin.toml"), plugin_toml)?;
    println!(
        "  {} Created plugins/{}/plugin.toml",
        "✓".bright_green(),
        name
    );

    let lib_rs = templates::plugin::generate_lib_rs(name);
    fs::write(plugin_path.join("src/lib.rs"), lib_rs)?;
    println!(
        "  {} Created plugins/{}/src/lib.rs",
        "✓".bright_green(),
        name
    );

    Ok(())
}
