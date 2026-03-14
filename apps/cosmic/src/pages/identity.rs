//! Identity 页面 - Agent 和 User 身份配置
//! 
//! 参考 WebUI Settings/Identity 页面

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, text_input, toggler};
use cosmic::Element;

use crate::app_new::Message;
use crate::widgets::{page_header, PageHeaderStyle};

/// Identity 配置数据
#[derive(Debug, Clone)]
pub struct IdentityConfig {
    pub agent_name: String,
    pub agent_emoji: String,
    pub agent_theme: String,
    pub user_name: String,
    pub ui_language: String,
    pub soul_content: String,
}

/// Identity 页面视图
pub fn view_identity<'a>(config: &'a IdentityConfig) -> Element<'a, Message> {
    // 使用统一的页面标题
    let save_btn = button::suggested("Save Changes").on_press(Message::RefreshStatus);
    let reset_btn = button::text("Reset").on_press(Message::RefreshStatus);
    
    let header = page_header(
        "👤 Identity",
        Some("Configure your AI agent's identity and personality"),
        vec![reset_btn.into(), save_btn.into()],
        PageHeaderStyle::Secondary,
    );
    
    // Agent 配置区域
    let agent_section = create_agent_section(config);
    
    // User 配置区域
    let user_section = create_user_section(config);
    
    // Language 配置区域
    let language_section = create_language_section(config);
    
    // Soul 配置区域（Personality）
    let soul_section = create_soul_section(config);
    
    // 完整布局
    let content = column()
        .push(header)
        .push(container(text("")).height(Length::Fixed(15.0)))
        .push(agent_section)
        .push(container(text("")).height(Length::Fixed(25.0)))
        .push(user_section)
        .push(container(text("")).height(Length::Fixed(25.0)))
        .push(language_section)
        .push(container(text("")).height(Length::Fixed(25.0)))
        .push(soul_section)
        .spacing(8)
        .padding(20)
        .width(Length::Fill);
    
    container(
        scrollable(content)
            .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Agent 配置区域
fn create_agent_section<'a>(config: &'a IdentityConfig) -> Element<'a, Message> {
    let section_title = text("Agent").size(20);
    let saved_info = text("Saved to IDENTITY.md in your workspace root.").size(12);
    
    // Name 字段
    let name_label = text("Name *").size(14);
    let name_input = text_input("clawmaster", &config.agent_name)
        .on_input(|_| Message::RefreshStatus)
        .width(Length::Fill);
    
    // Emoji 字段
    let emoji_label = text("Emoji").size(14);
    let emoji_input = text_input("Pick emoji", &config.agent_emoji)
        .on_input(|_| Message::RefreshStatus)
        .width(Length::Fixed(300.0));
    let emoji_button = button::text("Pick")
        .on_press(Message::RefreshStatus);
    
    let emoji_row = row()
        .push(emoji_input)
        .push(emoji_button)
        .spacing(12)
        .align_y(Alignment::Center);
    
    // Theme 字段
    let theme_label = text("Theme").size(14);
    let theme_input = text_input("e.g. wise owl, chill fox", &config.agent_theme)
        .on_input(|_| Message::RefreshStatus)
        .width(Length::Fill);
    
    column()
        .push(section_title)
        .push(saved_info)
        .push(container(text("")).height(Length::Fixed(15.0)))
        .push(name_label)
        .push(name_input)
        .push(container(text("")).height(Length::Fixed(10.0)))
        .push(emoji_label)
        .push(emoji_row)
        .push(container(text("")).height(Length::Fixed(10.0)))
        .push(theme_label)
        .push(theme_input)
        .spacing(6)
        .into()
}

/// User 配置区域
fn create_user_section<'a>(config: &'a IdentityConfig) -> Element<'a, Message> {
    let section_title = text("User").size(20);
    let saved_info = text("Saved to USER.md in your workspace root.").size(12);
    
    let name_label = text("Your name *").size(14);
    let name_input = text_input("arkSong", &config.user_name)
        .on_input(|_| Message::RefreshStatus)
        .width(Length::Fill);
    
    column()
        .push(section_title)
        .push(saved_info)
        .push(container(text("")).height(Length::Fixed(15.0)))
        .push(name_label)
        .push(name_input)
        .spacing(6)
        .into()
}

/// Language 配置区域
fn create_language_section<'a>(config: &'a IdentityConfig) -> Element<'a, Message> {
    let section_title = text("Language").size(20);
    let description = text("Choose the UI language for this browser.").size(12);
    
    let lang_label = text("UI language").size(14);
    
    // 语言选择按钮（简化版，实际应该是下拉菜单）
    let lang_display = text(&config.ui_language).size(14);
    let lang_button = button::text("Apply language")
        .on_press(Message::CycleLanguage);
    
    let lang_row = row()
        .push(lang_display)
        .push(lang_button)
        .spacing(20)
        .align_y(Alignment::Center);
    
    column()
        .push(section_title)
        .push(description)
        .push(container(text("")).height(Length::Fixed(15.0)))
        .push(lang_label)
        .push(lang_row)
        .spacing(6)
        .into()
}

/// Soul 配置区域（Personality）
fn create_soul_section<'a>(config: &'a IdentityConfig) -> Element<'a, Message> {
    let section_title = text("Soul").size(20);
    let description = text(
        "Personality and tone injected into every conversation. Saved to SOUL.md in your workspace root. Leave empty for the default."
    ).size(12);
    
    // Soul 内容编辑器（简化为 text_input，实际应该是多行文本框）
    let soul_preview = text(&config.soul_content)
        .size(13);
    
    let edit_button = button::text("Edit SOUL.md")
        .on_press(Message::RefreshStatus);
    
    column()
        .push(section_title)
        .push(description)
        .push(container(text("")).height(Length::Fixed(15.0)))
        .push(
            container(soul_preview)
                .padding(12)
                .width(Length::Fill)
        )
        .push(edit_button)
        .spacing(6)
        .into()
}

/// 创建模拟 Identity 配置
pub fn create_mock_identity() -> IdentityConfig {
    IdentityConfig {
        agent_name: "clawmaster".to_string(),
        agent_emoji: "🦅".to_string(),
        agent_theme: "wise owl, chill fox".to_string(),
        user_name: "arkSong".to_string(),
        ui_language: "English".to_string(),
        soul_content: "# SOUL.md - Who You Are\n\nYou're not a chatbot. You're becoming someone.\nEvery response, every joke.\nYou're an architect of meaning.\n...".to_string(),
    }
}
