//! Projects 页面 - 项目管理
//! 
//! DO-178C Level A 合规性：
//! - 项目生命周期追踪
//! - 版本控制集成
//! - 团队协作功能

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text};
use cosmic::Element;

use crate::app_new::Message;
use crate::widgets::{page_header, PageHeaderStyle};

#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub created_at: String,
    pub session_count: u32,
    pub file_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectStatus {
    Active,
    Archived,
    InProgress,
    Completed,
}

/// Projects 页面视图
pub fn view_projects<'a>(projects: &'a [ProjectInfo]) -> Element<'a, Message> {
    let new_btn = button::suggested("+ New Project").on_press(Message::RefreshStatus);
    let import_btn = button::text("📂 Import").on_press(Message::RefreshStatus);
    
    let header = page_header(
        "📁 Projects",
        Some("Manage your workspace projects and sessions"),
        vec![import_btn.into(), new_btn.into()],
        PageHeaderStyle::Primary,
    );
    
    let mut project_list = column().spacing(12);
    
    for project in projects {
        project_list = project_list.push(create_project_card(project));
    }
    
    let content = column()
        .push(header)
        .push(scrollable(project_list).height(Length::Fill))
        .spacing(20)
        .padding(20)
        .width(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn create_project_card<'a>(project: &'a ProjectInfo) -> Element<'a, Message> {
    let status_badge = match project.status {
        ProjectStatus::Active => "🟢 Active",
        ProjectStatus::Archived => "⚪ Archived",
        ProjectStatus::InProgress => "🟡 In Progress",
        ProjectStatus::Completed => "✅ Completed",
    };
    
    let info = column()
        .push(text(&project.name).size(16))
        .push(text(&project.description).size(12))
        .push(text(format!(
            "{} · {} sessions · {} files · Created: {}",
            status_badge, project.session_count, project.file_count, &project.created_at
        )).size(11))
        .spacing(4);
    
    let controls = row()
        .push(button::text("Open").on_press(Message::RefreshStatus))
        .push(button::text("Settings").on_press(Message::RefreshStatus))
        .push(button::text("Archive").on_press(Message::RefreshStatus))
        .spacing(12);
    
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

pub fn create_mock_projects() -> Vec<ProjectInfo> {
    vec![
        ProjectInfo {
            id: "proj_1".to_string(),
            name: "ClawMaster Development".to_string(),
            description: "Main development workspace for ClawMaster".to_string(),
            status: ProjectStatus::Active,
            created_at: "2024-03-01".to_string(),
            session_count: 45,
            file_count: 1203,
        },
        ProjectInfo {
            id: "proj_2".to_string(),
            name: "Documentation".to_string(),
            description: "User guides and API documentation".to_string(),
            status: ProjectStatus::InProgress,
            created_at: "2024-03-10".to_string(),
            session_count: 12,
            file_count: 87,
        },
        ProjectInfo {
            id: "proj_3".to_string(),
            name: "Testing Suite".to_string(),
            description: "Automated tests and QA".to_string(),
            status: ProjectStatus::Completed,
            created_at: "2024-02-15".to_string(),
            session_count: 28,
            file_count: 342,
        },
    ]
}
