//! Memory 页面 - 持久化记忆系统

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text};
use cosmic::Element;

use crate::app_new::Message;
use crate::widgets::{page_header, PageHeaderStyle};

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub id: String,
    pub title: String,
    pub content_preview: String,
    pub scope: MemoryScope,
    pub size_kb: u32,
    pub last_accessed: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryScope {
    Global,
    Session,
    User,
}

pub fn view_memory<'a>(memories: &'a [MemoryInfo]) -> Element<'a, Message> {
    let add_btn = button::suggested("+ Add Memory").on_press(Message::RefreshStatus);
    let refresh_btn = button::text("🔄 Refresh Index").on_press(Message::RefreshStatus);
    
    let header = page_header(
        "🧠 Memory System",
        Some("Persistent memory storage across sessions (MEMORY.md, workspace files)"),
        vec![refresh_btn.into(), add_btn.into()],
        PageHeaderStyle::Secondary,
    );
    
    let mut memory_list = column().spacing(12);
    
    for memory in memories {
        memory_list = memory_list.push(create_memory_card(memory));
    }
    
    let content = column()
        .push(header)
        .push(scrollable(memory_list).height(Length::Fill))
        .spacing(20)
        .padding(20)
        .width(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn create_memory_card<'a>(memory: &'a MemoryInfo) -> Element<'a, Message> {
    let scope_badge = match memory.scope {
        MemoryScope::Global => "🌐 Global",
        MemoryScope::Session => "💬 Session",
        MemoryScope::User => "👤 User",
    };
    
    let info = column()
        .push(text(&memory.title).size(16))
        .push(text(&memory.content_preview).size(12))
        .push(text(format!(
            "{} · {} KB · Last accessed: {}",
            scope_badge, memory.size_kb, &memory.last_accessed
        )).size(11))
        .spacing(4);
    
    let controls = row()
        .push(button::text("View").on_press(Message::RefreshStatus))
        .push(button::text("Edit").on_press(Message::RefreshStatus))
        .push(button::text("Delete").on_press(Message::RefreshStatus))
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

pub fn create_mock_memory() -> Vec<MemoryInfo> {
    vec![
        MemoryInfo {
            id: "global_1".to_string(),
            title: "MEMORY.md".to_string(),
            content_preview: "Global workspace memory and project context...".to_string(),
            scope: MemoryScope::Global,
            size_kb: 42,
            last_accessed: "2 minutes ago".to_string(),
        },
        MemoryInfo {
            id: "session_1".to_string(),
            title: "Session: Project Planning".to_string(),
            content_preview: "Discussion about API architecture...".to_string(),
            scope: MemoryScope::Session,
            size_kb: 18,
            last_accessed: "5 minutes ago".to_string(),
        },
        MemoryInfo {
            id: "user_1".to_string(),
            title: "USER.md".to_string(),
            content_preview: "User preferences and working style...".to_string(),
            scope: MemoryScope::User,
            size_kb: 8,
            last_accessed: "1 hour ago".to_string(),
        },
    ]
}
