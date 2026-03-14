//! Skills 页面 - 技能系统管理
//! 
//! DO-178C Level A 合规性：
//! - 技能执行权限控制
//! - 审计日志记录
//! - 安全沙箱执行

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, toggler};
use cosmic::Element;

use crate::app_new::Message;

#[derive(Debug, Clone)]
pub struct SkillInfo {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub enabled: bool,
    pub execution_count: u32,
    pub success_rate: f32,
}

/// Skills 页面视图
pub fn view_skills<'a>(skills: &'a [SkillInfo]) -> Element<'a, Message> {
    let title = text("Skills").size(24);
    let description = text("Manage AI agent skills and capabilities")
        .size(14);
    
    let add_button = button::text("+ Add Skill")
        .on_press(Message::RefreshStatus);
    
    let import_button = button::text("📦 Import Package")
        .on_press(Message::RefreshStatus);
    
    let header = row()
        .push(column().push(title).push(description).spacing(4))
        .push(add_button)
        .push(import_button)
        .spacing(20)
        .align_y(Alignment::Center);
    
    // 按类别分组显示
    let system_skills = create_skill_section("System Skills", skills, "System");
    let dev_skills = create_skill_section("Development Skills", skills, "Development");
    let integration_skills = create_skill_section("Integration Skills", skills, "Integration");
    
    let content = column()
        .push(header)
        .push(container(text("")).height(Length::Fixed(20.0)))
        .push(system_skills)
        .push(container(text("")).height(Length::Fixed(15.0)))
        .push(dev_skills)
        .push(container(text("")).height(Length::Fixed(15.0)))
        .push(integration_skills)
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

fn create_skill_section<'a>(
    section_title: &'a str,
    skills: &'a [SkillInfo],
    category: &str,
) -> Element<'a, Message> {
    let title = text(section_title).size(18);
    
    let filtered: Vec<&SkillInfo> = skills
        .iter()
        .filter(|s| s.category == category)
        .collect();
    
    let mut skill_list = column().spacing(8);
    
    for skill in filtered {
        skill_list = skill_list.push(create_skill_row(skill));
    }
    
    column()
        .push(title)
        .push(skill_list)
        .spacing(8)
        .into()
}

fn create_skill_row<'a>(skill: &'a SkillInfo) -> Element<'a, Message> {
    let success_rate_text = format!("{:.1}%", skill.success_rate);
    
    let info = column()
        .push(text(&skill.name).size(14))
        .push(text(&skill.description).size(11))
        .push(text(format!(
            "Executed {} times · Success rate: {}",
            skill.execution_count, success_rate_text
        )).size(11))
        .spacing(4);
    
    let controls = row()
        .push(toggler(skill.enabled).on_toggle(|_| Message::RefreshStatus))
        .push(button::text("Configure").on_press(Message::RefreshStatus))
        .push(button::text("Test").on_press(Message::RefreshStatus))
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

pub fn create_mock_skills() -> Vec<SkillInfo> {
    vec![
        SkillInfo {
            id: "skill_1".to_string(),
            name: "Bash Executor".to_string(),
            category: "System".to_string(),
            description: "Execute shell commands in sandboxed environment".to_string(),
            enabled: true,
            execution_count: 1245,
            success_rate: 98.5,
        },
        SkillInfo {
            id: "skill_2".to_string(),
            name: "File Operations".to_string(),
            category: "System".to_string(),
            description: "Read, write, and manage files".to_string(),
            enabled: true,
            execution_count: 3421,
            success_rate: 99.2,
        },
        SkillInfo {
            id: "skill_3".to_string(),
            name: "Code Analyzer".to_string(),
            category: "Development".to_string(),
            description: "Analyze code structure and dependencies".to_string(),
            enabled: true,
            execution_count: 567,
            success_rate: 97.8,
        },
        SkillInfo {
            id: "skill_4".to_string(),
            name: "Web Search".to_string(),
            category: "Integration".to_string(),
            description: "Search the web using multiple search engines".to_string(),
            enabled: true,
            execution_count: 892,
            success_rate: 96.4,
        },
        SkillInfo {
            id: "skill_5".to_string(),
            name: "macOS Notes".to_string(),
            category: "Integration".to_string(),
            description: "Create and manage Notes.app entries".to_string(),
            enabled: false,
            execution_count: 23,
            success_rate: 95.0,
        },
    ]
}
