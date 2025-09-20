use crate::cli::args::{CacheCommands, Cli, Commands, RepoCommands, ThemeCommands};
use crate::cli::commands::{
    run_export, run_game_session, run_history, run_repo_clear, run_repo_list, run_repo_play,
    run_stats,
};
use crate::logging::{setup_console_logging, setup_logging};
use crate::Result;

pub async fn run_cli(cli: Cli) -> Result<()> {
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
        Some(Commands::Theme { theme_command }) => run_theme_command(theme_command),
        None => run_game_session(cli),
    }
}

fn run_cache_command(cache_command: &CacheCommands) -> Result<()> {
    use crate::cache::CHALLENGE_CACHE;

    match cache_command {
        CacheCommands::Stats => match CHALLENGE_CACHE.stats() {
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
                return Err(crate::GitTypeError::TerminalError(e));
            }
        },
        CacheCommands::Clear => match CHALLENGE_CACHE.clear() {
            Ok(()) => {
                println!("Challenge cache cleared successfully.");
            }
            Err(e) => {
                eprintln!("Error clearing cache: {}", e);
                return Err(crate::GitTypeError::TerminalError(e));
            }
        },
        CacheCommands::List => match CHALLENGE_CACHE.list_keys() {
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
                return Err(crate::GitTypeError::TerminalError(e));
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

fn run_theme_command(theme_command: &ThemeCommands) -> Result<()> {
    use crate::config::{Theme, ThemeManager};

    let mut theme_manager = ThemeManager::new()
        .map_err(|e| crate::GitTypeError::TerminalError(e.to_string()))?;

    match theme_command {
        ThemeCommands::List => {
            println!("Available themes:");
            for theme in theme_manager.list_themes() {
                let current_indicator = if *theme_manager.get_current_theme() == match theme.as_str() {
                    "dark" => Theme::Dark,
                    "light" => Theme::Light,
                    "dark_original" => Theme::DarkOriginal,
                    "light_original" => Theme::LightOriginal,
                    "ascii" => Theme::Ascii,
                    name => Theme::Custom(name.to_string()),
                } {
                    " (current)"
                } else {
                    ""
                };
                println!("  {}{}", theme, current_indicator);
            }
        }
        ThemeCommands::Set { theme } => {
            let theme_enum = match theme.as_str() {
                "dark" => Theme::Dark,
                "light" => Theme::Light,
                "dark_original" => Theme::DarkOriginal,
                "light_original" => Theme::LightOriginal,
                "ascii" => Theme::Ascii,
                name => Theme::Custom(name.to_string()),
            };

            theme_manager.set_theme(theme_enum)
                .map_err(|e| crate::GitTypeError::TerminalError(e.to_string()))?;

            println!("Theme set to: {}", theme);
        }
        ThemeCommands::Current => {
            let current = match theme_manager.get_current_theme() {
                Theme::Dark => "dark",
                Theme::Light => "light",
                Theme::DarkOriginal => "dark_original",
                Theme::LightOriginal => "light_original",
                Theme::Ascii => "ascii",
                Theme::Custom(name) => name,
            };
            println!("Current theme: {}", current);
        }
    }

    Ok(())
}
