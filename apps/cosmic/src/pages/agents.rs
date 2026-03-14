//! Agents 页面 - 多 Agent 管理

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, toggler};
use cosmic::Element;

use crate::app_new::Message;
use crate::widgets::{page_header, PageHeaderStyle};

#[derive(Debug, Clone)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub role: String,
    pub enabled: bool,
    pub model: String,
    pub status: AgentStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    Active,
    Idle,
    Paused,
    Error,
}

pub fn view_agents<'a>(agents: &'a [AgentInfo]) -> Element<'a, Message> {
    let add_btn = button::suggested("+ Add Agent").on_press(Message::RefreshStatus);
    let import_btn = button::text("Import").on_press(Message::RefreshStatus);
    
    let header = page_header(
        "🤖 Agents",
        Some("Manage multiple AI agents with different roles and capabilities"),
        vec![import_btn.into(), add_btn.into()],
        PageHeaderStyle::Secondary,
    );
    
    let mut agent_list = column().spacing(12);
    
    for agent in agents {
        agent_list = agent_list.push(create_agent_card(agent));
    }
    
    let content = column()
        .push(header)
        .push(scrollable(agent_list).height(Length::Fill))
        .spacing(20)
        .padding(20)
        .width(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn create_agent_card<'a>(agent: &'a AgentInfo) -> Element<'a, Message> {
    let status_icon = match agent.status {
        AgentStatus::Active => "🟢",
        AgentStatus::Idle => "🟡",
        AgentStatus::Paused => "⏸️",
        AgentStatus::Error => "🔴",
    };
    
    let info = column()
        .push(text(&agent.name).size(16))
        .push(text(format!("Role: {} · Model: {}", &agent.role, &agent.model)).size(12))
        .spacing(4);
    
    let controls = row()
        .push(text(status_icon).size(16))
        .push(toggler(agent.enabled).on_toggle(|_| Message::RefreshStatus))
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

pub fn create_mock_agents() -> Vec<AgentInfo> {
    vec![
        AgentInfo {
            id: "main".to_string(),
            name: "ClawMaster Main".to_string(),
            role: "General Assistant".to_string(),
            enabled: true,
            model: "gpt-4".to_string(),
            status: AgentStatus::Active,
        },
        AgentInfo {
            id: "code".to_string(),
            name: "Code Specialist".to_string(),
            role: "Code Analysis & Generation".to_string(),
            enabled: true,
            model: "claude-3-opus".to_string(),
            status: AgentStatus::Idle,
        },
        AgentInfo {
            id: "research".to_string(),
            name: "Research Agent".to_string(),
            role: "Web Search & Analysis".to_string(),
            enabled: false,
            model: "gemini-pro".to_string(),
            status: AgentStatus::Paused,
        },
    ]
}
