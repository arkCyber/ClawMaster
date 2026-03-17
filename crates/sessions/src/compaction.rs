/// Summarize old messages when token count approaches model context limit.
/// This function takes old messages and creates a compact summary using an LLM.
pub async fn compact_session(
    messages: &[serde_json::Value],
) -> crate::Result<Vec<serde_json::Value>> {
    if messages.is_empty() {
        return Ok(Vec::new());
    }

    // For now, implement a simple compaction strategy:
    // Keep the first message (system prompt) and last N messages,
    // summarize everything in between.
    const KEEP_RECENT: usize = 10;

    if messages.len() <= KEEP_RECENT + 1 {
        // Not enough messages to compact
        return Ok(messages.to_vec());
    }

    let mut compacted = Vec::new();

    // Keep system message if present
    if let Some(first) = messages.first() {
        if first.get("role").and_then(|v| v.as_str()) == Some("system") {
            compacted.push(first.clone());
        }
    }

    // Messages to summarize (middle section)
    let start_idx = if compacted.is_empty() {
        0
    } else {
        1
    };
    let end_idx = messages.len().saturating_sub(KEEP_RECENT);

    if end_idx > start_idx {
        let to_summarize = &messages[start_idx..end_idx];

        // Create a summary message
        // TODO: In production, call an LLM API to generate the summary
        let summary_text = format!(
            "[Conversation summary: {} messages exchanged covering previous context]",
            to_summarize.len()
        );

        compacted.push(serde_json::json!({
            "role": "system",
            "content": summary_text,
            "_compacted": true,
            "_original_count": to_summarize.len()
        }));
    }

    // Keep recent messages
    let recent_start = messages.len().saturating_sub(KEEP_RECENT);
    compacted.extend_from_slice(&messages[recent_start..]);

    tracing::info!(
        "Compacted session: {} → {} messages",
        messages.len(),
        compacted.len()
    );

    Ok(compacted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compact_session_empty() {
        let messages: Vec<serde_json::Value> = vec![];
        let result = compact_session(&messages).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_compact_session_too_few_messages() {
        let messages = vec![
            serde_json::json!({"role": "system", "content": "You are helpful"}),
            serde_json::json!({"role": "user", "content": "Hello"}),
            serde_json::json!({"role": "assistant", "content": "Hi"}),
        ];
        let result = compact_session(&messages).await;
        assert!(result.is_ok());
        let compacted = result.unwrap();
        assert_eq!(compacted.len(), 3); // No compaction needed
    }

    #[tokio::test]
    async fn test_compact_session_with_system_message() {
        let mut messages =
            vec![serde_json::json!({"role": "system", "content": "You are helpful"})];
        // Add 20 user/assistant pairs
        for i in 0..20 {
            messages.push(serde_json::json!({"role": "user", "content": format!("Q{}", i)}));
            messages.push(serde_json::json!({"role": "assistant", "content": format!("A{}", i)}));
        }

        let result = compact_session(&messages).await;
        assert!(result.is_ok());
        let compacted = result.unwrap();

        // Should have: system + summary + last 10 messages
        assert_eq!(compacted.len(), 12);

        // First should be system message
        assert_eq!(compacted[0]["role"], "system");
        assert_eq!(compacted[0]["content"], "You are helpful");

        // Second should be summary
        assert_eq!(compacted[1]["role"], "system");
        assert!(compacted[1]["_compacted"].as_bool().unwrap());
        assert_eq!(compacted[1]["_original_count"], 30); // 20 pairs = 40 messages - 10 kept = 30
    }

    #[tokio::test]
    async fn test_compact_session_without_system_message() {
        let mut messages = vec![];
        // Add 20 user/assistant pairs without system message
        for i in 0..20 {
            messages.push(serde_json::json!({"role": "user", "content": format!("Q{}", i)}));
            messages.push(serde_json::json!({"role": "assistant", "content": format!("A{}", i)}));
        }

        let result = compact_session(&messages).await;
        assert!(result.is_ok());
        let compacted = result.unwrap();

        // Should have: summary + last 10 messages
        assert_eq!(compacted.len(), 11);

        // First should be summary
        assert_eq!(compacted[0]["role"], "system");
        assert!(compacted[0]["_compacted"].as_bool().unwrap());
    }

    #[tokio::test]
    async fn test_compact_session_preserves_recent_messages() {
        let mut messages = vec![serde_json::json!({"role": "system", "content": "System"})];
        for i in 0..15 {
            messages.push(serde_json::json!({"role": "user", "content": format!("Q{}", i)}));
        }

        let result = compact_session(&messages).await;
        assert!(result.is_ok());
        let compacted = result.unwrap();

        // Last 10 messages should be Q5-Q14
        let last_messages: Vec<String> = compacted
            .iter()
            .skip(2) // Skip system and summary
            .filter_map(|m| m["content"].as_str().map(String::from))
            .collect();

        assert_eq!(last_messages.len(), 10);
        assert_eq!(last_messages[0], "Q5");
        assert_eq!(last_messages[9], "Q14");
    }
}
