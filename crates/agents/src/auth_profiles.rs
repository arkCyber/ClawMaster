/// OAuth + API key credential management with token refresh, stored per-agent.
use secrecy::Secret;

pub struct AuthProfile {
    pub provider: String,
    pub credentials: Credentials,
}

pub enum Credentials {
    ApiKey(Secret<String>),
    OAuth {
        access_token: Secret<String>,
        refresh_token: Option<Secret<String>>,
        expires_at: Option<u64>,
    },
}

use std::time::{SystemTime, UNIX_EPOCH};

/// Refresh credentials if expired.
pub async fn refresh_if_needed(profile: &mut AuthProfile) -> anyhow::Result<()> {
    match &mut profile.credentials {
        Credentials::ApiKey(_) => {
            // API keys don't expire, no refresh needed
            Ok(())
        },
        Credentials::OAuth {
            access_token,
            refresh_token,
            expires_at,
        } => {
            // Check if token is expired or about to expire (within 5 minutes)
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            let needs_refresh = if let Some(exp) = expires_at {
                // Refresh if expired or expiring within 5 minutes
                *exp < now + 300
            } else {
                // No expiry info, assume valid
                false
            };

            if !needs_refresh {
                return Ok(());
            }

            // Need to refresh
            let Some(rt) = refresh_token else {
                return Err(anyhow::anyhow!(
                    "OAuth token expired but no refresh token available"
                ));
            };

            tracing::info!("Refreshing OAuth token for provider: {}", profile.provider);

            // TODO: Implement actual OAuth refresh flow based on provider
            // For now, return an error indicating refresh is needed
            // In production, this would call the provider's token endpoint
            Err(anyhow::anyhow!(
                "OAuth token refresh not yet implemented for provider: {}",
                profile.provider
            ))

            // Example implementation structure:
            // let new_tokens = refresh_oauth_token(&profile.provider, rt.expose_secret()).await?;
            // *access_token = Secret::new(new_tokens.access_token);
            // if let Some(new_rt) = new_tokens.refresh_token {
            //     *refresh_token = Some(Secret::new(new_rt));
            // }
            // *expires_at = Some(now + new_tokens.expires_in);
            // Ok(())
        },
    }
}
