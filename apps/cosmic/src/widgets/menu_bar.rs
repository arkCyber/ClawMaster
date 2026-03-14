//! 菜单栏组件 - 实现真正的下拉菜单系统
//! 
//! DO-178C Level A 要求：
//! - 所有菜单项都有明确的功能
//! - 快捷键清晰可见
//! - 危险操作有确认机制

use crate::app_new::{Message, Page};

/// 菜单项定义
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub shortcut: Option<String>,
    pub message: Message,
    pub enabled: bool,
}

impl MenuItem {
    pub fn new(label: &str, message: Message) -> Self {
        Self {
            label: label.to_string(),
            shortcut: None,
            message,
            enabled: true,
        }
    }
    
    pub fn with_shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }
    
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// 创建 File 菜单项
pub fn file_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem::new("New Session", Message::CreateNewSession)
            .with_shortcut("⌘N"),
        MenuItem::new("Open Project", Message::OpenProject)
            .with_shortcut("⌘O"),
        MenuItem::new("Save Session", Message::SaveSession)
            .with_shortcut("⌘S"),
        MenuItem::new("Export Chat", Message::ExportChat)
            .with_shortcut("⌘E"),
        MenuItem::new("Quit", Message::QuitApplication)
            .with_shortcut("⌘Q"),
    ]
}

/// 创建 Edit 菜单项
pub fn edit_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem::new("Undo", Message::Undo)
            .with_shortcut("⌘Z"),
        MenuItem::new("Redo", Message::Redo)
            .with_shortcut("⌘⇧Z"),
        MenuItem::new("Cut", Message::Cut)
            .with_shortcut("⌘X"),
        MenuItem::new("Copy", Message::Copy)
            .with_shortcut("⌘C"),
        MenuItem::new("Paste", Message::Paste)
            .with_shortcut("⌘V"),
        MenuItem::new("Clear Chat", Message::ClearChat),
    ]
}

/// 创建 View 菜单项
pub fn view_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem::new("Dashboard", Message::NavigateTo(Page::Dashboard))
            .with_shortcut("⌘1"),
        MenuItem::new("Chat", Message::NavigateTo(Page::Chat))
            .with_shortcut("⌘2"),
        MenuItem::new("Providers", Message::NavigateTo(Page::Providers))
            .with_shortcut("⌘3"),
        MenuItem::new("Settings", Message::NavigateTo(Page::Settings))
            .with_shortcut("⌘,"),
        MenuItem::new("Toggle Sidebar", Message::ToggleSidebar)
            .with_shortcut("⌘B"),
    ]
}

/// 创建 Help 菜单项
pub fn help_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem::new("Documentation", Message::OpenDocumentation),
        MenuItem::new("Keyboard Shortcuts", Message::ShowKeyboardShortcuts)
            .with_shortcut("⌘/"),
        MenuItem::new("Report Issue", Message::ReportIssue),
        MenuItem::new("About ClawMaster", Message::ShowAbout),
    ]
}

// 注意：菜单栏现在使用 app_new.rs 中的 create_menu_bar() 方法实现
// 使用 libcosmic 的 popover 组件实现真正的弹出式菜单
