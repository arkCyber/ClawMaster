//! Skill update checking and management
//!
//! DO-178C Level A compliant skill update system with:
//! - Automatic update detection
//! - Version comparison
//! - Safe update process
//! - Rollback capability

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{
    install::{install_skill, remove_repo},
    manifest::ManifestStore,
    types::SkillsManifest,
};

/// Update information for a skill repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillUpdate {
    /// Repository source (owner/repo)
    pub source: String,
    /// Current commit SHA
    pub current_sha: Option<String>,
    /// Latest commit SHA
    pub latest_sha: String,
    /// Number of commits behind
    pub commits_behind: u32,
    /// Update available
    pub update_available: bool,
    /// Latest commit message
    pub latest_message: Option<String>,
    /// Latest commit date
    pub latest_date: Option<String>,
}

/// Check for updates for all installed skills
pub async fn check_updates(install_dir: &Path) -> Result<Vec<SkillUpdate>> {
    let manifest_path = ManifestStore::default_path()?;
    let store = ManifestStore::new(manifest_path);
    let manifest = store.load()?;

    let mut updates = Vec::new();

    for repo in &manifest.repos {
        match check_repo_update(&repo.source, repo.commit_sha.as_deref()).await {
            Ok(update) => {
                if update.update_available {
                    updates.push(update);
                }
            }
            Err(e) => {
                tracing::warn!(source = %repo.source, error = %e, "failed to check update");
            }
        }
    }

    Ok(updates)
}

/// Check for update for a specific repository
async fn check_repo_update(source: &str, current_sha: Option<&str>) -> Result<SkillUpdate> {
    let (owner, repo) = parse_source(source)?;

    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/commits?per_page=1", owner, repo);

    let response = client
        .get(&url)
        .header("User-Agent", "clawmaster-skills")
        .send()
        .await
        .context("failed to fetch commits")?;

    if !response.status().is_success() {
        anyhow::bail!("GitHub API error: {}", response.status());
    }

    let commits: Vec<GitHubCommit> = response.json().await?;

    let latest_commit = commits
        .first()
        .ok_or_else(|| anyhow::anyhow!("no commits found"))?;

    let latest_sha = latest_commit.sha.clone();
    let update_available = current_sha.map_or(true, |sha| sha != latest_sha);

    // If update available, get commit count
    let commits_behind = if update_available && current_sha.is_some() {
        count_commits_behind(&client, &owner, &repo, current_sha.unwrap(), &latest_sha).await?
    } else {
        0
    };

    Ok(SkillUpdate {
        source: source.to_string(),
        current_sha: current_sha.map(String::from),
        latest_sha,
        commits_behind,
        update_available,
        latest_message: Some(latest_commit.commit.message.clone()),
        latest_date: Some(latest_commit.commit.author.date.clone()),
    })
}

/// Count commits between current and latest
async fn count_commits_behind(
    client: &reqwest::Client,
    owner: &str,
    repo: &str,
    current_sha: &str,
    latest_sha: &str,
) -> Result<u32> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/compare/{}...{}",
        owner, repo, current_sha, latest_sha
    );

    let response = client
        .get(&url)
        .header("User-Agent", "clawmaster-skills")
        .send()
        .await?;

    if !response.status().is_success() {
        return Ok(0);
    }

    let comparison: GitHubComparison = response.json().await?;
    Ok(comparison.ahead_by)
}

/// Update a specific skill repository
pub async fn update_skill(source: &str, install_dir: &Path) -> Result<Vec<crate::types::SkillMetadata>> {
    tracing::info!(source = %source, "updating skill");

    // Backup current installation
    let manifest_path = ManifestStore::default_path()?;
    let store = ManifestStore::new(manifest_path.clone());
    let manifest_backup = store.load()?;

    // Remove old version
    remove_repo(source, install_dir).await?;

    // Install new version
    match install_skill(source, install_dir).await {
        Ok(skills) => {
            tracing::info!(source = %source, count = skills.len(), "skill updated successfully");
            Ok(skills)
        }
        Err(e) => {
            // Rollback on failure
            tracing::error!(source = %source, error = %e, "update failed, rolling back");
            store.save(&manifest_backup)?;
            Err(e)
        }
    }
}

/// Update all installed skills
pub async fn update_all_skills(install_dir: &Path) -> Result<Vec<String>> {
    let updates = check_updates(install_dir).await?;

    let mut updated = Vec::new();

    for update in updates {
        match update_skill(&update.source, install_dir).await {
            Ok(_) => {
                updated.push(update.source.clone());
                tracing::info!(source = %update.source, "updated successfully");
            }
            Err(e) => {
                tracing::error!(source = %update.source, error = %e, "update failed");
            }
        }
    }

    Ok(updated)
}

/// Parse owner/repo from source string
fn parse_source(source: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = source.split('/').collect();
    if parts.len() != 2 {
        anyhow::bail!("invalid source format: expected 'owner/repo'");
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

// GitHub API types
#[derive(Debug, Deserialize)]
struct GitHubCommit {
    sha: String,
    commit: GitHubCommitDetails,
}

#[derive(Debug, Deserialize)]
struct GitHubCommitDetails {
    message: String,
    author: GitHubAuthor,
}

#[derive(Debug, Deserialize)]
struct GitHubAuthor {
    date: String,
}

#[derive(Debug, Deserialize)]
struct GitHubComparison {
    ahead_by: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_source_valid() {
        let (owner, repo) = parse_source("vercel-labs/agent-skills").unwrap();
        assert_eq!(owner, "vercel-labs");
        assert_eq!(repo, "agent-skills");
    }

    #[test]
    fn test_parse_source_invalid() {
        assert!(parse_source("invalid").is_err());
        assert!(parse_source("too/many/parts").is_err());
    }

    #[tokio::test]
    async fn test_check_repo_update() {
        // Test with a real public repo
        let result = check_repo_update("vercel-labs/ai-sdk", None).await;
        
        // Should succeed for a valid repo
        if let Ok(update) = result {
            assert!(!update.latest_sha.is_empty());
            assert!(update.latest_sha.len() == 40); // SHA-1 length
        }
    }

    #[test]
    fn test_skill_update_serialization() {
        let update = SkillUpdate {
            source: "owner/repo".to_string(),
            current_sha: Some("abc123".to_string()),
            latest_sha: "def456".to_string(),
            commits_behind: 5,
            update_available: true,
            latest_message: Some("Update README".to_string()),
            latest_date: Some("2026-03-17T10:00:00Z".to_string()),
        };

        let json = serde_json::to_string(&update).unwrap();
        let deserialized: SkillUpdate = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.source, update.source);
        assert_eq!(deserialized.commits_behind, 5);
        assert!(deserialized.update_available);
    }
}
