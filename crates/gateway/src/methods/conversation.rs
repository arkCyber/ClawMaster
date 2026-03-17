//! RPC method handlers for conversation history management.

use {
    super::{MethodContext, MethodRegistry},
    crate::conversation_history_rpc::ConversationHistoryRpc,
    clawmaster_protocol::ErrorShape,
    std::sync::Arc,
};

/// Register all conversation history RPC methods.
pub fn register(registry: &mut MethodRegistry, rpc: Arc<ConversationHistoryRpc>) {
    // conversation.recordTurn
    {
        let rpc = Arc::clone(&rpc);
        registry.register(
            "conversation.recordTurn",
            Box::new(move |ctx: MethodContext| {
                let rpc = Arc::clone(&rpc);
                Box::pin(async move {
                    rpc.record_turn(ctx.params)
                        .await
                        .map_err(|e| ErrorShape::new("INTERNAL_ERROR", e.to_string()))
                })
            }),
        );
    }

    // conversation.getHistory
    {
        let rpc = Arc::clone(&rpc);
        registry.register(
            "conversation.getHistory",
            Box::new(move |ctx: MethodContext| {
                let rpc = Arc::clone(&rpc);
                Box::pin(async move {
                    rpc.get_history(ctx.params)
                        .await
                        .map_err(|e| ErrorShape::new("INTERNAL_ERROR", e.to_string()))
                })
            }),
        );
    }

    // conversation.updateResolution
    {
        let rpc = Arc::clone(&rpc);
        registry.register(
            "conversation.updateResolution",
            Box::new(move |ctx: MethodContext| {
                let rpc = Arc::clone(&rpc);
                Box::pin(async move {
                    rpc.update_resolution(ctx.params)
                        .await
                        .map_err(|e| ErrorShape::new("INTERNAL_ERROR", e.to_string()))
                })
            }),
        );
    }

    // conversation.addFeedback
    {
        let rpc = Arc::clone(&rpc);
        registry.register(
            "conversation.addFeedback",
            Box::new(move |ctx: MethodContext| {
                let rpc = Arc::clone(&rpc);
                Box::pin(async move {
                    rpc.add_feedback(ctx.params)
                        .await
                        .map_err(|e| ErrorShape::new("INTERNAL_ERROR", e.to_string()))
                })
            }),
        );
    }

    // conversation.recordIssue
    {
        let rpc = Arc::clone(&rpc);
        registry.register(
            "conversation.recordIssue",
            Box::new(move |ctx: MethodContext| {
                let rpc = Arc::clone(&rpc);
                Box::pin(async move {
                    rpc.record_issue(ctx.params)
                        .await
                        .map_err(|e| ErrorShape::new("INTERNAL_ERROR", e.to_string()))
                })
            }),
        );
    }

    // conversation.getIssues
    {
        let rpc = Arc::clone(&rpc);
        registry.register(
            "conversation.getIssues",
            Box::new(move |ctx: MethodContext| {
                let rpc = Arc::clone(&rpc);
                Box::pin(async move {
                    rpc.get_issues(ctx.params)
                        .await
                        .map_err(|e| ErrorShape::new("INTERNAL_ERROR", e.to_string()))
                })
            }),
        );
    }

    // conversation.updateIssue
    {
        let rpc = Arc::clone(&rpc);
        registry.register(
            "conversation.updateIssue",
            Box::new(move |ctx: MethodContext| {
                let rpc = Arc::clone(&rpc);
                Box::pin(async move {
                    rpc.update_issue(ctx.params)
                        .await
                        .map_err(|e| ErrorShape::new("INTERNAL_ERROR", e.to_string()))
                })
            }),
        );
    }

    // conversation.searchHistory
    {
        let rpc = Arc::clone(&rpc);
        registry.register(
            "conversation.searchHistory",
            Box::new(move |ctx: MethodContext| {
                let rpc = Arc::clone(&rpc);
                Box::pin(async move {
                    rpc.search_history(ctx.params)
                        .await
                        .map_err(|e| ErrorShape::new("INTERNAL_ERROR", e.to_string()))
                })
            }),
        );
    }

    // conversation.getMetadata
    {
        let rpc = Arc::clone(&rpc);
        registry.register(
            "conversation.getMetadata",
            Box::new(move |ctx: MethodContext| {
                let rpc = Arc::clone(&rpc);
                Box::pin(async move {
                    rpc.get_metadata(ctx.params)
                        .await
                        .map_err(|e| ErrorShape::new("INTERNAL_ERROR", e.to_string()))
                })
            }),
        );
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            auth::{AuthMode, ResolvedAuth},
            conversation_history::SqliteConversationHistory,
            services::GatewayServices,
            state::GatewayState,
        },
        std::sync::Arc,
    };

    #[tokio::test]
    async fn test_conversation_methods_registered() {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(include_str!(
            "../../migrations/20260316000001_conversation_history.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();

        let store = Arc::new(SqliteConversationHistory::new(pool));
        let rpc = Arc::new(ConversationHistoryRpc::new(store));

        let mut registry = MethodRegistry::new();
        register(&mut registry, rpc);

        let methods = registry.method_names();
        assert!(methods.contains(&"conversation.recordTurn".to_string()));
        assert!(methods.contains(&"conversation.getHistory".to_string()));
        assert!(methods.contains(&"conversation.updateResolution".to_string()));
        assert!(methods.contains(&"conversation.addFeedback".to_string()));
        assert!(methods.contains(&"conversation.recordIssue".to_string()));
        assert!(methods.contains(&"conversation.getIssues".to_string()));
        assert!(methods.contains(&"conversation.updateIssue".to_string()));
        assert!(methods.contains(&"conversation.searchHistory".to_string()));
        assert!(methods.contains(&"conversation.getMetadata".to_string()));
    }

    #[tokio::test]
    async fn test_record_turn_method_call() {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(include_str!(
            "../../migrations/20260316000001_conversation_history.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();

        let store = Arc::new(SqliteConversationHistory::new(pool));
        let rpc = Arc::new(ConversationHistoryRpc::new(store));

        let mut registry = MethodRegistry::new();
        register(&mut registry, rpc);

        let ctx = MethodContext {
            request_id: "test-1".to_string(),
            method: "conversation.recordTurn".to_string(),
            params: serde_json::json!({
                "session_key": "test-session",
                "turn_number": 1,
                "user_message": "Hello",
                "assistant_response": "Hi there!",
                "model_id": "gpt-4",
                "provider_name": "openai"
            }),
            client_conn_id: "conn-1".to_string(),
            client_role: "operator".to_string(),
            client_scopes: vec!["operator.write".to_string()],
            state: Arc::new(GatewayState::new(
                ResolvedAuth {
                    mode: AuthMode::Token,
                    token: None,
                    password: None,
                },
                GatewayServices::noop(),
            )),
            channel: None,
        };

        let response = registry.dispatch(ctx).await;
        assert!(response.ok, "Response should be ok: {:?}", response.error);
        assert!(response.payload.is_some());
        let payload = response.payload.unwrap();
        assert!(payload["turn_id"].as_i64().unwrap() > 0);
    }
}
