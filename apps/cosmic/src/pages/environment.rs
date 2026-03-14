//! Environment 页面 - 环境变量配置

use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, scrollable, text, text_input};
use cosmic::Element;

use crate::app_new::Message;

#[derive(Debug, Clone)]
pub struct EnvVariable {
    pub key: String,
    pub value: String,
    pub description: String,
    pub is_secret: bool,
}

pub fn view_environment<'a>(variables: &'a [EnvVariable]) -> Element<'a, Message> {
    let title = text("Environment Variables").size(24);
    let description = text("Manage environment variables for all agents and channels")
        .size(14);
    
    let add_button = button::text("+ Add Variable")
        .on_press(Message::RefreshStatus);
    
    let header = row()
        .push(column().push(title).push(description).spacing(4))
        .push(add_button)
        .spacing(20)
        .align_y(Alignment::Center);
    
    let mut var_list = column().spacing(8);
    
    for var in variables {
        var_list = var_list.push(create_env_row(var));
    }
    
    let content = column()
        .push(header)
        .push(scrollable(var_list).height(Length::Fill))
        .spacing(20)
        .padding(20)
        .width(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn create_env_row<'a>(var: &'a EnvVariable) -> Element<'a, Message> {
    let key_text = text(&var.key).size(13);
    
    let value_display = if var.is_secret {
        text("••••••••").size(13)
    } else {
        text(&var.value).size(13)
    };
    
    let desc_text = text(&var.description).size(11);
    
    let info = column()
        .push(
            row()
                .push(key_text)
                .push(text("=").size(13))
                .push(value_display)
                .spacing(8)
        )
        .push(desc_text)
        .spacing(4);
    
    let controls = row()
        .push(button::text("Edit").on_press(Message::RefreshStatus))
        .push(button::text("Delete").on_press(Message::RefreshStatus))
        .spacing(8);
    
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

pub fn create_mock_env() -> Vec<EnvVariable> {
    vec![
        EnvVariable {
            key: "OPENAI_API_KEY".to_string(),
            value: "sk-***".to_string(),
            description: "OpenAI API key for GPT models".to_string(),
            is_secret: true,
        },
        EnvVariable {
            key: "ANTHROPIC_API_KEY".to_string(),
            value: "sk-ant-***".to_string(),
            description: "Anthropic API key for Claude".to_string(),
            is_secret: true,
        },
        EnvVariable {
            key: "LOG_LEVEL".to_string(),
            value: "info".to_string(),
            description: "Logging verbosity".to_string(),
            is_secret: false,
        },
        EnvVariable {
            key: "MAX_TOKENS".to_string(),
            value: "4096".to_string(),
            description: "Default max tokens for completions".to_string(),
            is_secret: false,
        },
    ]
}
