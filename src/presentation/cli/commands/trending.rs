use crate::infrastructure::cache::{TrendingRepository, TRENDING_CACHE};
use crate::presentation::cli::commands::run_game_session;
use crate::presentation::cli::views::{trending_repository_selection_view, trending_unified_view};
use crate::presentation::cli::Cli;
use crate::{GitTypeError, Result};
use reqwest::Client;
use std::time::Duration;

const SUPPORTED_LANGUAGES: &[(&str, &str)] = &[
    ("C", "C"),
    ("C#", "C#"),
    ("C++", "C++"),
    ("Dart", "Dart"),
    ("Go", "Go"),
    ("Haskell", "Haskell"),
    ("Java", "Java"),
    ("JavaScript", "JavaScript"),
    ("Kotlin", "Kotlin"),
    ("PHP", "PHP"),
    ("Python", "Python"),
    ("Ruby", "Ruby"),
    ("Rust", "Rust"),
    ("Scala", "Scala"),
    ("Swift", "Swift"),
    ("TypeScript", "TypeScript"),
];

fn validate_language(language: &str) -> bool {
    SUPPORTED_LANGUAGES.iter().any(|(display_name, lang_code)| {
        display_name.to_lowercase() == language.to_lowercase()
            || lang_code.to_lowercase() == language.to_lowercase()
    })
}

pub async fn run_trending(
    language: Option<String>,
    repo_name: Option<String>,
    period: String,
) -> Result<()> {
    // Validate language if provided
    if let Some(ref lang) = language {
        if !validate_language(lang) {
            let supported_langs: Vec<&str> =
                SUPPORTED_LANGUAGES.iter().map(|(name, _)| *name).collect();
            eprintln!("❌ Unsupported language: '{}'", lang);
            eprintln!("📚 Supported languages: {}", supported_langs.join(", "));
            return Err(GitTypeError::ValidationError(format!(
                "Unsupported language: {}",
                lang
            )));
        }
    }
    if let Some(name) = repo_name {
        // Direct repository selection by name
        let client = Client::new();
        let repos =
            fetch_trending_repositories_cached(&client, language.as_deref(), &period).await?;

        if let Some(repo) = select_repository_by_name(&repos, &name) {
            let repo_url = format!("https://github.com/{}", repo.repo_name);
            let cli = Cli {
                repo_path: None,
                repo: Some(repo_url),
                langs: None,
                config: None,
                command: None,
            };
            return run_game_session(cli);
        } else {
            eprintln!("⚠️ Repository '{}' not found in trending list", name);
            return Ok(());
        }
    } else if language.is_some() {
        // Language provided - show repositories directly
        let client = Client::new();
        let repos =
            fetch_trending_repositories_cached(&client, language.as_deref(), &period).await?;

        if repos.is_empty() {
            return Ok(());
        }

        match trending_repository_selection_view::render_trending_ui(repos.clone())? {
            Some(selection) => {
                if let Some(repo) = repos.get(selection) {
                    let repo_url = format!("https://github.com/{}", repo.repo_name);
                    let cli = Cli {
                        repo_path: None,
                        repo: Some(repo_url),
                        langs: None,
                        config: None,
                        command: None,
                    };
                    return run_game_session(cli);
                }
            }
            None => return Ok(()),
        }
    } else {
        // No language provided - show unified selection UI
        match trending_unified_view::render_trending_selection_ui().await? {
            Some(repo_url) => {
                let cli = Cli {
                    repo_path: None,
                    repo: Some(repo_url),
                    langs: None,
                    config: None,
                    command: None,
                };
                return run_game_session(cli);
            }
            None => return Ok(()),
        }
    }

    Ok(())
}

// Make this function public so it can be used from the UI
pub async fn fetch_trending_repositories_cached(
    client: &Client,
    language: Option<&str>,
    period: &str,
) -> Result<Vec<TrendingRepository>> {
    // Create cache key from parameters
    let cache_key = format!("{}:{}", language.unwrap_or("all"), period);

    // Check cache first
    if let Some(cached_repos) = TRENDING_CACHE.get(&cache_key) {
        log::debug!("Using cached trending repositories for key: {}", cache_key);
        return Ok(cached_repos);
    }

    // Clean up expired cache entries before making API call
    TRENDING_CACHE.cleanup_expired();

    // Rate limiting: wait a bit between API calls to be respectful
    let rate_limit_ms = 100; // 100ms
    tokio::time::sleep(Duration::from_millis(rate_limit_ms)).await;

    // Fetch from API
    let repos = fetch_trending_repositories(client, language, period).await?;

    // Cache the result
    TRENDING_CACHE.set(&cache_key, repos.clone());
    log::debug!("Cached trending repositories for key: {}", cache_key);

    Ok(repos)
}

async fn fetch_trending_repositories(
    client: &Client,
    language: Option<&str>,
    period: &str,
) -> Result<Vec<TrendingRepository>> {
    let api_period = match period {
        "daily" => "past_24_hours",
        "weekly" => "past_week",
        "monthly" => "past_month",
        _ => "past_24_hours",
    };

    let mut url = format!(
        "https://api.ossinsight.io/v1/trends/repos/?period={}",
        api_period
    );

    if let Some(lang) = language {
        // Map to correct language name format expected by API
        let api_lang = match lang.to_lowercase().as_str() {
            "javascript" => "JavaScript".to_string(),
            "typescript" => "TypeScript".to_string(),
            "c++" => "C++".to_string(),
            "c#" => "C#".to_string(),
            "php" => "PHP".to_string(),
            _ => {
                // Capitalize first letter for other languages
                let mut chars = lang.chars();
                match chars.next() {
                    None => lang.to_string(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            }
        };
        url = format!("{}&language={}", url, urlencoding::encode(&api_lang));
    }

    let response = client
        .get(&url)
        .header("User-Agent", "gittype")
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(GitTypeError::ApiError(format!(
            "OSS Insight API request failed: {}",
            response.status()
        )));
    }

    #[derive(serde::Deserialize)]
    struct ApiResponse {
        data: ApiData,
    }

    #[derive(serde::Deserialize)]
    struct ApiData {
        rows: Vec<RowData>,
    }

    #[derive(serde::Deserialize)]
    struct RowData {
        repo_name: String,
        primary_language: Option<String>,
        description: Option<String>,
        stars: Option<String>,
        forks: Option<String>,
        total_score: Option<String>,
    }

    let api_response: ApiResponse = response.json().await?;

    // Convert API response to TrendingRepository objects
    let repositories: Vec<TrendingRepository> = api_response
        .data
        .rows
        .into_iter()
        .map(|row| TrendingRepository {
            repo_name: row.repo_name,
            primary_language: row.primary_language,
            description: row.description,
            stars: row.stars.unwrap_or_else(|| "0".to_string()),
            forks: row.forks.unwrap_or_else(|| "0".to_string()),
            total_score: row.total_score.unwrap_or_else(|| "0".to_string()),
        })
        .collect();

    Ok(repositories)
}

fn select_repository_by_name<'a>(
    repos: &'a [TrendingRepository],
    name: &str,
) -> Option<&'a TrendingRepository> {
    repos
        .iter()
        .find(|repo| repo.repo_name.to_lowercase().contains(&name.to_lowercase()))
}
