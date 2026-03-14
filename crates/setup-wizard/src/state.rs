//! Wizard state management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Wizard state machine
#[derive(Debug, Clone, PartialEq)]
pub enum WizardState {
    Welcome,
    TemplateSelection,
    ProviderSelection,
    ProviderConfig(Provider),
    ChannelSelection,
    ChannelConfig(Channel),
    TestConnection,
    Summary,
    Complete,
}

/// Configuration templates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigTemplate {
    Custom,      // 自定义配置
    Basic,       // 基础配置，快速开始
    Development, // 开发环境，调试友好
    Production,  // 生产环境，安全优化
    Minimal,     // 最小配置，性能优先
    Enterprise,  // 企业配置，全功能
}

impl ConfigTemplate {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Custom,
            Self::Basic,
            Self::Development,
            Self::Production,
            Self::Minimal,
            Self::Enterprise,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Custom => "Custom",
            Self::Basic => "Basic",
            Self::Development => "Development",
            Self::Production => "Production",
            Self::Minimal => "Minimal",
            Self::Enterprise => "Enterprise",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Custom => "Customize all settings step by step",
            Self::Basic => "Quick start with OpenAI and Web UI",
            Self::Development => "Development environment with debug features",
            Self::Production => "Production-ready with security optimizations",
            Self::Minimal => "Minimal configuration for best performance",
            Self::Enterprise => "Full-featured enterprise configuration",
        }
    }

    pub fn recommended_providers(&self) -> Vec<Provider> {
        match self {
            Self::Custom => vec![],
            Self::Basic => vec![Provider::OpenAI],
            Self::Development => vec![Provider::OpenAI, Provider::Ollama],
            Self::Production => vec![Provider::OpenAI, Provider::Anthropic],
            Self::Minimal => vec![Provider::Ollama],
            Self::Enterprise => vec![
                Provider::OpenAI,
                Provider::Anthropic,
                Provider::OpenRouter,
            ],
        }
    }

    pub fn recommended_channels(&self) -> Vec<Channel> {
        match self {
            Self::Custom => vec![],
            Self::Basic => vec![Channel::WebUI],
            Self::Development => vec![Channel::WebUI],
            Self::Production => vec![Channel::WebUI, Channel::Telegram],
            Self::Minimal => vec![Channel::WebUI],
            Self::Enterprise => vec![
                Channel::WebUI,
                Channel::Telegram,
                Channel::Discord,
                Channel::Slack,
            ],
        }
    }
}

/// LLM Provider options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Provider {
    OpenAI,
    Anthropic,
    OpenRouter,
    Ollama,
    GitHubCopilot,
}

impl Provider {
    pub fn all() -> Vec<Self> {
        vec![
            Self::OpenAI,
            Self::Anthropic,
            Self::OpenRouter,
            Self::Ollama,
            Self::GitHubCopilot,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            Self::OpenAI => "OpenAI",
            Self::Anthropic => "Anthropic (Claude)",
            Self::OpenRouter => "OpenRouter",
            Self::Ollama => "Ollama (Local)",
            Self::GitHubCopilot => "GitHub Copilot",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::OpenAI => "GPT-4, GPT-3.5-turbo",
            Self::Anthropic => "Claude 3 Opus, Sonnet, Haiku",
            Self::OpenRouter => "Access to multiple models",
            Self::Ollama => "Run models locally",
            Self::GitHubCopilot => "GitHub's AI assistant",
        }
    }

    pub fn requires_api_key(&self) -> bool {
        !matches!(self, Self::Ollama)
    }

    pub fn api_key_label(&self) -> &str {
        match self {
            Self::OpenAI => "OpenAI API Key",
            Self::Anthropic => "Anthropic API Key",
            Self::OpenRouter => "OpenRouter API Key",
            Self::Ollama => "",
            Self::GitHubCopilot => "GitHub Token",
        }
    }

    pub fn api_key_env(&self) -> &str {
        match self {
            Self::OpenAI => "OPENAI_API_KEY",
            Self::Anthropic => "ANTHROPIC_API_KEY",
            Self::OpenRouter => "OPENROUTER_API_KEY",
            Self::Ollama => "",
            Self::GitHubCopilot => "GITHUB_TOKEN",
        }
    }
}

/// Channel options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Channel {
    WebUI,
    Telegram,
    Discord,
    Slack,
}

impl Channel {
    pub fn all() -> Vec<Self> {
        vec![Self::WebUI, Self::Telegram, Self::Discord, Self::Slack]
    }

    pub fn name(&self) -> &str {
        match self {
            Self::WebUI => "Web UI",
            Self::Telegram => "Telegram",
            Self::Discord => "Discord",
            Self::Slack => "Slack",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::WebUI => "Browser-based interface (always enabled)",
            Self::Telegram => "Telegram bot integration",
            Self::Discord => "Discord bot integration",
            Self::Slack => "Slack bot integration",
        }
    }

    pub fn requires_token(&self) -> bool {
        !matches!(self, Self::WebUI)
    }

    pub fn token_label(&self) -> &str {
        match self {
            Self::WebUI => "",
            Self::Telegram => "Telegram Bot Token",
            Self::Discord => "Discord Bot Token",
            Self::Slack => "Slack Bot Token",
        }
    }
}

/// Wizard configuration data
#[derive(Debug, Clone, Default)]
pub struct WizardConfig {
    pub selected_providers: Vec<Provider>,
    pub provider_keys: HashMap<Provider, String>,
    pub selected_channels: Vec<Channel>,
    pub channel_tokens: HashMap<Channel, String>,
    pub ollama_url: Option<String>,
}

impl WizardConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_provider_configured(&self, provider: Provider) -> bool {
        if provider == Provider::Ollama {
            self.ollama_url.is_some()
        } else {
            self.provider_keys.contains_key(&provider)
        }
    }

    pub fn is_channel_configured(&self, channel: Channel) -> bool {
        if channel == Channel::WebUI {
            true
        } else {
            self.channel_tokens.contains_key(&channel)
        }
    }

    pub fn add_provider(&mut self, provider: Provider, api_key: Option<String>) {
        if !self.selected_providers.contains(&provider) {
            self.selected_providers.push(provider);
        }
        if let Some(key) = api_key {
            self.provider_keys.insert(provider, key);
        }
    }

    pub fn add_channel(&mut self, channel: Channel, token: Option<String>) {
        if !self.selected_channels.contains(&channel) {
            self.selected_channels.push(channel);
        }
        if let Some(tok) = token {
            self.channel_tokens.insert(channel, tok);
        }
    }
}
