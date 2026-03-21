use crate::state::{Channel, ConfigTemplate, Provider};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_template_all() {
        let templates = ConfigTemplate::all();
        assert_eq!(templates.len(), 6);
        assert!(templates.contains(&ConfigTemplate::Custom));
        assert!(templates.contains(&ConfigTemplate::Basic));
        assert!(templates.contains(&ConfigTemplate::Development));
        assert!(templates.contains(&ConfigTemplate::Production));
        assert!(templates.contains(&ConfigTemplate::Minimal));
        assert!(templates.contains(&ConfigTemplate::Enterprise));
    }

    #[test]
    fn test_config_template_names() {
        assert_eq!(ConfigTemplate::Custom.name(), "Custom");
        assert_eq!(ConfigTemplate::Basic.name(), "Basic");
        assert_eq!(ConfigTemplate::Development.name(), "Development");
        assert_eq!(ConfigTemplate::Production.name(), "Production");
        assert_eq!(ConfigTemplate::Minimal.name(), "Minimal");
        assert_eq!(ConfigTemplate::Enterprise.name(), "Enterprise");
    }

    #[test]
    fn test_config_template_descriptions() {
        assert!(ConfigTemplate::Custom.description().contains("Customize"));
        assert!(ConfigTemplate::Basic.description().contains("Quick start"));
        assert!(
            ConfigTemplate::Development
                .description()
                .contains("Development")
        );
        assert!(
            ConfigTemplate::Production
                .description()
                .contains("Production")
        );
        assert!(ConfigTemplate::Minimal.description().contains("Minimal"));
        assert!(
            ConfigTemplate::Enterprise
                .description()
                .contains("enterprise")
        );
    }

    #[test]
    fn test_basic_template_providers() {
        let providers = ConfigTemplate::Basic.recommended_providers();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0], Provider::OpenAI);
    }

    #[test]
    fn test_development_template_providers() {
        let providers = ConfigTemplate::Development.recommended_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&Provider::OpenAI));
        assert!(providers.contains(&Provider::Ollama));
    }

    #[test]
    fn test_production_template_providers() {
        let providers = ConfigTemplate::Production.recommended_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&Provider::OpenAI));
        assert!(providers.contains(&Provider::Anthropic));
    }

    #[test]
    fn test_minimal_template_providers() {
        let providers = ConfigTemplate::Minimal.recommended_providers();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0], Provider::Ollama);
    }

    #[test]
    fn test_enterprise_template_providers() {
        let providers = ConfigTemplate::Enterprise.recommended_providers();
        assert_eq!(providers.len(), 3);
        assert!(providers.contains(&Provider::OpenAI));
        assert!(providers.contains(&Provider::Anthropic));
        assert!(providers.contains(&Provider::OpenRouter));
    }

    #[test]
    fn test_basic_template_channels() {
        let channels = ConfigTemplate::Basic.recommended_channels();
        assert_eq!(channels.len(), 1);
        assert_eq!(channels[0], Channel::WebUI);
    }

    #[test]
    fn test_production_template_channels() {
        let channels = ConfigTemplate::Production.recommended_channels();
        assert_eq!(channels.len(), 2);
        assert!(channels.contains(&Channel::WebUI));
        assert!(channels.contains(&Channel::Telegram));
    }

    #[test]
    fn test_enterprise_template_channels() {
        let channels = ConfigTemplate::Enterprise.recommended_channels();
        assert_eq!(channels.len(), 4);
        assert!(channels.contains(&Channel::WebUI));
        assert!(channels.contains(&Channel::Telegram));
        assert!(channels.contains(&Channel::Discord));
        assert!(channels.contains(&Channel::Slack));
    }

    #[test]
    fn test_custom_template_empty() {
        let providers = ConfigTemplate::Custom.recommended_providers();
        let channels = ConfigTemplate::Custom.recommended_channels();
        assert!(providers.is_empty());
        assert!(channels.is_empty());
    }
}
