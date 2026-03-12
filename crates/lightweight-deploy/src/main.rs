//! Lightweight deployment binary for ClawMaster
//! 
//! This is a single binary that provides easy deployment and configuration
//! inspired by MicroClaw's simple deployment experience.

use clap::{Parser, Subcommand};
use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{info, warn, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod deploy;
mod server;
mod utils;

use config::{LiteConfig, DeployMode};
use deploy::DeployManager;
use server::LiteServer;

#[derive(Parser)]
#[command(name = "clawmaster-lite")]
#[command(about = "Lightweight ClawMaster deployment")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
    
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
    
    /// Deployment mode
    #[arg(short, long, global = true, default_value = "auto")]
    mode: DeployMode,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the server
    Start {
        /// Port to bind to
        #[arg(short, long, default_value = "8080")]
        port: u16,
        
        /// Host to bind to
        #[arg(short, long, default_value = "0.0.0.0")]
        host: String,
    },
    /// Initialize configuration
    Init {
        /// Force overwrite existing config
        #[arg(long)]
        force: bool,
        
        /// Configuration template
        #[arg(long, default_value = "basic")]
        template: String,
    },
    /// Validate configuration
    Validate,
    /// Show system status
    Status,
    /// Generate quick start script
    GenerateScript {
        /// Output file path
        #[arg(short, long, default_value = "./start.sh")]
        output: PathBuf,
        
        /// Script type
        #[arg(long, default_value = "bash")]
        script_type: String,
    },
    /// Build Docker image
    BuildDocker {
        /// Image name
        #[arg(long, default_value = "clawmaster-lite")]
        name: String,
        
        /// Image tag
        #[arg(long, default_value = "latest")]
        tag: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    init_logging(cli.verbose);
    
    info!("Starting moltis-lite deployment");
    
    // Load configuration
    let config = load_config(cli.config.as_ref())?;
    
    // Execute command
    match cli.command {
        Commands::Start { port, host } => {
            start_server(config, host, port).await
        }
        Commands::Init { force, template } => {
            init_config(config, force, &template).await
        }
        Commands::Validate => {
            validate_config(config).await
        }
        Commands::Status => {
            show_status(config).await
        }
        Commands::GenerateScript { output, script_type } => {
            generate_script(config, output, &script_type).await
        }
        Commands::BuildDocker { name, tag } => {
            build_docker_image(config, &name, &tag).await
        }
    }
}

fn init_logging(verbose: bool) {
    let level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive(level.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn load_config(config_path: Option<&PathBuf>) -> Result<LiteConfig> {
    match config_path {
        Some(path) => {
            info!("Loading configuration from: {}", path.display());
            LiteConfig::from_file(path)
                .context("Failed to load configuration file")
        }
        None => {
            info!("Using default configuration");
            Ok(LiteConfig::default())
        }
    }
}

async fn start_server(config: LiteConfig, host: String, port: u16) -> Result<()> {
    info!("Starting server on {}:{}", host, port);
    
    let mut server = LiteServer::new(config);
    server.set_address(host, port);
    
    server.start().await
}

async fn init_config(mut config: LiteConfig, force: bool, template: &str) -> Result<()> {
    info!("Initializing configuration with template: {}", template);
    
    let mut deploy_manager = DeployManager::new(config);
    deploy_manager.init_config(force, template).await
}

async fn validate_config(config: LiteConfig) -> Result<()> {
    info!("Validating configuration");
    
    let deploy_manager = DeployManager::new(config);
    deploy_manager.validate().await
}

async fn show_status(config: LiteConfig) -> Result<()> {
    info!("Showing system status");
    
    let deploy_manager = DeployManager::new(config);
    deploy_manager.show_status().await
}

async fn generate_script(config: LiteConfig, output: PathBuf, script_type: &str) -> Result<()> {
    info!("Generating {} script: {}", script_type, output.display());
    
    let deploy_manager = DeployManager::new(config);
    deploy_manager.generate_script(output, script_type).await
}

async fn build_docker_image(config: LiteConfig, name: &str, tag: &str) -> Result<()> {
    info!("Building Docker image: {}:{}", name, tag);
    
    let deploy_manager = DeployManager::new(config);
    deploy_manager.build_docker_image(name, tag).await
}
