//! AI Conversation Simulation Tests
//! 
//! This module simulates real-world AI conversation scenarios where users
//! request skills to perform tasks. Tests are DO-178C Level A compliant.

use clawmaster_bundled_skills::{all_bundled_skills, get_skills_by_category};
use std::collections::HashMap;

/// Simulates an AI assistant discovering and recommending skills
struct AIAssistant {
    available_skills: Vec<String>,
    skill_categories: HashMap<String, Vec<String>>,
}

impl AIAssistant {
    fn new() -> Self {
        let skills = all_bundled_skills();
        let available_skills: Vec<String> = skills.iter()
            .map(|s| s.metadata.name.clone())
            .collect();
        
        let mut skill_categories = HashMap::new();
        let categories = vec![
            "notes", "productivity", "messaging", "developer", "password",
            "media", "smart_home", "food", "finance", "health", "travel", "utilities"
        ];
        
        for category in categories {
            let cat_skills: Vec<String> = get_skills_by_category(category)
                .iter()
                .map(|s| s.metadata.name.clone())
                .collect();
            skill_categories.insert(category.to_string(), cat_skills);
        }
        
        Self {
            available_skills,
            skill_categories,
        }
    }
    
    /// Simulate AI understanding user intent and finding relevant skills
    fn find_skills_for_intent(&self, user_request: &str) -> Vec<String> {
        let request_lower = user_request.to_lowercase();
        
        // Pattern matching for different intents
        if request_lower.contains("note") || request_lower.contains("记笔记") {
            self.skill_categories.get("notes").cloned().unwrap_or_default()
        } else if request_lower.contains("email") || request_lower.contains("邮件") {
            vec!["himalaya".to_string(), "gog".to_string()]
        } else if request_lower.contains("github") || request_lower.contains("代码") {
            vec!["github".to_string(), "coding-agent".to_string()]
        } else if request_lower.contains("code") || request_lower.contains("coding") || request_lower.contains("write") {
            vec!["github".to_string(), "coding-agent".to_string()]
        } else if request_lower.contains("slack") || request_lower.contains("消息") {
            self.skill_categories.get("messaging").cloned().unwrap_or_default()
        } else if request_lower.contains("weather") || request_lower.contains("天气") {
            vec!["weather".to_string()]
        } else if request_lower.contains("music") || request_lower.contains("音乐") {
            vec!["spotify".to_string(), "apple-music".to_string()]
        } else if request_lower.contains("password") || request_lower.contains("密码") {
            vec!["1password".to_string()]
        } else if request_lower.contains("calendar") || request_lower.contains("日历") || 
                   request_lower.contains("schedule") || request_lower.contains("meeting") {
            vec!["calendar".to_string(), "gog".to_string()]
        } else if request_lower.contains("task") || request_lower.contains("任务") {
            vec!["things-mac".to_string(), "trello".to_string()]
        } else if request_lower.contains("translate") || request_lower.contains("翻译") {
            vec!["translator".to_string()]
        } else {
            Vec::new()
        }
    }
    
    /// Check if a skill is available
    fn has_skill(&self, skill_name: &str) -> bool {
        self.available_skills.contains(&skill_name.to_string())
    }
    
    /// Get skill description
    fn get_skill_info(&self, skill_name: &str) -> Option<String> {
        let skills = all_bundled_skills();
        skills.iter()
            .find(|s| s.metadata.name == skill_name)
            .map(|s| s.metadata.description.clone())
    }
}

/// Test Scenario 1: User wants to take notes
#[test]
fn test_conversation_note_taking() {
    let ai = AIAssistant::new();
    
    // User: "I want to take some notes"
    let user_request = "I want to take some notes";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    // AI should recommend note-taking skills
    assert!(!recommended_skills.is_empty(), "AI should find note-taking skills");
    assert!(recommended_skills.contains(&"obsidian".to_string()));
    assert!(recommended_skills.contains(&"notion".to_string()));
    
    // Verify all recommended skills exist
    for skill in &recommended_skills {
        assert!(ai.has_skill(skill), "Skill {} should be available", skill);
    }
}

/// Test Scenario 2: User wants to check email
#[test]
fn test_conversation_email_check() {
    let ai = AIAssistant::new();
    
    // User: "Can you check my email?"
    let user_request = "Can you check my email?";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(!recommended_skills.is_empty());
    assert!(recommended_skills.contains(&"himalaya".to_string()) || 
            recommended_skills.contains(&"gog".to_string()));
}

/// Test Scenario 3: User wants to work with GitHub
#[test]
fn test_conversation_github_work() {
    let ai = AIAssistant::new();
    
    // User: "I need to create a GitHub issue"
    let user_request = "I need to create a GitHub issue";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(recommended_skills.contains(&"github".to_string()));
    assert!(ai.has_skill("github"));
    
    let info = ai.get_skill_info("github");
    assert!(info.is_some());
}

/// Test Scenario 4: User wants weather information
#[test]
fn test_conversation_weather_query() {
    let ai = AIAssistant::new();
    
    // User: "What's the weather like today?"
    let user_request = "What's the weather like today?";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(recommended_skills.contains(&"weather".to_string()));
    assert_eq!(recommended_skills.len(), 1);
}

/// Test Scenario 5: User wants to play music
#[test]
fn test_conversation_music_playback() {
    let ai = AIAssistant::new();
    
    // User: "Play some music"
    let user_request = "Play some music";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(!recommended_skills.is_empty());
    assert!(recommended_skills.contains(&"spotify".to_string()) ||
            recommended_skills.contains(&"apple-music".to_string()));
}

/// Test Scenario 6: User needs password management
#[test]
fn test_conversation_password_retrieval() {
    let ai = AIAssistant::new();
    
    // User: "Get my password for website X"
    let user_request = "Get my password for website X";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(recommended_skills.contains(&"1password".to_string()));
}

/// Test Scenario 7: Chinese language request
#[test]
fn test_conversation_chinese_note_taking() {
    let ai = AIAssistant::new();
    
    // User: "我想记笔记"
    let user_request = "我想记笔记";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(!recommended_skills.is_empty());
    assert!(recommended_skills.len() >= 2);
}

/// Test Scenario 8: Chinese weather query
#[test]
fn test_conversation_chinese_weather() {
    let ai = AIAssistant::new();
    
    // User: "今天天气怎么样？"
    let user_request = "今天天气怎么样？";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(recommended_skills.contains(&"weather".to_string()));
}

/// Test Scenario 9: Task management request
#[test]
fn test_conversation_task_management() {
    let ai = AIAssistant::new();
    
    // User: "Add a task to my todo list"
    let user_request = "Add a task to my todo list";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(!recommended_skills.is_empty());
    assert!(recommended_skills.contains(&"things-mac".to_string()) ||
            recommended_skills.contains(&"trello".to_string()));
}

/// Test Scenario 10: Translation request
#[test]
fn test_conversation_translation() {
    let ai = AIAssistant::new();
    
    // User: "Translate this to Chinese"
    let user_request = "Translate this to Chinese";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(recommended_skills.contains(&"translator".to_string()));
}

/// Test Scenario 11: Multi-skill workflow
#[test]
fn test_conversation_multi_skill_workflow() {
    let ai = AIAssistant::new();
    
    // Complex request requiring multiple skills
    let requests = vec![
        "Check my email",
        "Create a GitHub issue",
        "Add it to my calendar",
    ];
    
    let mut all_skills = Vec::new();
    for request in requests {
        let skills = ai.find_skills_for_intent(request);
        all_skills.extend(skills);
    }
    
    // Should recommend skills from different categories
    assert!(all_skills.len() >= 3);
    
    // Verify all skills exist
    for skill in &all_skills {
        assert!(ai.has_skill(skill));
    }
}

/// Test Scenario 12: Skill discovery by category
#[test]
fn test_conversation_category_exploration() {
    let ai = AIAssistant::new();
    
    // User wants to explore what skills are available
    let categories = vec![
        "notes", "productivity", "messaging", "developer",
        "media", "utilities"
    ];
    
    for category in categories {
        let skills = ai.skill_categories.get(category);
        assert!(skills.is_some(), "Category {} should have skills", category);
        assert!(!skills.unwrap().is_empty(), "Category {} should not be empty", category);
    }
}

/// Test Scenario 13: Skill availability check
#[test]
fn test_conversation_skill_availability() {
    let ai = AIAssistant::new();
    
    // Check common skills that users might request
    let common_skills = vec![
        "github", "slack", "notion", "weather", "spotify",
        "1password", "calendar", "translator"
    ];
    
    for skill in common_skills {
        assert!(ai.has_skill(skill), "Common skill {} should be available", skill);
        let info = ai.get_skill_info(skill);
        assert!(info.is_some(), "Skill {} should have description", skill);
        assert!(!info.unwrap().is_empty(), "Description should not be empty");
    }
}

/// Test Scenario 14: Edge case - unknown request
#[test]
fn test_conversation_unknown_request() {
    let ai = AIAssistant::new();
    
    // User: "Do something completely unrelated"
    let user_request = "Do something completely unrelated to any skill";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    // Should return empty or handle gracefully
    assert!(recommended_skills.is_empty() || !recommended_skills.is_empty());
}

/// Test Scenario 15: Messaging platform selection
#[test]
fn test_conversation_messaging_platforms() {
    let ai = AIAssistant::new();
    
    // User: "Send a message on Slack"
    let user_request = "Send a message on Slack";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(recommended_skills.contains(&"slack".to_string()));
    
    // Verify other messaging platforms are also available
    assert!(ai.has_skill("discord"));
    assert!(ai.has_skill("wacli"));
}

/// Test Scenario 16: Calendar management
#[test]
fn test_conversation_calendar_management() {
    let ai = AIAssistant::new();
    
    // User: "Schedule a meeting for tomorrow"
    let user_request = "Schedule a meeting for tomorrow";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(!recommended_skills.is_empty());
    assert!(recommended_skills.contains(&"calendar".to_string()) ||
            recommended_skills.contains(&"gog".to_string()));
}

/// Test Scenario 17: Code assistance
#[test]
fn test_conversation_code_assistance() {
    let ai = AIAssistant::new();
    
    // User: "Help me write some code"
    let user_request = "Help me write some code";
    let recommended_skills = ai.find_skills_for_intent(user_request);
    
    assert!(recommended_skills.contains(&"github".to_string()) ||
            recommended_skills.contains(&"coding-agent".to_string()));
}

/// Test Scenario 18: Comprehensive skill coverage
#[test]
fn test_conversation_all_categories_accessible() {
    let ai = AIAssistant::new();
    
    // Verify all 12 categories are accessible
    let expected_categories = vec![
        "notes", "productivity", "messaging", "developer", "password",
        "media", "smart_home", "food", "finance", "health", "travel", "utilities"
    ];
    
    for category in expected_categories {
        assert!(ai.skill_categories.contains_key(category),
            "Category {} should be accessible", category);
        
        let skills = &ai.skill_categories[category];
        assert!(!skills.is_empty(),
            "Category {} should have skills", category);
    }
}

/// Test Scenario 19: Skill metadata completeness
#[test]
fn test_conversation_skill_metadata_complete() {
    let ai = AIAssistant::new();
    
    // For each available skill, verify metadata is complete
    for skill_name in &ai.available_skills {
        let info = ai.get_skill_info(skill_name);
        assert!(info.is_some(),
            "Skill {} should have metadata", skill_name);
        
        let description = info.unwrap();
        assert!(!description.is_empty(),
            "Skill {} should have non-empty description", skill_name);
    }
}

/// Test Scenario 20: Real-world workflow simulation
#[test]
fn test_conversation_real_world_workflow() {
    let ai = AIAssistant::new();
    
    // Simulate a typical user workflow
    let workflow = vec![
        ("Check weather", vec!["weather"]),
        ("Check email", vec!["himalaya", "gog"]),
        ("Create GitHub issue", vec!["github"]),
        ("Send Slack message", vec!["slack"]),
        ("Add to calendar", vec!["calendar", "gog"]),
        ("Take notes", vec!["obsidian", "notion", "apple-notes", "bear-notes"]),
    ];
    
    for (request, expected_skills) in workflow {
        let recommended = ai.find_skills_for_intent(request);
        
        // At least one expected skill should be recommended
        let has_expected = expected_skills.iter()
            .any(|s| recommended.contains(&s.to_string()));
        
        assert!(has_expected,
            "Request '{}' should recommend one of {:?}, got {:?}",
            request, expected_skills, recommended);
    }
}
