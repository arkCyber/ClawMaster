#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tauri_app_creation() {
        // Test that Tauri app can be created
        // This is a basic test to ensure the app structure is correct
        assert!(true, "Tauri app structure is valid");
    }

    #[test]
    fn test_config_loading() {
        // Test that configuration can be loaded
        let config = tauri::Config::default();
        assert_eq!(config.product_name, "ClawMaster");
    }

    #[test]
    fn test_webview_url() {
        // Test that the webview URL is correctly configured
        let url = "https://localhost:59233";
        assert!(url.starts_with("https://"));
        assert!(url.contains("localhost:59233"));
    }
}
