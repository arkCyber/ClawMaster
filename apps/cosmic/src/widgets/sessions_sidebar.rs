//! Sessions 侧边栏组件
//! 
//! 显示所有会话列表，支持搜索、过滤、新建会话

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, text_input};
use cosmic::Element;

use crate::app_new::{Message, SessionInfo};

/// Sessions 侧边栏
pub fn sessions_sidebar<'a>(
    sessions: &'a [SessionInfo],
    current_session_id: Option<&'a str>,
    search_query: &'a str,
) -> Element<'a, Message> {
    // 顶部标题和新建按钮
    let header = row()
        .push(text("Sessions").size(18))
        .push(
            button::text("+ New")
                .on_press(Message::CreateNewSession)
        )
        .spacing(10)
        .align_y(Alignment::Center);
    
    // 搜索框
    let search_box = text_input("Search sessions...", search_query)
        .on_input(Message::SessionSearchChanged)
        .width(Length::Fill);
    
    // 过滤会话
    let filtered_sessions: Vec<&SessionInfo> = sessions
        .iter()
        .filter(|s| {
            if search_query.is_empty() {
                true
            } else {
                s.title.to_lowercase().contains(&search_query.to_lowercase())
            }
        })
        .collect();
    
    // 会话列表
    let mut session_list = column().spacing(4);
    
    for session in filtered_sessions {
        let is_selected = current_session_id == Some(&session.id);
        
        let session_item = create_session_item(session, is_selected);
        session_list = session_list.push(session_item);
    }
    
    // 完整布局
    let content = column()
        .push(header)
        .push(search_box)
        .push(
            scrollable(session_list)
                .height(Length::Fill)
        )
        .spacing(12)
        .padding(16);
    
    container(content)
        .width(Length::Fixed(280.0))
        .height(Length::Fill)
        .into()
}

/// 创建单个会话项
fn create_session_item<'a>(
    session: &'a SessionInfo,
    is_selected: bool,
) -> Element<'a, Message> {
    let title_text = text(&session.title)
        .size(14);
    
    let info_text = text(format!("{} msgs · {}", 
        session.message_count,
        &session.created_at
    ))
        .size(11);
    
    let status_indicator = if session.is_active {
        text("🟢").size(12)
    } else {
        text("⚪").size(12)
    };
    
    let content = column()
        .push(
            row()
                .push(title_text)
                .push(status_indicator)
                .spacing(8)
                .align_y(Alignment::Center)
        )
        .push(info_text)
        .spacing(4);
    
    let item_button = button::custom(
        container(content)
            .padding(12)
            .width(Length::Fill)
    )
        .on_press(Message::SelectSession(session.id.clone()));
    
    // 高亮选中项
    let styled_button = if is_selected {
        container(item_button)
            .padding(2)
    } else {
        container(item_button)
    };
    
    styled_button.into()
}
