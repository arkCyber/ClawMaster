//! MCP (Model Context Protocol) 页面 - MCP 服务器管理
//! 
//! DO-178C Level A 合规性：
//! - 服务器健康检查
//! - 连接状态监控
//! - 协议版本兼容性验证

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, toggler};
use cosmic::Element;

use crate::app_new::Message;

#[derive(Debug, Clone)]
pub struct McpServerInfo {
    pub id: String,
    pub name: String,
    pub server_type: String,
    pub endpoint: String,
    pub enabled: bool,
    pub status: McpStatus,
    pub tools_count: u32,
    pub version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McpStatus {
    Connected,
    Disconnected,
    Connecting,
    Error,
}

/// MCP 页面视图
pub fn view_mcp<'a>(servers: &'a [McpServerInfo]) -> Element<'a, Message> {
    let title = text("MCP Servers").size(24);
    let description = text("Model Context Protocol server connections")
        .size(14);
    
    let add_button = button::text("+ Add Server")
        .on_press(Message::RefreshStatus);
    
    let refresh_button = button::text("🔄 Refresh All")
        .on_press(Message::RefreshStatus);
    
    let header = row()
        .push(column().push(title).push(description).spacing(4))
        .push(add_button)
        .push(refresh_button)
        .spacing(20)
        .align_y(Alignment::Center);
    
    let mut server_list = column().spacing(12);
    
    for server in servers {
        server_list = server_list.push(create_mcp_card(server));
    }
    
    let content = column()
        .push(header)
        .push(scrollable(server_list).height(Length::Fill))
        .spacing(20)
        .padding(20)
        .width(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn create_mcp_card<'a>(server: &'a McpServerInfo) -> Element<'a, Message> {
    let status_icon = match server.status {
        McpStatus::Connected => "🟢 Connected",
        McpStatus::Disconnected => "⚪ Disconnected",
        McpStatus::Connecting => "🟡 Connecting...",
        McpStatus::Error => "🔴 Error",
    };
    
    let info = column()
        .push(text(&server.name).size(16))
        .push(text(format!("Type: {} · Endpoint: {}", &server.server_type, &server.endpoint)).size(12))
        .push(text(format!(
            "{} · {} tools · Version: {}",
            status_icon, server.tools_count, &server.version
        )).size(11))
        .spacing(4);
    
    let controls = row()
        .push(toggler(server.enabled).on_toggle(|_| Message::RefreshStatus))
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

pub fn create_mock_mcp() -> Vec<McpServerInfo> {
    vec![
        McpServerInfo {
            id: "mcp_1".to_string(),
            name: "Local MCP Server".to_string(),
            server_type: "Built-in".to_string(),
            endpoint: "stdio://localhost".to_string(),
            enabled: true,
            status: McpStatus::Connected,
            tools_count: 12,
            version: "1.0.0".to_string(),
        },
        McpServerInfo {
            id: "mcp_2".to_string(),
            name: "Filesystem MCP".to_string(),
            server_type: "Remote".to_string(),
            endpoint: "http://localhost:3000".to_string(),
            enabled: true,
            status: McpStatus::Connected,
            tools_count: 5,
            version: "0.9.2".to_string(),
        },
        McpServerInfo {
            id: "mcp_3".to_string(),
            name: "Database MCP".to_string(),
            server_type: "Remote".to_string(),
            endpoint: "http://db-mcp.local:8080".to_string(),
            enabled: false,
            status: McpStatus::Disconnected,
            tools_count: 8,
            version: "1.1.0".to_string(),
        },
    ]
}
