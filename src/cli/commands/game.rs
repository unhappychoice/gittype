use crate::cli::args::Cli;
use crate::extractor::ExtractionOptions;
use crate::game::screens::loading_screen::{LoadingScreen, ProcessingResult};
use crate::game::StageManager;
use crate::logging::{setup_console_logging, setup_logging};
use crate::{GitTypeError, Result};
use std::path::PathBuf;

pub fn run_game_session(cli: Cli) -> Result<()> {
    // Initialize logging first
    if let Err(e) = setup_logging() {
        // Fallback to console-only logging if file logging fails
        setup_console_logging();
        eprintln!("⚠️ Warning: Failed to setup file logging: {}", e);
        eprintln!("   Logs will only be shown in console.");
    }

    log::info!("Starting GitType game session");

    // Session repository will be initialized in DatabaseInitStep during loading screen

    let mut options = ExtractionOptions::default();

    if let Some(langs) = cli.langs {
        if let Err(unsupported_langs) =
            crate::extractor::models::language::LanguageRegistry::validate_languages(&langs)
        {
            eprintln!(
                "❌ Unsupported language(s): {}",
                unsupported_langs.join(", ")
            );
            eprintln!("💡 Supported languages:");
            let supported =
                crate::extractor::models::language::LanguageRegistry::get_supported_languages();
            let mut supported_display = supported.clone();
            supported_display.dedup();
            for chunk in supported_display.chunks(6) {
                eprintln!("   {}", chunk.join(", "));
            }
            std::process::exit(1);
        }

        options.languages = Some(langs);
        options.apply_language_filter();
    }

    let repo_spec = cli.repo.as_deref();
    let default_repo_path = cli.repo_path.unwrap_or_else(|| PathBuf::from("."));
    let initial_repo_path = if repo_spec.is_some() {
        None
    } else {
        Some(&default_repo_path)
    };

    let session_result = LoadingScreen::new()
        .and_then(|mut loading_screen| {
            log::info!(
                "Processing repository with repo_spec: {:?}, repo_path: {:?}",
                repo_spec,
                initial_repo_path
            );
            let result = loading_screen.process_repository(repo_spec, initial_repo_path, &options);
            let _ = loading_screen.cleanup();
            result
        })
        .and_then(|result| {
            log::info!("Found {} challenges", result.challenges.len());
            if let Some(ref git_repo) = result.git_repository {
                log::info!(
                    "Repository: {}/{} (branch: {:?}, commit: {:?}, dirty: {})",
                    git_repo.user_name,
                    git_repo.repository_name,
                    git_repo.branch,
                    git_repo.commit_hash,
                    git_repo.is_dirty
                );
            } else {
                log::info!("No git repository context available");
            }

            if result.challenges.is_empty() {
                log::warn!("No supported files found in repository");
                Err(GitTypeError::NoSupportedFiles)
            } else {
                Ok(result)
            }
        })
        .and_then(
            |ProcessingResult {
                 challenges,
                 git_repository,
             }| {
                log::info!("Starting game session with {} challenges", challenges.len());
                let mut stage_manager = StageManager::new(challenges);
                stage_manager.set_git_repository(git_repository);
                stage_manager.run_session()
            },
        );

    match session_result {
        Ok(_) => {
            log::info!("Game session completed successfully");
        }
        Err(e) => {
            log::error!("Game session failed with error: {}", e);
            handle_game_error(e)?;
        }
    }

    Ok(())
}

fn handle_game_error(e: GitTypeError) -> Result<()> {
    match e {
        GitTypeError::NoSupportedFiles => {
            eprintln!("❌ No code chunks found in the repository");
            eprintln!("💡 Try:");
            eprintln!("   • Using a different repository path");
            eprintln!("   • Adjusting --langs filter (e.g., --langs rust,python)");
            std::process::exit(1);
        }
        GitTypeError::RepositoryNotFound(path) => {
            eprintln!("❌ Repository not found at path: {}", path.display());
            eprintln!("💡 Ensure the path exists and is a valid repository");
            std::process::exit(1);
        }
        GitTypeError::RepositoryCloneError(git_error) => {
            eprintln!("❌ Failed to clone repository: {}", git_error);
            eprintln!("💡 Check:");
            eprintln!("   • Repository URL is correct");
            eprintln!("   • You have access to the repository");
            eprintln!("   • Internet connection is available");
            std::process::exit(1);
        }
        GitTypeError::ExtractionFailed(msg) => {
            eprintln!("❌ Code extraction failed: {}", msg);
            eprintln!("💡 Try using different --langs filter");
            std::process::exit(1);
        }
        GitTypeError::InvalidRepositoryFormat(msg) => {
            eprintln!("❌ Invalid repository format: {}", msg);
            eprintln!("💡 Supported formats:");
            eprintln!("   • owner/repo");
            eprintln!("   • https://github.com/owner/repo");
            eprintln!("   • git@github.com:owner/repo.git");
            std::process::exit(1);
        }
        GitTypeError::IoError(io_error) => {
            eprintln!("❌ IO error: {}", io_error);
            std::process::exit(1);
        }
        GitTypeError::DatabaseError(db_error) => {
            eprintln!("❌ Database error: {}", db_error);
            std::process::exit(1);
        }
        GitTypeError::GlobPatternError(glob_error) => {
            eprintln!("❌ Invalid glob pattern: {}", glob_error);
            eprintln!("💡 Check your glob patterns in ExtractionOptions");
            std::process::exit(1);
        }
        GitTypeError::SerializationError(json_error) => {
            eprintln!("❌ Serialization error: {}", json_error);
            std::process::exit(1);
        }
        GitTypeError::TerminalError(msg) => {
            eprintln!("❌ Terminal error: {}", msg);
            if msg.contains("No such device or address") {
                eprintln!("💡 This error often occurs in WSL or SSH environments where terminal features are limited.");
                eprintln!("   Try running GitType in a native terminal or GUI terminal emulator.");
            }
            std::process::exit(1);
        }
        GitTypeError::WalkDirError(walk_error) => {
            eprintln!("❌ Directory walk error: {}", walk_error);
            eprintln!("💡 Check directory permissions and try again");
            std::process::exit(1);
        }
        GitTypeError::TreeSitterLanguageError(lang_error) => {
            eprintln!("❌ Language parsing error: {}", lang_error);
            eprintln!("💡 This might be caused by unsupported language features");
            std::process::exit(1);
        }
    }
}
