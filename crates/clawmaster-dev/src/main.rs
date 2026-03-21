//! ClawMaster Developer CLI Tool
//!
//! DO-178C Level A compliant developer tool for creating and managing
//! ClawMaster plugins, skills, and tools.

use {
    anyhow::Result,
    clap::{Parser, Subcommand},
    colored::Colorize,
};

mod commands;
mod templates;
mod utils;

use commands::*;

#[derive(Parser)]
#[command(name = "clawmaster-dev")]
#[command(about = "ClawMaster Developer CLI Tool", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init {
        /// Project name
        name: String,

        /// Project type (plugin, skill, tool)
        #[arg(short, long, default_value = "plugin")]
        r#type: String,
    },

    /// Create a new component
    New {
        /// Component type (skill, tool, plugin)
        component_type: String,

        /// Component name
        name: String,
    },

    /// Start development server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Enable hot reload
        #[arg(long, default_value = "true")]
        hot_reload: bool,
    },

    /// Build the project
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
    },

    /// Run tests
    Test {
        /// Run only specific test
        #[arg(short, long)]
        test: Option<String>,
    },

    /// Publish to marketplace
    Publish {
        /// Dry run (don't actually publish)
        #[arg(long)]
        dry_run: bool,
    },

    /// View logs
    Logs {
        /// Follow log output
        #[arg(short, long)]
        follow: bool,

        /// Number of lines to show
        #[arg(short, long, default_value = "100")]
        lines: usize,
    },

    /// Validate project
    Validate,

    /// Generate documentation
    Docs {
        /// Open in browser after generation
        #[arg(long)]
        open: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging(cli.verbose);

    // Print banner
    print_banner();

    // Execute command
    match cli.command {
        Commands::Init { name, r#type } => {
            init::execute(&name, &r#type).await?;
        },
        Commands::New {
            component_type,
            name,
        } => {
            new::execute(&component_type, &name).await?;
        },
        Commands::Serve { port, hot_reload } => {
            serve::execute(port, hot_reload).await?;
        },
        Commands::Build { release } => {
            build::execute(release).await?;
        },
        Commands::Test { test } => {
            test::execute(test.as_deref()).await?;
        },
        Commands::Publish { dry_run } => {
            publish::execute(dry_run).await?;
        },
        Commands::Logs { follow, lines } => {
            logs::execute(follow, lines).await?;
        },
        Commands::Validate => {
            validate::execute().await?;
        },
        Commands::Docs { open } => {
            docs::execute(open).await?;
        },
    }

    Ok(())
}

fn init_logging(verbose: bool) {
    use tracing_subscriber::EnvFilter;

    let filter = if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}

fn print_banner() {
    println!(
        "{}",
        r#"
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║   ██████╗██╗      █████╗ ██╗    ██╗███╗   ███╗ █████╗    ║
║  ██╔════╝██║     ██╔══██╗██║    ██║████╗ ████║██╔══██╗   ║
║  ██║     ██║     ███████║██║ █╗ ██║██╔████╔██║███████║   ║
║  ██║     ██║     ██╔══██║██║███╗██║██║╚██╔╝██║██╔══██║   ║
║  ╚██████╗███████╗██║  ██║╚███╔███╔╝██║ ╚═╝ ██║██║  ██║   ║
║   ╚═════╝╚══════╝╚═╝  ╚═╝ ╚══╝╚══╝ ╚═╝     ╚═╝╚═╝  ╚═╝   ║
║                                                           ║
║              Developer CLI Tool v0.1.0                    ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
"#
        .bright_cyan()
    );
}
