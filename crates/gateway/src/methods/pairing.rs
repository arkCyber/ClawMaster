use clawmaster_protocol::{ErrorShape, error_codes};

use crate::broadcast::{BroadcastOpts, broadcast};

use super::MethodRegistry;

/// Verify Ed25519 signature
fn verify_ed25519_signature(signature: &str, data: &str, public_key: &str) -> anyhow::Result<bool> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let sig_bytes =
        hex::decode(signature).map_err(|e| anyhow::anyhow!("Invalid signature hex: {}", e))?;
    let signature = Signature::from_bytes(
        &sig_bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("Signature must be 64 bytes"))?,
    );

    let pk_bytes =
        hex::decode(public_key).map_err(|e| anyhow::anyhow!("Invalid public key hex: {}", e))?;
    let verifying_key = VerifyingKey::from_bytes(
        &pk_bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("Public key must be 32 bytes"))?,
    )
    .map_err(|e| anyhow::anyhow!("Invalid public key: {}", e))?;

    match verifying_key.verify(data.as_bytes(), &signature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Helper to get the pairing store, falling back to in-memory state.
fn get_pairing_store(
    state: &crate::state::GatewayState,
) -> Option<&std::sync::Arc<crate::pairing::PairingStore>> {
    state.pairing_store.as_ref()
}

pub(super) fn register(reg: &mut MethodRegistry) {
    // node.pair.request
    reg.register(
        "node.pair.request",
        Box::new(|ctx| {
            Box::pin(async move {
                let device_id = ctx
                    .params
                    .get("deviceId")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        ErrorShape::new(error_codes::INVALID_REQUEST, "missing deviceId")
                    })?;
                let display_name = ctx.params.get("displayName").and_then(|v| v.as_str());
                let platform = ctx
                    .params
                    .get("platform")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let public_key = ctx.params.get("publicKey").and_then(|v| v.as_str());

                let (id, nonce, device_id_out, display_name_out, platform_out) =
                    if let Some(store) = get_pairing_store(&ctx.state) {
                        let req = store
                            .request_pair(device_id, display_name, platform, public_key)
                            .await
                            .map_err(|e| ErrorShape::new(error_codes::INTERNAL, e.to_string()))?;
                        (
                            req.id,
                            req.nonce,
                            req.device_id,
                            req.display_name,
                            req.platform,
                        )
                    } else {
                        let req = ctx.state.inner.write().await.pairing.request_pair(
                            device_id,
                            display_name,
                            platform,
                            public_key,
                        );
                        (
                            req.id,
                            req.nonce,
                            req.device_id,
                            req.display_name,
                            req.platform,
                        )
                    };

                broadcast(
                    &ctx.state,
                    "node.pair.requested",
                    serde_json::json!({
                        "id": id,
                        "deviceId": device_id_out,
                        "displayName": display_name_out,
                        "platform": platform_out,
                    }),
                    BroadcastOpts::default(),
                )
                .await;

                Ok(serde_json::json!({
                    "id": id,
                    "nonce": nonce,
                }))
            })
        }),
    );

    // node.pair.list
    reg.register(
        "node.pair.list",
        Box::new(|ctx| {
            Box::pin(async move {
                if let Some(store) = get_pairing_store(&ctx.state) {
                    let pending = store
                        .list_pending()
                        .await
                        .map_err(|e| ErrorShape::new(error_codes::INTERNAL, e.to_string()))?;
                    let list: Vec<_> = pending
                        .iter()
                        .map(|r| {
                            serde_json::json!({
                                "id": r.id,
                                "deviceId": r.device_id,
                                "displayName": r.display_name,
                                "platform": r.platform,
                            })
                        })
                        .collect();
                    Ok(serde_json::json!(list))
                } else {
                    let inner = ctx.state.inner.read().await;
                    let list: Vec<_> = inner
                        .pairing
                        .list_pending()
                        .iter()
                        .map(|r| {
                            serde_json::json!({
                                "id": r.id,
                                "deviceId": r.device_id,
                                "displayName": r.display_name,
                                "platform": r.platform,
                            })
                        })
                        .collect();
                    Ok(serde_json::json!(list))
                }
            })
        }),
    );

    // node.pair.approve
    reg.register(
        "node.pair.approve",
        Box::new(|ctx| {
            Box::pin(async move {
                let pair_id = ctx
                    .params
                    .get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ErrorShape::new(error_codes::INVALID_REQUEST, "missing id"))?;

                let (token_str, scopes) = if let Some(store) = get_pairing_store(&ctx.state) {
                    let token = store.approve(pair_id).await.map_err(|e| {
                        ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                    })?;
                    (token.token, token.scopes)
                } else {
                    let token = ctx
                        .state
                        .inner
                        .write()
                        .await
                        .pairing
                        .approve(pair_id)
                        .map_err(|e| {
                            ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                        })?;
                    (token.token, token.scopes)
                };

                broadcast(
                    &ctx.state,
                    "node.pair.resolved",
                    serde_json::json!({
                        "id": pair_id, "status": "approved",
                    }),
                    BroadcastOpts::default(),
                )
                .await;

                Ok(serde_json::json!({
                    "deviceToken": token_str,
                    "scopes": scopes,
                }))
            })
        }),
    );

    // node.pair.reject
    reg.register(
        "node.pair.reject",
        Box::new(|ctx| {
            Box::pin(async move {
                let pair_id = ctx
                    .params
                    .get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ErrorShape::new(error_codes::INVALID_REQUEST, "missing id"))?;

                if let Some(store) = get_pairing_store(&ctx.state) {
                    store.reject(pair_id).await.map_err(|e| {
                        ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                    })?;
                } else {
                    ctx.state
                        .inner
                        .write()
                        .await
                        .pairing
                        .reject(pair_id)
                        .map_err(|e| {
                            ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                        })?;
                }

                broadcast(
                    &ctx.state,
                    "node.pair.resolved",
                    serde_json::json!({
                        "id": pair_id, "status": "rejected",
                    }),
                    BroadcastOpts::default(),
                )
                .await;

                Ok(serde_json::json!({}))
            })
        }),
    );

    // node.pair.verify - verify pairing signature
    reg.register(
        "node.pair.verify",
        Box::new(|ctx| {
            Box::pin(async move {
                let params = ctx
                    .params
                    .as_object()
                    .ok_or_else(|| ErrorShape::new(error_codes::INVALID_REQUEST, "params must be an object"))?;

                let signature = params
                    .get("signature")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ErrorShape::new(error_codes::INVALID_REQUEST, "signature required"))?;

                let data = params
                    .get("data")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ErrorShape::new(error_codes::INVALID_REQUEST, "data required"))?;

                let public_key = params
                    .get("public_key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ErrorShape::new(error_codes::INVALID_REQUEST, "public_key required"))?;

                // Verify signature using ed25519
                let verified = verify_ed25519_signature(signature, data, public_key)
                    .map_err(|e| ErrorShape::new(error_codes::INTERNAL, e.to_string()))?;

                Ok(serde_json::json!({
                    "verified": verified,
                    "timestamp": chrono::Utc::now().timestamp()
                }))
            })
        }),
    );

    // device.pair.list
    reg.register(
        "device.pair.list",
        Box::new(|ctx| {
            Box::pin(async move {
                if let Some(store) = get_pairing_store(&ctx.state) {
                    let devices = store
                        .list_devices()
                        .await
                        .map_err(|e| ErrorShape::new(error_codes::INTERNAL, e.to_string()))?;
                    let list: Vec<_> = devices
                        .iter()
                        .map(|d| {
                            serde_json::json!({
                                "deviceId": d.device_id,
                                "displayName": d.display_name,
                                "platform": d.platform,
                                "createdAt": d.created_at,
                            })
                        })
                        .collect();
                    Ok(serde_json::json!(list))
                } else {
                    let inner = ctx.state.inner.read().await;
                    let list: Vec<_> = inner
                        .pairing
                        .list_devices()
                        .iter()
                        .map(|d| {
                            serde_json::json!({
                                "deviceId": d.device_id,
                                "scopes": d.scopes,
                                "issuedAtMs": d.issued_at_ms,
                            })
                        })
                        .collect();
                    Ok(serde_json::json!(list))
                }
            })
        }),
    );

    // device.pair.approve (alias for node.pair.approve)
    reg.register(
        "device.pair.approve",
        Box::new(|ctx| {
            Box::pin(async move {
                let pair_id = ctx
                    .params
                    .get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ErrorShape::new(error_codes::INVALID_REQUEST, "missing id"))?;

                let (token_str, scopes) = if let Some(store) = get_pairing_store(&ctx.state) {
                    let token = store.approve(pair_id).await.map_err(|e| {
                        ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                    })?;
                    (token.token, token.scopes)
                } else {
                    let token = ctx
                        .state
                        .inner
                        .write()
                        .await
                        .pairing
                        .approve(pair_id)
                        .map_err(|e| {
                            ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                        })?;
                    (token.token, token.scopes)
                };

                broadcast(
                    &ctx.state,
                    "device.pair.resolved",
                    serde_json::json!({
                        "id": pair_id, "status": "approved",
                    }),
                    BroadcastOpts::default(),
                )
                .await;

                Ok(serde_json::json!({ "deviceToken": token_str, "scopes": scopes }))
            })
        }),
    );

    // device.pair.reject
    reg.register(
        "device.pair.reject",
        Box::new(|ctx| {
            Box::pin(async move {
                let pair_id = ctx
                    .params
                    .get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ErrorShape::new(error_codes::INVALID_REQUEST, "missing id"))?;

                if let Some(store) = get_pairing_store(&ctx.state) {
                    store.reject(pair_id).await.map_err(|e| {
                        ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                    })?;
                } else {
                    ctx.state
                        .inner
                        .write()
                        .await
                        .pairing
                        .reject(pair_id)
                        .map_err(|e| {
                            ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                        })?;
                }

                broadcast(
                    &ctx.state,
                    "device.pair.resolved",
                    serde_json::json!({
                        "id": pair_id, "status": "rejected",
                    }),
                    BroadcastOpts::default(),
                )
                .await;

                Ok(serde_json::json!({}))
            })
        }),
    );

    // device.token.create — pre-authorize a device and issue a token directly
    reg.register(
        "device.token.create",
        Box::new(|ctx| {
            Box::pin(async move {
                let display_name = ctx.params.get("displayName").and_then(|v| v.as_str());
                let platform = ctx
                    .params
                    .get("platform")
                    .and_then(|v| v.as_str())
                    .unwrap_or("remote");

                let (token_str, device_id, scopes) =
                    if let Some(store) = get_pairing_store(&ctx.state) {
                        let token = store
                            .create_device_token(display_name, platform)
                            .await
                            .map_err(|e| ErrorShape::new(error_codes::INTERNAL, e.to_string()))?;
                        (token.token, token.device_id, token.scopes)
                    } else {
                        let token = ctx
                            .state
                            .inner
                            .write()
                            .await
                            .pairing
                            .create_device_token(display_name, platform);
                        (token.token, token.device_id, token.scopes)
                    };

                Ok(serde_json::json!({
                    "deviceToken": token_str,
                    "deviceId": device_id,
                    "scopes": scopes,
                }))
            })
        }),
    );

    // device.token.rotate
    reg.register(
        "device.token.rotate",
        Box::new(|ctx| {
            Box::pin(async move {
                let device_id = ctx
                    .params
                    .get("deviceId")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        ErrorShape::new(error_codes::INVALID_REQUEST, "missing deviceId")
                    })?;

                let (token_str, scopes) = if let Some(store) = get_pairing_store(&ctx.state) {
                    let token = store.rotate_token(device_id).await.map_err(|e| {
                        ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                    })?;
                    (token.token, token.scopes)
                } else {
                    let token = ctx
                        .state
                        .inner
                        .write()
                        .await
                        .pairing
                        .rotate_token(device_id)
                        .map_err(|e| {
                            ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                        })?;
                    (token.token, token.scopes)
                };

                Ok(serde_json::json!({ "deviceToken": token_str, "scopes": scopes }))
            })
        }),
    );

    // device.token.revoke
    reg.register(
        "device.token.revoke",
        Box::new(|ctx| {
            Box::pin(async move {
                let device_id = ctx
                    .params
                    .get("deviceId")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        ErrorShape::new(error_codes::INVALID_REQUEST, "missing deviceId")
                    })?;

                if let Some(store) = get_pairing_store(&ctx.state) {
                    store.revoke_token(device_id).await.map_err(|e| {
                        ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                    })?;
                } else {
                    ctx.state
                        .inner
                        .write()
                        .await
                        .pairing
                        .revoke_token(device_id)
                        .map_err(|e| {
                            ErrorShape::new(error_codes::INVALID_REQUEST, e.to_string())
                        })?;
                }

                Ok(serde_json::json!({}))
            })
        }),
    );
}
