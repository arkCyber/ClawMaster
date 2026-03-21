use {
    colored::*,
    std::{fmt, path::PathBuf},
};

#[derive(Debug)]
pub enum UserError {
    ConfigNotFound {
        expected_path: PathBuf,
        suggestion: String,
    },
    ApiKeyMissing {
        provider: String,
        help_url: String,
    },
    PermissionDenied {
        resource: String,
        required_permission: String,
    },
    PortInUse {
        port: u16,
        suggestion: String,
    },
    DatabaseError {
        operation: String,
        suggestion: String,
    },
    ProviderUnavailable {
        provider: String,
        reason: String,
        alternatives: Vec<String>,
    },
    ChannelError {
        channel: String,
        error: String,
        fix_steps: Vec<String>,
    },
    InvalidConfiguration {
        field: String,
        expected: String,
        got: String,
    },
    SetupRequired {
        reason: String,
    },
    DependencyMissing {
        dependency: String,
        install_command: String,
    },
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConfigNotFound {
                expected_path,
                suggestion,
            } => {
                writeln!(f, "{}", "❌ Configuration file not found".red().bold())?;
                writeln!(f, "   {}: {}", "Expected".dimmed(), expected_path.display())?;
                writeln!(f, "   {} {}", "💡".yellow(), suggestion)?;
            },

            Self::ApiKeyMissing { provider, help_url } => {
                writeln!(
                    f,
                    "{}",
                    format!("❌ API key for {} is missing", provider)
                        .red()
                        .bold()
                )?;
                writeln!(
                    f,
                    "   {} Set your API key in one of these ways:",
                    "💡".yellow()
                )?;
                writeln!(
                    f,
                    "      1. Environment variable: {}={}",
                    format!("{}_API_KEY", provider.to_uppercase()).green(),
                    "sk-your-key".dimmed()
                )?;
                writeln!(
                    f,
                    "      2. Run: {}",
                    format!(
                        "clawmaster provider add {} --key YOUR_KEY",
                        provider.to_lowercase()
                    )
                    .green()
                )?;
                writeln!(
                    f,
                    "   {} Get your API key at: {}",
                    "🔗".blue(),
                    help_url.cyan()
                )?;
            },

            Self::PermissionDenied {
                resource,
                required_permission,
            } => {
                writeln!(f, "{}", "❌ Permission denied".red().bold())?;
                writeln!(f, "   {}: {}", "Resource".dimmed(), resource)?;
                writeln!(
                    f,
                    "   {}: {}",
                    "Required permission".dimmed(),
                    required_permission
                )?;
                writeln!(
                    f,
                    "   {} Check your access rights or contact an administrator",
                    "💡".yellow()
                )?;
            },

            Self::PortInUse { port, suggestion } => {
                writeln!(
                    f,
                    "{}",
                    format!("❌ Port {} is already in use", port).red().bold()
                )?;
                writeln!(f, "   {} Check what's using the port:", "🔍".blue())?;
                writeln!(f, "      {}", format!("lsof -i :{}", port).green())?;
                writeln!(f, "   {} {}", "💡".yellow(), suggestion)?;
            },

            Self::DatabaseError {
                operation,
                suggestion,
            } => {
                writeln!(
                    f,
                    "{}",
                    format!("❌ Database error during: {}", operation)
                        .red()
                        .bold()
                )?;
                writeln!(f, "   {} {}", "💡".yellow(), suggestion)?;
                writeln!(f, "   {} Try these recovery steps:", "🔧".blue())?;
                writeln!(f, "      1. {}", "clawmaster db status".green())?;
                writeln!(f, "      2. {}", "clawmaster db migrate".green())?;
                writeln!(
                    f,
                    "      3. {}",
                    "clawmaster backup restore <backup-file>".green()
                )?;
            },

            Self::ProviderUnavailable {
                provider,
                reason,
                alternatives,
            } => {
                writeln!(
                    f,
                    "{}",
                    format!("❌ Provider '{}' is unavailable", provider)
                        .red()
                        .bold()
                )?;
                writeln!(f, "   {}: {}", "Reason".dimmed(), reason)?;
                if !alternatives.is_empty() {
                    writeln!(f, "   {} Try these alternatives:", "💡".yellow())?;
                    for alt in alternatives {
                        writeln!(f, "      • {}", alt.green())?;
                    }
                }
            },

            Self::ChannelError {
                channel,
                error,
                fix_steps,
            } => {
                writeln!(
                    f,
                    "{}",
                    format!("❌ Channel '{}' error", channel).red().bold()
                )?;
                writeln!(f, "   {}: {}", "Error".dimmed(), error)?;
                if !fix_steps.is_empty() {
                    writeln!(f, "   {} Follow these steps to fix:", "🔧".blue())?;
                    for (i, step) in fix_steps.iter().enumerate() {
                        writeln!(f, "      {}. {}", i + 1, step)?;
                    }
                }
            },

            Self::InvalidConfiguration {
                field,
                expected,
                got,
            } => {
                writeln!(f, "{}", "❌ Invalid configuration".red().bold())?;
                writeln!(f, "   {}: {}", "Field".dimmed(), field)?;
                writeln!(f, "   {}: {}", "Expected".dimmed(), expected.green())?;
                writeln!(f, "   {}: {}", "Got".dimmed(), got.red())?;
                writeln!(
                    f,
                    "   {} Run {} to fix your configuration",
                    "💡".yellow(),
                    "clawmaster setup".green()
                )?;
            },

            Self::SetupRequired { reason } => {
                writeln!(f, "{}", "❌ Setup required".red().bold())?;
                writeln!(f, "   {}: {}", "Reason".dimmed(), reason)?;
                writeln!(f, "   {} Run the setup wizard:", "🚀".blue())?;
                writeln!(f, "      {}", "clawmaster setup".green().bold())?;
                writeln!(f, "   This will guide you through:")?;
                writeln!(f, "      • Configuring LLM providers")?;
                writeln!(f, "      • Setting up communication channels")?;
                writeln!(f, "      • Testing your configuration")?;
            },

            Self::DependencyMissing {
                dependency,
                install_command,
            } => {
                writeln!(
                    f,
                    "{}",
                    format!("❌ Missing dependency: {}", dependency)
                        .red()
                        .bold()
                )?;
                writeln!(f, "   {} Install it with:", "💡".yellow())?;
                writeln!(f, "      {}", install_command.green())?;
            },
        }
        Ok(())
    }
}

impl std::error::Error for UserError {}

pub fn format_error(err: &anyhow::Error) -> String {
    let err_str = err.to_string();

    if err_str.contains("No such file or directory") {
        if let Some(path) = extract_path(&err_str) {
            return UserError::ConfigNotFound {
                expected_path: PathBuf::from(path),
                suggestion: "Run 'clawmaster setup' to create the configuration file".to_string(),
            }
            .to_string();
        }
    }

    if err_str.contains("Address already in use") || err_str.contains("port") {
        if let Some(port) = extract_port(&err_str) {
            return UserError::PortInUse {
                port,
                suggestion: format!(
                    "Change the port in your config or stop the conflicting service"
                ),
            }
            .to_string();
        }
    }

    if err_str.contains("API key") || err_str.contains("authentication") {
        return UserError::ApiKeyMissing {
            provider: "OpenAI".to_string(),
            help_url: "https://platform.openai.com/api-keys".to_string(),
        }
        .to_string();
    }

    if err_str.contains("permission denied") || err_str.contains("Permission denied") {
        return UserError::PermissionDenied {
            resource: "file or directory".to_string(),
            required_permission: "read/write access".to_string(),
        }
        .to_string();
    }

    if err_str.contains("database") || err_str.contains("sqlite") {
        return UserError::DatabaseError {
            operation: "database operation".to_string(),
            suggestion: "Try running 'clawmaster db status' to check database health".to_string(),
        }
        .to_string();
    }

    format!(
        "{} {}\n   {} {}",
        "❌".red(),
        "An error occurred:".red().bold(),
        "Error:".dimmed(),
        err_str
    )
}

fn extract_path(err_str: &str) -> Option<&str> {
    err_str
        .split('\'')
        .nth(1)
        .or_else(|| err_str.split('"').nth(1))
}

fn extract_port(err_str: &str) -> Option<u16> {
    err_str.split(':').last()?.trim().parse().ok()
}

pub trait UserErrorExt {
    fn user_friendly(self) -> String;
}

impl UserErrorExt for anyhow::Error {
    fn user_friendly(self) -> String {
        format_error(&self)
    }
}

impl<T> UserErrorExt for Result<T, anyhow::Error> {
    fn user_friendly(self) -> String {
        match self {
            Ok(_) => String::new(),
            Err(e) => format_error(&e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_not_found() {
        let err = UserError::ConfigNotFound {
            expected_path: PathBuf::from("/home/user/.config/clawmaster/config.toml"),
            suggestion: "Run 'clawmaster setup'".to_string(),
        };

        let output = err.to_string();
        assert!(output.contains("Configuration file not found"));
        assert!(output.contains("clawmaster setup"));
    }

    #[test]
    fn test_api_key_missing() {
        let err = UserError::ApiKeyMissing {
            provider: "OpenAI".to_string(),
            help_url: "https://platform.openai.com".to_string(),
        };

        let output = err.to_string();
        assert!(output.contains("API key"));
        assert!(output.contains("OpenAI"));
        assert!(output.contains("platform.openai.com"));
    }

    #[test]
    fn test_port_in_use() {
        let err = UserError::PortInUse {
            port: 13131,
            suggestion: "Change the port".to_string(),
        };

        let output = err.to_string();
        assert!(output.contains("13131"));
        assert!(output.contains("lsof"));
    }

    #[test]
    fn test_format_error_path() {
        let err = anyhow::anyhow!("No such file or directory: '/path/to/config.toml'");
        let formatted = format_error(&err);
        assert!(formatted.contains("Configuration file not found"));
    }
}
