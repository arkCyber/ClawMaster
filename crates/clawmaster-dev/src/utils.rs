//! Utility functions

/// Check if a name is valid (alphanumeric, hyphens, underscores)
pub fn is_valid_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_name() {
        assert!(is_valid_name("my-plugin"));
        assert!(is_valid_name("my_plugin"));
        assert!(is_valid_name("MyPlugin123"));
        assert!(!is_valid_name("my plugin"));
        assert!(!is_valid_name("my@plugin"));
        assert!(!is_valid_name(""));
    }
}
