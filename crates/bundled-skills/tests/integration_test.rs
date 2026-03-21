//! Integration tests for bundled skills
//! DO-178C Level A compliant test suite

use {
    clawmaster_bundled_skills::{
        all_bundled_skills, get_skills_by_category, install_bundled_skills,
    },
    std::collections::HashSet,
    tempfile::TempDir,
};

#[test]
fn test_all_skills_count() {
    let skills = all_bundled_skills();
    assert_eq!(skills.len(), 53, "Must have exactly 53 bundled skills");
}

#[test]
fn test_no_duplicate_names() {
    let skills = all_bundled_skills();
    let mut names = HashSet::new();

    for skill in &skills {
        let name = &skill.metadata.name;
        assert!(
            names.insert(name.clone()),
            "Duplicate skill name found: {}",
            name
        );
    }
}

#[test]
fn test_all_skills_have_metadata() {
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

        // Verify SKILL.md format
        assert!(
            skill.body.contains("---"),
            "Skill body must contain frontmatter"
        );
        assert!(
            skill
                .body
                .contains(&format!("name: {}", skill.metadata.name)),
            "Skill body must contain name in frontmatter"
        );
    }
}

#[test]
fn test_category_notes() {
    let skills = get_skills_by_category("notes");
    assert_eq!(skills.len(), 4, "Notes category should have 4 skills");

    let names: Vec<_> = skills.iter().map(|s| s.metadata.name.as_str()).collect();
    assert!(names.contains(&"obsidian"));
    assert!(names.contains(&"notion"));
    assert!(names.contains(&"apple-notes"));
    assert!(names.contains(&"bear-notes"));
}

#[test]
fn test_category_productivity() {
    let skills = get_skills_by_category("productivity");
    assert_eq!(
        skills.len(),
        6,
        "Productivity category should have 6 skills"
    );

    let names: Vec<_> = skills.iter().map(|s| s.metadata.name.as_str()).collect();
    assert!(names.contains(&"gog"));
    assert!(names.contains(&"himalaya"));
    assert!(names.contains(&"things-mac"));
    assert!(names.contains(&"apple-reminders"));
    assert!(names.contains(&"trello"));
    assert!(names.contains(&"calendar"));
}

#[test]
fn test_category_messaging() {
    let skills = get_skills_by_category("messaging");
    assert_eq!(skills.len(), 5, "Messaging category should have 5 skills");

    let names: Vec<_> = skills.iter().map(|s| s.metadata.name.as_str()).collect();
    assert!(names.contains(&"wacli"));
    assert!(names.contains(&"imsg"));
    assert!(names.contains(&"bird"));
    assert!(names.contains(&"slack"));
    assert!(names.contains(&"discord"));
}

#[test]
fn test_category_developer() {
    let skills = get_skills_by_category("developer");
    assert_eq!(skills.len(), 4, "Developer category should have 4 skills");

    let names: Vec<_> = skills.iter().map(|s| s.metadata.name.as_str()).collect();
    assert!(names.contains(&"github"));
    assert!(names.contains(&"tmux"));
    assert!(names.contains(&"session-logs"));
    assert!(names.contains(&"coding-agent"));
}

#[test]
fn test_category_password() {
    let skills = get_skills_by_category("password");
    assert_eq!(skills.len(), 1, "Password category should have 1 skill");
    assert_eq!(skills[0].metadata.name, "1password");
}

#[test]
fn test_category_media() {
    let skills = get_skills_by_category("media");
    assert_eq!(skills.len(), 8, "Media category should have 8 skills");
}

#[test]
fn test_category_smart_home() {
    let skills = get_skills_by_category("smart_home");
    assert_eq!(skills.len(), 6, "Smart home category should have 6 skills");
}

#[test]
fn test_category_food() {
    let skills = get_skills_by_category("food");
    assert_eq!(skills.len(), 4, "Food category should have 4 skills");
}

#[test]
fn test_category_finance() {
    let skills = get_skills_by_category("finance");
    assert_eq!(skills.len(), 3, "Finance category should have 3 skills");
}

#[test]
fn test_category_health() {
    let skills = get_skills_by_category("health");
    assert_eq!(skills.len(), 4, "Health category should have 4 skills");
}

#[test]
fn test_category_travel() {
    let skills = get_skills_by_category("travel");
    assert_eq!(skills.len(), 3, "Travel category should have 3 skills");
}

#[test]
fn test_category_utilities() {
    let skills = get_skills_by_category("utilities");
    assert_eq!(skills.len(), 5, "Utilities category should have 5 skills");
}

#[test]
fn test_all_categories_sum_to_total() {
    let total = get_skills_by_category("notes").len()
        + get_skills_by_category("productivity").len()
        + get_skills_by_category("messaging").len()
        + get_skills_by_category("developer").len()
        + get_skills_by_category("password").len()
        + get_skills_by_category("media").len()
        + get_skills_by_category("smart_home").len()
        + get_skills_by_category("food").len()
        + get_skills_by_category("finance").len()
        + get_skills_by_category("health").len()
        + get_skills_by_category("travel").len()
        + get_skills_by_category("utilities").len();

    assert_eq!(total, 53, "All categories should sum to 53 skills");
}

#[test]
fn test_invalid_category_returns_empty() {
    let skills = get_skills_by_category("invalid_category");
    assert_eq!(skills.len(), 0, "Invalid category should return empty vec");
}

#[test]
fn test_all_skills_have_required_bins() {
    let skills = all_bundled_skills();

    for skill in &skills {
        // Most skills should have at least one required binary
        // Exception: session-logs doesn't require external binaries
        if skill.metadata.name != "session-logs" {
            assert!(
                !skill.metadata.requires.bins.is_empty(),
                "Skill {} should have required bins",
                skill.metadata.name
            );
        }
    }
}

#[test]
fn test_all_skills_have_allowed_tools() {
    let skills = all_bundled_skills();

    for skill in &skills {
        assert!(
            !skill.metadata.allowed_tools.is_empty(),
            "Skill {} should have allowed tools",
            skill.metadata.name
        );
    }
}

#[tokio::test]
async fn test_install_bundled_skills() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let install_dir = temp_dir.path();

    let result = install_bundled_skills(install_dir).await;
    assert!(result.is_ok(), "Installation should succeed");

    let count = result.unwrap();
    assert_eq!(count, 53, "Should install 53 skills");

    // Verify each skill directory was created
    let skills = all_bundled_skills();
    for skill in &skills {
        let skill_dir = install_dir.join(&skill.metadata.name);
        assert!(
            skill_dir.exists(),
            "Skill directory should exist: {:?}",
            skill_dir
        );

        let skill_md = skill_dir.join("SKILL.md");
        assert!(skill_md.exists(), "SKILL.md should exist: {:?}", skill_md);

        // Verify content
        let content = tokio::fs::read_to_string(&skill_md)
            .await
            .expect("Failed to read SKILL.md");
        assert!(!content.is_empty(), "SKILL.md should not be empty");
        assert!(content.contains("---"), "SKILL.md should have frontmatter");
    }
}

#[tokio::test]
async fn test_install_creates_directories() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let install_dir = temp_dir.path();

    install_bundled_skills(install_dir)
        .await
        .expect("Installation failed");

    // Check specific skills
    assert!(install_dir.join("obsidian").exists());
    assert!(install_dir.join("github").exists());
    assert!(install_dir.join("slack").exists());
    assert!(install_dir.join("1password").exists());
}

#[test]
fn test_skill_names_are_valid() {
    let skills = all_bundled_skills();

    for skill in &skills {
        let name = &skill.metadata.name;

        // Validate name format
        assert!(!name.is_empty(), "Name cannot be empty");
        assert!(name.len() <= 64, "Name must be <= 64 chars: {}", name);

        // Check characters
        for ch in name.chars() {
            assert!(
                ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-',
                "Invalid character in name '{}': '{}'",
                name,
                ch
            );
        }

        // No leading/trailing hyphens
        assert!(
            !name.starts_with('-'),
            "Name cannot start with hyphen: {}",
            name
        );
        assert!(
            !name.ends_with('-'),
            "Name cannot end with hyphen: {}",
            name
        );

        // No double hyphens
        assert!(
            !name.contains("--"),
            "Name cannot contain double hyphens: {}",
            name
        );
    }
}

#[test]
fn test_specific_skills_exist() {
    let skills = all_bundled_skills();
    let names: HashSet<_> = skills.iter().map(|s| s.metadata.name.as_str()).collect();

    // Test a sample from each category
    assert!(names.contains("obsidian"), "Should have obsidian skill");
    assert!(names.contains("github"), "Should have github skill");
    assert!(names.contains("slack"), "Should have slack skill");
    assert!(names.contains("1password"), "Should have 1password skill");
    assert!(names.contains("spotify"), "Should have spotify skill");
    assert!(names.contains("homekit"), "Should have homekit skill");
    assert!(names.contains("ubereats"), "Should have ubereats skill");
    assert!(names.contains("mint"), "Should have mint skill");
    assert!(names.contains("strava"), "Should have strava skill");
    assert!(names.contains("uber"), "Should have uber skill");
    assert!(names.contains("weather"), "Should have weather skill");
}

#[test]
fn test_skill_metadata_source() {
    let skills = all_bundled_skills();

    for skill in &skills {
        assert!(
            skill.metadata.source.is_some(),
            "Skill {} should have source set",
            skill.metadata.name
        );
    }
}

#[test]
fn test_skill_license() {
    let skills = all_bundled_skills();

    for skill in &skills {
        assert_eq!(
            skill.metadata.license.as_deref(),
            Some("MIT"),
            "Skill {} should have MIT license",
            skill.metadata.name
        );
    }
}
