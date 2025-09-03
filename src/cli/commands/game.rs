use crate::cli::args::Cli;
use crate::extractor::ExtractionOptions;
use crate::game::screens::loading_screen::{LoadingScreen, ProcessingResult};
use crate::game::StageManager;
use crate::{GitTypeError, Result};
use std::path::PathBuf;

pub fn run_game_session(cli: Cli) -> Result<()> {
    let mut options = ExtractionOptions::default();

    if let Some(include_patterns) = cli.include {
        options.include_patterns = include_patterns;
    }

    if let Some(exclude_patterns) = cli.exclude {
        options.exclude_patterns = exclude_patterns;
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
            let result = loading_screen.process_repository(repo_spec, initial_repo_path, &options);
            let _ = loading_screen.cleanup();
            result
        })
        .and_then(|result| {
            if result.challenges.is_empty() {
                Err(GitTypeError::NoSupportedFiles)
            } else {
                Ok(result)
            }
        })
        .and_then(
            |ProcessingResult {
                 challenges,
                 git_info,
             }| {
                let mut stage_manager = StageManager::new(challenges);
                stage_manager.set_git_info(git_info);
                stage_manager.run_session()
            },
        );

    match session_result {
        Ok(_) => {}
        Err(e) => handle_game_error(e)?,
    }

    Ok(())
}

fn handle_game_error(e: GitTypeError) -> Result<()> {
    match e {
        GitTypeError::NoSupportedFiles => {
            panic!("No code chunks found in the repository");
        }
        GitTypeError::RepositoryNotFound(path) => {
            panic!("Repository not found at path: {}", path.display());
        }
        GitTypeError::RepositoryCloneError(git_error) => {
            panic!("Failed to clone repository: {}", git_error);
        }
        GitTypeError::ExtractionFailed(msg) => {
            panic!("Code extraction failed: {}", msg);
        }
        GitTypeError::InvalidRepositoryFormat(msg) => {
            panic!("Invalid repository format: {}", msg);
        }
        GitTypeError::IoError(io_error) => {
            panic!("IO error: {}", io_error);
        }
        GitTypeError::DatabaseError(db_error) => {
            panic!("Database error: {}", db_error);
        }
        GitTypeError::GlobPatternError(glob_error) => {
            panic!("Glob pattern error: {}", glob_error);
        }
        GitTypeError::SerializationError(json_error) => {
            panic!("Serialization error: {}", json_error);
        }
        GitTypeError::TerminalError(msg) => {
            eprintln!("Terminal error: {}", msg);
            if msg.contains("No such device or address") {
                eprintln!("\nHint: This error often occurs in WSL or SSH environments where terminal features are limited.");
                eprintln!("Try running GitType in a native terminal or GUI terminal emulator.");
            }
            panic!("Terminal error: {}", msg);
        }
        GitTypeError::WalkDirError(walk_error) => {
            panic!("Directory walk error: {}", walk_error);
        }
        GitTypeError::TreeSitterLanguageError(lang_error) => {
            panic!("Tree-sitter language error: {}", lang_error);
        }
    }
}
