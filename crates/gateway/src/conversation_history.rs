//! Conversation history tracking with timestamps and resolution status.
//!
//! This module provides comprehensive tracking of all Q&A interactions,
//! including unresolved issues and their handling.

use {
    anyhow::Result,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
    sqlx::SqlitePool,
    std::time::{SystemTime, UNIX_EPOCH},
    tracing::{debug, error, info, warn},
};

/// A single conversation turn (question + answer pair).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub id: i64,
    pub session_key: String,
    pub turn_number: i64,
    pub user_message: String,
    pub assistant_response: String,
    pub created_at: i64,
    pub response_at: i64,
    pub duration_ms: i64,

    // Resolution tracking
    pub is_resolved: bool,
    pub resolution_status: Option<String>,
    pub user_feedback: Option<String>,
    pub feedback_at: Option<i64>,

    // Context
    pub model_id: String,
    pub provider_name: String,
    pub tool_calls_count: i64,
    pub tool_calls_json: Option<String>,

    // Token usage
    pub input_tokens: i64,
    pub output_tokens: i64,

    // Error tracking
    pub had_error: bool,
    pub error_message: Option<String>,
}

/// An unresolved issue that needs attention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnresolvedIssue {
    pub id: i64,
    pub conversation_turn_id: i64,
    pub session_key: String,
    pub issue_summary: String,
    pub user_query: String,
    pub failed_response: Option<String>,
    pub created_at: i64,
    pub last_attempt_at: Option<i64>,
    pub resolved_at: Option<i64>,
    pub attempt_count: i64,
    pub priority: String,
    pub status: String,
    pub resolution_method: Option<String>,
    pub resolution_notes: Option<String>,
    pub resolved_by: Option<String>,
}

/// Conversation metadata and statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMetadata {
    pub session_key: String,
    pub total_turns: i64,
    pub resolved_count: i64,
    pub unresolved_count: i64,
    pub first_message_at: i64,
    pub last_message_at: i64,
    pub avg_response_time_ms: i64,
    pub total_tokens: i64,
    pub error_count: i64,
    pub positive_feedback: i64,
    pub negative_feedback: i64,
}

/// Priority level for unresolved issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssuePriority {
    Low,
    Normal,
    High,
    Critical,
}

impl IssuePriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

/// Status of an unresolved issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueStatus {
    Open,
    InProgress,
    Resolved,
    Abandoned,
}

impl IssueStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::InProgress => "in_progress",
            Self::Resolved => "resolved",
            Self::Abandoned => "abandoned",
        }
    }
}

/// Conversation history store trait.
#[async_trait]
pub trait ConversationHistoryStore: Send + Sync {
    /// Record a new conversation turn.
    async fn record_turn(&self, turn: &ConversationTurn) -> Result<i64>;

    /// Get conversation history for a session.
    async fn get_session_history(
        &self,
        session_key: &str,
        limit: Option<i64>,
    ) -> Result<Vec<ConversationTurn>>;

    /// Update resolution status of a turn.
    async fn update_resolution(
        &self,
        turn_id: i64,
        is_resolved: bool,
        status: Option<&str>,
    ) -> Result<()>;

    /// Add user feedback to a turn.
    async fn add_feedback(&self, turn_id: i64, feedback: &str, is_positive: bool) -> Result<()>;

    /// Record an unresolved issue.
    async fn record_unresolved_issue(&self, issue: &UnresolvedIssue) -> Result<i64>;

    /// Get all unresolved issues.
    async fn get_unresolved_issues(
        &self,
        status: Option<IssueStatus>,
        priority: Option<IssuePriority>,
        limit: Option<i64>,
    ) -> Result<Vec<UnresolvedIssue>>;

    /// Update unresolved issue status.
    async fn update_issue_status(
        &self,
        issue_id: i64,
        status: IssueStatus,
        resolution_method: Option<&str>,
        notes: Option<&str>,
    ) -> Result<()>;

    /// Get conversation metadata.
    async fn get_metadata(&self, session_key: &str) -> Result<Option<ConversationMetadata>>;

    /// Search conversation history.
    async fn search_history(
        &self,
        query: &str,
        session_key: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<ConversationTurn>>;
}

/// SQLite implementation of conversation history store.
pub struct SqliteConversationHistory {
    pool: SqlitePool,
}

impl SqliteConversationHistory {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }
}

#[async_trait]
impl ConversationHistoryStore for SqliteConversationHistory {
    async fn record_turn(&self, turn: &ConversationTurn) -> Result<i64> {
        let turn_id = sqlx::query(
            "INSERT INTO conversation_turns (
                session_key, turn_number, user_message, assistant_response,
                created_at, response_at, duration_ms, is_resolved, resolution_status,
                model_id, provider_name, tool_calls_count, tool_calls_json,
                input_tokens, output_tokens, had_error, error_message
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&turn.session_key)
        .bind(turn.turn_number)
        .bind(&turn.user_message)
        .bind(&turn.assistant_response)
        .bind(turn.created_at)
        .bind(turn.response_at)
        .bind(turn.duration_ms)
        .bind(turn.is_resolved as i64)
        .bind(&turn.resolution_status)
        .bind(&turn.model_id)
        .bind(&turn.provider_name)
        .bind(turn.tool_calls_count)
        .bind(&turn.tool_calls_json)
        .bind(turn.input_tokens)
        .bind(turn.output_tokens)
        .bind(turn.had_error as i64)
        .bind(&turn.error_message)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        // Update metadata
        self.update_metadata(&turn.session_key, turn).await?;

        debug!(
            session_key = %turn.session_key,
            turn_number = turn.turn_number,
            turn_id = turn_id,
            "recorded conversation turn"
        );

        Ok(turn_id)
    }

    async fn get_session_history(
        &self,
        session_key: &str,
        limit: Option<i64>,
    ) -> Result<Vec<ConversationTurn>> {
        let limit = limit.unwrap_or(100);

        #[derive(sqlx::FromRow)]
        struct TurnRow {
            id: i64,
            session_key: String,
            turn_number: i64,
            user_message: String,
            assistant_response: String,
            created_at: i64,
            response_at: i64,
            duration_ms: i64,
            is_resolved: i64,
            resolution_status: Option<String>,
            user_feedback: Option<String>,
            feedback_at: Option<i64>,
            model_id: String,
            provider_name: String,
            tool_calls_count: i64,
            tool_calls_json: Option<String>,
            input_tokens: i64,
            output_tokens: i64,
            had_error: i64,
            error_message: Option<String>,
        }

        let rows = sqlx::query_as::<_, TurnRow>(
            "SELECT id, session_key, turn_number, user_message, assistant_response,
                    created_at, response_at, duration_ms, is_resolved, resolution_status,
                    user_feedback, feedback_at, model_id, provider_name, tool_calls_count,
                    tool_calls_json, input_tokens, output_tokens, had_error, error_message
             FROM conversation_turns
             WHERE session_key = ?
             ORDER BY turn_number DESC
             LIMIT ?",
        )
        .bind(session_key)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| ConversationTurn {
                id: row.id,
                session_key: row.session_key,
                turn_number: row.turn_number,
                user_message: row.user_message,
                assistant_response: row.assistant_response,
                created_at: row.created_at,
                response_at: row.response_at,
                duration_ms: row.duration_ms,
                is_resolved: row.is_resolved != 0,
                resolution_status: row.resolution_status,
                user_feedback: row.user_feedback,
                feedback_at: row.feedback_at,
                model_id: row.model_id,
                provider_name: row.provider_name,
                tool_calls_count: row.tool_calls_count,
                tool_calls_json: row.tool_calls_json,
                input_tokens: row.input_tokens,
                output_tokens: row.output_tokens,
                had_error: row.had_error != 0,
                error_message: row.error_message,
            })
            .collect())
    }

    async fn update_resolution(
        &self,
        turn_id: i64,
        is_resolved: bool,
        status: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE conversation_turns
             SET is_resolved = ?, resolution_status = ?
             WHERE id = ?",
        )
        .bind(is_resolved as i64)
        .bind(status)
        .bind(turn_id)
        .execute(&self.pool)
        .await?;

        debug!(turn_id, is_resolved, "updated turn resolution status");
        Ok(())
    }

    async fn add_feedback(&self, turn_id: i64, feedback: &str, is_positive: bool) -> Result<()> {
        let now = Self::current_timestamp();

        sqlx::query(
            "UPDATE conversation_turns
             SET user_feedback = ?, feedback_at = ?
             WHERE id = ?",
        )
        .bind(feedback)
        .bind(now)
        .bind(turn_id)
        .execute(&self.pool)
        .await?;

        // Update metadata feedback counters
        if let Some((session_key,)) = sqlx::query_as::<_, (String,)>(
            "SELECT session_key FROM conversation_turns WHERE id = ?",
        )
        .bind(turn_id)
        .fetch_optional(&self.pool)
        .await?
        {
            let field = if is_positive {
                "positive_feedback"
            } else {
                "negative_feedback"
            };
            sqlx::query(&format!(
                "UPDATE conversation_metadata SET {} = {} + 1 WHERE session_key = ?",
                field, field
            ))
            .bind(&session_key)
            .execute(&self.pool)
            .await?;
        }

        info!(turn_id, is_positive, "added user feedback");
        Ok(())
    }

    async fn record_unresolved_issue(&self, issue: &UnresolvedIssue) -> Result<i64> {
        let issue_id = sqlx::query(
            "INSERT INTO unresolved_issues (
                conversation_turn_id, session_key, issue_summary, user_query,
                failed_response, created_at, attempt_count, priority, status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(issue.conversation_turn_id)
        .bind(&issue.session_key)
        .bind(&issue.issue_summary)
        .bind(&issue.user_query)
        .bind(&issue.failed_response)
        .bind(issue.created_at)
        .bind(issue.attempt_count)
        .bind(&issue.priority)
        .bind(&issue.status)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        warn!(
            issue_id,
            session_key = %issue.session_key,
            priority = %issue.priority,
            "recorded unresolved issue"
        );

        Ok(issue_id)
    }

    async fn get_unresolved_issues(
        &self,
        status: Option<IssueStatus>,
        priority: Option<IssuePriority>,
        limit: Option<i64>,
    ) -> Result<Vec<UnresolvedIssue>> {
        let limit = limit.unwrap_or(50);

        let mut query = String::from(
            "SELECT id, conversation_turn_id, session_key, issue_summary, user_query,
                    failed_response, created_at, last_attempt_at, resolved_at,
                    attempt_count, priority, status, resolution_method, resolution_notes,
                    resolved_by
             FROM unresolved_issues WHERE 1=1",
        );

        if let Some(s) = status {
            query.push_str(&format!(" AND status = '{}'", s.as_str()));
        }

        if let Some(p) = priority {
            query.push_str(&format!(" AND priority = '{}'", p.as_str()));
        }

        query.push_str(" ORDER BY priority DESC, created_at DESC LIMIT ?");

        let rows = sqlx::query_as::<
            _,
            (
                i64,
                i64,
                String,
                String,
                String,
                Option<String>,
                i64,
                Option<i64>,
                Option<i64>,
                i64,
                String,
                String,
                Option<String>,
                Option<String>,
                Option<String>,
            ),
        >(&query)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| UnresolvedIssue {
                id: row.0,
                conversation_turn_id: row.1,
                session_key: row.2,
                issue_summary: row.3,
                user_query: row.4,
                failed_response: row.5,
                created_at: row.6,
                last_attempt_at: row.7,
                resolved_at: row.8,
                attempt_count: row.9,
                priority: row.10,
                status: row.11,
                resolution_method: row.12,
                resolution_notes: row.13,
                resolved_by: row.14,
            })
            .collect())
    }

    async fn update_issue_status(
        &self,
        issue_id: i64,
        status: IssueStatus,
        resolution_method: Option<&str>,
        notes: Option<&str>,
    ) -> Result<()> {
        let now = Self::current_timestamp();
        let resolved_at = if status == IssueStatus::Resolved {
            Some(now)
        } else {
            None
        };

        sqlx::query(
            "UPDATE unresolved_issues
             SET status = ?, last_attempt_at = ?, resolved_at = ?,
                 resolution_method = ?, resolution_notes = ?, attempt_count = attempt_count + 1
             WHERE id = ?",
        )
        .bind(status.as_str())
        .bind(now)
        .bind(resolved_at)
        .bind(resolution_method)
        .bind(notes)
        .bind(issue_id)
        .execute(&self.pool)
        .await?;

        info!(issue_id, status = %status.as_str(), "updated issue status");
        Ok(())
    }

    async fn get_metadata(&self, session_key: &str) -> Result<Option<ConversationMetadata>> {
        let row = sqlx::query_as::<_, (String, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>(
            "SELECT session_key, total_turns, resolved_count, unresolved_count,
                    first_message_at, last_message_at, avg_response_time_ms,
                    total_tokens, error_count, positive_feedback, negative_feedback
             FROM conversation_metadata
             WHERE session_key = ?",
        )
        .bind(session_key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| ConversationMetadata {
            session_key: r.0,
            total_turns: r.1,
            resolved_count: r.2,
            unresolved_count: r.3,
            first_message_at: r.4,
            last_message_at: r.5,
            avg_response_time_ms: r.6,
            total_tokens: r.7,
            error_count: r.8,
            positive_feedback: r.9,
            negative_feedback: r.10,
        }))
    }

    async fn search_history(
        &self,
        query: &str,
        session_key: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<ConversationTurn>> {
        let limit = limit.unwrap_or(50);
        let search_pattern = format!("%{}%", query);

        #[derive(sqlx::FromRow)]
        struct TurnRow {
            id: i64,
            session_key: String,
            turn_number: i64,
            user_message: String,
            assistant_response: String,
            created_at: i64,
            response_at: i64,
            duration_ms: i64,
            is_resolved: i64,
            resolution_status: Option<String>,
            user_feedback: Option<String>,
            feedback_at: Option<i64>,
            model_id: String,
            provider_name: String,
            tool_calls_count: i64,
            tool_calls_json: Option<String>,
            input_tokens: i64,
            output_tokens: i64,
            had_error: i64,
            error_message: Option<String>,
        }

        let mut sql = String::from(
            "SELECT id, session_key, turn_number, user_message, assistant_response,
                    created_at, response_at, duration_ms, is_resolved, resolution_status,
                    user_feedback, feedback_at, model_id, provider_name, tool_calls_count,
                    tool_calls_json, input_tokens, output_tokens, had_error, error_message
             FROM conversation_turns
             WHERE (user_message LIKE ? OR assistant_response LIKE ?)",
        );

        if let Some(sk) = session_key {
            sql.push_str(&format!(" AND session_key = '{}'", sk));
        }

        sql.push_str(" ORDER BY created_at DESC LIMIT ?");

        let rows = sqlx::query_as::<_, TurnRow>(&sql)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| ConversationTurn {
                id: row.id,
                session_key: row.session_key,
                turn_number: row.turn_number,
                user_message: row.user_message,
                assistant_response: row.assistant_response,
                created_at: row.created_at,
                response_at: row.response_at,
                duration_ms: row.duration_ms,
                is_resolved: row.is_resolved != 0,
                resolution_status: row.resolution_status,
                user_feedback: row.user_feedback,
                feedback_at: row.feedback_at,
                model_id: row.model_id,
                provider_name: row.provider_name,
                tool_calls_count: row.tool_calls_count,
                tool_calls_json: row.tool_calls_json,
                input_tokens: row.input_tokens,
                output_tokens: row.output_tokens,
                had_error: row.had_error != 0,
                error_message: row.error_message,
            })
            .collect())
    }
}

impl SqliteConversationHistory {
    /// Update conversation metadata after recording a turn.
    async fn update_metadata(&self, session_key: &str, turn: &ConversationTurn) -> Result<()> {
        // Check if metadata exists
        let exists = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM conversation_metadata WHERE session_key = ?",
        )
        .bind(session_key)
        .fetch_one(&self.pool)
        .await?
            > 0;

        if !exists {
            // Create new metadata
            sqlx::query(
                "INSERT INTO conversation_metadata (
                    session_key, total_turns, resolved_count, unresolved_count,
                    first_message_at, last_message_at, avg_response_time_ms,
                    total_tokens, error_count
                ) VALUES (?, 1, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(session_key)
            .bind(if turn.is_resolved {
                1
            } else {
                0
            })
            .bind(if turn.is_resolved {
                0
            } else {
                1
            })
            .bind(turn.created_at)
            .bind(turn.response_at)
            .bind(turn.duration_ms)
            .bind(turn.input_tokens + turn.output_tokens)
            .bind(if turn.had_error {
                1
            } else {
                0
            })
            .execute(&self.pool)
            .await?;
        } else {
            // Update existing metadata
            sqlx::query(
                "UPDATE conversation_metadata
                 SET total_turns = total_turns + 1,
                     resolved_count = resolved_count + ?,
                     unresolved_count = unresolved_count + ?,
                     last_message_at = ?,
                     avg_response_time_ms = (avg_response_time_ms * total_turns + ?) / (total_turns + 1),
                     total_tokens = total_tokens + ?,
                     error_count = error_count + ?
                 WHERE session_key = ?"
            )
            .bind(if turn.is_resolved { 1 } else { 0 })
            .bind(if turn.is_resolved { 0 } else { 1 })
            .bind(turn.response_at)
            .bind(turn.duration_ms)
            .bind(turn.input_tokens + turn.output_tokens)
            .bind(if turn.had_error { 1 } else { 0 })
            .bind(session_key)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        // Run migrations
        sqlx::query(include_str!(
            "../migrations/20260316000001_conversation_history.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_record_and_retrieve_turn() {
        let pool = setup_test_db().await;
        let store = SqliteConversationHistory::new(pool);

        let turn = ConversationTurn {
            id: 0,
            session_key: "test-session".to_string(),
            turn_number: 1,
            user_message: "What is Rust?".to_string(),
            assistant_response: "Rust is a systems programming language.".to_string(),
            created_at: 1700000000,
            response_at: 1700000002,
            duration_ms: 2000,
            is_resolved: true,
            resolution_status: Some("resolved".to_string()),
            user_feedback: None,
            feedback_at: None,
            model_id: "gpt-4".to_string(),
            provider_name: "openai".to_string(),
            tool_calls_count: 0,
            tool_calls_json: None,
            input_tokens: 10,
            output_tokens: 20,
            had_error: false,
            error_message: None,
        };

        let turn_id = store.record_turn(&turn).await.unwrap();
        assert!(turn_id > 0);

        let history = store
            .get_session_history("test-session", None)
            .await
            .unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].user_message, "What is Rust?");
    }

    #[tokio::test]
    async fn test_unresolved_issue_tracking() {
        let pool = setup_test_db().await;
        let store = SqliteConversationHistory::new(pool);

        // Record a turn first
        let turn = ConversationTurn {
            id: 0,
            session_key: "test-session".to_string(),
            turn_number: 1,
            user_message: "Fix my bug".to_string(),
            assistant_response: "I couldn't fix it.".to_string(),
            created_at: 1700000000,
            response_at: 1700000002,
            duration_ms: 2000,
            is_resolved: false,
            resolution_status: Some("unresolved".to_string()),
            user_feedback: None,
            feedback_at: None,
            model_id: "gpt-4".to_string(),
            provider_name: "openai".to_string(),
            tool_calls_count: 0,
            tool_calls_json: None,
            input_tokens: 10,
            output_tokens: 20,
            had_error: false,
            error_message: None,
        };

        let turn_id = store.record_turn(&turn).await.unwrap();

        // Record unresolved issue
        let issue = UnresolvedIssue {
            id: 0,
            conversation_turn_id: turn_id,
            session_key: "test-session".to_string(),
            issue_summary: "Bug fix failed".to_string(),
            user_query: "Fix my bug".to_string(),
            failed_response: Some("I couldn't fix it.".to_string()),
            created_at: 1700000002,
            last_attempt_at: None,
            resolved_at: None,
            attempt_count: 1,
            priority: "high".to_string(),
            status: "open".to_string(),
            resolution_method: None,
            resolution_notes: None,
            resolved_by: None,
        };

        let issue_id = store.record_unresolved_issue(&issue).await.unwrap();
        assert!(issue_id > 0);

        let issues = store
            .get_unresolved_issues(Some(IssueStatus::Open), None, None)
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].issue_summary, "Bug fix failed");
    }

    #[tokio::test]
    async fn test_feedback_tracking() {
        let pool = setup_test_db().await;
        let store = SqliteConversationHistory::new(pool);

        let turn = ConversationTurn {
            id: 0,
            session_key: "test-session".to_string(),
            turn_number: 1,
            user_message: "Test".to_string(),
            assistant_response: "Response".to_string(),
            created_at: 1700000000,
            response_at: 1700000002,
            duration_ms: 2000,
            is_resolved: true,
            resolution_status: Some("resolved".to_string()),
            user_feedback: None,
            feedback_at: None,
            model_id: "gpt-4".to_string(),
            provider_name: "openai".to_string(),
            tool_calls_count: 0,
            tool_calls_json: None,
            input_tokens: 10,
            output_tokens: 20,
            had_error: false,
            error_message: None,
        };

        let turn_id = store.record_turn(&turn).await.unwrap();

        store
            .add_feedback(turn_id, "Great answer!", true)
            .await
            .unwrap();

        let history = store
            .get_session_history("test-session", None)
            .await
            .unwrap();
        assert_eq!(history[0].user_feedback, Some("Great answer!".to_string()));

        let metadata = store.get_metadata("test-session").await.unwrap().unwrap();
        assert_eq!(metadata.positive_feedback, 1);
    }

    #[tokio::test]
    async fn test_search_history() {
        let pool = setup_test_db().await;
        let store = SqliteConversationHistory::new(pool);

        let turn = ConversationTurn {
            id: 0,
            session_key: "test-session".to_string(),
            turn_number: 1,
            user_message: "What is Rust programming?".to_string(),
            assistant_response: "Rust is great!".to_string(),
            created_at: 1700000000,
            response_at: 1700000002,
            duration_ms: 2000,
            is_resolved: true,
            resolution_status: Some("resolved".to_string()),
            user_feedback: None,
            feedback_at: None,
            model_id: "gpt-4".to_string(),
            provider_name: "openai".to_string(),
            tool_calls_count: 0,
            tool_calls_json: None,
            input_tokens: 10,
            output_tokens: 20,
            had_error: false,
            error_message: None,
        };

        store.record_turn(&turn).await.unwrap();

        let results = store.search_history("Rust", None, None).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].user_message.contains("Rust"));
    }
}
