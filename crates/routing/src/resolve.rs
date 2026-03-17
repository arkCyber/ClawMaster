use {crate::Result, clawmaster_common::types::MsgContext};

/// Resolved route: which agent handles this message and the session key.
#[derive(Debug, Clone)]
pub struct ResolvedRoute {
    pub agent_id: String,
    pub session_key: String,
}

/// Resolve which agent should handle a message, following the binding cascade.
/// Priority: peer → guild → team → account → channel → default
pub fn resolve_agent_route(msg: &MsgContext, config: &serde_json::Value) -> Result<ResolvedRoute> {
    // Try to get bindings from config
    let bindings = config
        .get("bindings")
        .and_then(|v: &serde_json::Value| v.as_object())
        .ok_or_else(|| crate::Error::NotConfigured)?;

    // Binding cascade: peer → guild → team → account → channel → default
    let agent_id = if let Some(_peer_id) = &msg.group_id {
        // 1. Check peer-level binding (using group_id as peer identifier)
        bindings
            .get("peer")
            .and_then(|v: &serde_json::Value| v.as_object())
            .and_then(|v| v.get(&msg.group_id.clone().unwrap_or_default()))
            .and_then(|v: &serde_json::Value| v.as_str())
            .or_else(|| {
                // 2. Check guild-level binding
                msg.guild_id.as_ref().and_then(|guild_id| {
                    bindings
                        .get("guild")
                        .and_then(|v: &serde_json::Value| v.as_object())
                        .and_then(|v| v.get(guild_id))
                        .and_then(|v: &serde_json::Value| v.as_str())
                })
            })
            .or_else(|| {
                // 3. Check team-level binding
                msg.team_id.as_ref().and_then(|team_id| {
                    bindings
                        .get("team")
                        .and_then(|v: &serde_json::Value| v.as_object())
                        .and_then(|v| v.get(team_id))
                        .and_then(|v: &serde_json::Value| v.as_str())
                })
            })
            .or_else(|| {
                // 4. Check account-level binding
                bindings
                    .get("account")
                    .and_then(|v: &serde_json::Value| v.as_object())
                    .and_then(|v| v.get(&msg.account_id))
                    .and_then(|v: &serde_json::Value| v.as_str())
            })
    } else {
        None
    }
    .or_else(|| {
        // 5. Check channel-level binding
        bindings
            .get("channel")
            .and_then(|v: &serde_json::Value| v.as_object())
            .and_then(|v| v.get(&msg.channel))
            .and_then(|v: &serde_json::Value| v.as_str())
    })
    .or_else(|| {
        // 6. Fall back to default agent
        bindings
            .get("default")
            .and_then(|v: &serde_json::Value| v.as_str())
    })
    .ok_or_else(|| crate::Error::NotConfigured)?;

    // Generate session key based on channel and optional peer
    let session_key = if let Some(peer_id) = &msg.group_id {
        format!("{}:{}", msg.channel, peer_id)
    } else {
        msg.channel.clone()
    };

    Ok(ResolvedRoute {
        agent_id: agent_id.to_string(),
        session_key,
    })
}
