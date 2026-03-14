//! Logs 页面 - 系统日志查看

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, text_input};
use cosmic::Element;

use crate::app_new::Message;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub fn view_logs<'a>(logs: &'a [LogEntry], filter: &'a str) -> Element<'a, Message> {
    let title = text("System Logs").size(24);
    
    let filter_input = text_input("Filter logs...", filter)
        .on_input(|_| Message::RefreshStatus)
        .width(Length::Fixed(300.0));
    
    let clear_button = button::text("Clear Logs")
        .on_press(Message::ClearChat);
    
    let header = row()
        .push(title)
        .push(filter_input)
        .push(clear_button)
        .spacing(20)
        .align_y(Alignment::Center);
    
    let mut log_list = column().spacing(2);
    
    for log in logs {
        log_list = log_list.push(create_log_entry(log));
    }
    
    let content = column()
        .push(header)
        .push(
            scrollable(log_list)
                .height(Length::Fill)
        )
        .spacing(12)
        .padding(20)
        .width(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn create_log_entry<'a>(log: &'a LogEntry) -> Element<'a, Message> {
    let level_text = match log.level {
        LogLevel::Debug => "🔍 DEBUG",
        LogLevel::Info => "ℹ️ INFO",
        LogLevel::Warning => "⚠️ WARN",
        LogLevel::Error => "❌ ERROR",
    };
    
    let entry = row()
        .push(text(&log.timestamp).size(11).width(Length::Fixed(90.0)))
        .push(text(level_text).size(11).width(Length::Fixed(80.0)))
        .push(text(&log.source).size(11).width(Length::Fixed(120.0)))
        .push(text(&log.message).size(11))
        .spacing(12);
    
    container(entry)
        .padding([4, 8])
        .width(Length::Fill)
        .into()
}

pub fn create_mock_logs() -> Vec<LogEntry> {
    vec![
        LogEntry {
            timestamp: "14:32:15".to_string(),
            level: LogLevel::Info,
            source: "gateway".to_string(),
            message: "WebSocket connection established".to_string(),
        },
        LogEntry {
            timestamp: "14:32:10".to_string(),
            level: LogLevel::Debug,
            source: "rpc".to_string(),
            message: "Processing session.list request".to_string(),
        },
        LogEntry {
            timestamp: "14:31:58".to_string(),
            level: LogLevel::Warning,
            source: "provider".to_string(),
            message: "OpenAI rate limit approaching (80%)".to_string(),
        },
        LogEntry {
            timestamp: "14:31:45".to_string(),
            level: LogLevel::Error,
            source: "channel".to_string(),
            message: "Discord channel connection timeout".to_string(),
        },
        LogEntry {
            timestamp: "14:31:30".to_string(),
            level: LogLevel::Info,
            source: "system".to_string(),
            message: "System health check passed".to_string(),
        },
    ]
}
