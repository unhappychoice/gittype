use shaku::HasComponent;

use std::sync::Arc;

use crate::domain::repositories::trending_repository::TrendingRepositoryInterface;
use crate::domain::services::theme_service::ThemeServiceInterface;
use crate::infrastructure::console::{Console, ConsoleImpl};
use crate::presentation::cli::commands::run_game_session;
use crate::presentation::cli::screen_runner::{run_screen, ScreenRunnerContext};
use crate::presentation::cli::Cli;
use crate::presentation::di::AppModule;
use crate::presentation::tui::screens::{
    TrendingLanguageSelectionScreen, TrendingRepositorySelectionScreen,
};
use crate::presentation::tui::ScreenType;
use crate::{GitTypeError, Result};

const SUPPORTED_LANGUAGES: &[(&str, &str)] = &[
    ("C", "C"),
    ("C#", "C#"),
    ("C++", "C++"),
    ("Dart", "Dart"),
    ("Elixir", "Elixir"),
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

pub fn run_trending(
    language: Option<String>,
    repo_name: Option<String>,
    period: String,
) -> Result<()> {
    let console = ConsoleImpl::new();

    // Create DI container and resolve TrendingRepository
    let container = AppModule::builder().build();
    let _trending_repository: Arc<dyn TrendingRepositoryInterface> = container.resolve();

    // Validate language if provided
    if let Some(ref lang) = language {
        if !validate_language(lang) {
            let supported_langs: Vec<&str> =
                SUPPORTED_LANGUAGES.iter().map(|(name, _)| *name).collect();
            console.eprintln(&format!("❌ Unsupported language: '{}'", lang))?;
            console.eprintln(&format!(
                "📚 Supported languages: {}",
                supported_langs.join(", ")
            ))?;
            return Err(GitTypeError::ValidationError(format!(
                "Unsupported language: {}",
                lang
            )));
        }
    }

    if let Some(name) = repo_name {
        // Direct repository selection by name
        // Assume name is in format "owner/repo" or just "repo"
        let repo_url = if name.contains('/') {
            format!("https://github.com/{}", name)
        } else {
            // If no slash, might need to search trending, but for now treat as error
            console.eprintln("⚠️ Repository name must be in format 'owner/repo'")?;
            return Ok(());
        };

        let cli = Cli {
            repo_path: None,
            repo: Some(repo_url),
            langs: None,
            command: None,
        };
        return run_game_session(cli);
    } else if language.is_some() {
        // Language provided - show repositories directly
        let _theme_service: Arc<dyn ThemeServiceInterface> = container.resolve();

        let selected_repo = run_screen::<TrendingRepositorySelectionScreen, _, _, _>(
            ScreenType::TrendingRepositorySelection,
            Some((language.clone(), period.clone())),
            Some(|screen: &TrendingRepositorySelectionScreen| {
                screen.get_selected_index().and_then(|idx| {
                    screen
                        .get_repositories()
                        .get(idx)
                        .map(|repo| repo.repo_name.clone())
                })
            }),
        )?;

        // If a repository was selected, start the game
        if let Some(repo_name) = selected_repo {
            let repo_url = format!("https://github.com/{}", repo_name);
            let cli = Cli {
                repo_path: None,
                repo: Some(repo_url),
                langs: None,
                command: None,
            };
            return run_game_session(cli);
        }
    } else {
        // No language provided - show language selection then repository selection
        let _theme_service: Arc<dyn ThemeServiceInterface> = container.resolve();

        // Use ScreenRunnerContext to share terminal state between screens
        let ctx = ScreenRunnerContext::new()?;

        // Step 1: Language selection
        let selected_language = ctx.run_screen::<TrendingLanguageSelectionScreen, _, _, _>(
            ScreenType::TrendingLanguageSelection,
            None::<()>,
            Some(|screen: &TrendingLanguageSelectionScreen| {
                screen.get_selected_language().map(|s| s.to_string())
            }),
        )?;

        if let Some(lang) = selected_language {
            // Step 2: Repository selection with selected language
            let selected_repo = ctx.run_screen::<TrendingRepositorySelectionScreen, _, _, _>(
                ScreenType::TrendingRepositorySelection,
                Some((Some(lang), period.clone())),
                Some(|screen: &TrendingRepositorySelectionScreen| {
                    screen.get_selected_index().and_then(|idx| {
                        screen
                            .get_repositories()
                            .get(idx)
                            .map(|repo| repo.repo_name.clone())
                    })
                }),
            )?;

            // Cleanup terminal before starting game
            ctx.cleanup()?;

            if let Some(repo_name) = selected_repo {
                let repo_url = format!("https://github.com/{}", repo_name);
                let cli = Cli {
                    repo_path: None,
                    repo: Some(repo_url),
                    langs: None,
                    command: None,
                };
                return run_game_session(cli);
            }
        } else {
            // User cancelled language selection
            ctx.cleanup()?;
        }
    }

    Ok(())
}
