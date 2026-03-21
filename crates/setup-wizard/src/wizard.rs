//! Setup wizard main logic

use {
    crate::{
        state::{Channel, Provider, WizardConfig, WizardState},
        ui::WizardUI,
    },
    anyhow::{Context, Result},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    ratatui::{Terminal, backend::CrosstermBackend},
    std::io,
};

pub struct SetupWizard {
    state: WizardState,
    config: WizardConfig,
    ui: WizardUI,
    should_quit: bool,
}

impl SetupWizard {
    pub fn new() -> Result<Self> {
        Ok(Self {
            state: WizardState::Welcome,
            config: WizardConfig::new(),
            ui: WizardUI::new(),
            should_quit: false,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_loop(&mut terminal).await;

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    async fn run_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui.render(f, &self.state, &self.config))?;

            if self.should_quit {
                break;
            }

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    self.handle_key(key.code).await?;
                }
            }
        }

        Ok(())
    }

    async fn handle_key(&mut self, key: KeyCode) -> Result<()> {
        match &self.state {
            WizardState::Welcome => self.handle_welcome_key(key),
            WizardState::TemplateSelection => self.handle_template_selection_key(key),
            WizardState::ProviderSelection => self.handle_provider_selection_key(key),
            WizardState::ProviderConfig(provider) => {
                self.handle_provider_config_key(key, *provider).await
            },
            WizardState::ChannelSelection => self.handle_channel_selection_key(key),
            WizardState::ChannelConfig(channel) => {
                self.handle_channel_config_key(key, *channel).await
            },
            WizardState::TestConnection => Ok(()),
            WizardState::Summary => self.handle_summary_key(key).await,
            WizardState::Complete => {
                self.should_quit = true;
                Ok(())
            },
        }
    }

    fn handle_welcome_key(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Enter => {
                self.state = WizardState::TemplateSelection;
            },
            KeyCode::Char('q') => {
                self.should_quit = true;
            },
            _ => {},
        }
        Ok(())
    }

    fn handle_template_selection_key(&mut self, key: KeyCode) -> Result<()> {
        use crate::state::ConfigTemplate;
        let templates = ConfigTemplate::all();
        match key {
            KeyCode::Up => {
                if self.ui.selected_template_index > 0 {
                    self.ui.selected_template_index -= 1;
                }
            },
            KeyCode::Down => {
                if self.ui.selected_template_index < templates.len() - 1 {
                    self.ui.selected_template_index += 1;
                }
            },
            KeyCode::Enter => {
                let template = templates[self.ui.selected_template_index];
                // Apply template recommendations
                self.config.selected_providers = template.recommended_providers();
                // Move to provider selection for customization
                self.state = WizardState::ProviderSelection;
                self.ui.selected_provider_index = 0;
            },
            KeyCode::Char('q') => {
                self.should_quit = true;
            },
            _ => {},
        }
        Ok(())
    }

    fn handle_provider_selection_key(&mut self, key: KeyCode) -> Result<()> {
        let providers = Provider::all();
        match key {
            KeyCode::Up => {
                if self.ui.selected_provider_index > 0 {
                    self.ui.selected_provider_index -= 1;
                }
            },
            KeyCode::Down => {
                if self.ui.selected_provider_index < providers.len() - 1 {
                    self.ui.selected_provider_index += 1;
                }
            },
            KeyCode::Char(' ') => {
                let provider = providers[self.ui.selected_provider_index];
                if self.config.selected_providers.contains(&provider) {
                    self.config.selected_providers.retain(|p| *p != provider);
                    self.config.provider_keys.remove(&provider);
                } else {
                    self.config.selected_providers.push(provider);
                    if provider.requires_api_key() {
                        self.ui.input_buffer.clear();
                        self.state = WizardState::ProviderConfig(provider);
                    }
                }
            },
            KeyCode::Enter => {
                if !self.config.selected_providers.is_empty() {
                    self.state = WizardState::ChannelSelection;
                    self.ui.selected_channel_index = 0;
                }
            },
            KeyCode::Char('q') => {
                self.should_quit = true;
            },
            _ => {},
        }
        Ok(())
    }

    async fn handle_provider_config_key(&mut self, key: KeyCode, provider: Provider) -> Result<()> {
        match key {
            KeyCode::Char(c) => {
                self.ui.input_buffer.push(c);
            },
            KeyCode::Backspace => {
                self.ui.input_buffer.pop();
            },
            KeyCode::Enter => {
                if !self.ui.input_buffer.is_empty() {
                    self.config
                        .provider_keys
                        .insert(provider, self.ui.input_buffer.clone());
                    self.ui.input_buffer.clear();
                    self.state = WizardState::ProviderSelection;
                }
            },
            KeyCode::Esc => {
                self.ui.input_buffer.clear();
                self.state = WizardState::ProviderSelection;
            },
            _ => {},
        }
        Ok(())
    }

    fn handle_channel_selection_key(&mut self, key: KeyCode) -> Result<()> {
        let channels = Channel::all();
        match key {
            KeyCode::Up => {
                if self.ui.selected_channel_index > 0 {
                    self.ui.selected_channel_index -= 1;
                }
            },
            KeyCode::Down => {
                if self.ui.selected_channel_index < channels.len() - 1 {
                    self.ui.selected_channel_index += 1;
                }
            },
            KeyCode::Char(' ') => {
                let channel = channels[self.ui.selected_channel_index];
                if self.config.selected_channels.contains(&channel) {
                    self.config.selected_channels.retain(|c| *c != channel);
                    self.config.channel_tokens.remove(&channel);
                } else {
                    self.config.selected_channels.push(channel);
                    if channel.requires_token() {
                        self.ui.input_buffer.clear();
                        self.state = WizardState::ChannelConfig(channel);
                    }
                }
            },
            KeyCode::Enter => {
                self.state = WizardState::Summary;
            },
            KeyCode::Esc => {
                self.state = WizardState::ProviderSelection;
            },
            KeyCode::Char('q') => {
                self.should_quit = true;
            },
            _ => {},
        }
        Ok(())
    }

    async fn handle_channel_config_key(&mut self, key: KeyCode, channel: Channel) -> Result<()> {
        match key {
            KeyCode::Char(c) => {
                self.ui.input_buffer.push(c);
            },
            KeyCode::Backspace => {
                self.ui.input_buffer.pop();
            },
            KeyCode::Enter => {
                if !self.ui.input_buffer.is_empty() {
                    self.config
                        .channel_tokens
                        .insert(channel, self.ui.input_buffer.clone());
                    self.ui.input_buffer.clear();
                    self.state = WizardState::ChannelSelection;
                }
            },
            KeyCode::Esc => {
                self.ui.input_buffer.clear();
                self.state = WizardState::ChannelSelection;
            },
            _ => {},
        }
        Ok(())
    }

    async fn handle_summary_key(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Enter => {
                self.save_config().await?;
                self.state = WizardState::Complete;
            },
            KeyCode::Esc => {
                self.state = WizardState::ChannelSelection;
            },
            KeyCode::Char('q') => {
                self.should_quit = true;
            },
            _ => {},
        }
        Ok(())
    }

    async fn save_config(&self) -> Result<()> {
        let config_dir =
            clawmaster_config::config_dir().context("Failed to get config directory")?;
        std::fs::create_dir_all(&config_dir).context("Failed to create config directory")?;

        let config_path = config_dir.join("clawmaster.toml");

        let mut toml_content = String::new();
        toml_content.push_str("# ClawMaster Configuration\n");
        toml_content.push_str("# Generated by setup wizard\n\n");

        if !self.config.selected_providers.is_empty() {
            toml_content.push_str("[providers]\n");
            for provider in &self.config.selected_providers {
                let enabled = match provider {
                    Provider::OpenAI => "openai",
                    Provider::Anthropic => "anthropic",
                    Provider::OpenRouter => "openrouter",
                    Provider::Ollama => "ollama",
                    Provider::GitHubCopilot => "github_copilot",
                };
                toml_content.push_str(&format!("{} = true\n", enabled));
            }
            toml_content.push_str("\n");
        }

        if !self.config.selected_channels.is_empty() {
            toml_content.push_str("[channels]\n");
            for channel in &self.config.selected_channels {
                let enabled = match channel {
                    Channel::WebUI => "web",
                    Channel::Telegram => "telegram",
                    Channel::Discord => "discord",
                    Channel::Slack => "slack",
                };
                toml_content.push_str(&format!("{} = true\n", enabled));
            }
            toml_content.push_str("\n");
        }

        std::fs::write(&config_path, toml_content).context("Failed to write config file")?;

        let env_path = config_dir.join(".env");
        let mut env_content = String::new();
        env_content.push_str("# ClawMaster Environment Variables\n");
        env_content.push_str("# Generated by setup wizard\n\n");

        for (provider, key) in &self.config.provider_keys {
            env_content.push_str(&format!("{}={}\n", provider.api_key_env(), key));
        }

        for (channel, token) in &self.config.channel_tokens {
            let env_var = match channel {
                Channel::Telegram => "TELEGRAM_BOT_TOKEN",
                Channel::Discord => "DISCORD_BOT_TOKEN",
                Channel::Slack => "SLACK_BOT_TOKEN",
                _ => continue,
            };
            env_content.push_str(&format!("{}={}\n", env_var, token));
        }

        std::fs::write(&env_path, env_content).context("Failed to write .env file")?;

        tracing::info!("Configuration saved to {:?}", config_path);
        tracing::info!("Environment variables saved to {:?}", env_path);

        Ok(())
    }
}
