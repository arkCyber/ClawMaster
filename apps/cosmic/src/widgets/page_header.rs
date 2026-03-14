//! 页面标题组件 - 统一的页面头部设计
//! 
//! DO-178C Level A 要求：
//! - 清晰的页面标识
//! - 一致的视觉层次
//! - 操作按钮统一布局

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, text};
use cosmic::Element;

use crate::app_new::Message;

/// 页面标题样式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageHeaderStyle {
    Primary,   // 主要页面（Dashboard, Chat）
    Secondary, // 配置页面（Settings, Identity）
    Utility,   // 工具页面（Logs, Crons）
}

/// 创建标准页面标题
/// 
/// # 参数
/// - `title`: 页面主标题
/// - `description`: 页面描述（可选）
/// - `actions`: 操作按钮列表
/// - `style`: 标题样式
pub fn page_header<'a>(
    title: &'a str,
    description: Option<&'a str>,
    actions: Vec<Element<'a, Message>>,
    style: PageHeaderStyle,
) -> Element<'a, Message> {
    // 标题字号根据样式调整
    let title_size = match style {
        PageHeaderStyle::Primary => 28,
        PageHeaderStyle::Secondary => 24,
        PageHeaderStyle::Utility => 22,
    };
    
    let title_text = text(title).size(title_size);
    
    // 左侧：标题和描述
    let mut left_section = column().push(title_text);
    
    if let Some(desc) = description {
        let desc_text = text(desc).size(13);
        left_section = left_section.push(desc_text).spacing(4);
    }
    
    // 右侧：操作按钮
    let mut actions_row = row().spacing(12).align_y(Alignment::Center);
    for action in actions {
        actions_row = actions_row.push(action);
    }
    
    // 完整标题栏
    let header_content = row()
        .push(left_section)
        .push(container(text("")).width(Length::Fill))  // 弹性空间
        .push(actions_row)
        .spacing(20)
        .padding([16, 24])
        .align_y(Alignment::Center);
    
    container(header_content)
        .width(Length::Fill)
        .into()
}

/// 创建简化版页面标题（仅标题，无操作按钮）
pub fn simple_page_header<'a>(
    title: &'a str,
    description: Option<&'a str>,
) -> Element<'a, Message> {
    page_header(title, description, vec![], PageHeaderStyle::Secondary)
}

/// 创建带返回按钮的页面标题
pub fn page_header_with_back<'a>(
    title: &'a str,
    description: Option<&'a str>,
    back_message: Message,
) -> Element<'a, Message> {
    let back_btn = button::text("← Back").on_press(back_message);
    
    page_header(title, description, vec![back_btn.into()], PageHeaderStyle::Secondary)
}
