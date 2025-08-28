use clap::{Parser, Subcommand};
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
                println!("Starting typing game for repository: {}", repo_path.display());
                println!("Languages: {:?}", cli.langs);
                println!("Unit: {}", cli.unit);
                println!("Max lines: {}", cli.max_lines);
                println!("Include patterns: {:?}", cli.include);
                println!("Exclude patterns: {:?}", cli.exclude);
                // TODO: Start typing game
            } else {
                println!("Please specify a repository path or use --help for usage information");
            }
        }
    }

    Ok(())
}