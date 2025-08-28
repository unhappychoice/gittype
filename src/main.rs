use clap::{Parser, Subcommand};
use gittype::game::{Challenge, StageManager};
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
            if let Some(_repo_path) = cli.repo_path {
                // Create sample challenges for multi-stage experience
                let challenges = create_sample_challenges();
                
                let mut stage_manager = StageManager::new(challenges);
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

fn create_sample_challenges() -> Vec<Challenge> {
    vec![
        Challenge::new(
            "stage1".to_string(),
            r#"fn hello() {
    println!("Hello, world!");
}"#.to_string(),
        )
        .with_source_info("src/main.rs".to_string(), 1, 3)
        .with_language("rust".to_string()),
        
        Challenge::new(
            "stage2".to_string(),
            r#"// This is a sample function
fn test() {
    // Print a greeting
    println!("hello");
    let x = 42; // Initialize variable
    return x;
}"#.to_string(),
        )
        .with_source_info("src/lib.rs".to_string(), 10, 16)
        .with_language("rust".to_string()),
        
        Challenge::new(
            "stage3".to_string(),
            r#"fn check_number(x: i32) {
    if x > 0 {
        println!("positive");
    } else if x < 0 {
        // Handle negative case
        println!("negative");
    } else {
        println!("zero");
    }
}"#.to_string(),
        )
        .with_source_info("src/utils.rs".to_string(), 20, 30)
        .with_language("rust".to_string()),
        
        Challenge::new(
            "stage4".to_string(),
            r#"/* Multi-line comment
   with more details */
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            // Recursive case
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
}"#.to_string(),
        )
        .with_source_info("src/algorithms.rs".to_string(), 5, 17)
        .with_language("rust".to_string()),
    ]
}