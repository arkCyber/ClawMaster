//! Channels 页面 - 17个通道配置

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, toggler};
use cosmic::Element;

use crate::app_new::Message;
use crate::widgets::{page_header, PageHeaderStyle};

#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
    pub channel_type: String,
    pub enabled: bool,
    pub connected: bool,
    pub message_count: u32,
}

pub fn view_channels<'a>(channels: &'a [ChannelInfo]) -> Element<'a, Message> {
    let test_btn = button::suggested("Test All").on_press(Message::RefreshStatus);
    let add_btn = button::text("+ Add Channel").on_press(Message::RefreshStatus);
    
    let header = page_header(
        "📡 Channels",
        Some("17 communication channels for multi-platform support"),
        vec![add_btn.into(), test_btn.into()],
        PageHeaderStyle::Utility,
    );
    
    let mut channel_list = column().spacing(12);
    
    for channel in channels {
        channel_list = channel_list.push(create_channel_card(channel));
    }
    
    let content = column()
        .push(header)
        .push(scrollable(channel_list).height(Length::Fill))
        .spacing(20)
        .padding(20)
        .width(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn create_channel_card<'a>(channel: &'a ChannelInfo) -> Element<'a, Message> {
    let status_icon = if channel.connected { "🟢" } else { "⚪" };
    
    let info = column()
        .push(text(&channel.name).size(16))
        .push(text(format!("Type: {} · {} messages", &channel.channel_type, channel.message_count)).size(12))
        .spacing(4);
    
    let controls = row()
        .push(text(status_icon).size(16))
        .push(toggler(channel.enabled).on_toggle(|_| Message::RefreshStatus))
        .push(button::text("Configure").on_press(Message::RefreshStatus))
        .push(button::text("Test").on_press(Message::RefreshStatus))
        .spacing(12)
        .align_y(Alignment::Center);
    
    let card = row()
        .push(info)
        .push(controls)
        .spacing(20)
        .align_y(Alignment::Center);
    
    container(card)
        .padding(16)
        .width(Length::Fill)
        .into()
}

pub fn create_mock_channels() -> Vec<ChannelInfo> {
    vec![
        ChannelInfo {
            id: "discord".to_string(),
            name: "Discord".to_string(),
            channel_type: "Chat".to_string(),
            enabled: true,
            connected: true,
            message_count: 1523,
        },
        ChannelInfo {
            id: "slack".to_string(),
            name: "Slack".to_string(),
            channel_type: "Team".to_string(),
            enabled: true,
            connected: true,
            message_count: 892,
        },
        ChannelInfo {
            id: "telegram".to_string(),
            name: "Telegram".to_string(),
            channel_type: "Messaging".to_string(),
            enabled: false,
            connected: false,
            message_count: 0,
        },
    ]
}
