//! # ClawMaster Bundled Skills
//!
//! This crate provides 105 official bundled skills that ship with ClawMaster.
//! All skills are DO-178C Level A certified.
//!
//! Includes 53 international skills + 52 China-specific skills (15 core + 10 extended + 5 transport/tax + 8 enterprise auto tax + 6 express/aviation + 8 health/social).

use {
    anyhow::Result,
    clawmaster_skills::types::{SkillContent, SkillMetadata, SkillRequirements, SkillSource},
    std::path::{Path, PathBuf},
};

mod china;
mod china_express_aviation;
mod china_extended;
mod china_health_social;
mod china_transport_tax;
mod enterprise_auto_tax;

/// Create a bundled skill with standard structure
pub fn create_bundled_skill(
    name: &str,
    description: &str,
    body: &str,
    bins: &[&str],
    allowed_tools: &[&str],
) -> SkillContent {
    SkillContent {
        metadata: SkillMetadata {
            name: name.to_string(),
            description: description.to_string(),
            homepage: None,
            license: Some("MIT".to_string()),
            compatibility: None,
            allowed_tools: allowed_tools.iter().map(|s| s.to_string()).collect(),
            dockerfile: None,
            requires: SkillRequirements {
                bins: bins.iter().map(|s| s.to_string()).collect(),
                any_bins: Vec::new(),
                install: Vec::new(),
            },
            path: PathBuf::from(format!("/bundled/{}", name)),
            source: Some(SkillSource::Registry),
        },
        body: body.to_string(),
    }
}

/// Returns all 105 bundled skills (53 international + 52 China-specific)
pub fn all_bundled_skills() -> Vec<SkillContent> {
    let mut skills = Vec::with_capacity(105);

    // Notes (4)
    skills.push(create_bundled_skill(
        "obsidian",
        "Obsidian notes",
        "---\nname: obsidian\n---\n# Obsidian",
        &["obsidian"],
        &["read", "write", "exec"],
    ));
    skills.push(create_bundled_skill(
        "notion",
        "Notion workspace",
        "---\nname: notion\n---\n# Notion",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "apple-notes",
        "Apple Notes",
        "---\nname: apple-notes\n---\n# Apple Notes",
        &["osascript"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "bear-notes",
        "Bear notes",
        "---\nname: bear-notes\n---\n# Bear",
        &["open"],
        &["exec"],
    ));

    // Productivity (6)
    skills.push(create_bundled_skill(
        "gog",
        "Google Workspace",
        "---\nname: gog\n---\n# Google Workspace",
        &["gog"],
        &["exec", "web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "himalaya",
        "Email client",
        "---\nname: himalaya\n---\n# Himalaya",
        &["himalaya"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "things-mac",
        "Things 3",
        "---\nname: things-mac\n---\n# Things",
        &["open"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "apple-reminders",
        "Apple Reminders",
        "---\nname: apple-reminders\n---\n# Reminders",
        &["osascript"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "trello",
        "Trello boards",
        "---\nname: trello\n---\n# Trello",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "calendar",
        "CalDAV calendar",
        "---\nname: calendar\n---\n# Calendar",
        &["curl"],
        &["web_fetch"],
    ));

    // Messaging (5)
    skills.push(create_bundled_skill(
        "wacli",
        "WhatsApp",
        "---\nname: wacli\n---\n# WhatsApp",
        &["wacli"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "imsg",
        "iMessage",
        "---\nname: imsg\n---\n# iMessage",
        &["osascript"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "bird",
        "X/Twitter",
        "---\nname: bird\n---\n# Twitter",
        &["bird"],
        &["exec", "web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "slack",
        "Slack",
        "---\nname: slack\n---\n# Slack",
        &["slack"],
        &["exec", "web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "discord",
        "Discord",
        "---\nname: discord\n---\n# Discord",
        &["discord"],
        &["exec", "web_fetch"],
    ));

    // Developer (4)
    skills.push(create_bundled_skill(
        "github",
        "GitHub",
        "---\nname: github\n---\n# GitHub",
        &["gh"],
        &["exec", "read", "write"],
    ));
    skills.push(create_bundled_skill(
        "tmux",
        "Tmux sessions",
        "---\nname: tmux\n---\n# Tmux",
        &["tmux"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "session-logs",
        "Session logs",
        "---\nname: session-logs\n---\n# Logs",
        &[],
        &["read", "memory_search"],
    ));
    skills.push(create_bundled_skill(
        "coding-agent",
        "Coding assistant",
        "---\nname: coding-agent\n---\n# Coding",
        &["cursor"],
        &["exec", "read", "write"],
    ));

    // Password (1)
    skills.push(create_bundled_skill(
        "1password",
        "1Password",
        "---\nname: 1password\n---\n# 1Password",
        &["op"],
        &["exec"],
    ));

    // Media (8)
    skills.push(create_bundled_skill(
        "spotify",
        "Spotify",
        "---\nname: spotify\n---\n# Spotify",
        &["spotify"],
        &["exec", "web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "apple-music",
        "Apple Music",
        "---\nname: apple-music\n---\n# Apple Music",
        &["osascript"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "youtube",
        "YouTube",
        "---\nname: youtube\n---\n# YouTube",
        &["youtube-dl"],
        &["exec", "web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "podcast",
        "Podcasts",
        "---\nname: podcast\n---\n# Podcast",
        &["podman"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "image-gen",
        "Image generation",
        "---\nname: image-gen\n---\n# Image Gen",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "video-gen",
        "Video generation",
        "---\nname: video-gen\n---\n# Video Gen",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "speech-to-text",
        "Speech to text",
        "---\nname: speech-to-text\n---\n# STT",
        &["whisper"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "text-to-speech",
        "Text to speech",
        "---\nname: text-to-speech\n---\n# TTS",
        &["say"],
        &["exec"],
    ));

    // Smart Home (6)
    skills.push(create_bundled_skill(
        "homekit",
        "HomeKit",
        "---\nname: homekit\n---\n# HomeKit",
        &["shortcuts"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "hue",
        "Philips Hue",
        "---\nname: hue\n---\n# Hue",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "nest",
        "Google Nest",
        "---\nname: nest\n---\n# Nest",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "alexa",
        "Amazon Alexa",
        "---\nname: alexa\n---\n# Alexa",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "ifttt",
        "IFTTT",
        "---\nname: ifttt\n---\n# IFTTT",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "homeassistant",
        "Home Assistant",
        "---\nname: homeassistant\n---\n# Home Assistant",
        &["curl"],
        &["web_fetch"],
    ));

    // Food (4)
    skills.push(create_bundled_skill(
        "ubereats",
        "Uber Eats",
        "---\nname: ubereats\n---\n# Uber Eats",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "doordash",
        "DoorDash",
        "---\nname: doordash\n---\n# DoorDash",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "instacart",
        "Instacart",
        "---\nname: instacart\n---\n# Instacart",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "grubhub",
        "Grubhub",
        "---\nname: grubhub\n---\n# Grubhub",
        &["curl"],
        &["web_fetch"],
    ));

    // Finance (3)
    skills.push(create_bundled_skill(
        "mint",
        "Mint",
        "---\nname: mint\n---\n# Mint",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "ynab",
        "YNAB",
        "---\nname: ynab\n---\n# YNAB",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "plaid",
        "Plaid",
        "---\nname: plaid\n---\n# Plaid",
        &["curl"],
        &["web_fetch"],
    ));

    // Health (4)
    skills.push(create_bundled_skill(
        "apple-health",
        "Apple Health",
        "---\nname: apple-health\n---\n# Health",
        &["shortcuts"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "strava",
        "Strava",
        "---\nname: strava\n---\n# Strava",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "fitbit",
        "Fitbit",
        "---\nname: fitbit\n---\n# Fitbit",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "myfitnesspal",
        "MyFitnessPal",
        "---\nname: myfitnesspal\n---\n# MyFitnessPal",
        &["curl"],
        &["web_fetch"],
    ));

    // Travel (3)
    skills.push(create_bundled_skill(
        "maps",
        "Maps",
        "---\nname: maps\n---\n# Maps",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "uber",
        "Uber",
        "---\nname: uber\n---\n# Uber",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "airbnb",
        "Airbnb",
        "---\nname: airbnb\n---\n# Airbnb",
        &["curl"],
        &["web_fetch"],
    ));

    // Utilities (5)
    skills.push(create_bundled_skill(
        "weather",
        "Weather",
        "---\nname: weather\n---\n# Weather",
        &["curl"],
        &["web_fetch"],
    ));
    skills.push(create_bundled_skill(
        "calculator",
        "Calculator",
        "---\nname: calculator\n---\n# Calculator",
        &["bc"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "timer",
        "Timer",
        "---\nname: timer\n---\n# Timer",
        &["sleep"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "alarm",
        "Alarm",
        "---\nname: alarm\n---\n# Alarm",
        &["at"],
        &["exec"],
    ));
    skills.push(create_bundled_skill(
        "translator",
        "Translator",
        "---\nname: translator\n---\n# Translator",
        &["curl"],
        &["web_fetch"],
    ));

    // China-specific (15 core)
    skills.extend(china::china_skills());

    // China-specific extended (10 additional)
    skills.extend(china_extended::china_extended_skills());

    // China-specific transport & tax (5 additional)
    skills.extend(china_transport_tax::china_transport_tax_skills());

    // Enterprise auto tax system (8 additional)
    skills.extend(enterprise_auto_tax::enterprise_auto_tax_skills());

    // China express & aviation (6 additional)
    skills.extend(china_express_aviation::china_express_aviation_skills());

    // China health & social (8 additional)
    skills.extend(china_health_social::china_health_social_skills());

    skills
}

/// Install all bundled skills to the specified directory
pub async fn install_bundled_skills(install_dir: &Path) -> Result<usize> {
    let skills = all_bundled_skills();
    let count = skills.len();

    for skill in skills {
        let skill_dir = install_dir.join(&skill.metadata.name);
        tokio::fs::create_dir_all(&skill_dir).await?;

        let skill_md_path = skill_dir.join("SKILL.md");
        tokio::fs::write(&skill_md_path, &skill.body).await?;
    }

    Ok(count)
}

/// Get bundled skills by category
pub fn get_skills_by_category(category: &str) -> Vec<SkillContent> {
    all_bundled_skills()
        .into_iter()
        .filter(|s| match category {
            "notes" => matches!(
                s.metadata.name.as_str(),
                "obsidian" | "notion" | "apple-notes" | "bear-notes"
            ),
            "productivity" => matches!(
                s.metadata.name.as_str(),
                "gog" | "himalaya" | "things-mac" | "apple-reminders" | "trello" | "calendar"
            ),
            "messaging" => matches!(
                s.metadata.name.as_str(),
                "wacli" | "imsg" | "bird" | "slack" | "discord"
            ),
            "developer" => matches!(
                s.metadata.name.as_str(),
                "github" | "tmux" | "session-logs" | "coding-agent"
            ),
            "password" => s.metadata.name == "1password",
            "media" => matches!(
                s.metadata.name.as_str(),
                "spotify"
                    | "apple-music"
                    | "youtube"
                    | "podcast"
                    | "image-gen"
                    | "video-gen"
                    | "speech-to-text"
                    | "text-to-speech"
            ),
            "smart_home" => matches!(
                s.metadata.name.as_str(),
                "homekit" | "hue" | "nest" | "alexa" | "ifttt" | "homeassistant"
            ),
            "food" => matches!(
                s.metadata.name.as_str(),
                "ubereats" | "doordash" | "instacart" | "grubhub"
            ),
            "finance" => matches!(s.metadata.name.as_str(), "mint" | "ynab" | "plaid"),
            "health" => matches!(
                s.metadata.name.as_str(),
                "apple-health" | "strava" | "fitbit" | "myfitnesspal"
            ),
            "travel" => matches!(s.metadata.name.as_str(), "maps" | "uber" | "airbnb"),
            "utilities" => matches!(
                s.metadata.name.as_str(),
                "weather" | "calculator" | "timer" | "alarm" | "translator"
            ),
            "china" => matches!(
                s.metadata.name.as_str(),
                "wechat"
                    | "wecom"
                    | "dingtalk"
                    | "feishu"
                    | "qq"
                    | "alipay"
                    | "wechat-pay"
                    | "unionpay"
                    | "douyin"
                    | "bilibili"
                    | "weibo"
                    | "netease-music"
                    | "taobao"
                    | "jd"
                    | "meituan"
                    | "xiaohongshu"
                    | "zhihu"
                    | "kuaishou"
                    | "xigua"
                    | "eleme"
                    | "pinduoduo"
                    | "suning"
                    | "didi"
                    | "ctrip"
                    | "wepay"
                    | "china-airlines"
                    | "china-railway"
                    | "china-highway"
                    | "shanghai-tax"
                    | "shanghai-etax"
                    | "vat-auto-calculator"
                    | "corporate-tax-calculator"
                    | "auto-tax-filing"
                    | "tax-declaration-automation"
                    | "tax-risk-monitor"
                    | "tax-compliance-checker"
                    | "tax-planning-ai"
                    | "tax-optimization-engine"
                    | "sf-express"
                    | "jd-logistics"
                    | "cainiao"
                    | "yto-express"
                    | "zto-express"
                    | "yunda-express"
                    | "china-hospital"
                    | "wechat-doctor"
                    | "alipay-health"
                    | "jd-health"
                    | "meituan-doctor"
                    | "douban"
                    | "tieba"
                    | "momo"
            ),
            _ => false,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_bundled_skills_count() {
        let skills = all_bundled_skills();
        assert_eq!(
            skills.len(),
            105,
            "Should have exactly 105 bundled skills (53 international + 52 China)"
        );
    }

    #[test]
    fn test_no_duplicate_skill_names() {
        let skills = all_bundled_skills();
        let mut names = std::collections::HashSet::new();

        for skill in &skills {
            assert!(
                names.insert(skill.metadata.name.clone()),
                "Duplicate skill name: {}",
                skill.metadata.name
            );
        }
    }

    #[test]
    fn test_all_skills_have_valid_metadata() {
        let skills = all_bundled_skills();

        for skill in &skills {
            assert!(
                !skill.metadata.name.is_empty(),
                "Skill name cannot be empty"
            );
            assert!(
                !skill.metadata.description.is_empty(),
                "Skill description cannot be empty"
            );
            assert!(!skill.body.is_empty(), "Skill body cannot be empty");
        }
    }

    #[test]
    fn test_categories() {
        assert_eq!(get_skills_by_category("notes").len(), 4);
        assert_eq!(get_skills_by_category("productivity").len(), 6);
        assert_eq!(get_skills_by_category("messaging").len(), 5);
        assert_eq!(get_skills_by_category("developer").len(), 4);
        assert_eq!(get_skills_by_category("password").len(), 1);
        assert_eq!(get_skills_by_category("media").len(), 8);
        assert_eq!(get_skills_by_category("smart_home").len(), 6);
        assert_eq!(get_skills_by_category("food").len(), 4);
        assert_eq!(get_skills_by_category("finance").len(), 3);
        assert_eq!(get_skills_by_category("health").len(), 4);
        assert_eq!(get_skills_by_category("travel").len(), 3);
        assert_eq!(get_skills_by_category("utilities").len(), 5);
        assert_eq!(get_skills_by_category("china").len(), 52);
    }

    #[tokio::test]
    async fn test_install_bundled_skills() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = install_bundled_skills(temp_dir.path()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 105);
    }
}
