//! Development server command

use {
    anyhow::Result,
    colored::Colorize,
    notify::{Event, EventKind, RecursiveMode, Watcher},
    std::{path::Path, sync::mpsc::channel, time::Duration},
};

pub async fn execute(port: u16, hot_reload: bool) -> Result<()> {
    println!(
        "{} Starting development server on port {}",
        "🚀".bright_green(),
        port.to_string().bright_cyan()
    );

    if hot_reload {
        println!(
            "  {} Hot reload: {}",
            "✓".bright_green(),
            "enabled".bright_green()
        );
    }

    println!(
        "\n{} Server is running at http://localhost:{}",
        "✨".bright_green(),
        port
    );
    println!("Press Ctrl+C to stop\n");

    if hot_reload {
        // Setup file watcher
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        })?;

        // Watch current directory
        watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

        println!("  {} Watching for file changes...", "👀".bright_yellow());

        // Spawn file watcher task
        let watcher_task = tokio::task::spawn_blocking(move || {
            loop {
                match rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(event) => {
                        match event.kind {
                            EventKind::Modify(_) | EventKind::Create(_) => {
                                if let Some(path) = event.paths.first() {
                                    if is_source_file(path) {
                                        println!(
                                            "\n  {} File changed: {}",
                                            "🔄".bright_cyan(),
                                            path.display().to_string().bright_white()
                                        );
                                        println!("  {} Reloading...", "⚡".bright_yellow());

                                        // Trigger rebuild
                                        if let Err(e) = rebuild_project() {
                                            println!(
                                                "  {} Rebuild failed: {}",
                                                "❌".bright_red(),
                                                e
                                            );
                                        } else {
                                            println!(
                                                "  {} Rebuild successful",
                                                "✅".bright_green()
                                            );
                                        }
                                    }
                                }
                            },
                            _ => {},
                        }
                    },
                    Err(_) => {
                        // Timeout, continue
                    },
                }
            }
        });

        // Wait for Ctrl+C
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("\n{} Server stopped", "🛑".bright_red());
            }
            _ = watcher_task => {
                println!("\n{} Watcher stopped unexpectedly", "⚠️".bright_yellow());
            }
        }
    } else {
        // Just wait for Ctrl+C without watching
        tokio::signal::ctrl_c().await?;
        println!("\n{} Server stopped", "🛑".bright_red());
    }

    Ok(())
}

fn is_source_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(ext.to_str(), Some("rs") | Some("toml") | Some("md"))
    } else {
        false
    }
}

fn rebuild_project() -> Result<()> {
    use std::process::Command;

    let output = Command::new("cargo").arg("build").output()?;

    if !output.status.success() {
        anyhow::bail!("cargo build failed");
    }

    Ok(())
}
