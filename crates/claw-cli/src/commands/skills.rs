//! Skills subcommands implementation.

use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use clawmaster_clawhub::types::{PublishSkillRequest, SkillFormat, SkillMetadata, SkillSearchQuery, SortOrder, SecurityStatus};
use reqwest::Client;
use time::OffsetDateTime;

const API_BASE: &str = "https://api.clawhub.io";

#[derive(Subcommand)]
pub enum SkillsCommands {
    /// Search for skills
    Search {
        /// Search query
        query: String,

        /// Category filter
        #[arg(short, long)]
        category: Option<String>,
    },

    /// Show skill information
    Info {
        /// Skill name
        name: String,

        /// Skill version (default: latest)
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Publish a skill
    Publish {
        /// Skill name
        name: String,

        /// Version
        #[arg(short, long)]
        version: String,

        /// Description
        #[arg(short, long)]
        description: String,

        /// Author
        #[arg(short, long)]
        author: String,

        /// License (SPDX identifier)
        #[arg(short, long)]
        license: String,

        /// GitHub repository (owner/repo)
        #[arg(short, long)]
        github_repo: String,

        /// Keywords (comma-separated)
        #[arg(short, long)]
        keywords: Option<String>,

        /// Categories (comma-separated)
        #[arg(short = 'C', long)]
        categories: Option<String>,
    },

    /// List published skills
    List {
        /// Category filter
        #[arg(short, long)]
        category: Option<String>,
    },
}

pub async fn execute(cmd: SkillsCommands) -> Result<()> {
    match cmd {
        SkillsCommands::Search { query, category } => search(&query, category.as_deref()).await,
        SkillsCommands::Info { name, version } => info(&name, version.as_deref()).await,
        SkillsCommands::Publish {
            name,
            version,
            description,
            author,
            license,
            github_repo,
            keywords,
            categories,
        } => {
            publish(
                &name,
                &version,
                &description,
                &author,
                &license,
                &github_repo,
                keywords.as_deref(),
                categories.as_deref(),
            )
            .await
        }
        SkillsCommands::List { category } => list(category.as_deref()).await,
    }
}

async fn search(query: &str, category: Option<&str>) -> Result<()> {
    println!("{} {}", "Searching for skills:".bright_blue().bold(), query);

    let client = Client::new();

    let search_query = SkillSearchQuery {
        query: Some(query.to_string()),
        category: category.map(String::from),
        skill_format: None,
        security_status: None,
        sort: SortOrder::Downloads,
        page: 0,
        page_size: 20,
    };

    let response = client
        .get(format!("{}/skills/search", API_BASE))
        .query(&search_query)
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Search failed: {}", response.status());
    }

    let skills: Vec<SkillMetadata> = response.json().await?;

    if skills.is_empty() {
        println!("{}", "No skills found.".yellow());
        return Ok(());
    }

    println!("\n{} {} skills found:\n", "✓".green().bold(), skills.len());

    for skill in skills {
        println!("{} {}", "●".bright_cyan(), skill.name.bright_white().bold());
        println!("  {} {}", "Version:".dimmed(), skill.version);
        println!("  {} {}", "Description:".dimmed(), skill.description);
        println!("  {} {}", "Author:".dimmed(), skill.author);
        println!("  {} {}", "Downloads:".dimmed(), skill.downloads);
        println!("  {} {:?}", "Format:".dimmed(), skill.skill_format);
        if let Some(repo) = skill.github_repo {
            println!("  {} {}", "GitHub:".dimmed(), repo);
        }
        println!();
    }

    Ok(())
}

async fn info(name: &str, version: Option<&str>) -> Result<()> {
    let version = version.unwrap_or("latest");
    println!("{} {}@{}", "Getting skill info:".bright_blue().bold(), name, version);

    let client = Client::new();
    let response = client
        .get(format!("{}/skills/{}/{}", API_BASE, name, version))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Skill not found: {}@{}", name, version);
    }

    let skill: SkillMetadata = response.json().await?;

    println!("\n{}", skill.name.bright_white().bold());
    println!("{} {}", "Version:".dimmed(), skill.version);
    println!("{} {}", "Description:".dimmed(), skill.description);
    println!("{} {}", "Author:".dimmed(), skill.author);
    println!("{} {}", "License:".dimmed(), skill.license);
    println!("{} {:?}", "Format:".dimmed(), skill.skill_format);
    println!("{} {}", "Downloads:".dimmed(), skill.downloads);
    println!("{} {}", "Stars:".dimmed(), skill.stars);

    if let Some(repo) = skill.github_repo {
        println!("{} {}", "GitHub:".dimmed(), repo);
        println!("\n{}", "Install:".bright_green().bold());
        println!("  moltis skills install {}", repo);
    }

    if !skill.keywords.is_empty() {
        println!("{} {}", "Keywords:".dimmed(), skill.keywords.join(", "));
    }

    if !skill.categories.is_empty() {
        println!("{} {}", "Categories:".dimmed(), skill.categories.join(", "));
    }

    Ok(())
}

async fn publish(
    name: &str,
    version: &str,
    description: &str,
    author: &str,
    license: &str,
    github_repo: &str,
    keywords: Option<&str>,
    categories: Option<&str>,
) -> Result<()> {
    println!("{} {}@{}", "Publishing skill:".bright_blue().bold(), name, version);

    let keywords_vec = keywords
        .map(|k| k.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let categories_vec = categories
        .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let metadata = SkillMetadata {
        name: name.to_string(),
        version: version.to_string(),
        description: description.to_string(),
        readme: None,
        author: author.to_string(),
        author_email: None,
        license: license.to_string(),
        repository: Some(format!("https://github.com/{}", github_repo)),
        homepage: None,
        keywords: keywords_vec,
        categories: categories_vec,
        skill_format: SkillFormat::SkillMd,
        github_repo: Some(github_repo.to_string()),
        commit_sha: None,
        downloads: 0,
        stars: 0,
        security_status: SecurityStatus::Pending,
        published_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    };

    let request = PublishSkillRequest { metadata };

    let client = Client::new();
    let response = client
        .post(format!("{}/skills", API_BASE))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.text().await?;
        anyhow::bail!("Publish failed: {}", error);
    }

    let publish_response: clawmaster_clawhub::types::PublishSkillResponse = response.json().await?;

    println!(
        "{} {}",
        "✓".green().bold(),
        publish_response.message.bright_white()
    );
    println!(
        "{} {}",
        "Install command:".dimmed(),
        publish_response.install_command
    );

    Ok(())
}

async fn list(category: Option<&str>) -> Result<()> {
    println!("{}", "Listing skills...".bright_blue().bold());

    let client = Client::new();

    let query = SkillSearchQuery {
        query: None,
        category: category.map(String::from),
        skill_format: None,
        security_status: None,
        sort: SortOrder::Downloads,
        page: 0,
        page_size: 50,
    };

    let response = client
        .get(format!("{}/skills", API_BASE))
        .query(&query)
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to list skills: {}", response.status());
    }

    let skills: Vec<SkillMetadata> = response.json().await?;

    if skills.is_empty() {
        println!("{}", "No skills found.".yellow());
        return Ok(());
    }

    println!("\n{} {} skills:\n", "✓".green().bold(), skills.len());

    for skill in skills {
        println!("{} {} ({})", "●".bright_cyan(), skill.name.bright_white().bold(), skill.version);
        println!("  {}", skill.description.dimmed());
        println!();
    }

    Ok(())
}
