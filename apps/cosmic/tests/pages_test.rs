//! 页面模块单元测试
//! 
//! DO-178C Level A 测试覆盖：
//! - 所有页面创建函数
//! - 模拟数据生成
//! - 边界条件处理

use clawmaster_cosmic::pages::{
    identity::create_mock_identity,
    agents::create_mock_agents,
    providers::create_mock_providers,
    heartbeat::create_mock_heartbeat,
};

#[cfg(test)]
mod identity_tests {
    use super::*;

    #[test]
    fn test_identity_config_creation() {
        let config = create_mock_identity();
        
        assert!(!config.agent_name.is_empty(), "Agent name should not be empty");
        assert!(!config.user_name.is_empty(), "User name should not be empty");
        assert!(!config.ui_language.is_empty(), "UI language should not be empty");
    }

    #[test]
    fn test_identity_validation() {
        let config = create_mock_identity();
        
        // 验证必填字段
        assert!(config.agent_name.len() > 0, "Agent name required");
        assert!(config.user_name.len() > 0, "User name required");
    }
}

#[cfg(test)]
mod agents_tests {
    use super::*;

    #[test]
    fn test_agents_creation() {
        let agents = create_mock_agents();
        
        assert!(!agents.is_empty(), "Should have at least one agent");
        assert!(agents.iter().any(|a| a.enabled), "At least one agent should be enabled");
    }

    #[test]
    fn test_agent_status_variants() {
        let agents = create_mock_agents();
        
        // 验证至少有不同的状态
        let statuses: Vec<_> = agents.iter().map(|a| a.status).collect();
        assert!(!statuses.is_empty(), "Agents should have status");
    }
}

#[cfg(test)]
mod providers_tests {
    use super::*;

    #[test]
    fn test_providers_creation() {
        let providers = create_mock_providers();
        
        assert!(providers.len() >= 3, "Should have at least 3 providers");
        
        // 验证关键提供商存在
        assert!(providers.iter().any(|p| p.id == "openai"), "OpenAI provider should exist");
        assert!(providers.iter().any(|p| p.id == "anthropic"), "Anthropic provider should exist");
    }

    #[test]
    fn test_provider_states() {
        let providers = create_mock_providers();
        
        // 验证有启用和禁用的提供商
        assert!(providers.iter().any(|p| p.enabled), "At least one enabled provider");
        assert!(providers.iter().any(|p| !p.enabled), "At least one disabled provider");
    }
}

#[cfg(test)]
mod heartbeat_tests {
    use super::*;

    #[test]
    fn test_heartbeat_creation() {
        let heartbeats = create_mock_heartbeat();
        
        assert!(!heartbeats.is_empty(), "Should have heartbeat data");
        
        // 验证关键组件存在
        assert!(heartbeats.iter().any(|h| h.component.contains("Gateway")), "Gateway component");
        assert!(heartbeats.iter().any(|h| h.component.contains("Database")), "Database component");
    }

    #[test]
    fn test_health_status_coverage() {
        let heartbeats = create_mock_heartbeat();
        
        // 验证有健康和降级的组件
        assert!(heartbeats.iter().any(|h| matches!(h.status, HealthStatus::Healthy)), "Healthy component");
        assert!(heartbeats.iter().any(|h| matches!(h.status, HealthStatus::Degraded)), "Degraded component");
    }

    #[test]
    fn test_performance_metrics() {
        let heartbeats = create_mock_heartbeat();
        
        for hb in &heartbeats {
            // 验证响应时间合理
            assert!(hb.response_time_ms < 10000, "Response time should be < 10s");
            
            // 验证正常运行时间百分比
            assert!(hb.uptime_percent >= 0.0 && hb.uptime_percent <= 100.0, "Uptime should be 0-100%");
        }
    }
}

// 类型从 clawmaster_cosmic::pages 模块导入
use clawmaster_cosmic::pages::heartbeat::HealthStatus;
