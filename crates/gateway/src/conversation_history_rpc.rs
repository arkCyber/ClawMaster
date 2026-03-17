//! RPC methods for conversation history management.

use {
    crate::conversation_history::{
        ConversationHistoryStore, ConversationTurn, IssuePriority, IssueStatus, UnresolvedIssue,
    },
    anyhow::Result,
    serde::Deserialize,
    serde_json::Value,
    std::sync::Arc,
};

/// Request to record a conversation turn.
#[derive(Debug, Deserialize)]
pub struct RecordTurnRequest {
    pub session_key: String,
    pub turn_number: i64,
    pub user_message: String,
    pub assistant_response: String,
    pub model_id: String,
    pub provider_name: String,
    pub tool_calls_count: Option<i64>,
    pub tool_calls_json: Option<String>,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub duration_ms: Option<i64>,
    pub had_error: Option<bool>,
    pub error_message: Option<String>,
}

/// Request to get conversation history.
#[derive(Debug, Deserialize)]
pub struct GetHistoryRequest {
    pub session_key: String,
    pub limit: Option<i64>,
}

/// Request to update resolution status.
#[derive(Debug, Deserialize)]
pub struct UpdateResolutionRequest {
    pub turn_id: i64,
    pub is_resolved: bool,
    pub status: Option<String>,
}

/// Request to add feedback.
#[derive(Debug, Deserialize)]
pub struct AddFeedbackRequest {
    pub turn_id: i64,
    pub feedback: String,
    pub is_positive: bool,
}

/// Request to record an unresolved issue.
#[derive(Debug, Deserialize)]
pub struct RecordIssueRequest {
    pub conversation_turn_id: i64,
    pub session_key: String,
    pub issue_summary: String,
    pub user_query: String,
    pub failed_response: Option<String>,
    pub priority: Option<String>,
}

/// Request to get unresolved issues.
#[derive(Debug, Deserialize)]
pub struct GetIssuesRequest {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub limit: Option<i64>,
}

/// Request to update issue status.
#[derive(Debug, Deserialize)]
pub struct UpdateIssueRequest {
    pub issue_id: i64,
    pub status: String,
    pub resolution_method: Option<String>,
    pub notes: Option<String>,
}

/// Request to search conversation history.
#[derive(Debug, Deserialize)]
pub struct SearchHistoryRequest {
    pub query: String,
    pub session_key: Option<String>,
    pub limit: Option<i64>,
}

/// Request to get conversation metadata.
#[derive(Debug, Deserialize)]
pub struct GetMetadataRequest {
    pub session_key: String,
}

/// RPC handler for conversation history methods.
pub struct ConversationHistoryRpc {
    store: Arc<dyn ConversationHistoryStore>,
}

impl ConversationHistoryRpc {
    pub fn new(store: Arc<dyn ConversationHistoryStore>) -> Self {
        Self { store }
    }

    /// Record a conversation turn.
    pub async fn record_turn(&self, params: Value) -> Result<Value> {
        let req: RecordTurnRequest = serde_json::from_value(params)?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let turn = ConversationTurn {
            id: 0,
            session_key: req.session_key,
            turn_number: req.turn_number,
            user_message: req.user_message,
            assistant_response: req.assistant_response,
            created_at: now,
            response_at: now + req.duration_ms.unwrap_or(0) / 1000,
            duration_ms: req.duration_ms.unwrap_or(0),
            is_resolved: !req.had_error.unwrap_or(false),
            resolution_status: if req.had_error.unwrap_or(false) {
                Some("unresolved".to_string())
            } else {
                Some("resolved".to_string())
            },
            user_feedback: None,
            feedback_at: None,
            model_id: req.model_id,
            provider_name: req.provider_name,
            tool_calls_count: req.tool_calls_count.unwrap_or(0),
            tool_calls_json: req.tool_calls_json,
            input_tokens: req.input_tokens.unwrap_or(0),
            output_tokens: req.output_tokens.unwrap_or(0),
            had_error: req.had_error.unwrap_or(false),
            error_message: req.error_message,
        };

        let turn_id = self.store.record_turn(&turn).await?;

        Ok(serde_json::json!({
            "turn_id": turn_id,
            "success": true
        }))
    }

    /// Get conversation history for a session.
    pub async fn get_history(&self, params: Value) -> Result<Value> {
        let req: GetHistoryRequest = serde_json::from_value(params)?;
        let history = self
            .store
            .get_session_history(&req.session_key, req.limit)
            .await?;
        Ok(serde_json::to_value(history)?)
    }

    /// Update resolution status of a turn.
    pub async fn update_resolution(&self, params: Value) -> Result<Value> {
        let req: UpdateResolutionRequest = serde_json::from_value(params)?;
        self.store
            .update_resolution(req.turn_id, req.is_resolved, req.status.as_deref())
            .await?;
        Ok(serde_json::json!({ "success": true }))
    }

    /// Add user feedback to a turn.
    pub async fn add_feedback(&self, params: Value) -> Result<Value> {
        let req: AddFeedbackRequest = serde_json::from_value(params)?;
        self.store
            .add_feedback(req.turn_id, &req.feedback, req.is_positive)
            .await?;
        Ok(serde_json::json!({ "success": true }))
    }

    /// Record an unresolved issue.
    pub async fn record_issue(&self, params: Value) -> Result<Value> {
        let req: RecordIssueRequest = serde_json::from_value(params)?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let issue = UnresolvedIssue {
            id: 0,
            conversation_turn_id: req.conversation_turn_id,
            session_key: req.session_key,
            issue_summary: req.issue_summary,
            user_query: req.user_query,
            failed_response: req.failed_response,
            created_at: now,
            last_attempt_at: None,
            resolved_at: None,
            attempt_count: 1,
            priority: req.priority.unwrap_or_else(|| "normal".to_string()),
            status: "open".to_string(),
            resolution_method: None,
            resolution_notes: None,
            resolved_by: None,
        };

        let issue_id = self.store.record_unresolved_issue(&issue).await?;

        Ok(serde_json::json!({
            "issue_id": issue_id,
            "success": true
        }))
    }

    /// Get unresolved issues.
    pub async fn get_issues(&self, params: Value) -> Result<Value> {
        let req: GetIssuesRequest = serde_json::from_value(params)?;

        let status = req.status.as_ref().and_then(|s| match s.as_str() {
            "open" => Some(IssueStatus::Open),
            "in_progress" => Some(IssueStatus::InProgress),
            "resolved" => Some(IssueStatus::Resolved),
            "abandoned" => Some(IssueStatus::Abandoned),
            _ => None,
        });

        let priority = req.priority.as_ref().and_then(|p| match p.as_str() {
            "low" => Some(IssuePriority::Low),
            "normal" => Some(IssuePriority::Normal),
            "high" => Some(IssuePriority::High),
            "critical" => Some(IssuePriority::Critical),
            _ => None,
        });

        let issues = self
            .store
            .get_unresolved_issues(status, priority, req.limit)
            .await?;
        Ok(serde_json::to_value(issues)?)
    }

    /// Update issue status.
    pub async fn update_issue(&self, params: Value) -> Result<Value> {
        let req: UpdateIssueRequest = serde_json::from_value(params)?;

        let status = match req.status.as_str() {
            "open" => IssueStatus::Open,
            "in_progress" => IssueStatus::InProgress,
            "resolved" => IssueStatus::Resolved,
            "abandoned" => IssueStatus::Abandoned,
            _ => anyhow::bail!("Invalid status: {}", req.status),
        };

        self.store
            .update_issue_status(
                req.issue_id,
                status,
                req.resolution_method.as_deref(),
                req.notes.as_deref(),
            )
            .await?;

        Ok(serde_json::json!({ "success": true }))
    }

    /// Search conversation history.
    pub async fn search_history(&self, params: Value) -> Result<Value> {
        let req: SearchHistoryRequest = serde_json::from_value(params)?;
        let results = self
            .store
            .search_history(&req.query, req.session_key.as_deref(), req.limit)
            .await?;
        Ok(serde_json::to_value(results)?)
    }

    /// Get conversation metadata.
    pub async fn get_metadata(&self, params: Value) -> Result<Value> {
        let req: GetMetadataRequest = serde_json::from_value(params)?;
        let metadata = self.store.get_metadata(&req.session_key).await?;
        Ok(serde_json::to_value(metadata)?)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::conversation_history::SqliteConversationHistory, sqlx::SqlitePool};

    async fn setup_test_store() -> Arc<dyn ConversationHistoryStore> {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(include_str!(
            "../migrations/20260316000001_conversation_history.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        Arc::new(SqliteConversationHistory::new(pool))
    }

    #[tokio::test]
    async fn test_record_turn_rpc() {
        let store = setup_test_store().await;
        let rpc = ConversationHistoryRpc::new(store);

        let params = serde_json::json!({
            "session_key": "test-session",
            "turn_number": 1,
            "user_message": "Hello",
            "assistant_response": "Hi there!",
            "model_id": "gpt-4",
            "provider_name": "openai",
            "duration_ms": 1500
        });

        let result = rpc.record_turn(params).await.unwrap();
        assert!(result["success"].as_bool().unwrap());
        assert!(result["turn_id"].as_i64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_get_history_rpc() {
        let store = setup_test_store().await;
        let rpc = ConversationHistoryRpc::new(store);

        // Record a turn first
        let record_params = serde_json::json!({
            "session_key": "test-session",
            "turn_number": 1,
            "user_message": "Test",
            "assistant_response": "Response",
            "model_id": "gpt-4",
            "provider_name": "openai"
        });
        rpc.record_turn(record_params).await.unwrap();

        // Get history
        let get_params = serde_json::json!({
            "session_key": "test-session",
            "limit": 10
        });

        let result = rpc.get_history(get_params).await.unwrap();
        let history: Vec<ConversationTurn> = serde_json::from_value(result).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].user_message, "Test");
    }

    #[tokio::test]
    async fn test_record_and_get_issues_rpc() {
        let store = setup_test_store().await;
        let rpc = ConversationHistoryRpc::new(store);

        // Record a turn with error
        let record_params = serde_json::json!({
            "session_key": "test-session",
            "turn_number": 1,
            "user_message": "Fix bug",
            "assistant_response": "Failed",
            "model_id": "gpt-4",
            "provider_name": "openai",
            "had_error": true,
            "error_message": "Tool execution failed"
        });
        let turn_result = rpc.record_turn(record_params).await.unwrap();
        let turn_id = turn_result["turn_id"].as_i64().unwrap();

        // Record issue
        let issue_params = serde_json::json!({
            "conversation_turn_id": turn_id,
            "session_key": "test-session",
            "issue_summary": "Bug fix failed",
            "user_query": "Fix bug",
            "failed_response": "Failed",
            "priority": "high"
        });

        let issue_result = rpc.record_issue(issue_params).await.unwrap();
        assert!(issue_result["success"].as_bool().unwrap());

        // Get issues
        let get_issues_params = serde_json::json!({
            "status": "open",
            "priority": "high"
        });

        let issues_result = rpc.get_issues(get_issues_params).await.unwrap();
        let issues: Vec<UnresolvedIssue> = serde_json::from_value(issues_result).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].issue_summary, "Bug fix failed");
    }
}
