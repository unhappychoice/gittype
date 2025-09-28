use crate::domain::repositories::trending_repository::{TrendingRepositoryInfo, TRENDING_REPOSITORY};
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
) -> Result<Vec<TrendingRepositoryInfo>> {
    let cache_key = format!("{}:{}", language.unwrap_or("all"), period);

    // Use the unified repository method that handles both caching and API fetching
    match TRENDING_REPOSITORY.get_trending_repositories(&cache_key, language, period).await {
        Ok(repos) => {
            log::debug!("Retrieved trending repositories for key: {}", cache_key);
            Ok(repos)
        }
        Err(e) => {
            log::warn!("Failed to retrieve trending repositories: {}", e);
            Ok(Vec::new()) // Return empty vec for graceful degradation
        }
    }
}

fn select_repository_by_name<'a>(
    repos: &'a [TrendingRepositoryInfo],
    name: &str,
) -> Option<&'a TrendingRepositoryInfo> {
    repos
        .iter()
        .find(|repo| repo.repo_name.to_lowercase().contains(&name.to_lowercase()))
}
