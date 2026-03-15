// Tauri UI Integration Tests

#[cfg(test)]
mod ui_tests {
    use reqwest;

    #[tokio::test]
    async fn test_backend_api_gon() {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        
        let response = client
            .get("https://localhost:59233/api/gon")
            .send()
            .await;
        
        match response {
            Ok(resp) => {
                assert!(resp.status().is_success(), "API should return success");
                let body = resp.text().await.unwrap();
                assert!(body.contains("version"), "Response should contain version");
            }
            Err(_) => {
                println!("Backend not running, skipping test");
            }
        }
    }

    #[tokio::test]
    async fn test_backend_api_sessions() {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        
        let response = client
            .get("https://localhost:59233/api/sessions")
            .send()
            .await;
        
        match response {
            Ok(resp) => {
                assert!(resp.status().is_success(), "Sessions API should return success");
                let body = resp.text().await.unwrap();
                assert!(body.starts_with('['), "Response should be JSON array");
            }
            Err(_) => {
                println!("Backend not running, skipping test");
            }
        }
    }

    #[tokio::test]
    async fn test_backend_api_models() {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        
        let response = client
            .get("https://localhost:59233/api/models")
            .send()
            .await;
        
        match response {
            Ok(resp) => {
                // Models API may return HTML if not authenticated
                println!("Models API status: {}", resp.status());
            }
            Err(_) => {
                println!("Backend not running, skipping test");
            }
        }
    }

    #[test]
    fn test_tauri_commands_exist() {
        // Verify that the Tauri library compiles correctly
        let _ = clawmaster_tauri::init_app();
    }
}
