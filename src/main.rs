use clap::{Parser, Subcommand};
use gittype::game::StageManager;
use gittype::extractor::{RepositoryLoader, ExtractionOptions};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gittype")]
#[command(about = "A typing practice tool using your own code repositories")]
#[command(version = "0.1.0")]
struct Cli {
    /// Repository path to extract code from
    #[arg(value_name = "REPO_PATH")]
    repo_path: Option<PathBuf>,

    /// Filter by programming languages
    #[arg(long, value_delimiter = ',')]
    langs: Option<Vec<String>>,

    /// Select extraction unit
    #[arg(long, default_value = "function")]
    unit: String,

    /// Game mode
    #[arg(long, default_value = "normal")]
    mode: String,

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
                // Extract challenges from the provided repository
                let mut loader = match RepositoryLoader::new() {
                    Ok(loader) => loader,
                    Err(e) => {
                        eprintln!("Failed to initialize code extractor: {}", e);
                        return Ok(());
                    }
                };

                // Build extraction options from CLI arguments
                let mut options = ExtractionOptions::default();
                
                if let Some(include_patterns) = cli.include {
                    options.include_patterns = include_patterns;
                }
                
                if let Some(exclude_patterns) = cli.exclude {
                    options.exclude_patterns = exclude_patterns;
                }
                
                options.max_lines = Some(cli.max_lines);

                // Load challenges based on extraction unit
                let challenges = match cli.unit.as_str() {
                    "function" => loader.load_functions_only(&repo_path, Some(options)),
                    "class" | "struct" => loader.load_classes_only(&repo_path, Some(options)),
                    "all" => loader.load_challenges_from_repository(&repo_path, Some(options)),
                    _ => {
                        eprintln!("Unknown unit type: {}. Use 'function', 'class', or 'all'", cli.unit);
                        return Ok(());
                    }
                };

                let available_challenges = match challenges {
                    Ok(challenges) => {
                        if challenges.is_empty() {
                            eprintln!("No code chunks found in the repository");
                            return Ok(());
                        }
                        println!("Found {} code challenges", challenges.len());
                        challenges
                    },
                    Err(e) => {
                        eprintln!("Error extracting code from repository: {}", e);
                        return Ok(());
                    }
                };
                
                // Pass all challenges to StageManager, it will build stages based on selected difficulty
                let mut stage_manager = StageManager::new(available_challenges);
                match stage_manager.run_session() {
                    Ok(_) => {
                        println!("Thanks for playing GitType!");
                    },
                    Err(e) => {
                        eprintln!("Error during game session: {}", e);
                    }
                }
            } else {
                println!("Please specify a repository path or use --help for usage information");
            }
        }
    }

    Ok(())
}
