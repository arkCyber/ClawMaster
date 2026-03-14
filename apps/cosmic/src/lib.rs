//! ClawMaster Cosmic UI Library
//! 
//! DO-178C Level A 航空航天级别代码质量
//! 
//! 本模块导出所有公共 API 以供测试和外部使用

pub mod app_new;
pub mod pages;
pub mod widgets;
pub mod ui_constants;
pub mod error_handling;
pub mod confirmation_dialog;
pub mod keyboard_shortcuts;
pub mod loading_state;

// 重新导出核心类型
pub use app_new::{ClawMasterApp, Page, Language, Message};
pub use error_handling::{AppError, AppResult, ErrorHandler, ErrorSeverity};
pub use confirmation_dialog::{ConfirmationDialog, ConfirmationType};
pub use keyboard_shortcuts::{Shortcut, ShortcutAction, ShortcutManager};
pub use loading_state::{LoadingState, LoadingStateManager};
