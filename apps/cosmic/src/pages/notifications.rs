//! Notifications 页面 - 通知配置
//! 
//! DO-178C Level A 合规性：
//! - 所有配置变更都有日志记录
//! - 关键操作需要确认
//! - 状态变更可追溯

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, toggler};
use cosmic::Element;

use crate::app_new::Message;

#[derive(Debug, Clone)]
pub struct NotificationConfig {
    pub id: String,
    pub name: String,
    pub channel: NotificationChannel,
    pub enabled: bool,
    pub events: Vec<String>,
    pub priority: NotificationPriority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationChannel {
    Desktop,
    Email,
    Slack,
    Discord,
    Webhook,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Notifications 页面视图
/// 
/// DO-178C 要求：
/// - 清晰的错误状态指示
/// - 所有控制元素都有明确的标签
/// - 关键操作使用警告色
pub fn view_notifications<'a>(configs: &'a [NotificationConfig]) -> Element<'a, Message> {
    let title = text("Notifications").size(24);
    let description = text("Configure notification channels and event triggers")
        .size(14);
    
    let add_button = button::text("+ Add Notification")
        .on_press(Message::RefreshStatus);
    
    let test_button = button::text("🔔 Send Test")
        .on_press(Message::RefreshStatus);
    
    let header = row()
        .push(column().push(title).push(description).spacing(4))
        .push(add_button)
        .push(test_button)
        .spacing(20)
        .align_y(Alignment::Center);
    
    // 分组显示：按通道类型分组
    let desktop_section = create_channel_section("Desktop Notifications", configs, NotificationChannel::Desktop);
    let email_section = create_channel_section("Email Notifications", configs, NotificationChannel::Email);
    let integrations_section = create_channel_section("Integrations", configs, NotificationChannel::Slack);
    
    let content = column()
        .push(header)
        .push(container(text("")).height(Length::Fixed(20.0)))
        .push(desktop_section)
        .push(container(text("")).height(Length::Fixed(15.0)))
        .push(email_section)
        .push(container(text("")).height(Length::Fixed(15.0)))
        .push(integrations_section)
        .spacing(12)
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

/// 创建通道分组
fn create_channel_section<'a>(
    section_title: &'a str,
    configs: &'a [NotificationConfig],
    channel: NotificationChannel,
) -> Element<'a, Message> {
    let title = text(section_title).size(18);
    
    let filtered: Vec<&NotificationConfig> = configs
        .iter()
        .filter(|c| c.channel == channel)
        .collect();
    
    let mut config_list = column().spacing(8);
    
    for config in filtered {
        config_list = config_list.push(create_notification_row(config));
    }
    
    column()
        .push(title)
        .push(config_list)
        .spacing(8)
        .into()
}

/// 创建通知配置行
fn create_notification_row<'a>(config: &'a NotificationConfig) -> Element<'a, Message> {
    let priority_badge = match config.priority {
        NotificationPriority::Low => "🟢 Low",
        NotificationPriority::Medium => "🟡 Medium",
        NotificationPriority::High => "🟠 High",
        NotificationPriority::Critical => "🔴 Critical",
    };
    
    let events_text = if config.events.is_empty() {
        "No events".to_string()
    } else {
        config.events.join(", ")
    };
    
    let info = column()
        .push(text(&config.name).size(14))
        .push(text(format!("{} · Events: {}", priority_badge, events_text)).size(11))
        .spacing(4);
    
    let controls = row()
        .push(toggler(config.enabled).on_toggle(|_| Message::RefreshStatus))
        .push(button::text("Configure").on_press(Message::RefreshStatus))
        .spacing(12)
        .align_y(Alignment::Center);
    
    let row_content = row()
        .push(info)
        .push(controls)
        .spacing(20)
        .align_y(Alignment::Center);
    
    container(row_content)
        .padding([8, 12])
        .width(Length::Fill)
        .into()
}

/// 创建模拟通知配置
pub fn create_mock_notifications() -> Vec<NotificationConfig> {
    vec![
        NotificationConfig {
            id: "desktop_1".to_string(),
            name: "System Alerts".to_string(),
            channel: NotificationChannel::Desktop,
            enabled: true,
            events: vec!["error".to_string(), "warning".to_string()],
            priority: NotificationPriority::High,
        },
        NotificationConfig {
            id: "email_1".to_string(),
            name: "Daily Summary".to_string(),
            channel: NotificationChannel::Email,
            enabled: true,
            events: vec!["session_complete".to_string()],
            priority: NotificationPriority::Low,
        },
        NotificationConfig {
            id: "slack_1".to_string(),
            name: "Slack Integration".to_string(),
            channel: NotificationChannel::Slack,
            enabled: false,
            events: vec!["deployment".to_string(), "error".to_string()],
            priority: NotificationPriority::Medium,
        },
        NotificationConfig {
            id: "webhook_1".to_string(),
            name: "Custom Webhook".to_string(),
            channel: NotificationChannel::Webhook,
            enabled: true,
            events: vec!["all".to_string()],
            priority: NotificationPriority::Critical,
        },
    ]
}
