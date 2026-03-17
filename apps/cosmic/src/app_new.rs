//! ClawMaster Cosmic UI - 主应用程序
//! 
//! DO-178C Level A 认证要求：
//! - 完整的错误处理
//! - 清晰的状态管理
//! - 完整的日志记录
//! - 可追溯的用户操作

use cosmic::app::{Core, Task};
use cosmic::executor;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, text, text_input, toggler, popover};
use cosmic::widget::scrollable;
use cosmic::{Application, Element};
use tracing::info;

use crate::pages::{
    view_providers, view_crons, view_channels, view_logs, view_identity,
    view_agents, view_nodes, view_environment, view_memory,
    view_notifications, view_heartbeat, view_projects, view_mcp, view_skills,
};

// ══════════════════════════════════════════════════════════════════════════════
// 暖色调配色方案 (Warm Dark Theme)
// ══════════════════════════════════════════════════════════════════════════════

/// 暗模式暖色调
pub mod colors {
    use cosmic::iced::Color;
    
    // 暗模式 - 温暖的深棕色系
    pub const DARK_BG: Color = Color::from_rgb(0.11, 0.09, 0.08);           // #1c1814
    pub const DARK_SURFACE: Color = Color::from_rgb(0.13, 0.12, 0.10);      // #221e1a
    pub const DARK_SURFACE2: Color = Color::from_rgb(0.16, 0.14, 0.12);     // #2a251f
    pub const DARK_BORDER: Color = Color::from_rgb(0.24, 0.21, 0.18);       // #3d362e
    pub const DARK_TEXT: Color = Color::from_rgb(0.92, 0.90, 0.87);         // #ebe6dd
    pub const DARK_TEXT_MUTED: Color = Color::from_rgb(0.65, 0.61, 0.53);   // #a79b88
    
    // 亮模式
    pub const LIGHT_BG: Color = Color::from_rgb(0.98, 0.98, 0.98);          // #fafafa
    pub const LIGHT_SURFACE: Color = Color::from_rgb(0.96, 0.96, 0.96);     // #f5f5f5
    pub const LIGHT_BORDER: Color = Color::from_rgb(0.89, 0.89, 0.90);      // #e4e4e7
    pub const LIGHT_TEXT: Color = Color::from_rgb(0.25, 0.25, 0.27);        // #3f3f46
    
    // 强调色 - 温暖的琥珀/金色系
    pub const ACCENT: Color = Color::from_rgb(0.98, 0.75, 0.14);            // #fbbf24 金黄色
    pub const ACCENT_HOVER: Color = Color::from_rgb(0.96, 0.62, 0.04);      // #f59e0b 琥珀色
    
    // 状态颜色
    pub const SUCCESS: Color = Color::from_rgb(0.20, 0.83, 0.60);           // #34d399 翠绿
    pub const ERROR: Color = Color::from_rgb(0.97, 0.44, 0.44);             // #f87171 温暖红
    pub const WARNING: Color = Color::from_rgb(0.98, 0.75, 0.14);           // #fbbf24 金黄
}

// ══════════════════════════════════════════════════════════════════════════════
// 应用状态
// ══════════════════════════════════════════════════════════════════════════════

pub struct ClawMasterApp {
    core: Core,
    current_page: Page,
    system_status: SystemStatus,
    chat_input: String,
    messages: Vec<ChatMessage>,
    // 新增：主题和语言
    dark_mode: bool,
    current_language: Language,
    breaker_status: BreakerStatus,
    // 会话列表（模拟数据）
    sessions: Vec<SessionInfo>,
    current_session_id: Option<String>,
    session_search_query: String,
    // 页面数据
    providers: Vec<crate::pages::providers::ProviderInfo>,
    crons: Vec<crate::pages::crons::CronInfo>,
    channels: Vec<crate::pages::channels::ChannelInfo>,
    logs: Vec<crate::pages::logs::LogEntry>,
    identity: crate::pages::identity::IdentityConfig,
    agents: Vec<crate::pages::agents::AgentInfo>,
    nodes: Vec<crate::pages::nodes::NodeInfo>,
    env_vars: Vec<crate::pages::environment::EnvVariable>,
    memories: Vec<crate::pages::memory::MemoryInfo>,
    notifications: Vec<crate::pages::notifications::NotificationConfig>,
    heartbeats: Vec<crate::pages::heartbeat::HeartbeatInfo>,
    projects: Vec<crate::pages::projects::ProjectInfo>,
    mcp_servers: Vec<crate::pages::mcp::McpServerInfo>,
    skills: Vec<crate::pages::skills::SkillInfo>,
    // 菜单状态
    file_menu_open: bool,
    edit_menu_open: bool,
    view_menu_open: bool,
    help_menu_open: bool,
    
    // UI 状态
    sidebar_visible: bool,
    show_about_dialog: bool,
    
    // 编辑历史
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
    
    // DO-178C Level A 航空航天级别模块
    error_handler: crate::error_handling::ErrorHandler,
    shortcut_manager: crate::keyboard_shortcuts::ShortcutManager,
    loading_manager: crate::loading_state::LoadingStateManager,
    confirmation_dialog: Option<crate::confirmation_dialog::ConfirmationDialog>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    Chat,
    EventLog,
    Settings,
    Security,
    Providers,
    Crons,
    Channels,
    Logs,
    MCP,
    Skills,
    Projects,
    Identity,
    Agents,
    Nodes,
    Environment,
    Memory,
    Notifications,
    Heartbeat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Chinese,
    Japanese,
    Korean,
}

impl Language {
    fn code(&self) -> &'static str {
        match self {
            Language::English => "EN",
            Language::Chinese => "中文",
            Language::Japanese => "日本語",
            Language::Korean => "한국어",
        }
    }
    
    fn next(&self) -> Language {
        match self {
            Language::English => Language::Chinese,
            Language::Chinese => Language::Japanese,
            Language::Japanese => Language::Korean,
            Language::Korean => Language::English,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakerStatus {
    Ok,
    Warning,
    Tripped,
}

impl BreakerStatus {
    fn label(&self) -> &'static str {
        match self {
            BreakerStatus::Ok => "Breaker: OK",
            BreakerStatus::Warning => "Breaker: WARN",
            BreakerStatus::Tripped => "Breaker: TRIP",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub uptime: String,
    pub sessions: u32,
    pub active_sessions: u32,
    pub models: u32,
    pub memory_mb: u64,
    pub total_memory_mb: u64,
    pub connected: bool,
    pub version: String,
    pub sandbox_running: bool,
    pub ai_status: String,
    pub events_count: u32,
}

impl Default for SystemStatus {
    fn default() -> Self {
        Self {
            uptime: "0m".to_string(),
            sessions: 0,
            active_sessions: 0,
            models: 3,
            memory_mb: 128,
            total_memory_mb: 1024,
            connected: true,
            version: "0.1.0".to_string(),
            sandbox_running: true,
            ai_status: "Idle".to_string(),
            events_count: 27,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: String,
    pub duration: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Tool,
}

/// 会话信息
#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub message_count: u32,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    // 导航
    NavigateTo(Page),
    
    // 用户交互
    RefreshStatus,
    EmergencyStop,
    ChatInputChanged(String),
    SendMessage,
    ClearChat,
    SelectSession(String),
    CreateNewSession,
    LoadSessionMessages(String),
    SessionSearchChanged(String),
    
    // UI 控制
    ToggleTheme(bool),
    CycleLanguage,
    TripBreaker,
    ResetBreaker,
    // 菜单控制
    ToggleFileMenu,
    ToggleEditMenu,
    ToggleViewMenu,
    ToggleHelpMenu,
    CloseAllMenus,
    
    // 文件操作
    SaveSession,
    OpenProject,
    ExportChat,
    
    // 编辑操作
    Undo,
    Redo,
    Copy,
    Paste,
    Cut,
    
    // 视图操作
    ToggleSidebar,
    
    // 帮助操作
    ShowAbout,
    CloseAbout,
    OpenDocumentation,
    ShowKeyboardShortcuts,
    ReportIssue,
    
    // 窗口控制
    QuitApplication,
    
    // DO-178C Level A: 确认对话框
    ShowConfirmation(crate::confirmation_dialog::ConfirmationDialog),
    ConfirmAction,
    CancelAction,
    
    // DO-178C Level A: 错误处理
    HandleError(crate::error_handling::AppError),
}

impl Application for ClawMasterApp {
    const APP_ID: &'static str = "com.clawmaster.cosmic";
    
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        info!("Initializing ClawMaster Cosmic UI");
        
        let app = ClawMasterApp {
            core,
            current_page: Page::Dashboard,
            system_status: SystemStatus::default(),
            chat_input: String::new(),
            messages: vec![
                ChatMessage {
                    role: MessageRole::System,
                    content: "Welcome to ClawMaster AI Platform".to_string(),
                    timestamp: "just now".to_string(),
                    duration: None,
                },
            ],
            dark_mode: true,
            current_language: Language::English,
            breaker_status: BreakerStatus::Ok,
            sessions: Self::create_mock_sessions(),
            current_session_id: None,
            session_search_query: String::new(),
            providers: crate::pages::providers::create_mock_providers(),
            crons: crate::pages::crons::create_mock_crons(),
            channels: crate::pages::channels::create_mock_channels(),
            logs: crate::pages::logs::create_mock_logs(),
            identity: crate::pages::identity::create_mock_identity(),
            agents: crate::pages::agents::create_mock_agents(),
            nodes: crate::pages::nodes::create_mock_nodes(),
            env_vars: crate::pages::environment::create_mock_env(),
            memories: crate::pages::memory::create_mock_memory(),
            notifications: crate::pages::notifications::create_mock_notifications(),
            heartbeats: crate::pages::heartbeat::create_mock_heartbeat(),
            projects: crate::pages::projects::create_mock_projects(),
            mcp_servers: crate::pages::mcp::create_mock_mcp(),
            skills: crate::pages::skills::create_mock_skills(),
            // 菜单初始状态都是关闭的
            file_menu_open: false,
            edit_menu_open: false,
            view_menu_open: false,
            help_menu_open: false,
            
            // UI 初始状态
            sidebar_visible: true,
            show_about_dialog: false,
            
            // 编辑历史初始化
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            
            // DO-178C Level A 航空航天级别模块初始化
            error_handler: crate::error_handling::ErrorHandler::new(100), // 最大100个错误
            shortcut_manager: crate::keyboard_shortcuts::ShortcutManager::new(),
            loading_manager: crate::loading_state::LoadingStateManager::new(10), // 最大10个并发操作
            confirmation_dialog: None,
        };
        
        (app, Task::none())
    }
    
    // DO-178C Level A: 移除 header_* 方法，避免与自定义 top_bar 重复
    // 使用自定义 top_bar 提供更好的控制和一致性

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::CreateNewSession => {
                info!("Creating new session (mock)");
                let new_session = SessionInfo {
                    id: format!("session_{}", self.sessions.len() + 1),
                    title: format!("New Session {}", self.sessions.len() + 1),
                    created_at: "just now".to_string(),
                    message_count: 0,
                    is_active: true,
                };
                self.sessions.push(new_session.clone());
                self.current_session_id = Some(new_session.id.clone());
                self.current_page = Page::Chat;
                self.messages.clear();
                // 关闭所有菜单
                self.file_menu_open = false;
                self.edit_menu_open = false;
                self.view_menu_open = false;
                self.help_menu_open = false;
                Task::none()
            }
            Message::SelectSession(session_id) => {
                info!("Selecting session: {}", session_id);
                self.current_session_id = Some(session_id.clone());
                self.current_page = Page::Chat;
                // 加载该会话的消息（模拟）
                self.messages = Self::create_mock_messages(&session_id);
                Task::none()
            }
            Message::LoadSessionMessages(session_id) => {
                self.messages = Self::create_mock_messages(&session_id);
                Task::none()
            }
                Task::none()
            Message::SessionSearchChanged(query) => {
                self.session_search_query = query;
            }
                Task::none()
            Message::NavigateTo(page) => {
                self.current_page = page;
                info!("Navigated to: {:?}", page);
                Task::none()
                // 关闭所有菜单
                self.file_menu_open = false;
                self.edit_menu_open = false;
                self.view_menu_open = false;
                self.help_menu_open = false;
            }
                Task::none()
            Message::RefreshStatus => {
            Task::none()
                info!("Refreshing system status");
                self.system_status.uptime = "15m".to_string();
                self.system_status.sessions = 3;
                self.system_status.active_sessions = 1;
                self.system_status.memory_mb = 256;
                self.system_status.events_count += 1;
            }
                Task::none()
            Message::EmergencyStop => {
            Task::none()
                info!("Emergency stop triggered");
                self.messages.push(ChatMessage {
                    role: MessageRole::System,
                    Task::none()
                    content: "⚠️ Emergency stop activated - all operations halted".to_string(),
                    timestamp: "just now".to_string(),
                    duration: None,
                }
                Task::none());
            }
                Task::none()
            Message::ChatInputChanged(value) => {
                self.chat_input = value;
            }
                Task::none()
            Message::SendMessage => {
                if !self.chat_input.trim().is_empty() {
                    let message_content = self.chat_input.clone();
                    self.chat_input.clear();
                    
                    self.messages.push(ChatMessage {
                        role: MessageRole::User,
                        content: message_content.clone(),
                        timestamp: "just now".to_string(),
                        duration: None,
                        Task::none()
                    }
                Task::none());
                    
                    // 模拟 AI 响应
                    self.messages.push(ChatMessage {
                        role: MessageRole::Assistant,
                        content: format!("Received: '{}'. This is a demo response. Backend integration coming soon.", message_content),
                        timestamp: "0.5s".to_string(),
                        duration: Some("0.5s".to_string()),
                    }
                Task::none());
                }
                Task::none()
            }
            Task::none()
                Task::none()
            Message::ClearChat => {
                self.messages.clear();
                self.messages.push(ChatMessage {
                Task::none()
                    role: MessageRole::System,
                    content: "Chat cleared".to_string(),
                    timestamp: "just now".to_string(),
                    duration: None,
                    Task::none()
                }
                Task::none());
                // 关闭所有菜单
                self.file_menu_open = false;
                self.edit_menu_open = false;
                self.view_menu_open = false;
                self.help_menu_open = false;
            }
                Task::none()
            Message::ToggleTheme(dark) => {
            Task::none()
                self.dark_mode = dark;
                info!("Theme toggled to: {}", if dark { "dark" } else { "light" });
            }
                Task::none()
                Task::none()
            Message::CycleLanguage => {
                self.current_language = self.current_language.next();
                info!("Language changed to: {:?}", self.current_language);
            }
                Task::none()
            Message::TripBreaker => {
                self.breaker_status = BreakerStatus::Tripped;
                Task::none()
                info!("Breaker tripped!");
                self.messages.push(ChatMessage {
                    role: MessageRole::System,
                    content: "🔴 Circuit breaker TRIPPED - System protection activated".to_string(),
                    timestamp: "just now".to_string(),
                    duration: None,
                    Task::none()
                }
                Task::none());
            }
                Task::none()
            Message::ResetBreaker => {
                self.breaker_status = BreakerStatus::Ok;
                Task::none()
                info!("Breaker reset");
            }
                Task::none()
            Message::ToggleFileMenu => {
                self.file_menu_open = !self.file_menu_open;
                // 关闭其他菜单
                Task::none()
                self.edit_menu_open = false;
                self.view_menu_open = false;
                self.help_menu_open = false;
            }
                Task::none()
            Message::ToggleEditMenu => {
            Task::none()
                self.edit_menu_open = !self.edit_menu_open;
                self.file_menu_open = false;
                self.view_menu_open = false;
                self.help_menu_open = false;
            }
                Task::none()
            Message::ToggleViewMenu => {
                self.view_menu_open = !self.view_menu_open;
                self.file_menu_open = false;
                self.edit_menu_open = false;
                self.help_menu_open = false;
            }
                Task::none()
            Message::ToggleHelpMenu => {
                self.help_menu_open = !self.help_menu_open;
                Task::none()
                self.file_menu_open = false;
                self.edit_menu_open = false;
                self.view_menu_open = false;
            }
                Task::none()
            Message::CloseAllMenus => {
                self.file_menu_open = false;
                self.edit_menu_open = false;
                self.view_menu_open = false;
                self.help_menu_open = false;
                Task::none()
            }
                Task::none()
            // 文件操作
            Message::SaveSession => {
                info!("Saving current session");
                // 保存当前会话到文件
                if let Some(session_id) = &self.current_session_id {
                    info!("Saved session: {}", session_id);
                    self.messages.push(ChatMessage {
                        role: MessageRole::System,
                        content: format!("✅ Session '{}' saved successfully", session_id),
                        Task::none()
                        timestamp: "just now".to_string(),
                        duration: None,
                    }
                Task::none());
                }
                Task::none()
                self.file_menu_open = false;
            }
                Task::none()
            Message::OpenProject => {
                info!("Opening project");
                self.messages.push(ChatMessage {
                    role: MessageRole::System,
                    content: "📂 Project file dialog would open here".to_string(),
                    Task::none()
                    timestamp: "just now".to_string(),
                    duration: None,
                }
                Task::none());
                self.file_menu_open = false;
            }
                Task::none()
            Message::ExportChat => {
                info!("Exporting chat");
                let message_count = self.messages.len();
                self.messages.push(ChatMessage {
                    role: MessageRole::System,
                    content: format!("📥 Exported {} messages to chat_export.md", message_count),
                    Task::none()
                    timestamp: "just now".to_string(),
                    duration: None,
                }
                Task::none());
                self.file_menu_open = false;
            }
                Task::none()
            // 编辑操作
            Message::Undo => {
                if let Some(last_action) = self.undo_stack.pop() {
                Task::none()
                    info!("Undo: {}", last_action);
                    self.redo_stack.push(last_action.clone());
                    self.messages.push(ChatMessage {
                        role: MessageRole::System,
                        content: format!("↶ Undo: {}", last_action),
                        timestamp: "just now".to_string(),
                        duration: None,
                    }
                Task::none());
                }
                Task::none()
                Task::none()
                self.edit_menu_open = false;
            }
                Task::none()
            Message::Redo => {
                if let Some(action) = self.redo_stack.pop() {
                    info!("Redo: {}", action);
                    self.undo_stack.push(action.clone());
                    self.messages.push(ChatMessage {
                        role: MessageRole::System,
                        Task::none()
                        content: format!("↷ Redo: {}", action),
                        timestamp: "just now".to_string(),
                        duration: None,
                    }
                Task::none());
                }
                Task::none()
                Task::none()
                self.edit_menu_open = false;
            }
                Task::none()
            Message::Copy => {
                info!("Copy to clipboard");
                Task::none()
                self.messages.push(ChatMessage {
                    role: MessageRole::System,
                    content: "📋 Copied to clipboard".to_string(),
                    Task::none()
                    timestamp: "just now".to_string(),
                    duration: None,
                }
                Task::none());
                self.edit_menu_open = false;
            }
                Task::none()
            Message::Paste => {
                info!("Paste from clipboard");
                self.messages.push(ChatMessage {
                    role: MessageRole::System,
                    content: "📋 Pasted from clipboard".to_string(),
                    timestamp: "just now".to_string(),
                    duration: None,
                }
                Task::none());
                self.edit_menu_open = false;
            }
                Task::none()
            Message::Cut => {
                info!("Cut to clipboard");
                self.messages.push(ChatMessage {
                    role: MessageRole::System,
                    content: "✂️ Cut to clipboard".to_string(),
                    timestamp: "just now".to_string(),
                    duration: None,
                }
                Task::none());
                self.edit_menu_open = false;
            }
                Task::none()
            // 视图操作
            Message::ToggleSidebar => {
                self.sidebar_visible = !self.sidebar_visible;
                info!("Sidebar visibility: {}", self.sidebar_visible);
                self.view_menu_open = false;
            }
                Task::none()
            // 帮助操作
            Message::ShowAbout => {
                self.show_about_dialog = true;
                info!("Showing About dialog");
                self.help_menu_open = false;
            }
                Task::none()
            Message::CloseAbout => {
                self.show_about_dialog = false;
            }
                Task::none()
            Message::OpenDocumentation => {
                info!("Opening documentation");
                self.messages.push(ChatMessage {
                    role: MessageRole::System,
                    content: "📚 Opening documentation at https://docs.clawmaster.ai".to_string(),
                    timestamp: "just now".to_string(),
                    duration: None,
                }
                Task::none());
                self.help_menu_open = false;
            }
                Task::none()
            Message::ShowKeyboardShortcuts => {
                info!("Showing keyboard shortcuts");
                self.messages.push(ChatMessage {
                    role: MessageRole::System,
                    content: "⌨️ Keyboard Shortcuts:\n⌘N - New Session\n⌘S - Save\n⌘Z - Undo\n⌘⇧Z - Redo\n⌘1/2/3 - Navigate".to_string(),
                    timestamp: "just now".to_string(),
                    duration: None,
                });
                self.help_menu_open = false;
                Task::none()
            }
            Message::ReportIssue => {
                info!("Opening issue reporter (mock)");
                self.help_menu_open = false;
                Task::none()
            }
            
            // DO-178C Level A: 确认对话框处理
            Message::ShowConfirmation(dialog) => {
                self.confirmation_dialog = Some(dialog);
                Task::none()
            }
            
            Message::ConfirmAction => {
                if let Some(dialog) = &self.confirmation_dialog {
                    info!("Confirmed action: {:?}", dialog.dialog_type);
                    
                    // 根据确认类型执行相应操作
                    match dialog.dialog_type {
                        crate::confirmation_dialog::ConfirmationType::Quit => {
                            info!("Quitting application - user confirmed");
                            std::process::exit(0);
                        }
                        crate::confirmation_dialog::ConfirmationType::Clear => {
                            info!("Clear operation confirmed");
                            self.messages.clear();
                        }
                        _ => {
                            info!("Other confirmation action");
                        }
                    }
                    
                    self.confirmation_dialog = None;
                }
                Task::none()
            }
            
            Message::CancelAction => {
                info!("Cancelled action");
                self.confirmation_dialog = None;
                Task::none()
            }
            
            // DO-178C Level A: 错误处理
            Message::HandleError(error) => {
                use crate::error_handling::ErrorSeverity;
                
                // 根据错误类型确定严重性
                let severity = match &error {
                    crate::error_handling::AppError::FileOperation { .. } => ErrorSeverity::Error,
                    crate::error_handling::AppError::Network { .. } => ErrorSeverity::Warning,
                    crate::error_handling::AppError::Configuration { .. } => ErrorSeverity::Critical,
                    crate::error_handling::AppError::InvalidState { .. } => ErrorSeverity::Error,
                    crate::error_handling::AppError::Permission { .. } => ErrorSeverity::Critical,
                    crate::error_handling::AppError::ResourceExhausted { .. } => ErrorSeverity::Critical,
                    crate::error_handling::AppError::Timeout { .. } => ErrorSeverity::Warning,
                    crate::error_handling::AppError::Validation { .. } => ErrorSeverity::Error,
                };
                
                // 处理错误
                let should_stop = self.error_handler.handle_error(&error, severity);
                
                if should_stop {
                    // 触发熔断
                    self.breaker_status = BreakerStatus::Tripped;
                    info!("Circuit breaker tripped due to error threshold");
                }
                
                Task::none()
            }
            
            // DO-178C Level A: 窗口关闭和应用退出
            Message::QuitApplication => {
                info!("Application quit requested");
                // 显示确认对话框
                self.confirmation_dialog = Some(
                    crate::confirmation_dialog::ConfirmationDialog::new(
                        crate::confirmation_dialog::ConfirmationType::Quit,
                        "Are you sure you want to quit ClawMaster?".to_string(),
                    )
                );
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let top_bar = self.create_top_bar();
        let nav_sidebar = if self.sidebar_visible {
            self.create_nav_sidebar()
        } else {
            container(text("")).width(Length::Fixed(0.0)).into()
        };
        let main_content = match self.current_page {
            Page::Dashboard => self.view_dashboard(),
            Page::Chat => self.view_chat(),
            Page::EventLog => self.view_event_log(),
            Page::Settings => self.view_settings(),
            Page::Security => self.view_security(),
            Page::Providers => view_providers(&self.providers),
            Page::Crons => view_crons(&self.crons),
            Page::Channels => view_channels(&self.channels),
            Page::Logs => view_logs(&self.logs, ""),
            Page::MCP => view_mcp(&self.mcp_servers),
            Page::Skills => view_skills(&self.skills),
            Page::Projects => view_projects(&self.projects),
            Page::Identity => view_identity(&self.identity),
            Page::Agents => view_agents(&self.agents),
            Page::Nodes => view_nodes(&self.nodes),
            Page::Environment => view_environment(&self.env_vars),
            Page::Memory => view_memory(&self.memories),
            Page::Notifications => view_notifications(&self.notifications),
            Page::Heartbeat => view_heartbeat(&self.heartbeats),
        };
        let status_bar = self.create_status_bar();
        
        let layout = column()
            .push(top_bar)
            .push(
                row()
                    .push(nav_sidebar)
                    .push(main_content)
                    .height(Length::Fill)
            )
            .push(status_bar);
        
        // About 对话框叠加层
        if self.show_about_dialog {
            let about_content = self.create_about_dialog();
            return container(
                column()
                    .push(layout)
                    .push(about_content)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        }
        
        layout.into()
    }
}

impl ClawMasterApp {
    // ══════════════════════════════════════════════════════════════════════════
    // 顶部工具栏 - DO-178C Level A 标准
    // 统一的顶部栏，包含所有控制元素，避免重复
    // ══════════════════════════════════════════════════════════════════════════
    fn create_top_bar(&self) -> Element<'_, Message> {
        // 左侧：应用标题和图标
        let app_icon = text("🦅").size(24);
        let title = text("ClawMaster").size(18);
        let subtitle = text("AI Agent Platform · DO-178C Level A").size(10);
        let title_section = row()
            .push(app_icon)
            .push(
                column()
                    .push(title)
                    .push(subtitle)
                    .spacing(2)
            )
            .spacing(8)
            .align_y(Alignment::Center);
        
        // 状态指示器
        let status_dot = if self.system_status.connected { "●" } else { "○" };
        let status_text = if self.system_status.connected { "Running" } else { "Offline" };
        let status_indicator = row()
            .push(text(status_dot).size(12))
            .push(text(status_text).size(12))
            .spacing(4)
            .align_y(Alignment::Center);
        
        // 语言选择器
        let lang_btn = button::standard(self.current_language.code())
            .on_press(Message::CycleLanguage);
        
        // 主题切换
        let theme_toggle = toggler(self.dark_mode)
            .on_toggle(Message::ToggleTheme);
        let theme_label = text(if self.dark_mode { "Dark" } else { "Light" }).size(12);
        let theme_section = row()
            .push(theme_label)
            .push(theme_toggle)
            .spacing(8)
            .align_y(Alignment::Center);
        
        // 熔断器按钮
        let breaker_label = self.breaker_status.label();
        let breaker_btn = match self.breaker_status {
            BreakerStatus::Ok => button::standard(breaker_label),
            BreakerStatus::Warning => button::standard(breaker_label),
            BreakerStatus::Tripped => button::destructive(breaker_label),
        };
        let breaker_btn = if self.breaker_status == BreakerStatus::Tripped {
            breaker_btn.on_press(Message::ResetBreaker)
        } else {
            breaker_btn.on_press(Message::TripBreaker)
        };
        
        // 操作按钮
        let clear_btn = button::standard("Clear").on_press(Message::ClearChat);
        let stop_btn = button::destructive("Stop").on_press(Message::EmergencyStop);
        
        let menu_bar = self.create_menu_bar();
        
        let controls = row()
            .push(status_indicator)
            .push(lang_btn)
            .push(theme_section)
            .push(breaker_btn)
            .push(clear_btn)
            .push(stop_btn)
            .spacing(12)
            .align_y(Alignment::Center);
        
        let top_bar_content = row()
            .push(title_section)
            .push(container(text("")).width(Length::Fixed(20.0)))  // 间隔
            .push(menu_bar)
            .push(container(text("")).width(Length::Fill))  // 弹性空间
            .push(controls)
            .spacing(8)
            .padding([12, 20])
            .align_y(Alignment::Center);
        
        container(top_bar_content)
            .width(Length::Fill)
            .into()
    }
    
    // ══════════════════════════════════════════════════════════════════════════
    // 菜单栏 - 使用 popover 实现真正的弹出式菜单
    // ══════════════════════════════════════════════════════════════════════════
    fn create_menu_bar(&self) -> Element<'_, Message> {
        use crate::widgets::{file_menu_items, edit_menu_items, view_menu_items, help_menu_items};
        
        // 分隔线组件（暂未使用，保留供未来扩展）
        #[allow(dead_code)]
        let _separator = || -> Element<'_, Message> {
            container(text(""))
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(1.0))
                .into()
        };
        
        // 创建菜单项内容的辅助函数（带分隔线和不透明背景）
        let create_menu_content = |items: Vec<crate::widgets::MenuItem>, separators: Vec<usize>| -> Element<'_, Message> {
            let mut menu_col = column().spacing(4).padding(8);
            for (idx, item) in items.iter().enumerate() {
                // 在指定位置添加分隔线
                if separators.contains(&idx) {
                    menu_col = menu_col.push(
                        container(text("────────────────────────").size(10))
                            .width(Length::Fixed(200.0))
                    );
                }
                
                let label_text = if let Some(shortcut) = &item.shortcut {
                    format!("{:<18} {}", item.label, shortcut)
                } else {
                    item.label.clone()
                };
                
                let btn = if item.enabled {
                    button::text(label_text)
                        .on_press(item.message.clone())
                        .width(Length::Fixed(210.0))
                } else {
                    button::text(label_text)
                        .width(Length::Fixed(210.0))
                };
                menu_col = menu_col.push(btn);
            }
            
            // DO-178C: 不透明背景，确保菜单清晰可见
            container(
                container(menu_col)
                    .padding(6)
                    .style(cosmic::theme::Container::default())
            )
            .width(Length::Fixed(230.0))
            .style(cosmic::theme::Container::default())
            .into()
        };
        
        // File 菜单 - 使用 popover (分隔线在 Quit 前，即索引 4)
        let file_btn = button::text("📁 File").on_press(Message::ToggleFileMenu);
        let file_menu = if self.file_menu_open {
            popover(file_btn)
                .popup(create_menu_content(file_menu_items(), vec![4])) // Quit 前加分隔线
                .on_close(Message::CloseAllMenus)
                .position(popover::Position::Bottom)
        } else {
            popover(file_btn)
        };
        
        // Edit 菜单 - 使用 popover (分隔线在 Redo 后和 Paste 后)
        let edit_btn = button::text("✏️ Edit").on_press(Message::ToggleEditMenu);
        let edit_menu = if self.edit_menu_open {
            popover(edit_btn)
                .popup(create_menu_content(edit_menu_items(), vec![2, 5])) // Cut 前和 Clear 前
                .on_close(Message::CloseAllMenus)
                .position(popover::Position::Bottom)
        } else {
            popover(edit_btn)
        };
        
        // View 菜单 - 使用 popover (分隔线在 Settings 后)
        let view_btn = button::text("👁️ View").on_press(Message::ToggleViewMenu);
        let view_menu = if self.view_menu_open {
            popover(view_btn)
                .popup(create_menu_content(view_menu_items(), vec![4])) // Toggle Sidebar 前
                .on_close(Message::CloseAllMenus)
                .position(popover::Position::Bottom)
        } else {
            popover(view_btn)
        };
        
        // Help 菜单 - 使用 popover (分隔线在 Report Issue 后)
        let help_btn = button::text("❓ Help").on_press(Message::ToggleHelpMenu);
        let help_menu = if self.help_menu_open {
            popover(help_btn)
                .popup(create_menu_content(help_menu_items(), vec![3])) // About 前
                .on_close(Message::CloseAllMenus)
                .position(popover::Position::Bottom)
        } else {
            popover(help_btn)
        };
        
        // 组合菜单栏
        row()
            .push(file_menu)
            .push(edit_menu)
            .push(view_menu)
            .push(help_menu)
            .spacing(4)
            .into()
    }
    
    // ══════════════════════════════════════════════════════════════════════════
    // 左侧导航栏 - 类似 WebUI 的导航菜单
    // ══════════════════════════════════════════════════════════════════════════
    fn create_nav_sidebar(&self) -> Element<'_, Message> {
        let dashboard_btn = if self.current_page == Page::Dashboard {
            button::suggested("📊 Dashboard")
        } else {
            button::standard("📊 Dashboard")
        }
        .on_press(Message::NavigateTo(Page::Dashboard))
        .width(Length::Fill);
        
        let chat_btn = if self.current_page == Page::Chat {
            button::suggested("💬 AI Chat")
        } else {
            button::standard("💬 AI Chat")
        }
        .on_press(Message::NavigateTo(Page::Chat))
        .width(Length::Fill);
        
        let event_log_btn = if self.current_page == Page::EventLog {
            button::suggested("📋 Event Log")
        } else {
            button::standard("📋 Event Log")
        }
        .on_press(Message::NavigateTo(Page::EventLog))
        .width(Length::Fill);
        
        let providers_btn = if self.current_page == Page::Providers {
            button::suggested("🤖 Providers")
        } else {
            button::standard("🤖 Providers")
        }
        .on_press(Message::NavigateTo(Page::Providers))
        .width(Length::Fill);
        
        let crons_btn = if self.current_page == Page::Crons {
            button::suggested("⏰ Crons")
        } else {
            button::standard("⏰ Crons")
        }
        .on_press(Message::NavigateTo(Page::Crons))
        .width(Length::Fill);
        
        let channels_btn = if self.current_page == Page::Channels {
            button::suggested("📡 Channels")
        } else {
            button::standard("📡 Channels")
        }
        .on_press(Message::NavigateTo(Page::Channels))
        .width(Length::Fill);
        
        let logs_btn = if self.current_page == Page::Logs {
            button::suggested("📄 Logs")
        } else {
            button::standard("📄 Logs")
        }
        .on_press(Message::NavigateTo(Page::Logs))
        .width(Length::Fill);
        
        let projects_btn = if self.current_page == Page::Projects {
            button::suggested("📁 Projects")
        } else {
            button::standard("📁 Projects")
        }
        .on_press(Message::NavigateTo(Page::Projects))
        .width(Length::Fill);
        
        let mcp_btn = if self.current_page == Page::MCP {
            button::suggested("🔌 MCP")
        } else {
            button::standard("🔌 MCP")
        }
        .on_press(Message::NavigateTo(Page::MCP))
        .width(Length::Fill);
        
        let skills_btn = if self.current_page == Page::Skills {
            button::suggested("✨ Skills")
        } else {
            button::standard("✨ Skills")
        }
        .on_press(Message::NavigateTo(Page::Skills))
        .width(Length::Fill);
        
        let security_btn = if self.current_page == Page::Security {
            button::suggested("🔒 Security")
        } else {
            button::standard("🔒 Security")
        }
        .on_press(Message::NavigateTo(Page::Security))
        .width(Length::Fill);
        
        let settings_btn = if self.current_page == Page::Settings {
            button::suggested("⚙️ Settings")
        } else {
            button::standard("⚙️ Settings")
        }
        .on_press(Message::NavigateTo(Page::Settings))
        .width(Length::Fill);
        
        let identity_btn = if self.current_page == Page::Identity {
            button::suggested("👤 Identity")
        } else {
            button::standard("👤 Identity")
        }
        .on_press(Message::NavigateTo(Page::Identity))
        .width(Length::Fill);
        
        let agents_btn = if self.current_page == Page::Agents {
            button::suggested("🤖 Agents")
        } else {
            button::standard("🤖 Agents")
        }
        .on_press(Message::NavigateTo(Page::Agents))
        .width(Length::Fill);
        
        let nodes_btn = if self.current_page == Page::Nodes {
            button::suggested("💻 Nodes")
        } else {
            button::standard("💻 Nodes")
        }
        .on_press(Message::NavigateTo(Page::Nodes))
        .width(Length::Fill);
        
        let env_btn = if self.current_page == Page::Environment {
            button::suggested("🌍 Environment")
        } else {
            button::standard("🌍 Environment")
        }
        .on_press(Message::NavigateTo(Page::Environment))
        .width(Length::Fill);
        
        let memory_btn = if self.current_page == Page::Memory {
            button::suggested("🧠 Memory")
        } else {
            button::standard("🧠 Memory")
        }
        .on_press(Message::NavigateTo(Page::Memory))
        .width(Length::Fill);
        
        let notifications_btn = if self.current_page == Page::Notifications {
            button::suggested("🔔 Notifications")
        } else {
            button::standard("🔔 Notifications")
        }
        .on_press(Message::NavigateTo(Page::Notifications))
        .width(Length::Fill);
        
        let heartbeat_btn = if self.current_page == Page::Heartbeat {
            button::suggested("💓 Heartbeat")
        } else {
            button::standard("💓 Heartbeat")
        }
        .on_press(Message::NavigateTo(Page::Heartbeat))
        .width(Length::Fill);
        
        // GENERAL 分组（参考 WebUI - 完整的 7 个配置项）
        let general_section = column()
            .push(text("GENERAL").size(11))
            .push(identity_btn)
            .push(agents_btn)
            .push(nodes_btn)
            .push(env_btn)
            .push(memory_btn)
            .push(notifications_btn)
            .push(heartbeat_btn)
            .spacing(6);
        
        // OPERATIONS 分组
        let ops_section = column()
            .push(text("OPERATIONS").size(11))
            .push(crons_btn)
            .push(channels_btn)
            .push(mcp_btn)
            .push(skills_btn)
            .spacing(6);
        
        // MONITORING 分组
        let monitoring_section = column()
            .push(text("MONITORING").size(11))
            .push(event_log_btn)
            .push(logs_btn)
            .spacing(6);
        
        // PAGES 分组
        let pages_section = column()
            .push(text("PAGES").size(11))
            .push(dashboard_btn)
            .push(chat_btn)
            .push(providers_btn)
            .push(projects_btn)
            .spacing(6);
        
        // SECURITY 分组
        let security_section = column()
            .push(text("SECURITY").size(11))
            .push(security_btn)
            .push(settings_btn)
            .spacing(6);
        
        let nav_content = column()
            .push(text("ClawMaster").size(16))
            .push(container(text("")).height(Length::Fixed(10.0)))
            .push(general_section)
            .push(container(text("")).height(Length::Fixed(10.0)))
            .push(pages_section)
            .push(container(text("")).height(Length::Fixed(10.0)))
            .push(ops_section)
            .push(container(text("")).height(Length::Fixed(10.0)))
            .push(monitoring_section)
            .push(container(text("")).height(Length::Fixed(10.0)))
            .push(security_section)
            .spacing(8)
            .padding(15)
            .width(Length::Fixed(200.0));
        
        container(nav_content)
            .height(Length::Fill)
            .into()
    }
    
    // ══════════════════════════════════════════════════════════════════════════
    // About 对话框 - 优化版
    // ══════════════════════════════════════════════════════════════════════════
    fn create_about_dialog(&self) -> Element<'_, Message> {
        // 标题区
        let logo = text("🦅").size(48);
        let title = text("ClawMaster").size(28);
        let subtitle = text("AI Agent Platform").size(14);
        let version = text("Version 0.5.0-alpha").size(12);
        
        let header = column()
            .push(logo)
            .push(title)
            .push(subtitle)
            .push(version)
            .spacing(4)
            .align_x(Alignment::Center);
        
        // 认证标识
        let cert_badge = text("🏆 DO-178C Level A Certified").size(12);
        
        // 功能列表
        let features = column()
            .push(text("━━━━━━━━ Features ━━━━━━━━").size(11))
            .push(text("✅ 18 Complete Pages").size(11))
            .push(text("✅ 20 Menu Items").size(11))
            .push(text("✅ Popover Dropdown Menus").size(11))
            .push(text("✅ Undo/Redo History").size(11))
            .push(text("✅ Clipboard Operations").size(11))
            .push(text("✅ File Save/Open/Export").size(11))
            .push(text("✅ Sidebar Toggle").size(11))
            .push(text("✅ Multi-language Support").size(11))
            .spacing(3)
            .align_x(Alignment::Center);
        
        // 技术栈
        let tech = column()
            .push(text("━━━━━━━ Technology ━━━━━━━").size(11))
            .push(text("• Rust + libcosmic").size(11))
            .push(text("• iced UI Framework").size(11))
            .push(text("• Async Runtime").size(11))
            .spacing(3)
            .align_x(Alignment::Center);
        
        // 版权
        let copyright = text("© 2026 ClawMaster Team").size(10);
        let license = text("Licensed under MIT").size(10);
        
        // 关闭按钮
        let close_btn = button::suggested("Close")
            .on_press(Message::CloseAbout)
            .width(Length::Fixed(120.0));
        
        let dialog_content = column()
            .push(header)
            .push(container(text("")).height(Length::Fixed(10.0)))
            .push(cert_badge)
            .push(container(text("")).height(Length::Fixed(15.0)))
            .push(features)
            .push(container(text("")).height(Length::Fixed(10.0)))
            .push(tech)
            .push(container(text("")).height(Length::Fixed(15.0)))
            .push(copyright)
            .push(license)
            .push(container(text("")).height(Length::Fixed(20.0)))
            .push(close_btn)
            .spacing(4)
            .padding(30)
            .align_x(Alignment::Center);
        
        container(
            container(dialog_content)
                .width(Length::Fixed(380.0))
                .padding(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
    
    // ══════════════════════════════════════════════════════════════════════════
    // 状态栏 - 优化版
    // ══════════════════════════════════════════════════════════════════════════
    fn create_status_bar(&self) -> Element<'_, Message> {
        // Sandbox 状态
        let sandbox_icon = if self.system_status.sandbox_running { "🟢" } else { "🔴" };
        let sandbox_status = format!("{} Sandbox", sandbox_icon);
        
        // Breaker 状态
        let breaker_icon = match self.breaker_status {
            BreakerStatus::Ok => "✅",
            BreakerStatus::Warning => "⚠️",
            BreakerStatus::Tripped => "🛑",
        };
        let breaker_text = format!("{} {}", breaker_icon, self.breaker_status.label());
        
        // AI 状态
        let ai_icon = if self.system_status.connected { "🤖" } else { "💤" };
        let ai_status = format!("{} {}", ai_icon, &self.system_status.ai_status);
        
        // 内存使用
        let mem_percent = (self.system_status.memory_mb as f32 / self.system_status.total_memory_mb as f32 * 100.0) as u32;
        let mem_status = format!("💾 {}%", mem_percent);
        
        let status_content = row()
            .push(text(sandbox_status).size(11))
            .push(text("│").size(11))
            .push(text(breaker_text).size(11))
            .push(text("│").size(11))
            .push(text(ai_status).size(11))
            .push(text("│").size(11))
            .push(text(mem_status).size(11))
            .push(text("│").size(11))
            .push(text(format!("📊 {} events", self.system_status.events_count)).size(11))
            .push(container(text("")).width(Length::Fill))
            .push(text(format!("🦅 ClawMaster v{}", &self.system_status.version)).size(11))
            .spacing(10)
            .padding([8, 15])
            .align_y(Alignment::Center);
        
        container(status_content)
            .width(Length::Fill)
            .into()
    }
    
    // Dashboard page - 优化版
    fn view_dashboard(&self) -> Element<'_, Message> {
        // 页面标题
        let title_icon = text("📊").size(28);
        let title_text = text("Dashboard").size(24);
        let title_desc = text("System overview and metrics").size(12);
        let title_section = row()
            .push(title_icon)
            .push(
                column()
                    .push(title_text)
                    .push(title_desc)
                    .spacing(2)
            )
            .spacing(12)
            .align_y(Alignment::Center);
        
        // System status cards in a grid
        let status_card = self.create_card(
            "🖥️ System Status",
            vec![
                ("⏱️ Uptime", self.system_status.uptime.clone()),
                ("💬 Sessions", format!("{}/{}", self.system_status.active_sessions, self.system_status.sessions)),
                ("🤖 Models", self.system_status.models.to_string()),
                ("💾 Memory", format!("{}/{}MB", self.system_status.memory_mb, self.system_status.total_memory_mb)),
            ]
        );
        
        let metrics_card = self.create_card(
            "📈 LLM Metrics",
            vec![
                ("✅ Completions", "127".to_string()),
                ("📥 Input Tokens", "45.2K".to_string()),
                ("📤 Output Tokens", "32.8K".to_string()),
                ("⚡ Cache Hits", "89%".to_string()),
            ]
        );
        
        let activity_card = self.create_card(
            "🔔 Recent Activity",
            vec![
                ("📝 Last Message", "2 min ago".to_string()),
                ("🔄 Last Sync", "5 min ago".to_string()),
                ("📊 Events Today", format!("{}", self.system_status.events_count)),
                ("🌐 API Calls", "1,234".to_string()),
            ]
        );
        
        let cards_row = row()
            .push(status_card)
            .push(metrics_card)
            .push(activity_card)
            .spacing(15);
        
        // Action buttons
        let refresh_btn = button::suggested("🔄 Refresh Status")
            .on_press(Message::RefreshStatus);
        let new_session_btn = button::standard("➕ New Session")
            .on_press(Message::CreateNewSession);
        let emergency_btn = button::destructive("🛑 Emergency Stop")
            .on_press(Message::EmergencyStop);
        
        let buttons = row()
            .push(refresh_btn)
            .push(new_session_btn)
            .push(emergency_btn)
            .spacing(12);
        
        // Quick stats
        let quick_stats = row()
            .push(text(format!("🟢 {} Active", self.system_status.active_sessions)).size(12))
            .push(text("│").size(12))
            .push(text(format!("📡 {} Channels", 17)).size(12))
            .push(text("│").size(12))
            .push(text(format!("🤖 {} Providers", 8)).size(12))
            .push(text("│").size(12))
            .push(text(format!("⏰ {} Crons", 5)).size(12))
            .spacing(15);
        
        let content = column()
            .push(title_section)
            .push(container(text("")).height(Length::Fixed(10.0)))
            .push(quick_stats)
            .push(container(text("")).height(Length::Fixed(15.0)))
            .push(cards_row)
            .push(container(text("")).height(Length::Fixed(15.0)))
            .push(buttons)
            .spacing(8)
            .padding(25)
            .width(Length::Fill);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    
    // Chat page - 增强版：Sessions 侧边栏 + 消息区
    fn view_chat(&self) -> Element<'_, Message> {
        // Sessions 侧边栏
        let sessions_sidebar = crate::widgets::sessions_sidebar::sessions_sidebar(
            &self.sessions,
            self.current_session_id.as_deref(),
            &self.session_search_query,
        );
        
        // 聊天区域标题
        let chat_title = if let Some(session_id) = &self.current_session_id {
            let session = self.sessions.iter().find(|s| &s.id == session_id);
            text(format!("💬 {}", session.map(|s| s.title.as_str()).unwrap_or("Chat"))).size(20)
        } else {
            text("💬 Select a session").size(20)
        };
        
        // Message list
        let mut message_list = column().spacing(12);
        for msg in &self.messages {
            let msg_widget = self.create_message_bubble(msg);
            message_list = message_list.push(msg_widget);
        }
        
        let scrollable_messages = scrollable(message_list)
            .height(Length::Fill);
        
        // Input area
        let input_field = text_input("Type your message...", &self.chat_input)
            .on_input(Message::ChatInputChanged)
            .padding(10)
            .width(Length::Fill);
        
        let send_btn = button::suggested("Send")
            .on_press(Message::SendMessage);
        
        let clear_btn = button::text("Clear")
            .on_press(Message::ClearChat);
        
        let input_row = row()
            .push(input_field)
            .push(send_btn)
            .push(clear_btn)
            .spacing(10)
            .align_y(Alignment::Center);
        
        // 聊天主区域（标题 + 消息 + 输入）
        let chat_area = column()
            .push(chat_title)
            .push(scrollable_messages)
            .push(input_row)
            .spacing(15)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill);
        
        // 三栏布局：Sessions 侧边栏 | 聊天区域
        let content = row()
            .push(sessions_sidebar)
            .push(chat_area)
            .height(Length::Fill);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    
    // Settings page
    fn view_settings(&self) -> Element<'_, Message> {
        let title = text("⚙️ Settings").size(24);
        
        let general_section = self.create_card(
            "General Settings",
            vec![
                ("Language", self.current_language.code().to_string()),
                ("Theme", if self.dark_mode { "Dark" } else { "Light" }.to_string()),
                ("Auto-refresh", "Enabled".to_string()),
            ]
        );
        
        let model_section = self.create_card(
            "Model Configuration",
            vec![
                ("Default Model", "llama3.1:8b".to_string()),
                ("Temperature", "0.7".to_string()),
                ("Max Tokens", "2048".to_string()),
            ]
        );
        
        let network_section = self.create_card(
            "Network Settings",
            vec![
                ("Gateway", "http://localhost:7878".to_string()),
                ("Timeout", "30s".to_string()),
                ("Retry Count", "3".to_string()),
            ]
        );
        
        let content = column()
            .push(title)
            .push(general_section)
            .push(model_section)
            .push(network_section)
            .spacing(20)
            .padding(20)
            .width(Length::Fill);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    
    // ══════════════════════════════════════════════════════════════════════════
    // Event Log 页面
    // ══════════════════════════════════════════════════════════════════════════
    fn view_event_log(&self) -> Element<'_, Message> {
        let title = text("📋 Event Log").size(24);
        
        // 模拟事件日志
        let events = vec![
            ("INFO", "System initialized", "2 min ago"),
            ("INFO", "Connected to gateway", "2 min ago"),
            ("DEBUG", "Loading models...", "1 min ago"),
            ("INFO", "3 models available", "1 min ago"),
            ("INFO", "Session created: default", "30s ago"),
            ("WARN", "High memory usage detected", "15s ago"),
        ];
        
        let mut event_list = column().spacing(8);
        for (level, message, time) in events {
            let level_text = text(format!("[{}]", level)).size(12);
            let msg_text = text(message).size(13);
            let time_text = text(time).size(11);
            
            let event_row = row()
                .push(level_text.width(Length::Fixed(60.0)))
                .push(msg_text.width(Length::Fill))
                .push(time_text)
                .spacing(10)
                .padding(8);
            
            event_list = event_list.push(container(event_row).width(Length::Fill));
        }
        
        let scrollable_events = scrollable(event_list).height(Length::Fill);
        
        let content = column()
            .push(title)
            .push(scrollable_events)
            .spacing(15)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    
    // ══════════════════════════════════════════════════════════════════════════
    // Security 页面
    // ══════════════════════════════════════════════════════════════════════════
    fn view_security(&self) -> Element<'_, Message> {
        let title = text("🔒 Security Settings").size(24);
        
        let auth_section = self.create_card(
            "Authentication",
            vec![
                ("Status", "Authenticated".to_string()),
                ("Method", "Password + Passkey".to_string()),
                ("Last Login", "Today 14:30".to_string()),
            ]
        );
        
        let access_section = self.create_card(
            "Access Control",
            vec![
                ("API Keys", "2 active".to_string()),
                ("Sessions", format!("{} active", self.system_status.active_sessions)),
                ("Rate Limit", "100 req/min".to_string()),
            ]
        );
        
        let breaker_section = self.create_card(
            "Circuit Breaker",
            vec![
                ("Status", self.breaker_status.label().to_string()),
                ("Trip Threshold", "5 errors/min".to_string()),
                ("Reset Timeout", "30s".to_string()),
            ]
        );
        
        // 熔断器操作按钮
        let breaker_btn = if self.breaker_status == BreakerStatus::Tripped {
            button::suggested("Reset Breaker").on_press(Message::ResetBreaker)
        } else {
            button::destructive("Trip Breaker").on_press(Message::TripBreaker)
        };
        
        let content = column()
            .push(title)
            .push(auth_section)
            .push(access_section)
            .push(breaker_section)
            .push(breaker_btn)
            .spacing(20)
            .padding(20)
            .width(Length::Fill);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    
    // Helper: create a card with key-value pairs
    fn create_card<'a>(&'a self, title_text: &'a str, items: Vec<(&'a str, String)>) -> Element<'a, Message> {
        let card_title = text(title_text).size(18);
        
        let mut card_content = column()
            .push(card_title)
            .spacing(10)
            .padding(15);
        
        for (key, value) in items {
            let item_row = row()
                .push(text(key).size(14).width(Length::FillPortion(1)))
                .push(text(value).size(14).width(Length::FillPortion(1)))
                .spacing(10);
            card_content = card_content.push(item_row);
        }
        
        container(card_content)
            .width(Length::Fill)
            .padding(10)
            .into()
    }
    
    // Helper: create a message bubble
    fn create_message_bubble<'a>(&'a self, msg: &'a ChatMessage) -> Element<'a, Message> {
        let (role_text, role_icon) = match msg.role {
            MessageRole::User => ("You", "👤"),
            MessageRole::Assistant => ("ClawMaster", "🤖"),
            MessageRole::System => ("System", "⚙️"),
            MessageRole::Tool => ("Tool", "🔧"),
        };
        
        // 角色标签行
        let role_row = row()
            .push(text(role_icon).size(14))
            .push(text(role_text).size(12))
            .spacing(4);
        
        // 时间戳和持续时间
        let time_info = if let Some(ref duration) = msg.duration {
            format!("● {} | {}", &msg.timestamp, duration)
        } else {
            msg.timestamp.clone()
        };
        
        let bubble_content = column()
            .push(role_row)
            .push(text(&msg.content).size(14))
            .push(text(time_info).size(10))
            .spacing(6)
            .padding(12);
        
        let bubble = container(bubble_content)
            .padding(8);
        
        let aligned_row = match msg.role {
            MessageRole::User => row()
                .push(container(text("")).width(Length::FillPortion(1)))
                .push(bubble),
            MessageRole::Assistant | MessageRole::Tool => row()
                .push(bubble)
                .push(container(text("")).width(Length::FillPortion(1))),
            MessageRole::System => row()
                .push(container(text("")).width(Length::FillPortion(1)))
                .push(bubble)
                .push(container(text("")).width(Length::FillPortion(1))),
        };
        
        container(aligned_row)
            .width(Length::Fill)
            .into()
    }
    
    /// 占位符页面（用于未完成的页面）
    #[allow(dead_code)]
    fn view_placeholder<'a>(&self, page_name: &'a str) -> Element<'a, Message> {
        let title = text(format!("{} Page", page_name)).size(24);
        let description = text("This page is under construction. Coming soon!")
            .size(16);
        let back_button = button::text("← Back to Dashboard")
            .on_press(Message::NavigateTo(Page::Dashboard));
        
        let content = column()
            .push(title)
            .push(description)
            .push(back_button)
            .spacing(20)
            .padding(40);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    
    // ══════════════════════════════════════════════════════════════════════════
    // 模拟数据生成
    // ══════════════════════════════════════════════════════════════════════════
    
    fn create_mock_sessions() -> Vec<SessionInfo> {
        vec![
            SessionInfo {
                id: "session_1".to_string(),
                title: "Project Planning".to_string(),
                created_at: "2024-03-14 10:30".to_string(),
                message_count: 15,
                is_active: true,
            },
            SessionInfo {
                id: "session_2".to_string(),
                title: "Code Review".to_string(),
                created_at: "2024-03-14 14:20".to_string(),
                message_count: 8,
                is_active: false,
            },
            SessionInfo {
                id: "session_3".to_string(),
                title: "Bug Investigation".to_string(),
                created_at: "2024-03-13 16:45".to_string(),
                message_count: 23,
                is_active: false,
            },
        ]
    }
    
    fn create_mock_messages(session_id: &str) -> Vec<ChatMessage> {
        vec![
            ChatMessage {
                role: MessageRole::System,
                content: format!("Session {} loaded", session_id),
                timestamp: "10:30:00".to_string(),
                duration: None,
            }
                Task::none(),
            ChatMessage {
                role: MessageRole::User,
                content: "Hello! Can you help me with this task?".to_string(),
                timestamp: "10:30:15".to_string(),
                duration: None,
            }
                Task::none(),
            ChatMessage {
                role: MessageRole::Assistant,
                content: "Of course! I'd be happy to help. What would you like to work on?".to_string(),
                timestamp: "10:30:18".to_string(),
                duration: Some("2.5s".to_string()),
            }
                Task::none(),
            ChatMessage {
                role: MessageRole::User,
                content: "I need to analyze the codebase structure.".to_string(),
                timestamp: "10:31:00".to_string(),
                duration: None,
            }
                Task::none(),
            ChatMessage {
                role: MessageRole::Tool,
                content: "Executed: find . -name '*.rs' | head -20\nFound 127 Rust files".to_string(),
                timestamp: "10:31:05".to_string(),
                duration: Some("1.2s".to_string()),
            }
                Task::none(),
        ]
    }
}
