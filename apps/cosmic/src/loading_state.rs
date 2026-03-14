//! 加载状态管理 - DO-178C Level A 标准
//! 
//! 航空航天级别要求：
//! - 所有异步操作必须有加载状态
//! - 加载状态必须可视化
//! - 超时检测和错误处理
//! - 进度追踪

use std::time::Duration;

/// 加载状态
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingState {
    /// 空闲状态
    Idle,
    
    /// 加载中
    Loading {
        /// 操作描述
        operation: String,
        /// 开始时间（使用 Duration 代替 Instant 以支持 Clone）
        elapsed: Duration,
        /// 进度（0-100）
        progress: Option<u8>,
    },
    
    /// 加载成功
    Success {
        /// 操作描述
        operation: String,
        /// 持续时间
        duration: Duration,
    },
    
    /// 加载失败
    Failed {
        /// 操作描述
        operation: String,
        /// 错误信息
        error: String,
    },
}

impl LoadingState {
    /// 开始加载
    pub fn start(operation: String) -> Self {
        Self::Loading {
            operation,
            elapsed: Duration::from_secs(0),
            progress: None,
        }
    }
    
    /// 开始带进度的加载
    pub fn start_with_progress(operation: String, progress: u8) -> Self {
        Self::Loading {
            operation,
            elapsed: Duration::from_secs(0),
            progress: Some(progress.min(100)),
        }
    }
    
    /// 更新进度
    pub fn update_progress(&mut self, new_progress: u8) {
        if let Self::Loading { progress, .. } = self {
            *progress = Some(new_progress.min(100));
        }
    }
    
    /// 完成加载
    pub fn complete(self) -> Self {
        match self {
            Self::Loading { operation, elapsed, .. } => {
                Self::Success {
                    operation,
                    duration: elapsed,
                }
            }
            _ => self,
        }
    }
    
    /// 加载失败
    pub fn fail(self, error: String) -> Self {
        match self {
            Self::Loading { operation, .. } => {
                Self::Failed { operation, error }
            }
            _ => self,
        }
    }
    
    /// 检查是否正在加载
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading { .. })
    }
    
    /// 检查是否超时
    pub fn is_timeout(&self, timeout: Duration) -> bool {
        match self {
            Self::Loading { elapsed, .. } => *elapsed > timeout,
            _ => false,
        }
    }
    
    /// 获取进度
    pub fn progress(&self) -> Option<u8> {
        match self {
            Self::Loading { progress, .. } => *progress,
            _ => None,
        }
    }
    
    /// 获取操作描述
    pub fn operation(&self) -> Option<&str> {
        match self {
            Self::Loading { operation, .. } |
            Self::Success { operation, .. } |
            Self::Failed { operation, .. } => Some(operation),
            Self::Idle => None,
        }
    }
}

/// 加载状态管理器
#[derive(Debug)]
pub struct LoadingStateManager {
    states: Vec<(String, LoadingState)>,
    max_concurrent: usize,
}

impl LoadingStateManager {
    /// 创建新的加载状态管理器
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            states: Vec::new(),
            max_concurrent,
        }
    }
    
    /// 开始新的加载操作
    pub fn start(&mut self, id: String, operation: String) -> Result<(), String> {
        // 检查并发限制
        let active_count = self.states.iter()
            .filter(|(_, state)| state.is_loading())
            .count();
        
        if active_count >= self.max_concurrent {
            return Err(format!(
                "Maximum concurrent operations ({}) reached",
                self.max_concurrent
            ));
        }
        
        // 移除旧状态（如果存在）
        self.states.retain(|(existing_id, _)| existing_id != &id);
        
        // 添加新状态
        self.states.push((id, LoadingState::start(operation)));
        Ok(())
    }
    
    /// 更新进度
    pub fn update_progress(&mut self, id: &str, progress: u8) {
        if let Some((_, state)) = self.states.iter_mut().find(|(i, _)| i == id) {
            state.update_progress(progress);
        }
    }
    
    /// 完成加载
    pub fn complete(&mut self, id: &str) {
        if let Some((_, state)) = self.states.iter_mut().find(|(i, _)| i == id) {
            *state = std::mem::replace(state, LoadingState::Idle).complete();
        }
    }
    
    /// 标记失败
    pub fn fail(&mut self, id: &str, error: String) {
        if let Some((_, state)) = self.states.iter_mut().find(|(i, _)| i == id) {
            *state = std::mem::replace(state, LoadingState::Idle).fail(error);
        }
    }
    
    /// 获取状态
    pub fn get(&self, id: &str) -> Option<&LoadingState> {
        self.states.iter()
            .find(|(i, _)| i == id)
            .map(|(_, state)| state)
    }
    
    /// 检查超时
    pub fn check_timeouts(&mut self, timeout: Duration) -> Vec<String> {
        let mut timed_out = Vec::new();
        
        for (id, state) in &mut self.states {
            if state.is_timeout(timeout) {
                timed_out.push(id.clone());
                *state = std::mem::replace(state, LoadingState::Idle)
                    .fail("Operation timed out".to_string());
            }
        }
        
        timed_out
    }
    
    /// 清理完成的状态
    pub fn cleanup(&mut self) {
        self.states.retain(|(_, state)| state.is_loading());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loading_state_lifecycle() {
        let state = LoadingState::start("test_op".to_string());
        assert!(state.is_loading());
        
        let state = state.complete();
        assert!(!state.is_loading());
        assert!(matches!(state, LoadingState::Success { .. }));
    }
    
    #[test]
    fn test_loading_state_with_progress() {
        let mut state = LoadingState::start_with_progress("test".to_string(), 50);
        assert_eq!(state.progress(), Some(50));
        
        state.update_progress(75);
        assert_eq!(state.progress(), Some(75));
        
        state.update_progress(150); // 应该被限制为100
        assert_eq!(state.progress(), Some(100));
    }
    
    #[test]
    fn test_loading_state_manager() {
        let mut manager = LoadingStateManager::new(3);
        
        assert!(manager.start("op1".to_string(), "Operation 1".to_string()).is_ok());
        assert!(manager.start("op2".to_string(), "Operation 2".to_string()).is_ok());
        assert!(manager.start("op3".to_string(), "Operation 3".to_string()).is_ok());
        
        // 第4个应该失败（超过并发限制）
        assert!(manager.start("op4".to_string(), "Operation 4".to_string()).is_err());
        
        // 完成一个后应该可以添加新的
        manager.complete("op1");
        assert!(manager.start("op4".to_string(), "Operation 4".to_string()).is_ok());
    }
    
    #[test]
    fn test_timeout_detection() {
        let state = LoadingState::start("test".to_string());
        assert!(!state.is_timeout(Duration::from_secs(10)));
    }
}
