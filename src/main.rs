use clap::{Parser, Subcommand};
use gittype::extractor::ExtractionOptions;
use gittype::game::screens::loading_screen::{LoadingScreen, ProcessingResult};
use gittype::game::{
    stage_manager::{cleanup_terminal, show_session_summary_on_interrupt},
    StageManager,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gittype")]
#[command(
    about = "A typing practice tool using your own code repositories - extracts all code chunks (functions, classes, methods, etc.)"
)]
#[command(version = "0.1.0")]
struct Cli {
    /// Repository path to extract code from
    #[arg(value_name = "REPO_PATH")]
    repo_path: Option<PathBuf>,

    /// GitHub repository URL or path to clone and play with (e.g., owner/repo, https://github.com/owner/repo, git@github.com:owner/repo.git)
    #[arg(long)]
    repo: Option<String>,

    /// Filter by programming languages
    #[arg(long, value_delimiter = ',')]
    langs: Option<Vec<String>>,

    /// Number of stages for normal mode
    #[arg(long, default_value_t = 3)]
    stages: usize,

    /// Glob patterns for files to include
    #[arg(long)]
    include: Option<Vec<String>>,

    /// Glob patterns for files to exclude
    #[arg(long)]
    exclude: Option<Vec<String>>,

    /// Path to config file
    #[arg(long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show session history
    History,
    /// Show analytics
    Stats,
    /// Export session data
    Export {
        /// Export format
        #[arg(long, default_value = "json")]
        format: String,
        /// Output file path
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    // Set up panic hook to ensure terminal cleanup
    std::panic::set_hook(Box::new(|_| {
        cleanup_terminal();
    }));

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        show_session_summary_on_interrupt();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::History) => {
            println!("Showing session history...");
            // TODO: Implement history display
        }
        Some(Commands::Stats) => {
            println!("Showing statistics...");
            // TODO: Implement stats display
        }
        Some(Commands::Export { format, output }) => {
            println!("Exporting data in {} format...", format);
            if let Some(path) = output {
                println!("Output file: {}", path.display());
            }
            // TODO: Implement export functionality
        }
        None => {
            // Build extraction options from CLI arguments
            let mut options = ExtractionOptions::default();

            if let Some(include_patterns) = cli.include {
                options.include_patterns = include_patterns;
            }

            if let Some(exclude_patterns) = cli.exclude {
                options.exclude_patterns = exclude_patterns;
            }

            // Process repository using LoadingScreen
            let repo_spec = cli.repo.as_deref();
            let default_repo_path = cli.repo_path.unwrap_or_else(|| PathBuf::from("."));
            let initial_repo_path = if repo_spec.is_some() {
                None
            } else {
                Some(&default_repo_path)
            };

            // Process repository using LoadingScreen
            // Process everything in a Result chain
            let session_result = LoadingScreen::new()
                .and_then(|mut loading_screen| {
                    let result =
                        loading_screen.process_repository(repo_spec, initial_repo_path, &options);
                    let _ = loading_screen.cleanup();
                    result
                })
                .and_then(|result| {
                    if result.challenges.is_empty() {
                        Err(gittype::GitTypeError::NoSupportedFiles)
                    } else {
                        Ok(result)
                    }
                })
                .and_then(
                    |ProcessingResult {
                         challenges,
                         git_info,
                     }| {
                        // Create StageManager with processed challenges
                        let mut stage_manager = StageManager::new(challenges);
                        stage_manager.set_git_info(git_info);

                        // Run session
                        stage_manager.run_session()
                    },
                );

            match session_result {
                Ok(_) => {
                    // println!("Thanks for playing GitType!");
                }
                Err(e) => {
                    // Handle all errors at the end of the chain
                    match e {
                        gittype::GitTypeError::NoSupportedFiles => {
                            panic!("No code chunks found in the repository");
                        }
                        gittype::GitTypeError::RepositoryNotFound(path) => {
                            panic!("Repository not found at path: {}", path.display());
                        }
                        gittype::GitTypeError::RepositoryCloneError(git_error) => {
                            panic!("Failed to clone repository: {}", git_error);
                        }
                        gittype::GitTypeError::ExtractionFailed(msg) => {
                            panic!("Code extraction failed: {}", msg);
                        }
                        gittype::GitTypeError::InvalidRepositoryFormat(msg) => {
                            panic!("Invalid repository format: {}", msg);
                        }
                        gittype::GitTypeError::IoError(io_error) => {
                            panic!("IO error: {}", io_error);
                        }
                        gittype::GitTypeError::DatabaseError(db_error) => {
                            panic!("Database error: {}", db_error);
                        }
                        gittype::GitTypeError::GlobPatternError(glob_error) => {
                            panic!("Glob pattern error: {}", glob_error);
                        }
                        gittype::GitTypeError::SerializationError(json_error) => {
                            panic!("Serialization error: {}", json_error);
                        }
                        gittype::GitTypeError::TerminalError(msg) => {
                            // Special handling for terminal errors with helpful hints
                            eprintln!("Terminal error: {}", msg);
                            if msg.contains("No such device or address") {
                                eprintln!("\nHint: This error often occurs in WSL or SSH environments where terminal features are limited.");
                                eprintln!("Try running GitType in a native terminal or GUI terminal emulator.");
                            }
                            panic!("Terminal error: {}", msg);
                        }
                        gittype::GitTypeError::WalkDirError(walk_error) => {
                            panic!("Directory walk error: {}", walk_error);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
