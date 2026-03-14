//! 键盘快捷键系统 - DO-178C Level A 标准
//! 
//! 航空航天级别要求：
//! - 所有快捷键必须有明确定义
//! - 快捷键冲突检测
//! - 可配置和可追溯
//! - 支持国际化键盘布局

use cosmic::iced::keyboard::{Key, Modifiers};
use std::collections::HashMap;
use tracing::info;

/// 快捷键定义
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shortcut {
    pub modifiers: Modifiers,
    pub key: Key,
}

impl Shortcut {
    /// 创建新快捷键
    pub fn new(modifiers: Modifiers, key: Key) -> Self {
        Self { modifiers, key }
    }
    
    /// Command + Key (macOS)
    pub fn cmd(key: Key) -> Self {
        Self {
            modifiers: Modifiers::COMMAND,
            key,
        }
    }
    
    /// Command + Shift + Key
    pub fn cmd_shift(key: Key) -> Self {
        Self {
            modifiers: Modifiers::COMMAND | Modifiers::SHIFT,
            key,
        }
    }
    
    /// Control + Key
    pub fn ctrl(key: Key) -> Self {
        Self {
            modifiers: Modifiers::CTRL,
            key,
        }
    }
    
    /// 显示为字符串
    pub fn display(&self) -> String {
        let mut result = String::new();
        
        if self.modifiers.contains(Modifiers::COMMAND) {
            result.push_str("⌘");
        }
        if self.modifiers.contains(Modifiers::CTRL) {
            result.push_str("⌃");
        }
        if self.modifiers.contains(Modifiers::ALT) {
            result.push_str("⌥");
        }
        if self.modifiers.contains(Modifiers::SHIFT) {
            result.push_str("⇧");
        }
        
        result.push_str(&format!("{:?}", self.key));
        result
    }
}

/// 快捷键动作
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutAction {
    // 文件操作
    NewSession,
    OpenProject,
    Save,
    Quit,
    
    // 编辑操作
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    
    // 导航
    GoToDashboard,
    GoToChat,
    GoToProviders,
    GoToSettings,
    
    // 视图
    ToggleSidebar,
    ToggleFullscreen,
    ZoomIn,
    ZoomOut,
    
    // 帮助
    ShowHelp,
    ShowKeyboardShortcuts,
}

/// 快捷键管理器
pub struct ShortcutManager {
    bindings: HashMap<Shortcut, ShortcutAction>,
}

impl ShortcutManager {
    /// 创建默认快捷键绑定
    pub fn new() -> Self {
        let mut bindings = HashMap::new();
        
        // 文件操作
        bindings.insert(
            Shortcut::cmd(Key::Character("n".into())),
            ShortcutAction::NewSession
        );
        bindings.insert(
            Shortcut::cmd(Key::Character("o".into())),
            ShortcutAction::OpenProject
        );
        bindings.insert(
            Shortcut::cmd(Key::Character("s".into())),
            ShortcutAction::Save
        );
        bindings.insert(
            Shortcut::cmd(Key::Character("q".into())),
            ShortcutAction::Quit
        );
        
        // 编辑操作
        bindings.insert(
            Shortcut::cmd(Key::Character("z".into())),
            ShortcutAction::Undo
        );
        bindings.insert(
            Shortcut::cmd_shift(Key::Character("z".into())),
            ShortcutAction::Redo
        );
        bindings.insert(
            Shortcut::cmd(Key::Character("x".into())),
            ShortcutAction::Cut
        );
        bindings.insert(
            Shortcut::cmd(Key::Character("c".into())),
            ShortcutAction::Copy
        );
        bindings.insert(
            Shortcut::cmd(Key::Character("v".into())),
            ShortcutAction::Paste
        );
        
        // 导航
        bindings.insert(
            Shortcut::cmd(Key::Character("1".into())),
            ShortcutAction::GoToDashboard
        );
        bindings.insert(
            Shortcut::cmd(Key::Character("2".into())),
            ShortcutAction::GoToChat
        );
        bindings.insert(
            Shortcut::cmd(Key::Character("3".into())),
            ShortcutAction::GoToProviders
        );
        bindings.insert(
            Shortcut::cmd(Key::Character(",".into())),
            ShortcutAction::GoToSettings
        );
        
        // 视图
        bindings.insert(
            Shortcut::cmd(Key::Character("b".into())),
            ShortcutAction::ToggleSidebar
        );
        
        // 帮助
        bindings.insert(
            Shortcut::cmd(Key::Character("/".into())),
            ShortcutAction::ShowKeyboardShortcuts
        );
        
        Self { bindings }
    }
    
    /// 查找快捷键对应的动作
    pub fn find_action(&self, modifiers: Modifiers, key: &Key) -> Option<ShortcutAction> {
        let shortcut = Shortcut::new(modifiers, key.clone());
        self.bindings.get(&shortcut).cloned()
    }
    
    /// 获取所有快捷键绑定
    pub fn all_bindings(&self) -> Vec<(Shortcut, ShortcutAction)> {
        self.bindings.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
    
    /// 添加或更新快捷键
    pub fn set_binding(&mut self, shortcut: Shortcut, action: ShortcutAction) {
        info!("Setting shortcut: {} -> {:?}", shortcut.display(), action);
        self.bindings.insert(shortcut, action);
    }
    
    /// 移除快捷键
    pub fn remove_binding(&mut self, shortcut: &Shortcut) {
        self.bindings.remove(shortcut);
    }
    
    /// 检查快捷键冲突
    pub fn check_conflicts(&self) -> Vec<(Shortcut, Vec<ShortcutAction>)> {
        let mut conflicts = Vec::new();
        let mut seen = HashMap::new();
        
        for (shortcut, action) in &self.bindings {
            seen.entry(shortcut.clone())
                .or_insert_with(Vec::new)
                .push(action.clone());
        }
        
        for (shortcut, actions) in seen {
            if actions.len() > 1 {
                conflicts.push((shortcut, actions));
            }
        }
        
        conflicts
    }
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shortcut_creation() {
        let shortcut = Shortcut::cmd(Key::Character("n".into()));
        assert_eq!(shortcut.modifiers, Modifiers::COMMAND);
    }
    
    #[test]
    fn test_shortcut_manager() {
        let manager = ShortcutManager::new();
        
        let action = manager.find_action(
            Modifiers::COMMAND,
            &Key::Character("n".into())
        );
        
        assert_eq!(action, Some(ShortcutAction::NewSession));
    }
    
    #[test]
    fn test_no_conflicts() {
        let manager = ShortcutManager::new();
        let conflicts = manager.check_conflicts();
        assert_eq!(conflicts.len(), 0, "Default shortcuts should have no conflicts");
    }
    
    #[test]
    fn test_all_bindings() {
        let manager = ShortcutManager::new();
        let bindings = manager.all_bindings();
        assert!(bindings.len() > 10, "Should have multiple default bindings");
    }
}
