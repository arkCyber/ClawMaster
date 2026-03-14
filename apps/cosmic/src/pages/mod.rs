//! 页面模块
//! 
//! 所有页面的统一入口

pub mod providers;
pub mod crons;
pub mod channels;
pub mod logs;
pub mod identity;
pub mod agents;
pub mod nodes;
pub mod environment;
pub mod memory;
pub mod notifications;
pub mod heartbeat;
pub mod projects;
pub mod mcp;
pub mod skills;

pub use providers::view_providers;
pub use crons::view_crons;
pub use channels::view_channels;
pub use logs::view_logs;
pub use identity::view_identity;
pub use agents::view_agents;
pub use nodes::view_nodes;
pub use environment::view_environment;
pub use memory::view_memory;
pub use notifications::view_notifications;
pub use heartbeat::view_heartbeat;
pub use projects::view_projects;
pub use mcp::view_mcp;
pub use skills::view_skills;

// 测试辅助函数（仅在测试时导出）
#[cfg(test)]
pub use providers::create_mock_providers;
#[cfg(test)]
pub use crons::create_mock_crons;
#[cfg(test)]
pub use channels::create_mock_channels;
#[cfg(test)]
pub use logs::create_mock_logs;
#[cfg(test)]
pub use identity::create_mock_identity;
#[cfg(test)]
pub use agents::create_mock_agents;
#[cfg(test)]
pub use nodes::create_mock_nodes;
#[cfg(test)]
pub use environment::create_mock_environment;
#[cfg(test)]
pub use memory::create_mock_memory;
#[cfg(test)]
pub use notifications::create_mock_notifications;
#[cfg(test)]
pub use heartbeat::create_mock_heartbeat;
#[cfg(test)]
pub use projects::create_mock_projects;
#[cfg(test)]
pub use mcp::create_mock_mcp;
#[cfg(test)]
pub use skills::create_mock_skills;
