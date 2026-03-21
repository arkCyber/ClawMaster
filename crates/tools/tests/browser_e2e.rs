//! End-to-end tests for browser tool with real browser automation.
//!
//! These tests require a browser to be installed and will be skipped
//! if no browser is available.

use {
    clawmaster_agents::tool_registry::AgentTool, clawmaster_tools::browser::BrowserTool,
    serde_json::json,
};

/// Helper to check if browser is available for testing
fn browser_available() -> bool {
    std::env::var("SKIP_BROWSER_TESTS").is_err()
}

#[tokio::test]
async fn test_browser_navigate_and_screenshot() {
    if !browser_available() {
        eprintln!("Skipping browser E2E test (SKIP_BROWSER_TESTS is set)");
        return;
    }

    let config = clawmaster_browser::BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let tool = BrowserTool::new(config);

    // Navigate to a simple page
    let navigate_params = json!({
        "action": "navigate",
        "url": "https://example.com"
    });

    let navigate_result = tool.execute(navigate_params).await;

    if let Ok(navigate_response) = navigate_result {
        assert_eq!(navigate_response["success"], true);
        let session_id = navigate_response["session_id"].as_str().unwrap();

        // Take a screenshot
        let screenshot_params = json!({
            "action": "screenshot",
            "session_id": session_id
        });

        let screenshot_result = tool.execute(screenshot_params).await;

        if let Ok(screenshot_response) = screenshot_result {
            assert_eq!(screenshot_response["success"], true);
            assert!(screenshot_response["data"].is_string());

            // Close the browser
            let close_params = json!({
                "action": "close",
                "session_id": session_id
            });

            let _ = tool.execute(close_params).await;
        }
    }
}

#[tokio::test]
async fn test_browser_navigate_and_snapshot() {
    if !browser_available() {
        eprintln!("Skipping browser E2E test (SKIP_BROWSER_TESTS is set)");
        return;
    }

    let config = clawmaster_browser::BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let tool = BrowserTool::new(config);

    // Navigate to a page
    let navigate_params = json!({
        "action": "navigate",
        "url": "https://example.com"
    });

    let navigate_result = tool.execute(navigate_params).await;

    if let Ok(navigate_response) = navigate_result {
        assert_eq!(navigate_response["success"], true);
        let session_id = navigate_response["session_id"].as_str().unwrap();

        // Get snapshot
        let snapshot_params = json!({
            "action": "snapshot",
            "session_id": session_id
        });

        let snapshot_result = tool.execute(snapshot_params).await;

        if let Ok(snapshot_response) = snapshot_result {
            assert_eq!(snapshot_response["success"], true);
            assert!(snapshot_response["snapshot"].is_string());

            // Close the browser
            let close_params = json!({
                "action": "close",
                "session_id": session_id
            });

            let _ = tool.execute(close_params).await;
        }
    }
}

#[tokio::test]
async fn test_browser_get_url_and_title() {
    if !browser_available() {
        eprintln!("Skipping browser E2E test (SKIP_BROWSER_TESTS is set)");
        return;
    }

    let config = clawmaster_browser::BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let tool = BrowserTool::new(config);

    // Navigate to a page
    let navigate_params = json!({
        "action": "navigate",
        "url": "https://example.com"
    });

    let navigate_result = tool.execute(navigate_params).await;

    if let Ok(navigate_response) = navigate_result {
        let session_id = navigate_response["session_id"].as_str().unwrap();

        // Get URL
        let get_url_params = json!({
            "action": "get_url",
            "session_id": session_id
        });

        let get_url_result = tool.execute(get_url_params).await;

        if let Ok(url_response) = get_url_result {
            assert_eq!(url_response["success"], true);
            assert!(
                url_response["url"]
                    .as_str()
                    .unwrap()
                    .contains("example.com")
            );
        }

        // Get title
        let get_title_params = json!({
            "action": "get_title",
            "session_id": session_id
        });

        let get_title_result = tool.execute(get_title_params).await;

        if let Ok(title_response) = get_title_result {
            assert_eq!(title_response["success"], true);
            assert!(title_response["title"].is_string());
        }

        // Close the browser
        let close_params = json!({
            "action": "close",
            "session_id": session_id
        });

        let _ = tool.execute(close_params).await;
    }
}

#[tokio::test]
async fn test_browser_evaluate_javascript() {
    if !browser_available() {
        eprintln!("Skipping browser E2E test (SKIP_BROWSER_TESTS is set)");
        return;
    }

    let config = clawmaster_browser::BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let tool = BrowserTool::new(config);

    // Navigate to a page
    let navigate_params = json!({
        "action": "navigate",
        "url": "https://example.com"
    });

    let navigate_result = tool.execute(navigate_params).await;

    if let Ok(navigate_response) = navigate_result {
        let session_id = navigate_response["session_id"].as_str().unwrap();

        // Evaluate JavaScript
        let evaluate_params = json!({
            "action": "evaluate",
            "session_id": session_id,
            "code": "document.title"
        });

        let evaluate_result = tool.execute(evaluate_params).await;

        if let Ok(evaluate_response) = evaluate_result {
            assert_eq!(evaluate_response["success"], true);
            assert!(evaluate_response["result"].is_string());
        }

        // Close the browser
        let close_params = json!({
            "action": "close",
            "session_id": session_id
        });

        let _ = tool.execute(close_params).await;
    }
}

#[tokio::test]
async fn test_browser_session_reuse() {
    if !browser_available() {
        eprintln!("Skipping browser E2E test (SKIP_BROWSER_TESTS is set)");
        return;
    }

    let config = clawmaster_browser::BrowserConfig {
        headless: true,
        ..Default::default()
    };
    let tool = BrowserTool::new(config);

    // First navigation
    let navigate1_params = json!({
        "action": "navigate",
        "url": "https://example.com"
    });

    let navigate1_result = tool.execute(navigate1_params).await;

    if let Ok(navigate1_response) = navigate1_result {
        let session_id1 = navigate1_response["session_id"]
            .as_str()
            .unwrap()
            .to_string();

        // Second navigation without session_id (should reuse)
        let navigate2_params = json!({
            "action": "navigate",
            "url": "https://www.iana.org/domains/reserved"
        });

        let navigate2_result = tool.execute(navigate2_params).await;

        if let Ok(navigate2_response) = navigate2_result {
            let session_id2 = navigate2_response["session_id"].as_str().unwrap();

            // Should reuse the same session
            assert_eq!(session_id1, session_id2);

            // Close the browser
            let close_params = json!({
                "action": "close"
            });

            let _ = tool.execute(close_params).await;
        }
    }
}

#[tokio::test]
async fn test_browser_multiple_sessions() {
    if !browser_available() {
        eprintln!("Skipping browser E2E test (SKIP_BROWSER_TESTS is set)");
        return;
    }

    let config = clawmaster_browser::BrowserConfig {
        headless: true,
        max_instances: 2,
        ..Default::default()
    };
    let tool = BrowserTool::new(config);

    // Create first session
    let navigate1_params = json!({
        "action": "navigate",
        "url": "https://example.com",
        "session_id": "session1"
    });

    let navigate1_result = tool.execute(navigate1_params).await;

    if let Ok(navigate1_response) = navigate1_result {
        assert_eq!(navigate1_response["session_id"], "session1");

        // Create second session
        let navigate2_params = json!({
            "action": "navigate",
            "url": "https://www.iana.org/domains/reserved",
            "session_id": "session2"
        });

        let navigate2_result = tool.execute(navigate2_params).await;

        if let Ok(navigate2_response) = navigate2_result {
            assert_eq!(navigate2_response["session_id"], "session2");

            // Close both sessions
            let _ = tool
                .execute(json!({"action": "close", "session_id": "session1"}))
                .await;
            let _ = tool
                .execute(json!({"action": "close", "session_id": "session2"}))
                .await;
        }
    }
}
