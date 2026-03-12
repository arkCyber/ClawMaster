//! Quick start example for moltis-lite

use anyhow::Result;
use std::path::PathBuf;
use clawmaster_lightweight_deploy::config::{LiteConfig, DeployMode};
use clawmaster_lightweight_deploy::deploy::DeployManager;
use clawmaster_lightweight_deploy::server::LiteServer;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== ClawMaster Lite Quick Start ===\n");

    // Step 1: Create configuration
    println!("1. Creating configuration...");
    let config = create_quick_config().await?;
    println!("✅ Configuration created");

    // Step 2: Validate configuration
    println!("\n2. Validating configuration...");
    validate_config(&config).await?;
    println!("✅ Configuration is valid");

    // Step 3: Show system status
    println!("\n3. Checking system status...");
    show_system_status(&config).await?;
    println!("✅ System ready");

    // Step 4: Generate deployment script
    println!("\n4. Generating deployment script...");
    generate_deployment_script(&config).await?;
    println!("✅ Deployment script generated");

    // Step 5: Start server (commented out for demo)
    println!("\n5. Server startup (demo mode)...");
    demonstrate_server_startup(&config).await?;
    println!("✅ Server ready to start");

    println!("\n=== Quick Start Complete ===");
    println!("\nNext steps:");
    println!("1. Run: moltis-lite start");
    println!("2. Open: http://localhost:8080");
    println!("3. Configure your channels");
    println!("4. Start chatting!");

    Ok(())
}

/// Create a quick configuration for demo
async fn create_quick_config() -> Result<LiteConfig> {
    println!("   Using basic template for quick start");
    
    let mut config = LiteConfig::from_template("basic")?;
    
    // Customize for quick start
    config.server.host = "127.0.0.1".to_string();
    config.server.port = 8080;
    config.server.base_url = "http://localhost:8080".to_string();
    
    // Enable debug logging for demo
    config.logging.level = "debug".to_string();
    config.logging.console_enabled = true;
    
    // Enable all features for demo
    config.features.agent_loop_enabled = true;
    config.features.chat_catchup_enabled = true;
    config.features.metrics_enabled = true;
    
    // Save configuration
    let config_path = PathBuf::from("./quick-start-config.toml");
    config.save_to_file(&config_path)?;
    println!("   Configuration saved to: {}", config_path.display());
    
    Ok(config)
}

/// Validate the configuration
async fn validate_config(config: &LiteConfig) -> Result<()> {
    config.validate()?;
    println!("   All configuration checks passed");
    Ok(())
}

/// Show system status
async fn show_system_status(config: &LiteConfig) -> Result<()> {
    let deploy_manager = DeployManager::new(config.clone());
    
    // Show configuration summary
    println!("   Configuration:");
    println!("     Mode: {}", config.mode);
    println!("     Server: {}:{}", config.server.host, config.server.port);
    println!("     Database: {:?}", config.database.db_type);
    
    // Show enabled channels
    let enabled_channels = [
        ("Discord", config.channels.discord),
        ("Telegram", config.channels.telegram),
        ("Slack", config.channels.slack),
        ("Matrix", config.channels.matrix),
    ]
    .iter()
    .filter_map(|(name, enabled)| if *enabled { Some(*name) } else { None })
    .collect::<Vec<_>>();
    
    println!("     Channels: {}", enabled_channels.join(", "));
    
    // Show enabled features
    println!("   Features:");
    println!("     Web Interface: {}", if config.features.web_enabled { "✅" } else { "❌" });
    println!("     API: {}", if config.features.api_enabled { "✅" } else { "❌" });
    println!("     Agent Loop: {}", if config.features.agent_loop_enabled { "✅" } else { "❌" });
    println!("     Chat Catchup: {}", if config.features.chat_catchup_enabled { "✅" } else { "❌" });
    println!("     Rate Limiting: {}", if config.features.rate_limiting_enabled { "✅" } else { "❌" });
    println!("     Metrics: {}", if config.features.metrics_enabled { "✅" } else { "❌" });
    
    Ok(())
}

/// Generate deployment script
async fn generate_deployment_script(config: &LiteConfig) -> Result<()> {
    let deploy_manager = DeployManager::new(config.clone());
    
    // Generate bash script
    let script_path = PathBuf::from("./quick-start.sh");
    deploy_manager.generate_script(script_path.clone(), "bash").await?;
    
    println!("   Bash script: {}", script_path.display());
    
    // Also generate PowerShell script for Windows users
    let ps_script_path = PathBuf::from("./quick-start.ps1");
    deploy_manager.generate_script(ps_script_path.clone(), "powershell").await?;
    
    println!("   PowerShell script: {}", ps_script_path.display());
    
    // Generate Docker Compose
    let docker_compose_path = PathBuf::from("./docker-compose.yml");
    deploy_manager.generate_script(docker_compose_path.clone(), "docker").await?;
    
    println!("   Docker Compose: {}", docker_compose_path.display());
    
    Ok(())
}

/// Demonstrate server startup (without actually starting)
async fn demonstrate_server_startup(config: &LiteConfig) -> Result<()> {
    println!("   Server configuration:");
    println!("     Host: {}", config.server.host);
    println!("     Port: {}", config.server.port);
    println!("     Base URL: {}", config.server.base_url);
    
    println!("   Would start with:");
    println!("     moltis-lite start --host {} --port {}", config.server.host, config.server.port);
    
    // Create server instance to show status
    let server = LiteServer::new(config.clone());
    let status = server.get_status().await;
    
    println!("   Server status:");
    println!("     Running: {}", status.running);
    println!("     Version: {}", status.version);
    println!("     Address: {}", status.address);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quick_config_creation() -> Result<()> {
        let config = create_quick_config().await?;
        
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert!(config.features.agent_loop_enabled);
        assert!(config.features.chat_catchup_enabled);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_system_status_display() -> Result<()> {
        let config = LiteConfig::from_template("basic")?;
        show_system_status(&config).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_script_generation() -> Result<()> {
        let config = LiteConfig::from_template("basic")?;
        generate_deployment_script(&config).await?;
        
        // Check if files were created
        assert!(std::path::Path::new("./quick-start.sh").exists());
        assert!(std::path::Path::new("./quick-start.ps1").exists());
        assert!(std::path::Path::new("./docker-compose.yml").exists());
        
        // Cleanup
        std::fs::remove_file("./quick-start.sh")?;
        std::fs::remove_file("./quick-start.ps1")?;
        std::fs::remove_file("./docker-compose.yml")?;
        
        Ok(())
    }
}
