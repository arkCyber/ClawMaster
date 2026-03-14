//! Nodes 页面 - 节点管理（分布式架构）

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text};
use cosmic::Element;

use crate::app_new::Message;
use crate::widgets::{page_header, PageHeaderStyle};

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub id: String,
    pub name: String,
    pub address: String,
    pub status: NodeStatus,
    pub cpu_usage: u32,
    pub memory_mb: u32,
    pub sessions: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Online,
    Offline,
    Connecting,
    Error,
}

pub fn view_nodes<'a>(nodes: &'a [NodeInfo]) -> Element<'a, Message> {
    let add_btn = button::suggested("+ Add Node").on_press(Message::RefreshStatus);
    let refresh_btn = button::text("🔄 Refresh").on_press(Message::RefreshStatus);
    
    let header = page_header(
        "💻 Nodes",
        Some("Distributed ClawMaster nodes for high availability"),
        vec![refresh_btn.into(), add_btn.into()],
        PageHeaderStyle::Secondary,
    );
    
    let mut node_list = column().spacing(12);
    
    for node in nodes {
        node_list = node_list.push(create_node_card(node));
    }
    
    let content = column()
        .push(header)
        .push(scrollable(node_list).height(Length::Fill))
        .spacing(20)
        .padding(20)
        .width(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn create_node_card<'a>(node: &'a NodeInfo) -> Element<'a, Message> {
    let status_text = match node.status {
        NodeStatus::Online => "🟢 Online",
        NodeStatus::Offline => "⚪ Offline",
        NodeStatus::Connecting => "🟡 Connecting...",
        NodeStatus::Error => "🔴 Error",
    };
    
    let info = column()
        .push(text(&node.name).size(16))
        .push(text(&node.address).size(12))
        .push(text(format!(
            "CPU: {}% · Memory: {}MB · Sessions: {}",
            node.cpu_usage, node.memory_mb, node.sessions
        )).size(12))
        .spacing(4);
    
    let controls = row()
        .push(text(status_text).size(12))
        .push(button::text("Configure").on_press(Message::RefreshStatus))
        .push(button::text("Health Check").on_press(Message::RefreshStatus))
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

pub fn create_mock_nodes() -> Vec<NodeInfo> {
    vec![
        NodeInfo {
            id: "local".to_string(),
            name: "Local Node".to_string(),
            address: "localhost:59233".to_string(),
            status: NodeStatus::Online,
            cpu_usage: 45,
            memory_mb: 512,
            sessions: 3,
        },
        NodeInfo {
            id: "cloud1".to_string(),
            name: "Cloud Node 1".to_string(),
            address: "cloud1.clawmaster.ai:59233".to_string(),
            status: NodeStatus::Online,
            cpu_usage: 23,
            memory_mb: 1024,
            sessions: 7,
        },
    ]
}
