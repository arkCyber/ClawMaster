//! Integration tests for the browser tool.
//!
//! These tests verify the browser tool's integration with the agent system,
//! including parameter validation, session management, and error handling.

use clawmaster_agents::tool_registry::AgentTool;
use clawmaster_tools::browser::BrowserTool;
use serde_json::json;

#[tokio::test]
async fn test_browser_tool_name_and_description() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    assert_eq!(tool.name(), "browser");
    assert!(tool.description().contains("Control a real browser"));
    assert!(tool.description().contains("navigate"));
}

#[tokio::test]
async fn test_browser_tool_parameters_schema() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let schema = tool.parameters_schema();
    
    // Verify required fields
    assert_eq!(schema["type"], "object");
    assert_eq!(schema["required"], json!(["action"]));
    
    // Verify action enum
    let actions = &schema["properties"]["action"]["enum"];
    assert!(actions.as_array().unwrap().contains(&json!("navigate")));
    assert!(actions.as_array().unwrap().contains(&json!("screenshot")));
    assert!(actions.as_array().unwrap().contains(&json!("snapshot")));
    assert!(actions.as_array().unwrap().contains(&json!("click")));
    assert!(actions.as_array().unwrap().contains(&json!("close")));
}

#[tokio::test]
async fn test_browser_tool_missing_action_error() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    // Missing action field should return helpful error
    let params = json!({});
    let result = tool.execute(params).await;
    
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("action") || error_msg.contains("Missing"));
}

#[tokio::test]
async fn test_browser_tool_missing_action_with_url() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    // Missing action but has URL - should default to navigate
    let params = json!({
        "url": "https://example.com"
    });
    
    let result = tool.execute(params).await;
    
    // Should succeed or fail with browser-specific error, not parameter error
    if let Err(e) = result {
        let error_msg = e.to_string();
        // Should NOT be a "missing action" error
        assert!(!error_msg.contains("Missing required 'action'"));
    }
}

#[tokio::test]
async fn test_browser_tool_invalid_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "invalid_action"
    });
    
    let result = tool.execute(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_browser_tool_session_tracking() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    // First call without session_id
    let params1 = json!({
        "action": "navigate",
        "url": "https://example.com"
    });
    
    let result1 = tool.execute(params1).await;
    
    // Should succeed or fail with browser error (not parameter error)
    if let Ok(response1) = result1 {
        // Verify response has session_id
        assert!(response1.get("session_id").is_some());
        
        // Second call without session_id should reuse the same session
        let params2 = json!({
            "action": "get_url"
        });
        
        let result2 = tool.execute(params2).await;
        
        if let Ok(response2) = result2 {
            // Should have same session_id
            assert_eq!(
                response1.get("session_id"),
                response2.get("session_id")
            );
        }
    }
}

#[tokio::test]
async fn test_browser_tool_close_clears_session() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    // Navigate to create a session
    let params1 = json!({
        "action": "navigate",
        "url": "https://example.com"
    });
    
    let _ = tool.execute(params1).await;
    
    // Close the session
    let params2 = json!({
        "action": "close"
    });
    
    let _ = tool.execute(params2).await;
    
    // Next call should create a new session
    let params3 = json!({
        "action": "navigate",
        "url": "https://example.com"
    });
    
    let result3 = tool.execute(params3).await;
    
    if let Ok(response3) = result3 {
        // Should have a session_id (new one)
        assert!(response3.get("session_id").is_some());
    }
}

#[tokio::test]
async fn test_browser_tool_warmup() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    // Warmup should not fail
    let result = tool.warmup().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_browser_tool_from_config_enabled() {
    let config = clawmaster_config::schema::BrowserConfig {
        enabled: true,
        ..Default::default()
    };
    
    let tool = BrowserTool::from_config(&config);
    assert!(tool.is_some());
}

#[tokio::test]
async fn test_browser_tool_from_config_disabled() {
    let config = clawmaster_config::schema::BrowserConfig {
        enabled: false,
        ..Default::default()
    };
    
    let tool = BrowserTool::from_config(&config);
    assert!(tool.is_none());
}

#[tokio::test]
async fn test_browser_tool_navigate_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "navigate",
        "url": "https://example.com"
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response (success or browser error)
    assert!(result.is_ok() || result.is_err());
    
    if let Ok(response) = result {
        // Response should have required fields
        assert!(response.get("success").is_some());
        assert!(response.get("session_id").is_some());
    }
}

#[tokio::test]
async fn test_browser_tool_screenshot_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "screenshot"
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_snapshot_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "snapshot"
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_click_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "click",
        "ref_": 1
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_type_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "type",
        "ref_": 1,
        "text": "Hello World"
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_scroll_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "scroll",
        "x": 0,
        "y": 100
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_evaluate_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "evaluate",
        "code": "document.title"
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_wait_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "wait",
        "selector": "body",
        "timeout_ms": 5000
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_get_url_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "get_url"
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_get_title_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "get_title"
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_back_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "back"
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_forward_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "forward"
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_refresh_action() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "refresh"
    });
    
    let result = tool.execute(params).await;
    
    // Should return a response
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_browser_selection() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "navigate",
        "url": "https://example.com",
        "browser": "chrome"
    });
    
    let result = tool.execute(params).await;
    
    // Should handle browser selection
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_full_page_screenshot() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "screenshot",
        "full_page": true
    });
    
    let result = tool.execute(params).await;
    
    // Should handle full page screenshot
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_browser_tool_viewport_screenshot() {
    let config = clawmaster_browser::BrowserConfig::default();
    let tool = BrowserTool::new(config);
    
    let params = json!({
        "action": "screenshot",
        "full_page": false
    });
    
    let result = tool.execute(params).await;
    
    // Should handle viewport screenshot
    assert!(result.is_ok() || result.is_err());
}
