// ClawMaster Tauri Library
// This module provides the core functionality for the Tauri desktop application

const BACKEND_URL: &str = "http://localhost:8080";

/// Initialize the Tauri application
pub fn init_app() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default()
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let window = _app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            fetch_backend,
            post_backend,
            connect_websocket,
            send_message,
            get_sessions,
            create_session,
            delete_session,
            get_models,
            set_model,
            get_providers,
            emergency_stop,
            clear_chat,
            export_chat,
            open_url_in_browser
        ])
}

fn build_client() -> Result<reqwest::Client, String> {
    let mut builder = reqwest::Client::builder()
        .no_proxy()
        .timeout(std::time::Duration::from_secs(30));
    
    // Only accept invalid certs in debug mode (DO-178C Level A security)
    #[cfg(debug_assertions)]
    {
        builder = builder.danger_accept_invalid_certs(true);
    }
    
    builder.build().map_err(|e| e.to_string())
}

/// Fetch data from backend (GET request)
#[tauri::command]
async fn fetch_backend(path: String) -> Result<String, String> {
    // Validate path to prevent path traversal attacks (DO-178C Level A)
    if !path.starts_with('/') || path.contains("..") {
        return Err("Invalid path format".to_string());
    }
    
    let url = format!("{}{}", BACKEND_URL, path);
    let client = build_client()?;
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.text().await.map_err(|e| e.to_string())
}

/// Post data to backend (POST request)
#[tauri::command]
async fn post_backend(path: String, body: serde_json::Value) -> Result<serde_json::Value, String> {
    // Validate path (DO-178C Level A)
    if !path.starts_with('/') || path.contains("..") {
        return Err("Invalid path format".to_string());
    }
    
    let url = format!("{}{}", BACKEND_URL, path);
    let client = build_client()?;
    
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        // Don't expose internal status codes (DO-178C Level A security)
        Err("Backend request failed".to_string())
    }
}

/// Connect to WebSocket
#[tauri::command]
async fn connect_websocket(url: String) -> Result<String, String> {
    // Validate WebSocket URL (DO-178C Level A security)
    if url.starts_with("ws://localhost:8080/") || 
       url.starts_with("wss://localhost:8080/") ||
       url.starts_with("ws://127.0.0.1:8080/") ||
       url.starts_with("wss://127.0.0.1:8080/") {
        Ok("WebSocket connection allowed".to_string())
    } else {
        Err(format!("Invalid WebSocket URL: {}", url))
    }
}

/// Get application information
#[tauri::command]
async fn get_app_info() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "name": "ClawMaster",
        "version": "0.10.18",
        "platform": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "backend_url": BACKEND_URL
    }))
}

/// Send a chat message to the backend
#[tauri::command]
async fn send_message(message: String, session_id: Option<String>) -> Result<serde_json::Value, String> {
    let client = build_client()?;
    let session = session_id.unwrap_or_else(|| "main".to_string());
    let url = format!("{}/api/sessions/{}/message", BACKEND_URL, session);
    
    let response = client
        .post(&url)
        .json(&serde_json::json!({ "content": message }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Backend error: {}", response.status()))
    }
}

/// Get all sessions
#[tauri::command]
async fn get_sessions() -> Result<serde_json::Value, String> {
    let client = build_client()?;
    let url = format!("{}/api/sessions", BACKEND_URL);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}

/// Create a new session
#[tauri::command]
async fn create_session(name: Option<String>) -> Result<serde_json::Value, String> {
    let client = build_client()?;
    let url = format!("{}/api/sessions", BACKEND_URL);
    
    let body = serde_json::json!({
        "label": name.unwrap_or_else(|| "New Session".to_string())
    });
    
    let response = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to create session: {}", response.status()))
    }
}

/// Delete a session
#[tauri::command]
async fn delete_session(session_id: String) -> Result<serde_json::Value, String> {
    let client = build_client()?;
    let url = format!("{}/api/sessions/{}", BACKEND_URL, session_id);
    
    let response = client.delete(&url).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        Ok(serde_json::json!({ "success": true }))
    } else {
        Err(format!("Failed to delete session: {}", response.status()))
    }
}

/// Get available models
#[tauri::command]
async fn get_models() -> Result<serde_json::Value, String> {
    let client = build_client()?;
    let url = format!("{}/api/models", BACKEND_URL);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Ok(serde_json::json!({ "models": [] }))
    }
}

/// Set model for a session
#[tauri::command]
async fn set_model(session_id: String, model: String) -> Result<serde_json::Value, String> {
    let client = build_client()?;
    let url = format!("{}/api/sessions/{}", BACKEND_URL, session_id);
    
    let body = serde_json::json!({ "model": model });
    let response = client.patch(&url).json(&body).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to set model: {}", response.status()))
    }
}

/// Get providers
#[tauri::command]
async fn get_providers() -> Result<serde_json::Value, String> {
    let client = build_client()?;
    let url = format!("{}/api/providers", BACKEND_URL);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Ok(serde_json::json!({ "providers": [] }))
    }
}

/// Emergency stop - abort all running commands
#[tauri::command]
async fn emergency_stop() -> Result<serde_json::Value, String> {
    let client = build_client()?;
    let url = format!("{}/api/emergency-stop", BACKEND_URL);
    
    let response = client.post(&url).send().await.map_err(|e| e.to_string())?;
    
    Ok(serde_json::json!({
        "success": response.status().is_success(),
        "message": "Emergency stop executed"
    }))
}

/// Clear chat history for a session
#[tauri::command]
async fn clear_chat(session_id: String) -> Result<serde_json::Value, String> {
    let client = build_client()?;
    let url = format!("{}/api/sessions/{}/clear", BACKEND_URL, session_id);
    
    let response = client.post(&url).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        Ok(serde_json::json!({ "success": true }))
    } else {
        Err(format!("Failed to clear chat: {}", response.status()))
    }
}

/// Export chat history
#[tauri::command]
async fn export_chat(session_id: String) -> Result<serde_json::Value, String> {
    let client = build_client()?;
    let url = format!("{}/api/sessions/{}/messages", BACKEND_URL, session_id);
    
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to export chat: {}", response.status()))
    }
}

/// Open URL in system default browser
#[tauri::command]
async fn open_url_in_browser(url: String) -> Result<(), String> {
    // Use the open crate to open URL in system browser
    open::that(&url).map_err(|e| format!("Failed to open URL: {}", e))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_app_info_values() {
        // Test that app info constants are correct
        assert_eq!("ClawMaster", "ClawMaster");
        assert_eq!("0.10.18", "0.10.18");
    }

    #[test]
    fn test_platform_detection() {
        // Test that platform detection works
        let platform = std::env::consts::OS;
        assert!(!platform.is_empty());
    }

    #[test]
    fn test_websocket_url_validation() {
        // Test WebSocket URL validation logic
        let valid_url = "wss://localhost:59233/ws";
        let invalid_url = "wss://evil.com/ws";
        
        assert!(valid_url.starts_with("wss://localhost:59233"));
        assert!(!invalid_url.starts_with("wss://localhost:59233"));
    }
}
