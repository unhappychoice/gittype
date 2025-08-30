use clap::{Parser, Subcommand};
use gittype::game::{StageManager, stage_manager::show_session_summary_on_interrupt};
use gittype::extractor::{ExtractionOptions, RepositoryLoader, ProgressReporter};
use gittype::game::screens::loading_screen::LoadingScreen;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gittype")]
#[command(about = "A typing practice tool using your own code repositories - extracts all code chunks (functions, classes, methods, etc.)")]
#[command(version = "0.1.0")]
struct Cli {
    /// Repository path to extract code from
    #[arg(value_name = "REPO_PATH")]
    repo_path: Option<PathBuf>,

    /// Filter by programming languages
    #[arg(long, value_delimiter = ',')]
    langs: Option<Vec<String>>,



    /// Number of stages for normal mode
    #[arg(long, default_value_t = 3)]
    stages: usize,

    /// Maximum lines per challenge
    #[arg(long, default_value_t = 40)]
    max_lines: usize,

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
    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        show_session_summary_on_interrupt();
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
    
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
            if let Some(repo_path) = cli.repo_path {
                // Build extraction options from CLI arguments
                let mut options = ExtractionOptions::default();
                
                if let Some(include_patterns) = cli.include {
                    options.include_patterns = include_patterns;
                }
                
                if let Some(exclude_patterns) = cli.exclude {
                    options.exclude_patterns = exclude_patterns;
                }
                
                options.max_lines = Some(cli.max_lines);

                // Show loading screen during startup
                let loading_screen = match LoadingScreen::new() {
                    Ok(screen) => Some(screen),
                    Err(e) => {
                        eprintln!("Failed to initialize loading screen: {}, continuing without it...", e);
                        None
                    }
                };
                
                if let Some(ref screen) = loading_screen {
                    let _ = screen.show_initial();
                }

                let mut loader = match RepositoryLoader::new() {
                    Ok(loader) => loader,
                    Err(e) => {
                        eprintln!("Failed to initialize code extractor: {}", e);
                        return Ok(());
                    }
                };

                // Load all code chunks (functions, classes, methods, etc.)
                let mut available_challenges = if let Some(ref screen) = loading_screen {
                    loader.load_challenges_from_repository_with_progress(&repo_path, Some(options.clone()), screen)?
                } else {
                    loader.load_challenges_from_repository(&repo_path, Some(options.clone()))?
                };
                
                // Generate additional Zen challenges for all source files in the repository
                if let Some(ref screen) = loading_screen {
                    screen.set_phase("Generating Zen challenges from all files".to_string());
                }
                let all_file_zen_challenges = loader.load_all_files_as_zen_challenges(&repo_path)?;
                available_challenges.extend(all_file_zen_challenges);

                if available_challenges.is_empty() {
                    if let Some(ref screen) = loading_screen {
                        let _ = screen.show_completion();
                    }
                    eprintln!("No code chunks found in the repository");
                    return Ok(());
                }
                if let Some(ref screen) = loading_screen {
                    let _ = screen.show_completion();
                }
                
                // Create StageManager with pre-generated challenges
                let mut stage_manager = StageManager::new(available_challenges);
                match stage_manager.run_session() {
                    Ok(_) => {
                        // println!("Thanks for playing GitType!");
                    },
                    Err(e) => {
                        eprintln!("Error during game session: {}", e);
                        if e.to_string().contains("No such device or address") {
                            eprintln!("\nHint: This error often occurs in WSL or SSH environments where terminal features are limited.");
                            eprintln!("Try running GitType in a native terminal or GUI terminal emulator.");
                        }
                    }
                }
            } else {
                println!("Please specify a repository path or use --help for usage information");
            }
        }
    }

    Ok(())
}
