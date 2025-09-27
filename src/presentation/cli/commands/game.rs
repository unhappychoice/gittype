use crate::presentation::cli::args::Cli;
use crate::domain::models::ExtractionOptions;
use crate::presentation::game::models::ScreenType;
use crate::presentation::game::screen_manager::ScreenManager;
use crate::{GitTypeError, Result};
use std::path::PathBuf;

pub fn run_game_session(cli: Cli) -> Result<()> {
    log::info!("Starting GitType game session");

    // Check for updates before starting the game session
    let should_exit = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            if let Ok(Some(entry)) = crate::infrastructure::version::checker::VersionChecker::check_for_updates().await {
                if entry.update_available {
                    return crate::infrastructure::version::checker::VersionChecker::display_update_notification(&entry)
                        .map(|should_continue| !should_continue);
                }
            }
            Ok(false)
        })
    })?;

    if should_exit {
        log::info!("User exited after update notification");
        return Ok(());
    }

    // Initialize theme manager
    if let Err(e) = crate::domain::services::theme_manager::ThemeManager::init(cli.config.clone()) {
        log::warn!("Failed to initialize theme manager: {}", e);
        eprintln!("⚠️ Warning: Failed to load theme configuration: {}", e);
        eprintln!("   Using default theme.");
    }

    // Session repository will be initialized in DatabaseInitStep during loading screen

    let mut options = ExtractionOptions::default();

    if let Some(langs) = cli.langs {
        if let Err(unsupported_langs) =
            crate::domain::services::extractor::LanguageRegistry::validate_languages(&langs)
        {
            eprintln!(
                "❌ Unsupported language(s): {}",
                unsupported_langs.join(", ")
            );
            eprintln!("💡 Supported languages:");
            let supported =
                crate::domain::services::extractor::LanguageRegistry::get_supported_languages();
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

    // Initialize GameData and set processing parameters
    use crate::presentation::game::GameData;
    GameData::initialize()?;
    GameData::set_processing_parameters(repo_spec, initial_repo_path, &options)?;

    // Initialize ScreenManager with all screens
    ScreenManager::with_instance(|screen_manager| -> Result<()> {
        let mut manager = screen_manager.borrow_mut();

        log::info!(
            "Initializing all screens with GameData parameters: repo_spec={:?}, repo_path={:?}",
            repo_spec,
            initial_repo_path
        );
        manager.initialize_all_screens()?;

        manager.initialize_terminal()?;
        manager.set_current_screen(ScreenType::Loading)?;
        Ok(())
    })?;

    // Run ScreenManager normally - LoadingScreen will handle processing internally
    // StageRepository and SessionManager will be initialized automatically when data is ready
    let session_result = ScreenManager::run_global();

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
    // Log the error details for debugging before handling user-friendly output
    crate::logging::log_error_to_file(&e);

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
        GitTypeError::PanicError(msg) => {
            eprintln!("💥 Application panic occurred: {}", msg);
            eprintln!("💡 This indicates an unexpected error. Please report this issue.");
            std::process::exit(1);
        }
        GitTypeError::HttpError(http_error) => {
            eprintln!("❌ HTTP request failed: {}", http_error);
            eprintln!("💡 Check your internet connection and try again");
            std::process::exit(1);
        }
        GitTypeError::ApiError(msg) => {
            eprintln!("❌ API error: {}", msg);
            eprintln!("💡 The service may be temporarily unavailable");
            std::process::exit(1);
        }
        GitTypeError::ValidationError(msg) => {
            eprintln!("❌ {}", msg);
            std::process::exit(1);
        }
    }
}
