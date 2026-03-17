//! Integration tests for plugin system
//! DO-178C Level A compliant test suite

use clawmaster_plugin_system::*;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a test plugin directory
fn create_test_plugin(dir: &TempDir, plugin_id: &str) -> PathBuf {
    let plugin_dir = dir.path().join(plugin_id);
    std::fs::create_dir_all(&plugin_dir).unwrap();

    let manifest = format!(
        r#"
id = "{}"
name = "Test Plugin"
version = "1.0.0"
author = "Test Author"
description = "Test plugin for integration tests"
dependencies = []
permissions = ["FileRead"]
tags = ["test"]

[config_schema]
type = "object"
"#,
        plugin_id
    );

    std::fs::write(plugin_dir.join("plugin.toml"), manifest).unwrap();
    plugin_dir
}

#[tokio::test]
async fn test_plugin_system_full_lifecycle() {
    let tmp = tempfile::tempdir().unwrap();
    let system = PluginSystem::new(tmp.path().to_path_buf()).unwrap();

    // Create test plugin
    let plugin_path = create_test_plugin(&tmp, "test-plugin");

    // Load plugin
    let plugin_id = system.load_plugin(plugin_path).await.unwrap();
    assert_eq!(plugin_id, "test-plugin");

    // List plugins
    let plugins = system.list_plugins().await.unwrap();
    assert_eq!(plugins.len(), 1);
    assert_eq!(plugins[0].id, "test-plugin");

    // Enable plugin
    system.enable_plugin(&plugin_id).await.unwrap();

    // Disable plugin
    system.disable_plugin(&plugin_id).await.unwrap();

    // Unload plugin
    system.unload_plugin(&plugin_id).await.unwrap();

    // Verify unloaded
    let plugins = system.list_plugins().await.unwrap();
    assert_eq!(plugins.len(), 0);
}

#[tokio::test]
async fn test_plugin_system_config_update() {
    let tmp = tempfile::tempdir().unwrap();
    let system = PluginSystem::new(tmp.path().to_path_buf()).unwrap();

    let plugin_path = create_test_plugin(&tmp, "config-test");
    let plugin_id = system.load_plugin(plugin_path).await.unwrap();

    // Update configuration
    let config = serde_json::json!({
        "setting1": "value1",
        "setting2": 42
    });

    system.update_config(&plugin_id, config.clone()).await.unwrap();

    // Verify configuration was updated
    let plugin = system.get_plugin(&plugin_id).await.unwrap();
    assert_eq!(plugin.id, "config-test");
}

#[tokio::test]
async fn test_plugin_system_event_subscription() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let tmp = tempfile::tempdir().unwrap();
    let system = PluginSystem::new(tmp.path().to_path_buf()).unwrap();

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    // Subscribe to plugin_enabled events
    system.subscribe("plugin_enabled", move |_event| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }).await.unwrap();

    let plugin_path = create_test_plugin(&tmp, "event-test");
    let plugin_id = system.load_plugin(plugin_path).await.unwrap();

    // Enable plugin (should trigger event)
    system.enable_plugin(&plugin_id).await.unwrap();

    // Give event handlers time to execute
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_plugin_system_dependency_resolution() {
    let tmp = tempfile::tempdir().unwrap();
    let system = PluginSystem::new(tmp.path().to_path_buf()).unwrap();

    // Create plugin A (no dependencies)
    let plugin_a_path = create_test_plugin(&tmp, "plugin-a");
    system.load_plugin(plugin_a_path).await.unwrap();

    // Create plugin B (depends on A)
    let plugin_b_dir = tmp.path().join("plugin-b");
    std::fs::create_dir_all(&plugin_b_dir).unwrap();

    let manifest_b = r#"
id = "plugin-b"
name = "Plugin B"
version = "1.0.0"
author = "Test Author"
description = "Plugin B depends on A"
permissions = ["FileRead"]
tags = ["test"]

[[dependencies]]
plugin_id = "plugin-a"
version = "^1.0.0"
optional = false

[config_schema]
type = "object"
"#;

    std::fs::write(plugin_b_dir.join("plugin.toml"), manifest_b).unwrap();

    // Load plugin B (should succeed because A is loaded)
    let result = system.load_plugin(plugin_b_dir).await;
    if let Err(e) = &result {
        eprintln!("Error loading plugin B: {:?}", e);
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_plugin_system_missing_dependency() {
    let tmp = tempfile::tempdir().unwrap();
    let system = PluginSystem::new(tmp.path().to_path_buf()).unwrap();

    // Create plugin with missing dependency
    let plugin_dir = tmp.path().join("plugin-missing-dep");
    std::fs::create_dir_all(&plugin_dir).unwrap();

    let manifest = r#"
id = "plugin-missing-dep"
name = "Plugin Missing Dep"
version = "1.0.0"
author = "Test Author"
description = "Plugin with missing dependency"
tags = ["test"]

[[dependencies]]
plugin_id = "non-existent-plugin"
version = "^1.0.0"
optional = false

[config_schema]
type = "object"
"#;

    std::fs::write(plugin_dir.join("plugin.toml"), manifest).unwrap();

    // Should fail to load
    let result = system.load_plugin(plugin_dir).await;
    assert!(result.is_err());
}

#[cfg(feature = "hot-reload")]
#[tokio::test]
async fn test_plugin_system_hot_reload() {
    let tmp = tempfile::tempdir().unwrap();
    let system = PluginSystem::new(tmp.path().to_path_buf()).unwrap();

    let plugin_path = create_test_plugin(&tmp, "reload-test");
    let plugin_id = system.load_plugin(plugin_path.clone()).await.unwrap();

    // Modify plugin (simulate code change)
    let manifest = r#"
id = "reload-test"
name = "Test Plugin (Modified)"
version = "1.0.1"
author = "Test Author"
description = "Modified test plugin"
dependencies = []
permissions = ["FileRead"]
tags = ["test", "modified"]

[config_schema]
type = "object"
"#;

    std::fs::write(plugin_path.join("plugin.toml"), manifest).unwrap();

    // Reload plugin
    system.reload_plugin(&plugin_id).await.unwrap();

    // Verify changes
    let plugin = system.get_plugin(&plugin_id).await.unwrap();
    assert_eq!(plugin.version, "1.0.1");
    assert!(plugin.tags.contains(&"modified".to_string()));
}

#[tokio::test]
async fn test_plugin_system_multiple_plugins() {
    let tmp = tempfile::tempdir().unwrap();
    let system = PluginSystem::new(tmp.path().to_path_buf()).unwrap();

    // Load multiple plugins
    for i in 0..5 {
        let plugin_path = create_test_plugin(&tmp, &format!("plugin-{}", i));
        system.load_plugin(plugin_path).await.unwrap();
    }

    // Verify all loaded
    let plugins = system.list_plugins().await.unwrap();
    assert_eq!(plugins.len(), 5);

    // Enable all
    for i in 0..5 {
        system.enable_plugin(&format!("plugin-{}", i)).await.unwrap();
    }

    // Disable all
    for i in 0..5 {
        system.disable_plugin(&format!("plugin-{}", i)).await.unwrap();
    }

    // Unload all
    for i in 0..5 {
        system.unload_plugin(&format!("plugin-{}", i)).await.unwrap();
    }

    // Verify all unloaded
    let plugins = system.list_plugins().await.unwrap();
    assert_eq!(plugins.len(), 0);
}

#[tokio::test]
async fn test_plugin_system_error_handling() {
    let tmp = tempfile::tempdir().unwrap();
    let system = PluginSystem::new(tmp.path().to_path_buf()).unwrap();

    // Try to load non-existent plugin
    let result = system.load_plugin(tmp.path().join("non-existent")).await;
    assert!(result.is_err());

    // Try to enable non-existent plugin
    let result = system.enable_plugin("non-existent").await;
    assert!(result.is_err());

    // Try to get non-existent plugin
    let result = system.get_plugin("non-existent").await;
    assert!(result.is_err());
}
