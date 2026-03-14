use crate::ToolResult;
use serde::{Deserialize, Serialize};

/// Execution context that maintains state across iterations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub task: String,
    pub thoughts: Vec<String>,
    pub tool_results: Vec<ToolResult>,
    pub iteration: usize,
}

impl ExecutionContext {
    pub fn new(task: String) -> Self {
        Self {
            task,
            thoughts: Vec::new(),
            tool_results: Vec::new(),
            iteration: 0,
        }
    }

    pub fn add_thought(&mut self, thought: String) {
        self.thoughts.push(thought);
        self.iteration += 1;
    }

    pub fn add_tool_result(&mut self, result: ToolResult) {
        self.tool_results.push(result);
    }

    pub fn get_last_thought(&self) -> Option<&String> {
        self.thoughts.last()
    }

    pub fn get_last_tool_result(&self) -> Option<&ToolResult> {
        self.tool_results.last()
    }

    pub fn get_successful_results(&self) -> Vec<&ToolResult> {
        self.tool_results
            .iter()
            .filter(|r| r.success)
            .collect()
    }

    pub fn get_failed_results(&self) -> Vec<&ToolResult> {
        self.tool_results
            .iter()
            .filter(|r| !r.success)
            .collect()
    }

    pub fn get_summary(&self) -> String {
        let mut summary = format!("Task: {}\n\n", self.task);
        
        summary.push_str(&format!("Iterations: {}\n", self.iteration));
        summary.push_str(&format!("Thoughts: {}\n", self.thoughts.len()));
        summary.push_str(&format!("Tool executions: {}\n", self.tool_results.len()));
        
        let successful = self.get_successful_results().len();
        let failed = self.get_failed_results().len();
        summary.push_str(&format!("Successful: {}, Failed: {}\n\n", successful, failed));
        
        if !self.thoughts.is_empty() {
            summary.push_str("Recent thoughts:\n");
            for thought in self.thoughts.iter().rev().take(3) {
                summary.push_str(&format!("- {}\n", thought));
            }
        }
        
        summary
    }

    pub fn clear(&mut self) {
        self.thoughts.clear();
        self.tool_results.clear();
        self.iteration = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = ExecutionContext::new("test task".to_string());
        assert_eq!(ctx.task, "test task");
        assert_eq!(ctx.iteration, 0);
        assert!(ctx.thoughts.is_empty());
        assert!(ctx.tool_results.is_empty());
    }

    #[test]
    fn test_add_thought() {
        let mut ctx = ExecutionContext::new("test".to_string());
        ctx.add_thought("First thought".to_string());
        
        assert_eq!(ctx.iteration, 1);
        assert_eq!(ctx.thoughts.len(), 1);
        assert_eq!(ctx.get_last_thought(), Some(&"First thought".to_string()));
    }

    #[test]
    fn test_add_tool_result() {
        let mut ctx = ExecutionContext::new("test".to_string());
        
        let result = ToolResult {
            tool_name: "test_tool".to_string(),
            output: "output".to_string(),
            success: true,
            error: None,
        };
        
        ctx.add_tool_result(result);
        assert_eq!(ctx.tool_results.len(), 1);
        assert_eq!(ctx.get_successful_results().len(), 1);
        assert_eq!(ctx.get_failed_results().len(), 0);
    }

    #[test]
    fn test_get_summary() {
        let mut ctx = ExecutionContext::new("test task".to_string());
        ctx.add_thought("Thinking...".to_string());
        
        let summary = ctx.get_summary();
        assert!(summary.contains("test task"));
        assert!(summary.contains("Iterations: 1"));
    }

    #[test]
    fn test_clear() {
        let mut ctx = ExecutionContext::new("test".to_string());
        ctx.add_thought("thought".to_string());
        
        ctx.clear();
        assert_eq!(ctx.iteration, 0);
        assert!(ctx.thoughts.is_empty());
    }
}
