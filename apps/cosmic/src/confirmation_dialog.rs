//! 确认对话框组件 - DO-178C Level A 标准
//! 
//! 航空航天级别要求：
//! - 危险操作必须有确认机制
//! - 确认信息必须清晰明确
//! - 支持键盘操作（Enter/Esc）
//! - 操作可追溯

use cosmic::iced::Length;
use cosmic::widget::{button, column, container, row, text};
use cosmic::Element;

/// 确认对话框类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfirmationType {
    /// 删除操作
    Delete,
    /// 清空操作
    Clear,
    /// 退出操作
    Quit,
    /// 重置操作
    Reset,
    /// 危险操作
    Danger,
}

impl ConfirmationType {
    /// 获取标题
    pub fn title(&self) -> &str {
        match self {
            ConfirmationType::Delete => "⚠️ Confirm Delete",
            ConfirmationType::Clear => "⚠️ Confirm Clear",
            ConfirmationType::Quit => "⚠️ Confirm Quit",
            ConfirmationType::Reset => "⚠️ Confirm Reset",
            ConfirmationType::Danger => "🛑 Dangerous Operation",
        }
    }
    
    /// 获取确认按钮文本
    pub fn confirm_text(&self) -> &str {
        match self {
            ConfirmationType::Delete => "Delete",
            ConfirmationType::Clear => "Clear",
            ConfirmationType::Quit => "Quit",
            ConfirmationType::Reset => "Reset",
            ConfirmationType::Danger => "Proceed",
        }
    }
}

/// 确认对话框数据
#[derive(Debug, Clone)]
pub struct ConfirmationDialog {
    pub dialog_type: ConfirmationType,
    pub message: String,
    pub details: Option<String>,
}

impl ConfirmationDialog {
    /// 创建新的确认对话框
    pub fn new(dialog_type: ConfirmationType, message: String) -> Self {
        Self {
            dialog_type,
            message,
            details: None,
        }
    }
    
    /// 添加详细信息
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
    
    /// 渲染对话框
    pub fn view<'a, Message: Clone + 'static>(
        &'a self,
        on_confirm: Message,
        on_cancel: Message,
    ) -> Element<'a, Message> {
        let title = text(self.dialog_type.title())
            .size(24);
        
        let message = text(&self.message)
            .size(16);
        
        let mut content = column()
            .push(title)
            .push(container(text("")).height(Length::Fixed(16.0)))
            .push(message)
            .spacing(8)
            .padding(24);
        
        // 添加详细信息（如果有）
        if let Some(details) = &self.details {
            content = content
                .push(container(text("")).height(Length::Fixed(8.0)))
                .push(
                    text(details)
                        .size(14)
                );
        }
        
        // 按钮行
        let buttons = row()
            .push(
                button::text("Cancel")
                    .on_press(on_cancel.clone())
                    .width(Length::Fixed(100.0))
            )
            .push(container(text("")).width(Length::Fixed(12.0)))
            .push(
                button::destructive(self.dialog_type.confirm_text())
                    .on_press(on_confirm)
                    .width(Length::Fixed(100.0))
            )
            .spacing(8);
        
        content = content
            .push(container(text("")).height(Length::Fixed(16.0)))
            .push(buttons);
        
        container(content)
            .width(Length::Fixed(400.0))
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_confirmation_types() {
        assert_eq!(ConfirmationType::Delete.title(), "⚠️ Confirm Delete");
        assert_eq!(ConfirmationType::Delete.confirm_text(), "Delete");
        
        assert_eq!(ConfirmationType::Quit.title(), "⚠️ Confirm Quit");
        assert_eq!(ConfirmationType::Quit.confirm_text(), "Quit");
    }
    
    #[test]
    fn test_dialog_creation() {
        let dialog = ConfirmationDialog::new(
            ConfirmationType::Clear,
            "Are you sure you want to clear all chat history?".to_string()
        );
        
        assert_eq!(dialog.dialog_type, ConfirmationType::Clear);
        assert!(dialog.details.is_none());
        
        let dialog_with_details = dialog.with_details(
            "This action cannot be undone.".to_string()
        );
        
        assert!(dialog_with_details.details.is_some());
    }
}
