//! Crons 页面 - 定时任务管理

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, toggler};
use cosmic::Element;

use crate::app_new::Message;
use crate::widgets::{page_header, PageHeaderStyle};

#[derive(Debug, Clone)]
pub struct CronInfo {
    pub id: String,
    pub name: String,
    pub schedule: String,
    pub enabled: bool,
    pub last_run: Option<String>,
    pub next_run: String,
    pub status: CronStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CronStatus {
    Active,
    Paused,
    Failed,
    Running,
}

pub fn view_crons<'a>(crons: &'a [CronInfo]) -> Element<'a, Message> {
    let add_btn = button::suggested("+ Add Cron").on_press(Message::RefreshStatus);
    let import_btn = button::text("Import").on_press(Message::RefreshStatus);
    
    let header = page_header(
        "⏰ Scheduled Tasks",
        Some("Manage cron jobs and scheduled automations"),
        vec![import_btn.into(), add_btn.into()],
        PageHeaderStyle::Utility,
    );
    
    let mut cron_list = column().spacing(12);
    
    for cron in crons {
        cron_list = cron_list.push(create_cron_card(cron));
    }
    
    let content = column()
        .push(header)
        .push(scrollable(cron_list).height(Length::Fill))
        .spacing(20)
        .padding(20)
        .width(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn create_cron_card<'a>(cron: &'a CronInfo) -> Element<'a, Message> {
    let status_icon = match cron.status {
        CronStatus::Active => "🟢",
        CronStatus::Paused => "⏸️",
        CronStatus::Failed => "🔴",
        CronStatus::Running => "🔄",
    };
    
    let info = column()
        .push(text(&cron.name).size(16))
        .push(text(format!("Schedule: {}", &cron.schedule)).size(12))
        .push(text(format!("Next run: {}", &cron.next_run)).size(12))
        .spacing(4);
    
    let controls = row()
        .push(text(status_icon).size(16))
        .push(toggler(cron.enabled).on_toggle(|_| Message::RefreshStatus))
        .push(button::text("Run Now").on_press(Message::RefreshStatus))
        .push(button::text("Edit").on_press(Message::RefreshStatus))
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

pub fn create_mock_crons() -> Vec<CronInfo> {
    vec![
        CronInfo {
            id: "cron_1".to_string(),
            name: "Daily Backup".to_string(),
            schedule: "0 2 * * *".to_string(),
            enabled: true,
            last_run: Some("2024-03-14 02:00".to_string()),
            next_run: "2024-03-15 02:00".to_string(),
            status: CronStatus::Active,
        },
        CronInfo {
            id: "cron_2".to_string(),
            name: "Weekly Report".to_string(),
            schedule: "0 9 * * 1".to_string(),
            enabled: true,
            last_run: Some("2024-03-11 09:00".to_string()),
            next_run: "2024-03-18 09:00".to_string(),
            status: CronStatus::Active,
        },
    ]
}
