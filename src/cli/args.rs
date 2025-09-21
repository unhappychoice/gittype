use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gittype")]
#[command(
    about = "A typing practice tool using your own code repositories - extracts all code chunks (functions, classes, methods, etc.)",
    long_about = "GitType turns your own source code into typing challenges. \
                  Practice typing by using functions, classes, and methods from your actual projects. \
                  \n\nExamples:\n  \
                  gittype                           # Use current directory\n  \
                  gittype /path/to/repo             # Use specific repository\n  \
                  gittype --repo owner/repo         # Clone and use GitHub repository\n  \
                  gittype --langs rust,python       # Filter by languages"
)]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Repository path to extract code from (defaults to current directory if not specified)
    #[arg(
        value_name = "REPO_PATH",
        help = "Repository path to extract code from"
    )]
    pub repo_path: Option<PathBuf>,

    /// GitHub repository URL or path to clone and play with
    #[arg(
        long,
        help = "GitHub repository URL or path to clone and play with",
        long_help = "GitHub repository URL or path to clone and play with. \
                     Supports formats:\n  \
                     - owner/repo\n  \
                     - https://github.com/owner/repo\n  \
                     - git@github.com:owner/repo.git"
    )]
    pub repo: Option<String>,

    /// Filter by programming languages (comma-separated)
    #[arg(
        long,
        value_delimiter = ',',
        help = "Filter by programming languages (comma-separated)",
        long_help = "Filter by programming languages (comma-separated). \
                     Supported languages:\n  \
                     rust, typescript, javascript, python, ruby, go, swift, \
                     kotlin, java, php, csharp, c, cpp, haskell, dart, scala\n  \
                     Example: --langs rust,python,typescript"
    )]
    pub langs: Option<Vec<String>>,

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
    /// Manage challenge cache
    Cache {
        #[command(subcommand)]
        cache_command: CacheCommands,
    },
    /// Manage repositories
    Repo {
        #[command(subcommand)]
        repo_command: RepoCommands,
    },
    /// Manage color themes
    Theme {
        #[command(subcommand)]
        theme_command: ThemeCommands,
    },
}

#[derive(Subcommand)]
pub enum CacheCommands {
    /// Show cache statistics
    Stats,
    /// Clear all cached challenges
    Clear,
    /// List cached repository keys
    List,
}
#[derive(Subcommand)]
pub enum RepoCommands {
    /// List all cached repositories
    List,
    /// Clear all cached repositories
    Clear {
        /// Force clear without confirmation
        #[arg(long)]
        force: bool,
    },
    /// Play a cached repository interactively
    Play,
}

#[derive(Subcommand)]
pub enum ThemeCommands {
    /// List available themes
    List,
    /// Set the current theme
    Set {
        /// Theme name (default, original, ascii, or custom theme name)
        theme: String,
    },
    /// Show current theme
    Current,
    /// Set color mode (dark or light)
    Mode {
        /// Color mode (dark or light)
        mode: String,
    },
    /// Toggle color mode between dark and light
    Toggle,
}
