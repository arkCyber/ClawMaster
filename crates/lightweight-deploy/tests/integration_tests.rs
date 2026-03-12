//! Integration tests for lightweight deployment

use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;
use clawmaster_lightweight_deploy::config::{LiteConfig, DeployMode, DatabaseType};

#[tokio::test]
async fn test_config_template_generation() -> Result<()> {
    // Test basic template
    let basic_config = LiteConfig::from_template("basic")?;
    assert!(matches!(basic_config.mode, DeployMode::Auto));
    assert!(basic_config.channels.discord);
    assert!(basic_config.channels.telegram);
    
    // Test development template
    let dev_config = LiteConfig::from_template("development")?;
    assert!(matches!(dev_config.mode, DeployMode::Development));
    assert_eq!(dev_config.server.host, "127.0.0.1");
    assert_eq!(dev_config.server.port, 3000);
    assert_eq!(dev_config.logging.level, "debug");
    
    // Test production template
    let prod_config = LiteConfig::from_template("production")?;
    assert!(matches!(prod_config.mode, DeployMode::Production));
    assert!(matches!(prod_config.database.db_type, DatabaseType::PostgreSQL));
    assert!(prod_config.security.cors_origins.is_empty());
    
    // Test minimal template
    let minimal_config = LiteConfig::from_template("minimal")?;
    assert!(matches!(minimal_config.mode, DeployMode::Minimal));
    assert!(!minimal_config.features.agent_loop_enabled);
    assert!(!minimal_config.features.chat_catchup_enabled);
    
    // Test enterprise template
    let ent_config = LiteConfig::from_template("enterprise")?;
    assert!(matches!(ent_config.mode, DeployMode::Enterprise));
    assert!(ent_config.channels.discord);
    assert!(ent_config.channels.telegram);
    assert!(ent_config.channels.slack);
    assert!(ent_config.channels.whatsapp);
    
    Ok(())
}

#[tokio::test]
async fn test_config_validation() -> Result<()> {
    let mut config = LiteConfig::default();
    
    // Valid configuration should pass
    assert!(config.validate().is_ok());
    
    // Invalid port should fail
    config.server.port = 0;
    assert!(config.validate().is_err());
    
    // Reset port
    config.server.port = 8080;
    
    // PostgreSQL without URL should fail
    config.database.db_type = DatabaseType::PostgreSQL;
    assert!(config.validate().is_err());
    
    // Add URL should pass
    config.database.url = Some("postgresql://user:pass@localhost/db".to_string());
    assert!(config.validate().is_ok());
    
    // Auth enabled without JWT secret should fail
    config.security.auth_enabled = true;
    config.security.jwt_secret = None;
    assert!(config.validate().is_err());
    
    // Add JWT secret should pass
    config.security.jwt_secret = Some("secret".to_string());
    assert!(config.validate().is_ok());
    
    Ok(())
}

#[tokio::test]
async fn test_config_file_operations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("test_config.toml");
    
    // Create test configuration
    let config = LiteConfig::from_template("development")?;
    
    // Save to file
    config.save_to_file(&config_path)?;
    assert!(config_path.exists());
    
    // Load from file
    let loaded_config = LiteConfig::from_file(&config_path)?;
    assert_eq!(loaded_config.server.host, config.server.host);
    assert_eq!(loaded_config.server.port, config.server.port);
    assert_eq!(loaded_config.mode, config.mode);
    
    Ok(())
}

#[tokio::test]
async fn test_deploy_manager_initialization() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("clawmaster-lite.toml");
    
    // Create deploy manager
    let config = LiteConfig::from_template("basic")?;
    let mut deploy_manager = clawmaster_lightweight_deploy::deploy::DeployManager::new(config);
    
    // Initialize config
    deploy_manager.init_config(true, "basic").await?;
    
    // Check if config file was created
    assert!(config_path.exists());
    
    // Validate config
    deploy_manager.validate().await?;
    
    // Show status (should not panic)
    deploy_manager.show_status().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_script_generation() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config = LiteConfig::from_template("basic")?;
    let deploy_manager = clawmaster_lightweight_deploy::deploy::DeployManager::new(config);
    
    // Test bash script generation
    let bash_script_path = temp_dir.path().join("deploy.sh");
    deploy_manager.generate_script(bash_script_path.clone(), "bash").await?;
    assert!(bash_script_path.exists());
    
    let bash_content = fs::read_to_string(&bash_script_path)?;
    assert!(bash_content.contains("#!/bin/bash"));
    assert!(bash_content.contains("clawmaster-lite start"));
    
    // Test PowerShell script generation
    let ps_script_path = temp_dir.path().join("deploy.ps1");
    deploy_manager.generate_script(ps_script_path.clone(), "powershell").await?;
    assert!(ps_script_path.exists());
    
    let ps_content = fs::read_to_string(&ps_script_path)?;
    assert!(ps_content.contains("clawmaster-lite start"));
    
    // Test Docker Compose generation
    let docker_compose_path = temp_dir.path().join("docker-compose.yml");
    deploy_manager.generate_script(docker_compose_path.clone(), "docker").await?;
    assert!(docker_compose_path.exists());
    
    let docker_content = fs::read_to_string(&docker_compose_path)?;
    assert!(docker_content.contains("version: '3.8'"));
    assert!(docker_content.contains("clawmaster-lite:"));
    
    Ok(())
}

#[tokio::test]
async fn test_docker_generation() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config = LiteConfig::from_template("production")?;
    let deploy_manager = clawmaster_lightweight_deploy::deploy::DeployManager::new(config);
    
    // Generate Docker files
    deploy_manager.build_docker_image("test-moltis", "v1.0").await?;
    
    // Check if Dockerfile was created
    let dockerfile_path = temp_dir.path().join("Dockerfile");
    assert!(dockerfile_path.exists());
    
    let dockerfile_content = fs::read_to_string(&dockerfile_path)?;
    assert!(dockerfile_content.contains("FROM rust:"));
    assert!(dockerfile_content.contains("clawmaster-lite"));
    
    // Check if .dockerignore was created
    let dockerignore_path = temp_dir.path().join(".dockerignore");
    assert!(dockerignore_path.exists());
    
    let dockerignore_content = fs::read_to_string(&dockerignore_path)?;
    assert!(dockerignore_content.contains("target/"));
    assert!(dockerignore_content.contains(".git/"));
    
    Ok(())
}

#[tokio::test]
async fn test_server_creation() -> Result<()> {
    let config = LiteConfig::from_template("minimal")?;
    let mut server = clawmaster_lightweight_deploy::server::LiteServer::new(config);
    
    // Set custom address
    server.set_address("127.0.0.1".to_string(), 9999);
    
    // Get status (should not panic)
    let status = server.get_status().await;
    assert!(!status.running);
    assert_eq!(status.address, "127.0.0.1:9999");
    
    Ok(())
}

#[tokio::test]
async fn test_utility_functions() -> Result<()> {
    use clawmaster_lightweight_deploy::utils::*;
    
    // Test directory operations
    let temp_dir = TempDir::new()?;
    let test_dir = temp_dir.path().join("test_subdir");
    
    ensure_directory(&test_dir)?;
    assert!(test_dir.exists());
    
    // Test file operations
    let test_file = temp_dir.path().join("test_file.txt");
    fs::write(&test_file, "test content")?;
    assert!(check_file_readable(&test_file)?);
    
    // Test random string generation
    let random1 = generate_random_string(10);
    let random2 = generate_random_string(10);
    assert_eq!(random1.len(), 10);
    assert_eq!(random2.len(), 10);
    assert_ne!(random1, random2);
    
    // Test port availability
    let available_port = find_available_port(30000)?;
    assert!(is_port_available(available_port)?);
    
    // Test URL validation
    assert!(validate_url("http://localhost:8080").is_ok());
    assert!(validate_url("https://example.com").is_ok());
    assert!(validate_url("invalid-url").is_err());
    
    // Test formatting functions
    assert_eq!(format_bytes(0), "0 B");
    assert_eq!(format_bytes(1024), "1.0 KB");
    assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
    
    assert_eq!(format_duration(std::time::Duration::from_secs(30)), "30s");
    assert_eq!(format_duration(std::time::Duration::from_secs(90)), "1m 30s");
    assert_eq!(format_duration(std::time::Duration::from_secs(3661)), "1h 1m 1s");
    
    // Test system requirements
    let sys_info = check_system_requirements()?;
    assert!(!sys_info.os.is_empty());
    assert!(!sys_info.arch.is_empty());
    
    let recommendations = sys_info.get_recommendations();
    assert!(!recommendations.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_deploy_mode_parsing() -> Result<()> {
    // Test string parsing
    assert!(matches!("auto".parse::<DeployMode>()?, DeployMode::Auto));
    assert!(matches!("development".parse::<DeployMode>()?, DeployMode::Development));
    assert!(matches!("dev".parse::<DeployMode>()?, DeployMode::Development));
    assert!(matches!("production".parse::<DeployMode>()?, DeployMode::Production));
    assert!(matches!("prod".parse::<DeployMode>()?, DeployMode::Production));
    assert!(matches!("minimal".parse::<DeployMode>()?, DeployMode::Minimal));
    assert!(matches!("enterprise".parse::<DeployMode>()?, DeployMode::Enterprise));
    assert!(matches!("ent".parse::<DeployMode>()?, DeployMode::Enterprise));
    
    // Test invalid parsing
    assert!("invalid".parse::<DeployMode>().is_err());
    
    // Test display formatting
    assert_eq!(DeployMode::Auto.to_string(), "auto");
    assert_eq!(DeployMode::Development.to_string(), "development");
    assert_eq!(DeployMode::Production.to_string(), "production");
    assert_eq!(DeployMode::Minimal.to_string(), "minimal");
    assert_eq!(DeployMode::Enterprise.to_string(), "enterprise");
    
    Ok(())
}

#[tokio::test]
async fn test_feature_flags() -> Result<()> {
    let mut config = LiteConfig::from_template("minimal")?;
    
    // Minimal template should have limited features
    assert!(config.features.web_enabled);
    assert!(config.features.api_enabled);
    assert!(!config.features.agent_loop_enabled);
    assert!(!config.features.chat_catchup_enabled);
    assert!(!config.features.rate_limiting_enabled);
    assert!(!config.features.metrics_enabled);
    assert!(!config.features.tracing_enabled);
    
    // Enterprise template should have all features
    let ent_config = LiteConfig::from_template("enterprise")?;
    assert!(ent_config.features.web_enabled);
    assert!(ent_config.features.api_enabled);
    assert!(ent_config.features.agent_loop_enabled);
    assert!(ent_config.features.chat_catchup_enabled);
    assert!(ent_config.features.rate_limiting_enabled);
    assert!(ent_config.features.metrics_enabled);
    assert!(ent_config.features.tracing_enabled);
    
    Ok(())
}

#[tokio::test]
async fn test_channel_configuration() -> Result<()> {
    let config = LiteConfig::from_template("basic")?;
    
    // Basic template should have Discord and Telegram
    assert!(config.channels.discord);
    assert!(config.channels.telegram);
    assert!(!config.channels.slack);
    assert!(!config.channels.matrix);
    assert!(!config.channels.whatsapp);
    assert!(!config.channels.wechat);
    assert!(!config.channels.dingtalk);
    assert!(!config.channels.feishu);
    
    // Enterprise template should have all channels
    let ent_config = LiteConfig::from_template("enterprise")?;
    assert!(ent_config.channels.discord);
    assert!(ent_config.channels.telegram);
    assert!(ent_config.channels.slack);
    assert!(ent_config.channels.matrix);
    assert!(ent_config.channels.whatsapp);
    assert!(ent_config.channels.wechat);
    assert!(ent_config.channels.dingtalk);
    assert!(ent_config.channels.feishu);
    
    Ok(())
}
