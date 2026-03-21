use {
    anyhow::{Context, Result},
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    std::path::{Path, PathBuf},
    tokio::fs,
    tracing::{debug, info},
};

const AGENTS_FILE_NAME: &str = "AGENTS.md";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsMemory {
    path: PathBuf,
    #[serde(skip)]
    content: String,
    last_modified: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub timestamp: DateTime<Utc>,
    pub category: MemoryCategory,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryCategory {
    UserPreference,
    ProjectContext,
    LearningRecord,
    ImportantDecision,
    ConversationSummary,
    Custom(String),
}

impl AgentsMemory {
    pub async fn load() -> Result<Self> {
        let path = Self::get_agents_file_path()?;

        if !path.exists() {
            info!("AGENTS.md not found, creating new file at {:?}", path);
            Self::create_default(&path).await?;
        }

        let content = fs::read_to_string(&path)
            .await
            .context("Failed to read AGENTS.md")?;

        let metadata = fs::metadata(&path).await?;
        let last_modified = metadata.modified().ok().and_then(|t| {
            DateTime::from_timestamp(
                t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                0,
            )
        });

        Ok(Self {
            path,
            content,
            last_modified,
        })
    }

    pub async fn reload(&mut self) -> Result<()> {
        self.content = fs::read_to_string(&self.path)
            .await
            .context("Failed to reload AGENTS.md")?;

        let metadata = fs::metadata(&self.path).await?;
        self.last_modified = metadata.modified().ok().and_then(|t| {
            DateTime::from_timestamp(
                t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                0,
            )
        });

        debug!("Reloaded AGENTS.md");
        Ok(())
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub async fn append_entry(&mut self, entry: MemoryEntry) -> Result<()> {
        let formatted_entry = self.format_entry(&entry);

        self.content.push_str("\n\n");
        self.content.push_str(&formatted_entry);

        fs::write(&self.path, &self.content)
            .await
            .context("Failed to write AGENTS.md")?;

        info!("Appended entry to AGENTS.md: {:?}", entry.category);
        Ok(())
    }

    pub async fn append_raw(&mut self, content: &str) -> Result<()> {
        self.content.push_str("\n\n");
        self.content.push_str(content);

        fs::write(&self.path, &self.content)
            .await
            .context("Failed to write AGENTS.md")?;

        debug!("Appended raw content to AGENTS.md");
        Ok(())
    }

    pub async fn update_section(&mut self, section_name: &str, new_content: &str) -> Result<()> {
        let section_marker = format!("## {}", section_name);

        if let Some(start_idx) = self.content.find(&section_marker) {
            let after_marker = &self.content[start_idx + section_marker.len()..];

            let end_idx = after_marker
                .find("\n## ")
                .map(|i| start_idx + section_marker.len() + i)
                .unwrap_or(self.content.len());

            let mut new_full_content = String::new();
            new_full_content.push_str(&self.content[..start_idx]);
            new_full_content.push_str(&section_marker);
            new_full_content.push('\n');
            new_full_content.push_str(new_content);
            new_full_content.push('\n');
            if end_idx < self.content.len() {
                new_full_content.push_str(&self.content[end_idx..]);
            }

            self.content = new_full_content;
        } else {
            self.content
                .push_str(&format!("\n\n## {}\n{}\n", section_name, new_content));
        }

        fs::write(&self.path, &self.content)
            .await
            .context("Failed to write AGENTS.md")?;

        info!("Updated section '{}' in AGENTS.md", section_name);
        Ok(())
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        self.content
            .lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .map(|s| s.to_string())
            .collect()
    }

    pub fn extract_section(&self, section_name: &str) -> Option<String> {
        let section_marker = format!("## {}", section_name);

        if let Some(start_idx) = self.content.find(&section_marker) {
            let after_marker = &self.content[start_idx + section_marker.len()..];

            let end_idx = after_marker.find("\n## ").unwrap_or(after_marker.len());

            Some(after_marker[..end_idx].trim().to_string())
        } else {
            None
        }
    }

    fn format_entry(&self, entry: &MemoryEntry) -> String {
        let category_section = match &entry.category {
            MemoryCategory::UserPreference => "User Preferences",
            MemoryCategory::ProjectContext => "Project Context",
            MemoryCategory::LearningRecord => "Learning Records",
            MemoryCategory::ImportantDecision => "Important Decisions",
            MemoryCategory::ConversationSummary => "Conversation Summaries",
            MemoryCategory::Custom(name) => name,
        };

        format!(
            "### {} - {}\n{}",
            entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            category_section,
            entry.content
        )
    }

    fn get_agents_file_path() -> Result<PathBuf> {
        let config_dir =
            clawmaster_config::config_dir().context("Failed to get config directory")?;
        Ok(config_dir.join(AGENTS_FILE_NAME))
    }

    async fn create_default(path: &Path) -> Result<()> {
        let default_content = r#"# AGENTS.md - Long-term Memory

This file stores the agent's long-term memory across all conversations.

## User Preferences

- Language: English
- Timezone: UTC

## Project Context

- Project: ClawMaster
- Tech Stack: Rust, Tokio, Axum
- Architecture: Modular crates

## Learning Records

## Important Decisions

## Conversation Summaries

## Custom Notes
"#;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(path, default_content)
            .await
            .context("Failed to create default AGENTS.md")?;

        info!("Created default AGENTS.md at {:?}", path);
        Ok(())
    }
}

impl MemoryEntry {
    pub fn new(category: MemoryCategory, content: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            category,
            content: content.into(),
        }
    }

    pub fn user_preference(content: impl Into<String>) -> Self {
        Self::new(MemoryCategory::UserPreference, content)
    }

    pub fn project_context(content: impl Into<String>) -> Self {
        Self::new(MemoryCategory::ProjectContext, content)
    }

    pub fn learning_record(content: impl Into<String>) -> Self {
        Self::new(MemoryCategory::LearningRecord, content)
    }

    pub fn important_decision(content: impl Into<String>) -> Self {
        Self::new(MemoryCategory::ImportantDecision, content)
    }

    pub fn conversation_summary(content: impl Into<String>) -> Self {
        Self::new(MemoryCategory::ConversationSummary, content)
    }

    pub fn custom(category_name: impl Into<String>, content: impl Into<String>) -> Self {
        Self::new(MemoryCategory::Custom(category_name.into()), content)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::TempDir};

    async fn create_test_memory() -> (AgentsMemory, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("AGENTS.md");

        AgentsMemory::create_default(&path).await.unwrap();

        let memory = AgentsMemory {
            path: path.clone(),
            content: fs::read_to_string(&path).await.unwrap(),
            last_modified: Some(Utc::now()),
        };

        (memory, temp_dir)
    }

    #[tokio::test]
    async fn test_create_default() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("AGENTS.md");

        AgentsMemory::create_default(&path).await.unwrap();

        assert!(path.exists());
        let content = fs::read_to_string(&path).await.unwrap();
        assert!(content.contains("# AGENTS.md"));
        assert!(content.contains("## User Preferences"));
    }

    #[tokio::test]
    async fn test_append_entry() {
        let (mut memory, _temp_dir) = create_test_memory().await;

        let entry = MemoryEntry::user_preference("Preferred language: Rust");
        memory.append_entry(entry).await.unwrap();

        assert!(memory.content.contains("Preferred language: Rust"));
        assert!(memory.content.contains("User Preferences"));
    }

    #[tokio::test]
    async fn test_update_section() {
        let (mut memory, _temp_dir) = create_test_memory().await;

        memory
            .update_section("User Preferences", "- Language: Chinese\n- Timezone: UTC+8")
            .await
            .unwrap();

        let section = memory.extract_section("User Preferences").unwrap();
        assert!(section.contains("Language: Chinese"));
        assert!(section.contains("Timezone: UTC+8"));
    }

    #[tokio::test]
    async fn test_search() {
        let (mut memory, _temp_dir) = create_test_memory().await;

        memory
            .append_raw("Important note about Rust performance")
            .await
            .unwrap();

        let results = memory.search("Rust");
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.contains("Rust")));
    }

    #[tokio::test]
    async fn test_extract_section() {
        let (memory, _temp_dir) = create_test_memory().await;

        let section = memory.extract_section("Project Context");
        assert!(section.is_some());
        assert!(section.unwrap().contains("ClawMaster"));
    }

    #[tokio::test]
    async fn test_reload() {
        let (mut memory, temp_dir) = create_test_memory().await;

        fs::write(&memory.path, "# Updated content\n\nNew data")
            .await
            .unwrap();

        memory.reload().await.unwrap();
        assert!(memory.content.contains("Updated content"));
    }

    #[tokio::test]
    async fn test_multiple_entries() {
        let (mut memory, _temp_dir) = create_test_memory().await;

        memory
            .append_entry(MemoryEntry::learning_record("Learned about async/await"))
            .await
            .unwrap();

        memory
            .append_entry(MemoryEntry::important_decision("Decided to use Tokio"))
            .await
            .unwrap();

        assert!(memory.content.contains("Learned about async/await"));
        assert!(memory.content.contains("Decided to use Tokio"));
    }
}
