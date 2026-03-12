//! ClawHub CLI tool.
//!
//! Command-line interface for searching, installing, and publishing Wasm tools.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

mod commands;

#[derive(Parser)]
#[command(name = "claw")]
#[command(about = "ClawHub - Wasm Tool Package Manager", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for tools
    Search {
        /// Search query
        query: String,
        
        /// Category filter
        #[arg(short, long)]
        category: Option<String>,
    },
    
    /// Install a tool
    Install {
        /// Tool name
        name: String,
        
        /// Tool version (default: latest)
        #[arg(short, long)]
        version: Option<String>,
    },
    
    /// List installed tools
    List,
    
    /// Publish a tool
    Publish {
        /// Path to Wasm file
        wasm_file: String,
        
        /// Tool metadata file (JSON)
        #[arg(short, long)]
        metadata: String,
    },
    
    /// Show tool information
    Info {
        /// Tool name
        name: String,
        
        /// Tool version (default: latest)
        #[arg(short, long)]
        version: Option<String>,
    },
    
    /// Skills management
    #[command(subcommand)]
    Skills(commands::skills::SkillsCommands),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Search { query, category } => {
            commands::search::execute(&query, category.as_deref()).await?;
        }
        Commands::Install { name, version } => {
            commands::install::execute(&name, version.as_deref()).await?;
        }
        Commands::List => {
            commands::list::execute().await?;
        }
        Commands::Publish { wasm_file, metadata } => {
            commands::publish::execute(&wasm_file, &metadata).await?;
        }
        Commands::Info { name, version } => {
            commands::info::execute(&name, version.as_deref()).await?;
        }
        Commands::Skills(cmd) => {
            commands::skills::execute(cmd).await?;
        }
    }
    
    Ok(())
}
