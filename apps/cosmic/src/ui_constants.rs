//! UI 常量和主题系统
//! 
//! DO-178C Level A 要求：
//! - 所有 UI 常量集中管理
//! - 一致的设计系统
//! - 易于维护和修改

use cosmic::iced::Color;

// ══════════════════════════════════════════════════════════════════════════
// 间距系统
// ══════════════════════════════════════════════════════════════════════════

pub const SPACING_XS: u16 = 4;
pub const SPACING_SM: u16 = 8;
pub const SPACING_MD: u16 = 12;
pub const SPACING_LG: u16 = 16;
pub const SPACING_XL: u16 = 24;
pub const SPACING_XXL: u16 = 32;

// ══════════════════════════════════════════════════════════════════════════
// 尺寸系统
// ══════════════════════════════════════════════════════════════════════════

pub const SIDEBAR_WIDTH: f32 = 200.0;
pub const MENU_WIDTH: f32 = 230.0;
pub const MENU_ITEM_WIDTH: f32 = 210.0;
pub const BUTTON_HEIGHT: f32 = 36.0;
pub const ICON_SIZE: u16 = 20;
pub const ICON_SIZE_SM: u16 = 16;
pub const ICON_SIZE_LG: u16 = 24;

// ══════════════════════════════════════════════════════════════════════════
// 圆角系统
// ══════════════════════════════════════════════════════════════════════════

pub const BORDER_RADIUS_SM: f32 = 4.0;
pub const BORDER_RADIUS_MD: f32 = 8.0;
pub const BORDER_RADIUS_LG: f32 = 12.0;

// ══════════════════════════════════════════════════════════════════════════
// 字体大小
// ══════════════════════════════════════════════════════════════════════════

pub const FONT_SIZE_XS: u16 = 10;
pub const FONT_SIZE_SM: u16 = 12;
pub const FONT_SIZE_MD: u16 = 14;
pub const FONT_SIZE_LG: u16 = 16;
pub const FONT_SIZE_XL: u16 = 20;
pub const FONT_SIZE_XXL: u16 = 24;
pub const FONT_SIZE_TITLE: u16 = 28;

// ══════════════════════════════════════════════════════════════════════════
// 颜色系统（语义化颜色）
// ══════════════════════════════════════════════════════════════════════════

/// 主色调 - 用于主要操作按钮
pub const COLOR_PRIMARY: Color = Color::from_rgb(0.2, 0.6, 1.0);

/// 成功色 - 用于成功状态和确认操作
pub const COLOR_SUCCESS: Color = Color::from_rgb(0.2, 0.8, 0.4);

/// 警告色 - 用于警告信息
pub const COLOR_WARNING: Color = Color::from_rgb(1.0, 0.7, 0.0);

/// 危险色 - 用于危险操作和错误
pub const COLOR_DANGER: Color = Color::from_rgb(1.0, 0.3, 0.3);

/// 信息色 - 用于一般信息提示
pub const COLOR_INFO: Color = Color::from_rgb(0.4, 0.7, 1.0);

/// 中性色 - 用于次要元素
pub const COLOR_NEUTRAL: Color = Color::from_rgb(0.5, 0.5, 0.5);

// ══════════════════════════════════════════════════════════════════════════
// 状态颜色
// ══════════════════════════════════════════════════════════════════════════

/// 在线/连接状态
pub const COLOR_ONLINE: Color = Color::from_rgb(0.2, 0.8, 0.4);

/// 离线/断开状态
pub const COLOR_OFFLINE: Color = Color::from_rgb(0.6, 0.6, 0.6);

/// 错误状态
pub const COLOR_ERROR: Color = Color::from_rgb(1.0, 0.3, 0.3);

/// 降级状态
pub const COLOR_DEGRADED: Color = Color::from_rgb(1.0, 0.7, 0.0);

// ══════════════════════════════════════════════════════════════════════════
// 窗口配置
// ══════════════════════════════════════════════════════════════════════════

pub const WINDOW_MIN_WIDTH: f32 = 1200.0;
pub const WINDOW_MIN_HEIGHT: f32 = 700.0;
pub const WINDOW_MAX_WIDTH: f32 = 2000.0;
pub const WINDOW_MAX_HEIGHT: f32 = 1400.0;
pub const WINDOW_DEFAULT_WIDTH: f32 = 1400.0;
pub const WINDOW_DEFAULT_HEIGHT: f32 = 900.0;

// ══════════════════════════════════════════════════════════════════════════
// 动画和过渡
// ══════════════════════════════════════════════════════════════════════════

pub const TRANSITION_DURATION_MS: u64 = 200;
pub const ANIMATION_DURATION_MS: u64 = 300;

// ══════════════════════════════════════════════════════════════════════════
// Z-Index 层级
// ══════════════════════════════════════════════════════════════════════════

pub const Z_INDEX_BASE: u16 = 0;
pub const Z_INDEX_SIDEBAR: u16 = 10;
pub const Z_INDEX_HEADER: u16 = 20;
pub const Z_INDEX_MENU: u16 = 30;
pub const Z_INDEX_MODAL: u16 = 40;
pub const Z_INDEX_TOOLTIP: u16 = 50;

// ══════════════════════════════════════════════════════════════════════════
// 辅助函数
// ══════════════════════════════════════════════════════════════════════════

/// 根据状态返回对应的颜色
pub fn status_color(is_ok: bool) -> Color {
    if is_ok {
        COLOR_SUCCESS
    } else {
        COLOR_DANGER
    }
}

/// 根据百分比返回对应的颜色（0-100）
pub fn percentage_color(percent: f32) -> Color {
    if percent >= 80.0 {
        COLOR_SUCCESS
    } else if percent >= 50.0 {
        COLOR_WARNING
    } else {
        COLOR_DANGER
    }
}

/// 根据健康状态返回颜色
pub enum HealthLevel {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

pub fn health_color(level: HealthLevel) -> Color {
    match level {
        HealthLevel::Healthy => COLOR_SUCCESS,
        HealthLevel::Degraded => COLOR_WARNING,
        HealthLevel::Unhealthy => COLOR_DANGER,
        HealthLevel::Unknown => COLOR_NEUTRAL,
    }
}
