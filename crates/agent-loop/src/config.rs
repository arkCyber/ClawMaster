//! Configuration for agent loop behavior

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for the agent loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentLoopConfig {
    /// Maximum number of tool iterations per message
    pub max_tool_iterations: usize,
    /// Timeout for the entire loop execution
    pub loop_timeout: Duration,
    /// Timeout for individual tool executions
    pub tool_timeout: Duration,
    /// Whether to enable tool chaining
    pub enable_tool_chaining: bool,
    /// Whether to track intermediate results
    pub track_intermediate_results: bool,
    /// Maximum memory usage for intermediate results (in bytes)
    pub max_intermediate_memory: usize,
}

impl Default for AgentLoopConfig {
    fn default() -> Self {
        Self {
            max_tool_iterations: 10,
            loop_timeout: Duration::from_secs(300), // 5 minutes
            tool_timeout: Duration::from_secs(30),   // 30 seconds
            enable_tool_chaining: true,
            track_intermediate_results: true,
            max_intermediate_memory: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Configuration for individual tool iterations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolIterationConfig {
    /// Current iteration number (0-indexed)
    pub iteration: usize,
    /// Maximum allowed iterations
    pub max_iterations: usize,
    /// Time elapsed since loop start
    pub elapsed_time: Duration,
    /// Maximum allowed time
    pub max_time: Duration,
    /// Memory used by intermediate results
    pub memory_used: usize,
    /// Maximum allowed memory
    pub max_memory: usize,
}

impl ToolIterationConfig {
    /// Check if the current iteration should continue
    pub fn should_continue(&self) -> bool {
        self.iteration < self.max_iterations 
            && self.elapsed_time < self.max_time 
            && self.memory_used < self.max_memory
    }
    
    /// Get remaining iterations
    pub fn remaining_iterations(&self) -> usize {
        self.max_iterations.saturating_sub(self.iteration)
    }
    
    /// Get remaining time
    pub fn remaining_time(&self) -> Duration {
        self.max_time.saturating_sub(self.elapsed_time)
    }
    
    /// Get remaining memory
    pub fn remaining_memory(&self) -> usize {
        self.max_memory.saturating_sub(self.memory_used)
    }
}
