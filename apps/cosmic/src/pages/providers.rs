//! Providers 页面 - LLM 提供商配置
//! 
//! 支持的提供商：
//! - OpenAI (GPT-4, GPT-3.5)
//! - Anthropic (Claude)
//! - Google (Gemini)
//! - Ollama (本地模型)
//! - OpenRouter (多模型聚合)
//! - Azure OpenAI
//! - AWS Bedrock
//! - Groq
//! - Together AI
//! - Mistral AI

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, text_input, toggler};
use cosmic::Element;

use crate::app_new::Message;
use crate::widgets::{page_header, PageHeaderStyle};

#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub enabled: bool,
    pub api_key_set: bool,
    pub models_count: u32,
    pub status: ProviderStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderStatus {
    Connected,
    Disconnected,
    Testing,
    Error,
}

/// Providers 页面视图
pub fn view_providers<'a>(providers: &'a [ProviderInfo]) -> Element<'a, Message> {
    let test_btn = button::suggested("Test All").on_press(Message::RefreshStatus);
    let add_btn = button::text("+ Add Provider").on_press(Message::RefreshStatus);
    
    let header = page_header(
        "🤖 LLM Providers",
        Some("Manage your LLM provider connections and API keys"),
        vec![add_btn.into(), test_btn.into()],
        PageHeaderStyle::Primary,
    );
    
    // 提供商列表
    let mut provider_list = column().spacing(12);
    
    for provider in providers {
        let provider_card = create_provider_card(provider);
        provider_list = provider_list.push(provider_card);
    }
    
    // 完整布局
    let content = column()
        .push(header)
        .push(
            scrollable(provider_list)
                .height(Length::Fill)
        )
        .spacing(20)
        .padding(20)
        .width(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// 创建提供商卡片
fn create_provider_card<'a>(provider: &'a ProviderInfo) -> Element<'a, Message> {
    // 状态指示器
    let status_text = match provider.status {
        ProviderStatus::Connected => "🟢 Connected",
        ProviderStatus::Disconnected => "⚪ Disconnected",
        ProviderStatus::Testing => "🟡 Testing...",
        ProviderStatus::Error => "🔴 Error",
    };
    
    // 提供商信息
    let name_text = text(&provider.name)
        .size(18);
    
    let type_text = text(&provider.provider_type)
        .size(12);
    
    let models_text = text(format!("{} models", provider.models_count))
        .size(12);
    
    let api_key_status = if provider.api_key_set {
        text("🔑 API Key Set").size(12)
    } else {
        text("⚠️ No API Key").size(12)
    };
    
    let status_indicator = text(status_text).size(12);
    
    // 左侧信息
    let info_column = column()
        .push(name_text)
        .push(
            row()
                .push(type_text)
                .push(text("·").size(12))
                .push(models_text)
                .spacing(8)
        )
        .push(
            row()
                .push(api_key_status)
                .push(text("·").size(12))
                .push(status_indicator)
                .spacing(8)
        )
        .spacing(6);
    
    // 右侧控制
    let enable_toggle = toggler(provider.enabled)
        .on_toggle(|_| Message::NavigateTo(crate::app_new::Page::Dashboard));
    
    let configure_button = button::text("Configure")
        .on_press(Message::NavigateTo(crate::app_new::Page::Dashboard));
    
    let test_button = button::text("Test")
        .on_press(Message::NavigateTo(crate::app_new::Page::Dashboard));
    
    let controls = row()
        .push(enable_toggle)
        .push(configure_button)
        .push(test_button)
        .spacing(12)
        .align_y(Alignment::Center);
    
    // 完整卡片
    let card_content = row()
        .push(info_column)
        .push(controls)
        .spacing(20)
        .align_y(Alignment::Center);
    
    container(card_content)
        .padding(16)
        .width(Length::Fill)
        .into()
}

/// 创建模拟提供商数据
pub fn create_mock_providers() -> Vec<ProviderInfo> {
    vec![
        ProviderInfo {
            id: "openai".to_string(),
            name: "OpenAI".to_string(),
            provider_type: "Cloud API".to_string(),
            enabled: true,
            api_key_set: true,
            models_count: 8,
            status: ProviderStatus::Connected,
        },
        ProviderInfo {
            id: "anthropic".to_string(),
            name: "Anthropic".to_string(),
            provider_type: "Cloud API".to_string(),
            enabled: true,
            api_key_set: true,
            models_count: 3,
            status: ProviderStatus::Connected,
        },
        ProviderInfo {
            id: "ollama".to_string(),
            name: "Ollama".to_string(),
            provider_type: "Local".to_string(),
            enabled: true,
            api_key_set: false,
            models_count: 12,
            status: ProviderStatus::Connected,
        },
        ProviderInfo {
            id: "google".to_string(),
            name: "Google AI".to_string(),
            provider_type: "Cloud API".to_string(),
            enabled: false,
            api_key_set: false,
            models_count: 4,
            status: ProviderStatus::Disconnected,
        },
        ProviderInfo {
            id: "openrouter".to_string(),
            name: "OpenRouter".to_string(),
            provider_type: "Aggregator".to_string(),
            enabled: true,
            api_key_set: true,
            models_count: 50,
            status: ProviderStatus::Connected,
        },
    ]
}
