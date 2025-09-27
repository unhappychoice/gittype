use crate::infrastructure::cache::{TrendingRepository, TRENDING_CACHE};
use crate::infrastructure::http::OssInsightClient;
use crate::presentation::cli::commands::run_game_session;
use crate::presentation::cli::views::{trending_repository_selection_view, trending_unified_view};
use crate::presentation::cli::Cli;
use crate::{GitTypeError, Result};

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
        let repos =
            fetch_trending_repositories_cached(&(), language.as_deref(), &period).await?;

        if let Some(repo) = select_repository_by_name(&repos, &name) {
            let repo_url = format!("https://github.com/{}", repo.repo_name);
            let cli = Cli {
                repo_path: None,
                repo: Some(repo_url),
                langs: None,
                command: None,
            };
            return run_game_session(cli);
        } else {
            eprintln!("⚠️ Repository '{}' not found in trending list", name);
            return Ok(());
        }
    } else if language.is_some() {
        // Language provided - show repositories directly
        let repos =
            fetch_trending_repositories_cached(&(), language.as_deref(), &period).await?;

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
                        command: None,
                };
                return run_game_session(cli);
            }
            None => return Ok(()),
        }
    }

    Ok(())
}

pub async fn fetch_trending_repositories_cached(
    _client: &(),
    language: Option<&str>,
    period: &str,
) -> Result<Vec<TrendingRepository>> {
    let cache_key = format!("{}:{}", language.unwrap_or("all"), period);

    if let Some(cached_repos) = TRENDING_CACHE.get(&cache_key) {
        log::debug!("Using cached trending repositories for key: {}", cache_key);
        return Ok(cached_repos);
    }

    TRENDING_CACHE.cleanup_expired();

    let rate_limit_ms = 100;
    tokio::time::sleep(std::time::Duration::from_millis(rate_limit_ms)).await;

    let api_client = OssInsightClient::new();
    let repos = api_client
        .fetch_trending_repositories(language, period)
        .await?;

    TRENDING_CACHE.set(&cache_key, repos.clone());
    log::debug!("Cached trending repositories for key: {}", cache_key);

    Ok(repos)
}

fn select_repository_by_name<'a>(
    repos: &'a [TrendingRepository],
    name: &str,
) -> Option<&'a TrendingRepository> {
    repos
        .iter()
        .find(|repo| repo.repo_name.to_lowercase().contains(&name.to_lowercase()))
}
