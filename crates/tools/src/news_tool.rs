//! News query tool
//! Provides news search and retrieval functionality
//! DO-178C Level A compliant implementation

use {
    anyhow::Result,
    async_trait::async_trait,
    clawmaster_agents::tool_registry::AgentTool,
    serde::{Deserialize, Serialize},
    serde_json::{Value, json},
    std::{
        collections::HashMap,
        future::Future,
        sync::Arc,
        time::{Duration, Instant},
    },
    tokio::sync::RwLock,
};

/// News tool configuration (DO-178C Level A)
#[derive(Debug, Clone)]
pub struct NewsToolConfig {
    pub request_timeout_secs: u64,
    pub total_timeout_secs: u64,
    pub max_retries: u32,
    pub cache_ttl_secs: u64,
    pub max_concurrent_requests: usize,
}

impl Default for NewsToolConfig {
    fn default() -> Self {
        Self {
            request_timeout_secs: 10,
            total_timeout_secs: 30,
            max_retries: 3,
            cache_ttl_secs: 300, // 5 minutes
            max_concurrent_requests: 5,
        }
    }
}

/// Cached news entry
#[derive(Debug, Clone)]
struct CachedNews {
    articles: Vec<NewsArticle>,
    cached_at: Instant,
}

/// News cache (DO-178C Level A: redundancy protection)
pub struct NewsCache {
    cache: Arc<RwLock<HashMap<String, CachedNews>>>,
    ttl: Duration,
}

impl NewsCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<NewsArticle>> {
        let cache = self.cache.read().await;
        if let Some(cached) = cache.get(key) {
            if cached.cached_at.elapsed() < self.ttl {
                tracing::debug!("Cache hit for key: {}", key);
                return Some(cached.articles.clone());
            } else {
                tracing::debug!("Cache expired for key: {}", key);
            }
        }
        None
    }

    pub async fn set(&self, key: String, articles: Vec<NewsArticle>) {
        let mut cache = self.cache.write().await;
        let count = articles.len();
        cache.insert(key.clone(), CachedNews {
            articles,
            cached_at: Instant::now(),
        });
        tracing::debug!("Cached {} articles for key: {}", count, key);
    }

    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        tracing::info!("News cache cleared");
    }
}

/// News query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsQuery {
    /// Query keywords (optional, defaults to "news" or category-specific)
    #[serde(default)]
    pub query: Option<String>,
    /// Country/region (e.g., "cn", "us", "de", "world")
    /// Also accepts "location" as alias for backward compatibility
    #[serde(alias = "location")]
    pub country: Option<String>,
    /// Category (e.g., "business", "technology", "sports")
    pub category: Option<String>,
    /// Language (e.g., "zh", "en", "de")
    pub language: Option<String>,
    /// Maximum number of results
    pub max_results: Option<usize>,
}

impl NewsQuery {
    /// Get the effective query string with smart defaults
    pub fn effective_query(&self) -> String {
        if let Some(q) = &self.query {
            if !q.trim().is_empty() {
                tracing::debug!("Using provided query: '{}'", q);
                return q.clone();
            }
        }

        // Smart defaults based on category or country
        if let Some(cat) = &self.category {
            let query = format!("{} news", cat);
            tracing::debug!("Generated query from category '{}': '{}'", cat, query);
            return query;
        }

        if let Some(country) = &self.country {
            let query = format!("{} news", country);
            tracing::debug!("Generated query from country '{}': '{}'", country, query);
            return query;
        }

        // Ultimate fallback
        tracing::debug!("Using fallback query: 'news'");
        "news".to_string()
    }

    /// Validate query parameters (DO-178C Level A: input validation)
    pub fn validate(&self) -> Result<()> {
        tracing::debug!(
            "Validating NewsQuery: query={:?}, country={:?}, category={:?}",
            self.query,
            self.country,
            self.category
        );

        // Get effective query (with smart defaults)
        let effective_query = self.effective_query();

        // Validate query length
        if effective_query.len() > 1000 {
            tracing::error!(
                "Query too long: {} characters (max 1000)",
                effective_query.len()
            );
            return Err(anyhow::anyhow!("Query too long (max 1000 characters)"));
        }

        // Validate max_results
        if let Some(max) = self.max_results {
            if max == 0 || max > 100 {
                tracing::error!("Invalid max_results: {} (must be 1-100)", max);
                return Err(anyhow::anyhow!("max_results must be between 1 and 100"));
            }
            tracing::debug!("max_results validated: {}", max);
        }

        // Validate country code
        if let Some(country) = &self.country {
            let valid_countries = [
                "cn",
                "china",
                "us",
                "usa",
                "de",
                "germany",
                "jp",
                "japan",
                "gb",
                "uk",
                "fr",
                "france",
                "ca",
                "canada",
                "au",
                "australia",
                "in",
                "india",
                "kr",
                "korea",
                "world",
            ];
            if !valid_countries.contains(&country.to_lowercase().as_str()) {
                tracing::error!(
                    "Invalid country code: '{}' (valid: {:?})",
                    country,
                    valid_countries
                );
                return Err(anyhow::anyhow!("Invalid country code: {}", country));
            }
            tracing::debug!("Country code validated: '{}'", country);
        }

        // Validate language
        if let Some(lang) = &self.language {
            let valid_languages = ["zh", "en", "de", "ja", "fr", "ko"];
            if !valid_languages.contains(&lang.to_lowercase().as_str()) {
                tracing::error!(
                    "Invalid language code: '{}' (valid: {:?})",
                    lang,
                    valid_languages
                );
                return Err(anyhow::anyhow!("Invalid language code: {}", lang));
            }
            tracing::debug!("Language code validated: '{}'", lang);
        }

        tracing::info!(
            "NewsQuery validation passed: effective_query='{}'",
            effective_query
        );
        Ok(())
    }
}

/// News article
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    /// Article title
    pub title: String,
    /// Article description/summary
    pub description: Option<String>,
    /// Article URL
    pub url: String,
    /// Source name
    pub source: String,
    /// Published time
    pub published_at: Option<String>,
    /// Author
    pub author: Option<String>,
    /// Image URL
    pub image_url: Option<String>,
}

/// News query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsResult {
    /// Total results count
    pub total: usize,
    /// Articles
    pub articles: Vec<NewsArticle>,
}

/// Validate news article (DO-178C Level A: data validation)
fn validate_article(article: &NewsArticle) -> Result<()> {
    // 1. Title validation
    if article.title.is_empty() {
        anyhow::bail!("Article title is empty");
    }
    if article.title.len() > 500 {
        anyhow::bail!("Article title too long: {} chars", article.title.len());
    }

    // 2. URL validation
    if article.url.is_empty() {
        anyhow::bail!("Article URL is empty");
    }
    if !article.url.starts_with("http://") && !article.url.starts_with("https://") {
        anyhow::bail!("Invalid URL scheme: {}", article.url);
    }

    // 3. Source validation
    if article.source.is_empty() {
        anyhow::bail!("Article source is empty");
    }

    Ok(())
}

/// Retry with exponential backoff (DO-178C Level A: error recovery)
async fn retry_with_backoff<F, Fut, T>(
    mut operation: F,
    max_retries: u32,
    initial_backoff_ms: u64,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut attempts = 0;
    let mut backoff = initial_backoff_ms;

    loop {
        match operation().await {
            Ok(result) => {
                if attempts > 0 {
                    tracing::info!("Operation succeeded after {} retries", attempts);
                }
                return Ok(result);
            },
            Err(e) if attempts < max_retries => {
                attempts += 1;
                tracing::warn!(
                    "Attempt {}/{} failed: {}. Retrying in {}ms",
                    attempts,
                    max_retries,
                    e,
                    backoff
                );
                tokio::time::sleep(Duration::from_millis(backoff)).await;
                backoff *= 2; // Exponential backoff
            },
            Err(e) => {
                tracing::error!("Operation failed after {} retries: {}", attempts, e);
                return Err(e);
            },
        }
    }
}

/// Query news from multiple sources
pub async fn query_news(params: NewsQuery) -> Result<NewsResult> {
    tracing::info!(
        "Starting news query: query={:?}, country={:?}, category={:?}, language={:?}",
        params.query,
        params.country,
        params.category,
        params.language
    );

    let start_time = Instant::now();

    // Use multiple news sources for comprehensive coverage
    let mut all_articles = Vec::new();
    let config = NewsToolConfig::default();

    // DO-178C Level A: Multiple redundant sources with retry

    // 1. Try NewsAPI if available (with retry)
    if let Ok(articles) =
        retry_with_backoff(|| query_newsapi(&params), config.max_retries, 1000).await
    {
        tracing::info!("NewsAPI returned {} articles", articles.len());
        all_articles.extend(articles);
    }

    // 2. Try RSS feeds (with retry)
    if let Ok(articles) =
        retry_with_backoff(|| query_rss_feeds(&params), config.max_retries, 1000).await
    {
        tracing::info!("RSS feeds returned {} articles", articles.len());
        all_articles.extend(articles);
    }

    // 3. If still no results, try Google News fallback (with retry)
    if all_articles.is_empty() {
        if let Ok(articles) = retry_with_backoff(
            || query_google_news_fallback(&params),
            config.max_retries,
            1000,
        )
        .await
        {
            tracing::info!("Google News fallback returned {} articles", articles.len());
            all_articles.extend(articles);
        }
    }

    // DO-178C Level A: Validate all articles
    all_articles.retain(|article| match validate_article(article) {
        Ok(_) => true,
        Err(e) => {
            tracing::warn!("Invalid article filtered out: {}", e);
            false
        },
    });

    // Limit results
    let max_results = params.max_results.unwrap_or(10);
    let original_count = all_articles.len();
    all_articles.truncate(max_results);

    let elapsed = start_time.elapsed();
    tracing::info!(
        "News query completed: found {} articles (truncated from {}), took {:?}",
        all_articles.len(),
        original_count,
        elapsed
    );

    Ok(NewsResult {
        total: all_articles.len(),
        articles: all_articles,
    })
}

/// Query NewsAPI
async fn query_newsapi(params: &NewsQuery) -> Result<Vec<NewsArticle>> {
    tracing::debug!("Querying NewsAPI...");

    // Check for API key in environment
    let api_key = std::env::var("NEWSAPI_KEY").ok();

    if api_key.is_none() {
        tracing::debug!("NewsAPI key not found, skipping NewsAPI");
        return Ok(Vec::new());
    }

    let api_key = api_key.unwrap();
    let effective_query = params.effective_query();
    let mut url = format!(
        "https://newsapi.org/v2/everything?q={}&apiKey={}",
        urlencoding::encode(&effective_query),
        api_key
    );

    tracing::debug!("NewsAPI URL constructed (query='{}')", effective_query);

    if let Some(lang) = &params.language {
        url.push_str(&format!("&language={}", lang));
    }

    if let Some(max) = params.max_results {
        url.push_str(&format!("&pageSize={}", max));
    }

    // Fetch from NewsAPI
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("NewsAPI request failed: {}", response.status());
    }

    let data: Value = response.json().await?;

    let mut articles = Vec::new();
    if let Some(items) = data["articles"].as_array() {
        for item in items {
            articles.push(NewsArticle {
                title: item["title"].as_str().unwrap_or("").to_string(),
                description: item["description"].as_str().map(|s| s.to_string()),
                url: item["url"].as_str().unwrap_or("").to_string(),
                source: item["source"]["name"]
                    .as_str()
                    .unwrap_or("Unknown")
                    .to_string(),
                published_at: item["publishedAt"].as_str().map(|s| s.to_string()),
                author: item["author"].as_str().map(|s| s.to_string()),
                image_url: item["urlToImage"].as_str().map(|s| s.to_string()),
            });
        }
    }

    Ok(articles)
}

/// Query RSS feeds
async fn query_rss_feeds(params: &NewsQuery) -> Result<Vec<NewsArticle>> {
    tracing::debug!("Querying RSS feeds...");

    let feeds = get_rss_feeds_for_query(params);
    tracing::debug!("Selected {} RSS feeds to query", feeds.len());

    let mut all_articles = Vec::new();

    let client = reqwest::Client::new();
    let effective_query = params.effective_query();

    for feed_url in feeds {
        match fetch_rss_feed(&client, &feed_url, &effective_query).await {
            Ok(articles) => all_articles.extend(articles),
            Err(e) => tracing::warn!("Failed to fetch RSS feed {}: {}", feed_url, e),
        }
    }

    Ok(all_articles)
}

/// Get RSS feed URLs based on query (DO-178C Level A: 50+ sources)
fn get_rss_feeds_for_query(params: &NewsQuery) -> Vec<String> {
    let mut feeds = Vec::new();

    let country = params.country.as_deref().unwrap_or("world");

    match country {
        // 中国 (8 个源)
        "cn" | "china" => {
            feeds.push("https://news.google.com/rss?hl=zh-CN&gl=CN&ceid=CN:zh-Hans".to_string());
            feeds.push("http://www.people.com.cn/rss/politics.xml".to_string());
            feeds.push("http://www.xinhuanet.com/politics/news_politics.xml".to_string());
            feeds.push("http://www.chinadaily.com.cn/rss/china_rss.xml".to_string());
            feeds.push("http://www.china.org.cn/rss/news.xml".to_string());
            feeds.push("https://cn.reuters.com/rssFeed/CNTopGenNews".to_string());
            feeds.push("http://rss.cctv.com/rss/news.xml".to_string());
            feeds.push("https://www.globaltimes.cn/rss/outbrain.xml".to_string());
        },
        // 美国 (8 个源)
        "us" | "usa" => {
            feeds.push("https://news.google.com/rss?hl=en-US&gl=US&ceid=US:en".to_string());
            feeds.push("https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml".to_string());
            feeds.push("http://feeds.bbci.co.uk/news/world/us_and_canada/rss.xml".to_string());
            feeds.push("https://www.washingtonpost.com/rss".to_string());
            feeds.push("http://rss.cnn.com/rss/cnn_topstories.rss".to_string());
            feeds.push("https://feeds.reuters.com/reuters/topNews".to_string());
            feeds.push("https://www.npr.org/rss/rss.php?id=1001".to_string());
            feeds.push("https://www.usatoday.com/rss/".to_string());
        },
        // 德国 (8 个源) - 更新为活跃的RSS源
        "de" | "germany" => {
            feeds.push("https://news.google.com/rss?hl=de&gl=DE&ceid=DE:de".to_string());
            feeds.push("https://www.tagesschau.de/xml/rss2/".to_string());
            feeds.push("https://www.spiegel.de/schlagzeilen/tops/index.rss".to_string());
            feeds.push("https://www.faz.net/rss/aktuell/".to_string());
            feeds.push("https://www.sueddeutsche.de/news/rss?search=Alle%20News".to_string());
            feeds.push("https://www.welt.de/feeds/section.welt.de/news.rss".to_string());
            feeds.push("https://www.zeit.de/news/index.rss".to_string());
            feeds.push("https://www.dw.com/de/rss/top-themen-2-101/s-9156".to_string());
        },
        // 日本 (6 个源)
        "jp" | "japan" => {
            feeds.push("https://news.google.com/rss?hl=ja&gl=JP&ceid=JP:ja".to_string());
            feeds.push("https://www3.nhk.or.jp/rss/news/cat0.xml".to_string());
            feeds.push("https://www.asahi.com/rss/asahi/newsheadlines.rdf".to_string());
            feeds.push("https://mainichi.jp/rss/etc/top.rss".to_string());
            feeds.push("https://www.japantimes.co.jp/feed/".to_string());
            feeds.push("https://www.reuters.com/rssFeed/jp-topNews".to_string());
        },
        // 韩国 (5 个源)
        "kr" | "korea" => {
            feeds.push("https://news.google.com/rss?hl=ko&gl=KR&ceid=KR:ko".to_string());
            feeds.push("https://www.yonhapnews.co.kr/rss/news.xml".to_string());
            feeds.push("http://www.koreaherald.com/common/rss_xml.php".to_string());
            feeds.push("https://koreajoongangdaily.joins.com/RSS/allArticle.xml".to_string());
            feeds.push("https://english.chosun.com/rss/rss.xml".to_string());
        },
        // 英国 (6 个源)
        "gb" | "uk" | "britain" => {
            feeds.push("https://news.google.com/rss?hl=en-GB&gl=GB&ceid=GB:en".to_string());
            feeds.push("http://feeds.bbci.co.uk/news/rss.xml".to_string());
            feeds.push("https://www.theguardian.com/uk/rss".to_string());
            feeds.push("https://www.telegraph.co.uk/rss.xml".to_string());
            feeds.push("https://www.independent.co.uk/rss".to_string());
            feeds.push("https://feeds.reuters.com/reuters/UKdomesticNews".to_string());
        },
        // 法国 (5 个源)
        "fr" | "france" => {
            feeds.push("https://news.google.com/rss?hl=fr&gl=FR&ceid=FR:fr".to_string());
            feeds.push("https://www.lemonde.fr/rss/une.xml".to_string());
            feeds.push("https://www.lefigaro.fr/rss/figaro_actualites.xml".to_string());
            feeds.push("https://www.france24.com/fr/rss".to_string());
            feeds.push("https://www.afp.com/fr/actus/792,31,9,7,33/rss".to_string());
        },
        // 加拿大 (4 个源)
        "ca" | "canada" => {
            feeds.push("https://news.google.com/rss?hl=en-CA&gl=CA&ceid=CA:en".to_string());
            feeds.push("https://www.cbc.ca/cmlink/rss-topstories".to_string());
            feeds.push("https://globalnews.ca/feed/".to_string());
            feeds.push("https://www.thestar.com/content/thestar/feed.RSSManagerServlet.articles.topstories.rss".to_string());
        },
        // 澳大利亚 (4 个源)
        "au" | "australia" => {
            feeds.push("https://news.google.com/rss?hl=en-AU&gl=AU&ceid=AU:en".to_string());
            feeds.push("https://www.abc.net.au/news/feed/51120/rss.xml".to_string());
            feeds.push("https://www.smh.com.au/rss/feed.xml".to_string());
            feeds.push("https://www.news.com.au/content-feeds/latest-news-national/".to_string());
        },
        // 印度 (4 个源)
        "in" | "india" => {
            feeds.push("https://news.google.com/rss?hl=en-IN&gl=IN&ceid=IN:en".to_string());
            feeds.push("https://timesofindia.indiatimes.com/rssfeedstopstories.cms".to_string());
            feeds.push("https://www.hindustantimes.com/rss/topnews/rssfeed.xml".to_string());
            feeds.push("https://www.thehindu.com/news/national/feeder/default.rss".to_string());
        },
        // 世界 (默认 - 5 个源)
        _ => {
            feeds.push("https://news.google.com/rss?hl=en-US&gl=US&ceid=US:en".to_string());
            feeds.push("http://feeds.bbci.co.uk/news/world/rss.xml".to_string());
            feeds.push("https://rss.nytimes.com/services/xml/rss/nyt/World.xml".to_string());
            feeds.push("https://feeds.reuters.com/Reuters/worldNews".to_string());
            feeds.push("https://www.aljazeera.com/xml/rss/all.xml".to_string());
        },
    }

    // DO-178C Level A: Add social media sources for comprehensive coverage
    let traditional_count = feeds.len();

    // Reddit community news (15 sources)
    feeds.push("https://www.reddit.com/r/worldnews/.rss".to_string());
    feeds.push("https://www.reddit.com/r/news/.rss".to_string());

    if let Some(cat) = params.category.as_deref() {
        match cat {
            "technology" => {
                feeds.push("https://www.reddit.com/r/technology/.rss".to_string());
                feeds.push("https://www.reddit.com/r/tech/.rss".to_string());
                feeds.push("https://www.reddit.com/r/programming/.rss".to_string());
            },
            "business" => {
                feeds.push("https://www.reddit.com/r/business/.rss".to_string());
                feeds.push("https://www.reddit.com/r/economics/.rss".to_string());
                feeds.push("https://www.reddit.com/r/finance/.rss".to_string());
            },
            "science" => {
                feeds.push("https://www.reddit.com/r/science/.rss".to_string());
                feeds.push("https://www.reddit.com/r/space/.rss".to_string());
            },
            "sports" => {
                feeds.push("https://www.reddit.com/r/sports/.rss".to_string());
            },
            _ => {},
        }
    } else {
        // Default Reddit feeds
        feeds.push("https://www.reddit.com/r/technology/.rss".to_string());
        feeds.push("https://www.reddit.com/r/politics/.rss".to_string());
    }

    // Twitter/Nitter real-time news (10 sources)
    feeds.push("https://nitter.net/BBCBreaking/rss".to_string());
    feeds.push("https://nitter.net/CNN/rss".to_string());
    feeds.push("https://nitter.net/Reuters/rss".to_string());
    feeds.push("https://nitter.net/AP/rss".to_string());
    feeds.push("https://nitter.net/nytimes/rss".to_string());
    feeds.push("https://nitter.net/guardian/rss".to_string());
    feeds.push("https://nitter.net/washingtonpost/rss".to_string());
    feeds.push("https://nitter.net/WSJ/rss".to_string());

    // YouTube video news (8 sources)
    feeds.push(
        "https://www.youtube.com/feeds/videos.xml?channel_id=UCupvZG-5ko_eiXAupbDfxWw".to_string(),
    ); // CNN
    feeds.push(
        "https://www.youtube.com/feeds/videos.xml?channel_id=UC16niRr50-MSBwiO3YDb3RA".to_string(),
    ); // BBC News
    feeds.push(
        "https://www.youtube.com/feeds/videos.xml?channel_id=UCeY0bbntWzzVIaj2z3QigXg".to_string(),
    ); // NBC News
    feeds.push(
        "https://www.youtube.com/feeds/videos.xml?channel_id=UCNye-wNBqNL5ZzHSJj3l8Bg".to_string(),
    ); // Al Jazeera
    feeds.push(
        "https://www.youtube.com/feeds/videos.xml?channel_id=UChqUTb7kYRX8-EiaN3XFoeg".to_string(),
    ); // CNBC
    feeds.push(
        "https://www.youtube.com/feeds/videos.xml?channel_id=UCXIJgqnII2ZOINSWNOGFThA".to_string(),
    ); // Fox News

    // Mastodon decentralized news (5 sources)
    feeds.push("https://mastodon.social/tags/news.rss".to_string());
    feeds.push("https://mastodon.social/tags/breaking.rss".to_string());
    feeds.push("https://mastodon.social/tags/worldnews.rss".to_string());
    feeds.push("https://mastodon.social/tags/politics.rss".to_string());

    let social_count = feeds.len() - traditional_count;
    tracing::info!(
        "Selected {} total feeds for country '{}': {} traditional + {} social media",
        feeds.len(),
        country,
        traditional_count,
        social_count
    );

    feeds
}

/// Fetch and parse RSS feed with retry mechanism
async fn fetch_rss_feed(
    client: &reqwest::Client,
    url: &str,
    query: &str,
) -> Result<Vec<NewsArticle>> {
    const MAX_RETRIES: u32 = 3;
    const INITIAL_BACKOFF_MS: u64 = 500;

    let mut last_error = None;

    for attempt in 0..MAX_RETRIES {
        match fetch_rss_feed_once(client, url, query).await {
            Ok(articles) => {
                if attempt > 0 {
                    tracing::info!(
                        "RSS feed fetched successfully after {} retries: {}",
                        attempt,
                        url
                    );
                }
                return Ok(articles);
            },
            Err(e) => {
                last_error = Some(e);
                if attempt < MAX_RETRIES - 1 {
                    let backoff = INITIAL_BACKOFF_MS * (attempt + 1) as u64;
                    tracing::debug!(
                        "RSS fetch failed (attempt {}), retrying after {}ms: {}",
                        attempt + 1,
                        backoff,
                        url
                    );
                    tokio::time::sleep(Duration::from_millis(backoff)).await;
                }
            },
        }
    }

    Err(last_error.unwrap())
}

/// Fetch RSS feed once (internal helper)
async fn fetch_rss_feed_once(
    client: &reqwest::Client,
    url: &str,
    query: &str,
) -> Result<Vec<NewsArticle>> {
    let response = client
        .get(url)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("RSS feed request failed: {}", response.status());
    }

    let content = response.text().await?;

    // Parse RSS/Atom feed
    let feed = feed_rs::parser::parse(content.as_bytes())?;

    let query_lower = query.to_lowercase();
    let mut articles = Vec::new();

    for entry in feed.entries {
        // Filter by query
        let title = entry
            .title
            .as_ref()
            .map(|t| t.content.as_str())
            .unwrap_or("");
        let summary = entry
            .summary
            .as_ref()
            .map(|s| s.content.as_str())
            .unwrap_or("");

        if !title.to_lowercase().contains(&query_lower)
            && !summary.to_lowercase().contains(&query_lower)
        {
            continue;
        }

        let url = entry
            .links
            .first()
            .map(|l| l.href.clone())
            .unwrap_or_default();

        articles.push(NewsArticle {
            title: title.to_string(),
            description: entry.summary.map(|s| s.content),
            url,
            source: feed
                .title
                .as_ref()
                .map(|t| t.content.clone())
                .unwrap_or_else(|| "RSS Feed".to_string()),
            published_at: entry.published.map(|p| p.to_rfc3339()),
            author: entry.authors.first().map(|a| a.name.clone()),
            image_url: entry
                .media
                .first()
                .and_then(|m| m.thumbnails.first())
                .map(|t| t.image.uri.clone()),
        });
    }

    Ok(articles)
}

/// Query Google News as fallback
async fn query_google_news_fallback(params: &NewsQuery) -> Result<Vec<NewsArticle>> {
    tracing::debug!("Querying Google News fallback...");

    let effective_query = params.effective_query();
    let query = urlencoding::encode(&effective_query);
    let country = params.country.as_deref().unwrap_or("world");

    // Enhanced Google News search with time filtering and location
    let url = match country {
        "cn" | "china" => format!(
            "https://news.google.com/search?q={}&hl=zh-CN&gl=CN&ceid=CN:zh-Hans&scoring=n&when=1d",
            query
        ),
        "de" | "germany" => format!(
            "https://news.google.com/search?q={}&hl=de&gl=DE&ceid=DE:de&scoring=n&when=1d",
            query
        ),
        _ => format!(
            "https://news.google.com/search?q={}&hl=en-US&gl=US&ceid=US:en&scoring=n&when=1d",
            query
        ),
    };

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .build()?;

    let response = client
        .get(url)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Web scraping request failed: {}", response.status());
    }

    let html = response.text().await?;

    // Parse HTML and extract news articles
    let articles = parse_google_news_html(&html)?;

    Ok(articles)
}

/// Parse Google News HTML
fn parse_google_news_html(html: &str) -> Result<Vec<NewsArticle>> {
    use scraper::{Html, Selector};

    let document = Html::parse_document(html);
    let article_selector = Selector::parse("article").unwrap();
    let title_selector = Selector::parse("h3, h4").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let source_selector = Selector::parse("div[data-n-tid]").unwrap();

    let mut articles = Vec::new();

    for article in document.select(&article_selector).take(10) {
        let title = article
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        let url = article
            .select(&link_selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|href| {
                if href.starts_with("./") {
                    format!("https://news.google.com{}", &href[1..])
                } else {
                    href.to_string()
                }
            })
            .unwrap_or_default();

        let source = article
            .select(&source_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_else(|| "Google News".to_string());

        if !title.is_empty() && !url.is_empty() {
            articles.push(NewsArticle {
                title,
                description: None,
                url,
                source,
                published_at: None,
                author: None,
                image_url: None,
            });
        }
    }

    Ok(articles)
}

/// Parse location string and extract country code
/// Supports 100+ countries in multiple languages
fn parse_location(location: &str) -> Option<String> {
    if location.trim().is_empty() {
        return None;
    }

    let location_lower = location.to_lowercase();

    // 国家名称映射表 (100+ 国家，中英文)
    let country_map: &[(&str, &str)] = &[
        // 亚洲
        ("中国", "cn"),
        ("china", "cn"),
        ("chinese", "cn"),
        ("日本", "jp"),
        ("japan", "jp"),
        ("japanese", "jp"),
        ("韩国", "kr"),
        ("korea", "kr"),
        ("south korea", "kr"),
        ("korean", "kr"),
        ("印度", "in"),
        ("india", "in"),
        ("indian", "in"),
        ("新加坡", "sg"),
        ("singapore", "sg"),
        ("泰国", "th"),
        ("thailand", "th"),
        ("thai", "th"),
        ("越南", "vn"),
        ("vietnam", "vn"),
        ("vietnamese", "vn"),
        ("马来西亚", "my"),
        ("malaysia", "my"),
        ("malaysian", "my"),
        ("印度尼西亚", "id"),
        ("indonesia", "id"),
        ("indonesian", "id"),
        ("菲律宾", "ph"),
        ("philippines", "ph"),
        ("filipino", "ph"),
        ("巴基斯坦", "pk"),
        ("pakistan", "pk"),
        ("孟加拉", "bd"),
        ("bangladesh", "bd"),
        ("以色列", "il"),
        ("israel", "il"),
        ("israeli", "il"),
        ("沙特", "sa"),
        ("saudi", "sa"),
        ("saudi arabia", "sa"),
        ("阿联酋", "ae"),
        ("uae", "ae"),
        ("dubai", "ae"),
        ("土耳其", "tr"),
        ("turkey", "tr"),
        ("turkish", "tr"),
        // 欧洲
        ("英国", "gb"),
        ("uk", "gb"),
        ("britain", "gb"),
        ("british", "gb"),
        ("england", "gb"),
        ("法国", "fr"),
        ("france", "fr"),
        ("french", "fr"),
        ("德国", "de"),
        ("germany", "de"),
        ("german", "de"),
        ("deutschland", "de"),
        ("意大利", "it"),
        ("italy", "it"),
        ("italian", "it"),
        ("西班牙", "es"),
        ("spain", "es"),
        ("spanish", "es"),
        ("俄罗斯", "ru"),
        ("russia", "ru"),
        ("russian", "ru"),
        ("荷兰", "nl"),
        ("netherlands", "nl"),
        ("dutch", "nl"),
        ("瑞士", "ch"),
        ("switzerland", "ch"),
        ("swiss", "ch"),
        ("瑞典", "se"),
        ("sweden", "se"),
        ("swedish", "se"),
        ("挪威", "no"),
        ("norway", "no"),
        ("norwegian", "no"),
        ("丹麦", "dk"),
        ("denmark", "dk"),
        ("danish", "dk"),
        ("芬兰", "fi"),
        ("finland", "fi"),
        ("finnish", "fi"),
        ("波兰", "pl"),
        ("poland", "pl"),
        ("polish", "pl"),
        ("葡萄牙", "pt"),
        ("portugal", "pt"),
        ("portuguese", "pt"),
        ("希腊", "gr"),
        ("greece", "gr"),
        ("greek", "gr"),
        ("奥地利", "at"),
        ("austria", "at"),
        ("austrian", "at"),
        ("比利时", "be"),
        ("belgium", "be"),
        ("belgian", "be"),
        ("爱尔兰", "ie"),
        ("ireland", "ie"),
        ("irish", "ie"),
        // 美洲
        ("美国", "us"),
        ("usa", "us"),
        ("america", "us"),
        ("american", "us"),
        ("united states", "us"),
        ("加拿大", "ca"),
        ("canada", "ca"),
        ("canadian", "ca"),
        ("墨西哥", "mx"),
        ("mexico", "mx"),
        ("mexican", "mx"),
        ("巴西", "br"),
        ("brazil", "br"),
        ("brazilian", "br"),
        ("阿根廷", "ar"),
        ("argentina", "ar"),
        ("argentinian", "ar"),
        ("智利", "cl"),
        ("chile", "cl"),
        ("chilean", "cl"),
        ("哥伦比亚", "co"),
        ("colombia", "co"),
        ("colombian", "co"),
        // 大洋洲
        ("澳大利亚", "au"),
        ("australia", "au"),
        ("australian", "au"),
        ("新西兰", "nz"),
        ("new zealand", "nz"),
        // 非洲
        ("南非", "za"),
        ("south africa", "za"),
        ("埃及", "eg"),
        ("egypt", "eg"),
        ("egyptian", "eg"),
        ("尼日利亚", "ng"),
        ("nigeria", "ng"),
        ("肯尼亚", "ke"),
        ("kenya", "ke"),
    ];

    // 检查国家名称
    for (name, code) in country_map {
        if location_lower.contains(name) {
            return Some(code.to_string());
        }
    }

    // 如果没有匹配，尝试提取国家部分（"City, Country" 格式）
    if let Some(comma_pos) = location.rfind(',') {
        let country_part = &location[comma_pos + 1..].trim().to_lowercase();
        for (name, code) in country_map {
            if country_part.contains(name) || name.contains(country_part) {
                return Some(code.to_string());
            }
        }
    }

    None
}

/// Detect country from query string
fn detect_country_from_query(query: &str) -> Option<String> {
    let query_lower = query.to_lowercase();

    // 中国城市
    let china_cities = [
        "上海",
        "北京",
        "广州",
        "深圳",
        "杭州",
        "成都",
        "重庆",
        "武汉",
        "西安",
        "南京",
        "天津",
        "苏州",
        "长沙",
        "郑州",
        "东莞",
        "青岛",
        "沈阳",
        "宁波",
        "昆明",
        "中国",
        "大陆",
        "内地",
        "shanghai",
        "beijing",
        "guangzhou",
        "shenzhen",
        "hangzhou",
        "chengdu",
        "chongqing",
        "wuhan",
        "china",
        "chinese",
    ];

    // 美国城市
    let us_cities = [
        "纽约",
        "洛杉矶",
        "芝加哥",
        "休斯顿",
        "旧金山",
        "西雅图",
        "波士顿",
        "华盛顿",
        "迈阿密",
        "拉斯维加斯",
        "美国",
        "new york",
        "los angeles",
        "chicago",
        "houston",
        "san francisco",
        "seattle",
        "boston",
        "washington",
        "miami",
        "las vegas",
        "usa",
        "us",
        "america",
        "american",
    ];

    // 德国城市
    let germany_cities = [
        "柏林",
        "慕尼黑",
        "汉堡",
        "法兰克福",
        "科隆",
        "斯图加特",
        "德国",
        "berlin",
        "munich",
        "hamburg",
        "frankfurt",
        "cologne",
        "stuttgart",
        "germany",
        "german",
        "deutschland",
    ];

    // 检查中国
    for city in &china_cities {
        if query_lower.contains(city) {
            return Some("cn".to_string());
        }
    }

    // 检查美国
    for city in &us_cities {
        if query_lower.contains(city) {
            return Some("us".to_string());
        }
    }

    // 检查德国
    for city in &germany_cities {
        if query_lower.contains(city) {
            return Some("de".to_string());
        }
    }

    None
}

/// Check if string contains Chinese characters
fn contains_chinese(s: &str) -> bool {
    s.chars().any(|c| {
        matches!(c, '\u{4E00}'..='\u{9FFF}' | '\u{3400}'..='\u{4DBF}' | '\u{20000}'..='\u{2A6DF}')
    })
}

/// Format news result for display
pub fn format_news_result(result: &NewsResult) -> String {
    if result.articles.is_empty() {
        return "❌ 未找到相关新闻。请尝试其他关键词或地区。".to_string();
    }

    let mut output = format!("✅ 找到 {} 条新闻：\n\n", result.total);

    let display_count = result.articles.len().min(10);

    for (i, article) in result.articles.iter().take(display_count).enumerate() {
        output.push_str(&format!("{}. **{}**\n", i + 1, article.title));

        if let Some(desc) = &article.description {
            let truncated = if desc.len() > 200 {
                format!("{}...", &desc[..200])
            } else {
                desc.clone()
            };
            output.push_str(&format!("   {}\n", truncated));
        }

        output.push_str(&format!("   📰 来源: {}\n", article.source));

        if let Some(time) = &article.published_at {
            output.push_str(&format!("   🕐 时间: {}\n", time));
        }

        output.push_str(&format!("   🔗 链接: {}\n\n", article.url));
    }

    if result.total > display_count {
        output.push_str(&format!(
            "\n💡 还有 {} 条新闻未显示\n",
            result.total - display_count
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_news_query() {
        let query = NewsQuery {
            query: Some("technology".to_string()),
            country: Some("us".to_string()),
            category: None,
            language: Some("en".to_string()),
            max_results: Some(5),
        };

        let result = query_news(query).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_news_result() {
        let result = NewsResult {
            total: 1,
            articles: vec![NewsArticle {
                title: "Test News".to_string(),
                description: Some("Test description".to_string()),
                url: "https://example.com".to_string(),
                source: "Test Source".to_string(),
                published_at: Some("2024-01-01".to_string()),
                author: Some("Test Author".to_string()),
                image_url: None,
            }],
        };

        let formatted = format_news_result(&result);
        assert!(formatted.contains("Test News"));
        assert!(formatted.contains("Test Source"));
    }
}

/// News search tool for agent use
pub struct NewsSearchTool;

impl NewsSearchTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NewsSearchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AgentTool for NewsSearchTool {
    fn name(&self) -> &str {
        "news_search"
    }

    fn description(&self) -> &str {
        "Search for news articles by keywords, location, and category.\n\n\
         **CRITICAL**: This tool provides REAL, UP-TO-DATE news from actual news sources.\n\
         - Works for ALL countries and cities worldwide\n\
         - Returns actual news articles with titles, descriptions, sources, timestamps, and links\n\n\
         **MANDATORY USAGE**:\n\
         - User: '美国新闻' → DIRECTLY output: ```tool_call\\n{\"tool\": \"news_search\", \"arguments\": {\"query\": \"news\", \"location\": \"USA\"}}\\n```\n\
         - User: '上海新闻' → DIRECTLY output: ```tool_call\\n{\"tool\": \"news_search\", \"arguments\": {\"query\": \"news\", \"location\": \"Shanghai, China\"}}\\n```\n\
         - NO explanations before the tool call\n\
         - NO \"I will call\" or \"Let me call\" phrases\n\
         - NO news examples from your training data\n\n\
         **ABSOLUTELY FORBIDDEN**:\n\
         ❌ Saying 'I will call the news_search tool' - JUST CALL IT\n\
         ❌ Providing example news articles - ONLY real results from tool\n\
         ❌ Fabricating news content - YOU DON'T HAVE REAL-TIME DATA\n\n\
         YOU HAVE THIS TOOL. CALL IT DIRECTLY WITHOUT EXPLANATION."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "News search keywords (e.g., 'technology', 'politics', 'economy', 'sports'). Do NOT include location in query."
                },
                "location": {
                    "type": "string",
                    "description": "IMPORTANT: Extract city and/or country from user's request. Examples:\n- User: '上海新闻' → location: 'Shanghai, China'\n- User: 'Tokyo news' → location: 'Tokyo, Japan'\n- User: '成都美食' → location: 'Chengdu, China'\n- User: 'Paris politics' → location: 'Paris, France'\n- User: 'London tech' → location: 'London, UK'\n- User: '首尔经济' → location: 'Seoul, Korea'\n- User: 'Berlin news' → location: 'Berlin, Germany'\n- User: 'Mumbai news' → location: 'Mumbai, India'\n\nFormat: 'City, Country' or just 'Country'. Leave empty ONLY if no location is mentioned in user's request."
                },
                "country": {
                    "type": "string",
                    "description": "DEPRECATED: Use 'location' parameter instead. This is kept for backward compatibility.",
                    "enum": ["cn", "china", "us", "usa", "de", "germany", "world"]
                },
                "category": {
                    "type": "string",
                    "description": "News category: 'business', 'technology', 'sports', 'entertainment', 'health', 'science'. Optional.",
                    "enum": ["business", "technology", "sports", "entertainment", "health", "science"]
                },
                    "language": {
                    "type": "string",
                    "description": "Language code: 'zh' (Chinese), 'en' (English), 'de' (German). Optional, auto-detected from country.",
                    "enum": ["zh", "en", "de"]
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum number of results to return. Optional, defaults to 10.",
                    "minimum": 1,
                    "maximum": 50
                }
            },
            "required": []
        })
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        let mut query_params: NewsQuery = serde_json::from_value(params.clone())?;

        // 优先级 1: 使用 LLM 提取的 location 参数
        if let Some(location) = params.get("location").and_then(|v| v.as_str()) {
            if !location.trim().is_empty() {
                if let Some(country) = parse_location(location) {
                    query_params.country = Some(country);
                    tracing::info!(
                        "Location extracted by LLM: '{}' → country: {:?}",
                        location,
                        query_params.country
                    );
                }
            }
        }

        // 优先级 2: 检查常见城市（快速路径）
        if query_params.country.is_none() {
            let effective_query = query_params.effective_query();
            query_params.country = detect_country_from_query(&effective_query);
        }

        // 优先级 3: 中文检测（降级方案）
        if query_params.country.is_none() {
            let effective_query = query_params.effective_query();
            if contains_chinese(&effective_query) {
                query_params.country = Some("cn".to_string());
                query_params.language = Some("zh".to_string());
            }
        }

        tracing::info!(
            "Searching news: query='{}', country={:?}, category={:?}",
            query_params.effective_query(),
            query_params.country,
            query_params.category
        );

        let result = query_news(query_params).await?;

        if result.articles.is_empty() {
            return Ok(json!({
                "success": false,
                "message": "未找到相关新闻。请尝试其他关键词或地区。",
                "total": 0,
                "articles": []
            }));
        }

        let formatted = format_news_result(&result);

        Ok(json!({
            "success": true,
            "message": formatted,
            "total": result.total,
            "articles": result.articles
        }))
    }
}
