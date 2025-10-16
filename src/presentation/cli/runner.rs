use crate::domain::repositories::challenge_repository::CHALLENGE_REPOSITORY;
use crate::infrastructure::logging::{setup_console_logging, setup_logging};
use crate::presentation::cli::args::{CacheCommands, RepoCommands};
use crate::presentation::cli::commands::{
    run_export, run_game_session, run_history, run_repo_clear, run_repo_list, run_repo_play,
    run_stats, run_trending,
};
use crate::presentation::cli::{Cli, Commands};
use crate::{GitTypeError, Result};

pub fn run_cli(cli: Cli) -> Result<()> {
    // Initialize logging first for all commands
    if let Err(e) = setup_logging() {
        setup_console_logging();
        eprintln!("⚠️ Warning: Failed to setup file logging: {}", e);
        eprintln!("   Logs will only be shown in console.");
    }

    match &cli.command {
        Some(Commands::History) => run_history(),
        Some(Commands::Stats) => run_stats(),
        Some(Commands::Export { format, output }) => run_export(format.clone(), output.clone()),
        Some(Commands::Cache { cache_command }) => run_cache_command(cache_command),
        Some(Commands::Repo { repo_command }) => run_repo_command(repo_command),
        Some(Commands::Trending {
            language,
            repo_name,
            period,
        }) => run_trending(language.clone(), repo_name.clone(), period.clone()),
        None => run_game_session(cli),
    }
}

fn run_cache_command(cache_command: &CacheCommands) -> Result<()> {
    match cache_command {
        CacheCommands::Stats => match CHALLENGE_REPOSITORY.get_cache_stats() {
            Ok((file_count, total_bytes)) => {
                println!("Challenge Cache Statistics:");
                println!("  Cached repositories: {}", file_count);
                if total_bytes > 0 {
                    if total_bytes < 1024 {
                        println!("  Total size: {} bytes", total_bytes);
                    } else if total_bytes < 1024 * 1024 {
                        println!("  Total size: {:.1} KB", total_bytes as f64 / 1024.0);
                    } else if total_bytes < 1024 * 1024 * 1024 {
                        println!(
                            "  Total size: {:.1} MB",
                            total_bytes as f64 / (1024.0 * 1024.0)
                        );
                    } else {
                        println!(
                            "  Total size: {:.1} GB",
                            total_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
                        );
                    }
                } else {
                    println!("  Total size: 0 bytes");
                }
            }
            Err(e) => {
                eprintln!("Error getting cache stats: {}", e);
                return Err(GitTypeError::TerminalError(e));
            }
        },
        CacheCommands::Clear => match CHALLENGE_REPOSITORY.clear_cache() {
            Ok(()) => {
                println!("Challenge cache cleared successfully.");
            }
            Err(e) => {
                eprintln!("Error clearing cache: {}", e);
                return Err(GitTypeError::TerminalError(e));
            }
        },
        CacheCommands::List => match CHALLENGE_REPOSITORY.list_cache_keys() {
            Ok(keys) => {
                if keys.is_empty() {
                    println!("No cached challenges found.");
                } else {
                    println!("Cached repository keys:");
                    for key in keys {
                        println!("  {}", key);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error listing cache keys: {}", e);
                return Err(GitTypeError::TerminalError(e));
            }
        },
    }

    Ok(())
}

fn run_repo_command(repo_command: &RepoCommands) -> Result<()> {
    match repo_command {
        RepoCommands::List => run_repo_list(),
        RepoCommands::Clear { force } => run_repo_clear(*force),
        RepoCommands::Play => run_repo_play(),
    }
}
