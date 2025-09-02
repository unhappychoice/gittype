use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gittype")]
#[command(
    about = "A typing practice tool using your own code repositories - extracts all code chunks (functions, classes, methods, etc.)"
)]
#[command(version = "0.1.0")]
pub struct Cli {
    /// Repository path to extract code from
    #[arg(value_name = "REPO_PATH")]
    pub repo_path: Option<PathBuf>,

    /// GitHub repository URL or path to clone and play with (e.g., owner/repo, https://github.com/owner/repo, git@github.com:owner/repo.git)
    #[arg(long)]
    pub repo: Option<String>,

    /// Filter by programming languages
    #[arg(long, value_delimiter = ',')]
    pub langs: Option<Vec<String>>,

    /// Number of stages for normal mode
    #[arg(long, default_value_t = 3)]
    pub stages: usize,

    /// Glob patterns for files to include
    #[arg(long)]
    pub include: Option<Vec<String>>,

    /// Glob patterns for files to exclude
    #[arg(long)]
    pub exclude: Option<Vec<String>>,

    /// Path to config file
    #[arg(long)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
