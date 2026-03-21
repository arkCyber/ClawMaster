use {
    anyhow::{Context, Result},
    serde::{Deserialize, Serialize},
    std::path::{Path, PathBuf},
    tokio::fs,
    tracing::{debug, info},
};

const SOUL_FILE_NAME: &str = "SOUL.md";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Soul {
    path: PathBuf,
    #[serde(skip)]
    content: String,
    personality: PersonalityTraits,
    behavior: BehaviorRules,
    constraints: Constraints,
    custom_sections: Vec<CustomSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalityTraits {
    pub style: Vec<String>,
    pub tone: Vec<String>,
    pub expertise: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BehaviorRules {
    pub always_do: Vec<String>,
    pub never_do: Vec<String>,
    pub preferences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Constraints {
    pub safety: Vec<String>,
    pub privacy: Vec<String>,
    pub confirmation_required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSection {
    pub title: String,
    pub content: String,
}

impl Soul {
    pub async fn load() -> Result<Self> {
        let path = Self::get_soul_file_path()?;

        if !path.exists() {
            info!("SOUL.md not found, creating default file at {:?}", path);
            Self::create_default(&path).await?;
        }

        let content = fs::read_to_string(&path)
            .await
            .context("Failed to read SOUL.md")?;

        let soul = Self::parse(&content)?;

        Ok(Self {
            path,
            content,
            ..soul
        })
    }

    pub async fn reload(&mut self) -> Result<()> {
        self.content = fs::read_to_string(&self.path)
            .await
            .context("Failed to reload SOUL.md")?;

        let parsed = Self::parse(&self.content)?;
        self.personality = parsed.personality;
        self.behavior = parsed.behavior;
        self.constraints = parsed.constraints;
        self.custom_sections = parsed.custom_sections;

        debug!("Reloaded SOUL.md");
        Ok(())
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn personality(&self) -> &PersonalityTraits {
        &self.personality
    }

    pub fn behavior(&self) -> &BehaviorRules {
        &self.behavior
    }

    pub fn constraints(&self) -> &Constraints {
        &self.constraints
    }

    pub fn get_system_prompt(&self) -> String {
        let mut prompt = String::new();

        prompt.push_str("# AI Personality and Behavior\n\n");

        if !self.personality.style.is_empty() {
            prompt.push_str("## Style\n");
            for style in &self.personality.style {
                prompt.push_str(&format!("- {}\n", style));
            }
            prompt.push('\n');
        }

        if !self.personality.tone.is_empty() {
            prompt.push_str("## Tone\n");
            for tone in &self.personality.tone {
                prompt.push_str(&format!("- {}\n", tone));
            }
            prompt.push('\n');
        }

        if !self.personality.expertise.is_empty() {
            prompt.push_str("## Expertise\n");
            for expertise in &self.personality.expertise {
                prompt.push_str(&format!("- {}\n", expertise));
            }
            prompt.push('\n');
        }

        if !self.behavior.always_do.is_empty() {
            prompt.push_str("## Always Do\n");
            for rule in &self.behavior.always_do {
                prompt.push_str(&format!("- {}\n", rule));
            }
            prompt.push('\n');
        }

        if !self.behavior.never_do.is_empty() {
            prompt.push_str("## Never Do\n");
            for rule in &self.behavior.never_do {
                prompt.push_str(&format!("- {}\n", rule));
            }
            prompt.push('\n');
        }

        if !self.constraints.safety.is_empty() {
            prompt.push_str("## Safety Constraints\n");
            for constraint in &self.constraints.safety {
                prompt.push_str(&format!("- {}\n", constraint));
            }
            prompt.push('\n');
        }

        prompt
    }

    fn parse(content: &str) -> Result<Self> {
        let mut personality = PersonalityTraits::default();
        let mut behavior = BehaviorRules::default();
        let mut constraints = Constraints::default();
        let mut custom_sections = Vec::new();

        let mut current_section = String::new();
        let mut current_content = Vec::new();

        for line in content.lines() {
            if line.starts_with("## ") {
                if !current_section.is_empty() {
                    Self::process_section(
                        &current_section,
                        &current_content,
                        &mut personality,
                        &mut behavior,
                        &mut constraints,
                        &mut custom_sections,
                    );
                }
                current_section = line[3..].trim().to_string();
                current_content.clear();
            } else if line.starts_with("- ") {
                current_content.push(line[2..].trim().to_string());
            }
        }

        if !current_section.is_empty() {
            Self::process_section(
                &current_section,
                &current_content,
                &mut personality,
                &mut behavior,
                &mut constraints,
                &mut custom_sections,
            );
        }

        Ok(Self {
            path: PathBuf::new(),
            content: String::new(),
            personality,
            behavior,
            constraints,
            custom_sections,
        })
    }

    fn process_section(
        section: &str,
        content: &[String],
        personality: &mut PersonalityTraits,
        behavior: &mut BehaviorRules,
        constraints: &mut Constraints,
        custom_sections: &mut Vec<CustomSection>,
    ) {
        match section.to_lowercase().as_str() {
            "personality" | "style" => {
                personality.style.extend(content.iter().cloned());
            },
            "tone" => {
                personality.tone.extend(content.iter().cloned());
            },
            "expertise" | "专业领域" => {
                personality.expertise.extend(content.iter().cloned());
            },
            "behavior" | "always do" | "行为准则" => {
                behavior.always_do.extend(content.iter().cloned());
            },
            "never do" | "限制" => {
                behavior.never_do.extend(content.iter().cloned());
            },
            "preferences" | "偏好" => {
                behavior.preferences.extend(content.iter().cloned());
            },
            "safety" | "安全约束" => {
                constraints.safety.extend(content.iter().cloned());
            },
            "privacy" | "隐私" => {
                constraints.privacy.extend(content.iter().cloned());
            },
            "confirmation required" | "需要确认" => {
                constraints
                    .confirmation_required
                    .extend(content.iter().cloned());
            },
            _ => {
                custom_sections.push(CustomSection {
                    title: section.to_string(),
                    content: content.join("\n"),
                });
            },
        }
    }

    fn get_soul_file_path() -> Result<PathBuf> {
        let config_dir =
            clawmaster_config::config_dir().context("Failed to get config directory")?;
        Ok(config_dir.join(SOUL_FILE_NAME))
    }

    async fn create_default(path: &Path) -> Result<()> {
        let default_content = r#"# SOUL.md - AI Personality Configuration

This file defines the personality, behavior, and constraints for the AI assistant.

## Personality

- Professional yet approachable
- Helpful and proactive
- Clear and concise communication

## Tone

- Friendly but not overly casual
- Respectful and patient
- Encouraging and supportive

## Expertise

- Rust programming
- System architecture
- DevOps and deployment
- AI and machine learning

## Behavior

- Always provide code examples when relevant
- Explain technical decisions and trade-offs
- Suggest improvements and best practices
- Ask clarifying questions when needed

## Never Do

- Execute dangerous operations without confirmation
- Access or modify sensitive data without permission
- Make assumptions about user requirements
- Provide incomplete or untested solutions

## Safety

- Require confirmation for destructive operations
- Validate all inputs before processing
- Follow security best practices
- Respect user privacy

## Confirmation Required

- Deleting files or data
- Modifying system configurations
- Installing dependencies
- Making network requests to external services
"#;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(path, default_content)
            .await
            .context("Failed to create default SOUL.md")?;

        info!("Created default SOUL.md at {:?}", path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::TempDir};

    async fn create_test_soul() -> (Soul, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("SOUL.md");

        Soul::create_default(&path).await.unwrap();

        let soul = Soul {
            path: path.clone(),
            content: fs::read_to_string(&path).await.unwrap(),
            personality: PersonalityTraits::default(),
            behavior: BehaviorRules::default(),
            constraints: Constraints::default(),
            custom_sections: Vec::new(),
        };

        (soul, temp_dir)
    }

    #[tokio::test]
    async fn test_create_default() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("SOUL.md");

        Soul::create_default(&path).await.unwrap();

        assert!(path.exists());
        let content = fs::read_to_string(&path).await.unwrap();
        assert!(content.contains("# SOUL.md"));
        assert!(content.contains("## Personality"));
    }

    #[tokio::test]
    async fn test_parse() {
        let content = r#"# SOUL.md

## Personality
- Professional
- Helpful

## Tone
- Friendly
- Respectful

## Expertise
- Rust programming
- System design
"#;

        let soul = Soul::parse(content).unwrap();
        assert_eq!(soul.personality.style.len(), 2);
        assert_eq!(soul.personality.tone.len(), 2);
        assert_eq!(soul.personality.expertise.len(), 2);
    }

    #[tokio::test]
    async fn test_get_system_prompt() {
        let soul = Soul {
            path: PathBuf::new(),
            content: String::new(),
            personality: PersonalityTraits {
                style: vec!["Professional".to_string()],
                tone: vec!["Friendly".to_string()],
                expertise: vec!["Rust".to_string()],
            },
            behavior: BehaviorRules {
                always_do: vec!["Provide examples".to_string()],
                never_do: vec!["Make assumptions".to_string()],
                preferences: vec![],
            },
            constraints: Constraints {
                safety: vec!["Validate inputs".to_string()],
                privacy: vec![],
                confirmation_required: vec![],
            },
            custom_sections: vec![],
        };

        let prompt = soul.get_system_prompt();
        assert!(prompt.contains("Professional"));
        assert!(prompt.contains("Friendly"));
        assert!(prompt.contains("Rust"));
        assert!(prompt.contains("Provide examples"));
    }

    #[tokio::test]
    async fn test_reload() {
        let (mut soul, _temp_dir) = create_test_soul().await;

        let new_content = r#"# SOUL.md

## Personality
- Updated style
"#;

        fs::write(&soul.path, new_content).await.unwrap();
        soul.reload().await.unwrap();

        assert!(soul.content.contains("Updated style"));
    }
}
