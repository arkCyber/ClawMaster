/// Auto-migrate config from legacy schemas.
/// Returns true if migration was performed.
pub fn migrate_if_needed(config: &mut serde_json::Value) -> crate::Result<bool> {
    let Some(obj) = config.as_object_mut() else {
        return Ok(false);
    };

    // Get current schema version (default to 0 if not present)
    let current_version = obj
        .get("schema_version")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    const LATEST_VERSION: u64 = 1;

    if current_version >= LATEST_VERSION {
        // Already at latest version
        return Ok(false);
    }

    tracing::info!(
        "Migrating config from version {} to {}",
        current_version,
        LATEST_VERSION
    );

    let mut migrated = false;

    // Apply migrations in sequence
    if current_version < 1 {
        migrate_v0_to_v1(obj)?;
        migrated = true;
    }

    // Update schema version
    obj.insert(
        "schema_version".to_string(),
        serde_json::Value::Number(LATEST_VERSION.into()),
    );

    if migrated {
        tracing::info!("Config migration completed successfully");
    }

    Ok(migrated)
}

/// Migrate from version 0 (no version) to version 1.
fn migrate_v0_to_v1(config: &mut serde_json::Map<String, serde_json::Value>) -> crate::Result<()> {
    // Example migrations:
    // 1. Rename old field names
    if let Some(old_field) = config.remove("api_key") {
        config.insert("provider_api_key".to_string(), old_field);
    }

    // 2. Convert old format to new format
    if let Some(providers) = config
        .get_mut("providers")
        .and_then(|providers| providers.as_array_mut())
    {
        for provider in providers {
            if let Some(obj) = provider.as_object_mut() {
                if let Some(provider_type) = obj.get("type").and_then(|v| v.as_str())
                    && provider_type == "openrouter"
                {
                    obj.insert("name".to_string(), serde_json::json!("openrouter"));
                }
                // Add default enabled field if missing
                obj.entry("enabled".to_string())
                    .or_insert(serde_json::Value::Bool(true));
            }
        }
    }

    // 3. Add new required fields with defaults
    config
        .entry("log_level".to_string())
        .or_insert(serde_json::Value::String("info".to_string()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_if_needed_not_object() {
        let mut config = serde_json::json!("not an object");
        let result = migrate_if_needed(&mut config);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // No migration
    }

    #[test]
    fn test_migrate_if_needed_already_latest() {
        let mut config = serde_json::json!({
            "schema_version": 1,
            "some_field": "value"
        });
        let result = migrate_if_needed(&mut config);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // No migration needed
    }

    #[test]
    fn test_migrate_if_needed_v0_to_v1() {
        let mut config = serde_json::json!({
            "api_key": "secret123",
            "providers": [
                {"name": "openai"},
                {"name": "anthropic", "enabled": false}
            ]
        });

        let result = migrate_if_needed(&mut config);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Migration performed

        let obj = config.as_object().unwrap();

        // Check schema version updated
        assert_eq!(obj["schema_version"], 1);

        // Check api_key renamed
        assert!(obj.get("api_key").is_none());
        assert_eq!(obj["provider_api_key"], "secret123");

        // Check providers have enabled field
        let providers = obj["providers"].as_array().unwrap();
        assert_eq!(providers[0]["enabled"], true); // Added default
        assert_eq!(providers[1]["enabled"], false); // Preserved existing

        // Check log_level added
        assert_eq!(obj["log_level"], "info");
    }

    #[test]
    fn test_migrate_if_needed_no_api_key() {
        let mut config = serde_json::json!({
            "some_field": "value"
        });

        let result = migrate_if_needed(&mut config);
        assert!(result.is_ok());
        assert!(result.unwrap());

        let obj = config.as_object().unwrap();
        assert_eq!(obj["schema_version"], 1);
        assert_eq!(obj["log_level"], "info");
        assert!(obj.get("provider_api_key").is_none());
    }

    #[test]
    fn test_migrate_if_needed_preserves_existing_log_level() {
        let mut config = serde_json::json!({
            "log_level": "debug"
        });

        let result = migrate_if_needed(&mut config);
        assert!(result.is_ok());

        let obj = config.as_object().unwrap();
        assert_eq!(obj["log_level"], "debug"); // Preserved
    }

    #[test]
    fn test_migrate_if_needed_empty_config() {
        let mut config = serde_json::json!({});

        let result = migrate_if_needed(&mut config);
        assert!(result.is_ok());
        assert!(result.unwrap());

        let obj = config.as_object().unwrap();
        assert_eq!(obj["schema_version"], 1);
        assert_eq!(obj["log_level"], "info");
    }

    #[test]
    fn test_migrate_v0_to_v1_providers_not_array() {
        let mut config = serde_json::json!({
            "providers": "not an array"
        });

        let result = migrate_if_needed(&mut config);
        assert!(result.is_ok()); // Should not fail

        let obj = config.as_object().unwrap();
        assert_eq!(obj["schema_version"], 1);
    }
}
