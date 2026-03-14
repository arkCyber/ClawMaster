//! UI rendering for the setup wizard

use crate::state::{Channel, Provider, WizardConfig, WizardState, ConfigTemplate};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub struct WizardUI {
    pub selected_provider_index: usize,
    pub selected_channel_index: usize,
    pub selected_template_index: usize,
    pub input_buffer: String,
    pub show_help: bool,
}

impl WizardUI {
    pub fn new() -> Self {
        Self {
            selected_provider_index: 0,
            selected_channel_index: 0,
            selected_template_index: 0,
            input_buffer: String::new(),
            show_help: false,
        }
    }

    pub fn render(&self, frame: &mut Frame, state: &WizardState, config: &WizardConfig) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(frame.size());

        self.render_header(frame, chunks[0]);
        self.render_content(frame, chunks[1], state, config);
        self.render_footer(frame, chunks[2], state);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new("🦾 ClawMaster Setup Wizard")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect, state: &WizardState) {
        let help_text = match state {
            WizardState::Welcome => "Press Enter to continue, q to quit",
            WizardState::TemplateSelection => "↑/↓: Navigate, Enter: Select template, q: Quit",
            WizardState::ProviderSelection => "↑/↓: Navigate, Space: Select, Enter: Continue, q: Quit",
            WizardState::ProviderConfig(_) => "Type API key, Enter: Save, Esc: Back",
            WizardState::ChannelSelection => "↑/↓: Navigate, Space: Select, Enter: Continue, q: Quit",
            WizardState::ChannelConfig(_) => "Type token, Enter: Save, Esc: Back",
            WizardState::TestConnection => "Testing connection... Please wait",
            WizardState::Summary => "Enter: Save and start, Esc: Back, q: Quit",
            WizardState::Complete => "Press any key to exit",
        };

        let footer = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, area);
    }

    fn render_content(
        &self,
        frame: &mut Frame,
        area: Rect,
        state: &WizardState,
        config: &WizardConfig,
    ) {
        match state {
            WizardState::Welcome => self.render_welcome(frame, area),
            WizardState::TemplateSelection => self.render_template_selection(frame, area),
            WizardState::ProviderSelection => self.render_provider_selection(frame, area, config),
            WizardState::ProviderConfig(provider) => {
                self.render_provider_config(frame, area, *provider)
            }
            WizardState::ChannelSelection => self.render_channel_selection(frame, area, config),
            WizardState::ChannelConfig(channel) => {
                self.render_channel_config(frame, area, *channel)
            }
            WizardState::TestConnection => self.render_test_connection(frame, area),
            WizardState::Summary => self.render_summary(frame, area, config),
            WizardState::Complete => self.render_complete(frame, area),
        }
    }

    fn render_template_selection(&self, frame: &mut Frame, area: Rect) {
        let templates = ConfigTemplate::all();
        let items: Vec<ListItem> = templates
            .iter()
            .enumerate()
            .map(|(i, template)| {
                let is_selected = i == self.selected_template_index;
                
                let line = Line::from(vec![
                    Span::raw(if is_selected { "→ " } else { "  " }),
                    Span::styled(
                        template.name(),
                        if is_selected {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        },
                    ),
                    Span::raw(" - "),
                    Span::styled(
                        template.description(),
                        Style::default().fg(Color::Gray),
                    ),
                ]);
                
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Select Configuration Template"))
            .highlight_style(Style::default().bg(Color::DarkGray));

        frame.render_widget(list, area);
    }

    fn render_welcome(&self, frame: &mut Frame, area: Rect) {
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Welcome to ClawMaster!",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("This wizard will help you set up:"),
            Line::from(""),
            Line::from("  • LLM Provider (OpenAI, Anthropic, etc.)"),
            Line::from("  • Communication Channels (Telegram, Discord, etc.)"),
            Line::from("  • Basic Configuration"),
            Line::from(""),
            Line::from("The setup takes about 2-3 minutes."),
            Line::from(""),
            Line::from(Span::styled(
                "Press Enter to begin",
                Style::default().fg(Color::Yellow),
            )),
        ];

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Welcome"));
        frame.render_widget(paragraph, area);
    }

    fn render_provider_selection(
        &self,
        frame: &mut Frame,
        area: Rect,
        config: &WizardConfig,
    ) {
        let providers = Provider::all();
        let items: Vec<ListItem> = providers
            .iter()
            .enumerate()
            .map(|(i, provider)| {
                let is_selected = i == self.selected_provider_index;
                let is_configured = config.is_provider_configured(*provider);

                let checkbox = if config.selected_providers.contains(provider) {
                    "[✓]"
                } else {
                    "[ ]"
                };

                let status = if is_configured { " ✓" } else { "" };

                let line = Line::from(vec![
                    Span::raw(if is_selected { "→ " } else { "  " }),
                    Span::raw(checkbox),
                    Span::raw(" "),
                    Span::styled(
                        provider.name(),
                        if is_selected {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        },
                    ),
                    Span::raw(status),
                    Span::raw(" - "),
                    Span::styled(provider.description(), Style::default().fg(Color::Gray)),
                ]);

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Select LLM Providers (Space to toggle)"),
        );

        frame.render_widget(list, area);
    }

    fn render_provider_config(&self, frame: &mut Frame, area: Rect, provider: Provider) {
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("Configure {}", provider.name()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!("Enter your {}:", provider.api_key_label())),
            Line::from(""),
            Line::from(Span::styled(
                &self.input_buffer,
                Style::default().fg(Color::Green),
            )),
            Line::from(""),
            Line::from(Span::styled(
                format!("(Set as {} environment variable)", provider.api_key_env()),
                Style::default().fg(Color::Gray),
            )),
        ];

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("API Key Configuration"),
            );
        frame.render_widget(paragraph, area);
    }

    fn render_channel_selection(&self, frame: &mut Frame, area: Rect, config: &WizardConfig) {
        let channels = Channel::all();
        let items: Vec<ListItem> = channels
            .iter()
            .enumerate()
            .map(|(i, channel)| {
                let is_selected = i == self.selected_channel_index;
                let is_configured = config.is_channel_configured(*channel);

                let checkbox = if config.selected_channels.contains(channel) {
                    "[✓]"
                } else {
                    "[ ]"
                };

                let status = if is_configured { " ✓" } else { "" };

                let line = Line::from(vec![
                    Span::raw(if is_selected { "→ " } else { "  " }),
                    Span::raw(checkbox),
                    Span::raw(" "),
                    Span::styled(
                        channel.name(),
                        if is_selected {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        },
                    ),
                    Span::raw(status),
                    Span::raw(" - "),
                    Span::styled(channel.description(), Style::default().fg(Color::Gray)),
                ]);

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Select Communication Channels (Space to toggle)"),
        );

        frame.render_widget(list, area);
    }

    fn render_channel_config(&self, frame: &mut Frame, area: Rect, channel: Channel) {
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("Configure {}", channel.name()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!("Enter your {}:", channel.token_label())),
            Line::from(""),
            Line::from(Span::styled(
                &self.input_buffer,
                Style::default().fg(Color::Green),
            )),
        ];

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Channel Configuration"),
            );
        frame.render_widget(paragraph, area);
    }

    fn render_test_connection(&self, frame: &mut Frame, area: Rect) {
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Testing Connection...",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Verifying API keys and connectivity..."),
            Line::from(""),
            Line::from("This may take a few seconds."),
        ];

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Testing"));
        frame.render_widget(paragraph, area);
    }

    fn render_summary(&self, frame: &mut Frame, area: Rect, config: &WizardConfig) {
        let mut lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Setup Summary",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled("Providers:", Style::default().add_modifier(Modifier::BOLD))),
        ];

        for provider in &config.selected_providers {
            lines.push(Line::from(format!("  • {}", provider.name())));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Channels:",
            Style::default().add_modifier(Modifier::BOLD),
        )));

        for channel in &config.selected_channels {
            lines.push(Line::from(format!("  • {}", channel.name())));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Press Enter to save configuration and start ClawMaster",
            Style::default().fg(Color::Yellow),
        )));

        let paragraph = Paragraph::new(lines)
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL).title("Summary"));
        frame.render_widget(paragraph, area);
    }

    fn render_complete(&self, frame: &mut Frame, area: Rect) {
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "✓ Setup Complete!",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Configuration saved to ~/.config/clawmaster/clawmaster.toml"),
            Line::from(""),
            Line::from("Starting ClawMaster..."),
            Line::from(""),
            Line::from("Access the Web UI at: https://localhost:13131"),
            Line::from(""),
            Line::from(Span::styled(
                "Press any key to exit",
                Style::default().fg(Color::Yellow),
            )),
        ];

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Complete"));
        frame.render_widget(paragraph, area);
    }
}

impl Default for WizardUI {
    fn default() -> Self {
        Self::new()
    }
}
