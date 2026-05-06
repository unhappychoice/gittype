use crate::domain::repositories::challenge_repository::ChallengeRepositoryInterface;
use crate::infrastructure::logging::{setup_console_logging, setup_logging};
use crate::presentation::cli::args::{CacheCommands, RepoCommands};
use crate::presentation::cli::commands::{
    run_export, run_game_session, run_history, run_repo_clear, run_repo_list, run_repo_play,
    run_stats, run_trending,
};
use crate::presentation::cli::{Cli, Commands};
use crate::presentation::di::AppModule;
use crate::{GitTypeError, Result};
use shaku::HasComponent;

pub fn run_cli(cli: Cli) -> Result<()> {
    if let Err(e) = setup_logging() {
        setup_console_logging();
        eprintln!("⚠️ Warning: Failed to setup file logging: {}", e);
        eprintln!("   Logs will only be shown in console.");
    }

    match &cli.command {
        Some(Commands::History) => run_history(),
        Some(Commands::Stats) => run_stats(),
        Some(Commands::Export { format, output }) => run_export(format.clone(), output.clone()),
        Some(Commands::Cache { cache_command }) => {
            let module = AppModule::builder().build();
            let challenge_repository: &dyn ChallengeRepositoryInterface = module.resolve_ref();
            run_cache_command(cache_command, challenge_repository)
        }
        Some(Commands::Repo { repo_command }) => run_repo_command(repo_command),
        Some(Commands::Trending {
            language,
            repo_name,
            period,
        }) => run_trending(language.clone(), repo_name.clone(), period.clone()),
        None => run_game_session(cli),
    }
}

fn run_cache_command(
    cache_command: &CacheCommands,
    challenge_repository: &dyn ChallengeRepositoryInterface,
) -> Result<()> {
    match cache_command {
        CacheCommands::Stats => match challenge_repository.get_cache_stats() {
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
                return Err(GitTypeError::TerminalError(e.to_string()));
            }
        },
        CacheCommands::Clear => match challenge_repository.clear_cache() {
            Ok(()) => {
                println!("Challenge cache cleared successfully.");
            }
            Err(e) => {
                eprintln!("Error clearing cache: {}", e);
                return Err(GitTypeError::TerminalError(e.to_string()));
            }
        },
        CacheCommands::List => match challenge_repository.list_cache_keys() {
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
                return Err(GitTypeError::TerminalError(e.to_string()));
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

#[cfg(test)]
mod tests {
    use super::run_cache_command;
    use crate::domain::models::{Challenge, GitRepository};
    use crate::domain::repositories::challenge_repository::ChallengeRepositoryInterface;
    use crate::presentation::cli::args::CacheCommands;
    use crate::presentation::tui::screens::loading_screen::ProgressReporter;
    use crate::{GitTypeError, Result};

    #[derive(Debug)]
    struct MockChallengeRepository {
        stats: std::result::Result<(usize, u64), String>,
        keys: Vec<String>,
    }

    impl MockChallengeRepository {
        fn with_stats(total_bytes: u64) -> Self {
            Self {
                stats: Ok((2, total_bytes)),
                keys: vec![],
            }
        }

        fn with_keys(keys: Vec<String>) -> Self {
            Self {
                stats: Ok((0, 0)),
                keys,
            }
        }

        fn with_stats_error(message: &str) -> Self {
            Self {
                stats: Err(message.to_string()),
                keys: vec![],
            }
        }
    }

    impl ChallengeRepositoryInterface for MockChallengeRepository {
        fn save_challenges(
            &self,
            _repo: &GitRepository,
            _challenges: &[Challenge],
            _reporter: Option<&dyn ProgressReporter>,
        ) -> Result<()> {
            Ok(())
        }

        fn load_challenges_with_progress(
            &self,
            _repo: &GitRepository,
            _reporter: Option<&dyn ProgressReporter>,
        ) -> Result<Option<Vec<Challenge>>> {
            Ok(None)
        }

        fn get_cache_stats(&self) -> Result<(usize, u64)> {
            self.stats
                .clone()
                .map_err(|message| GitTypeError::TerminalError(message.to_string()))
        }

        fn clear_cache(&self) -> Result<()> {
            Ok(())
        }

        fn invalidate_repository(&self, _repo: &GitRepository) -> Result<bool> {
            Ok(false)
        }

        fn list_cache_keys(&self) -> Result<Vec<String>> {
            Ok(self.keys.clone())
        }
    }

    #[test]
    fn run_cache_stats_accepts_each_size_bucket() {
        [512, 2 * 1024, 2 * 1024 * 1024, 2 * 1024 * 1024 * 1024]
            .into_iter()
            .map(MockChallengeRepository::with_stats)
            .for_each(|repository| {
                let result = run_cache_command(&CacheCommands::Stats, &repository);

                assert!(result.is_ok());
            });
    }

    #[test]
    fn run_cache_list_accepts_existing_keys() {
        let repository = MockChallengeRepository::with_keys(vec![
            "github.com/owner/first:abc123".to_string(),
            "github.com/owner/second:def456".to_string(),
        ]);

        let result = run_cache_command(&CacheCommands::List, &repository);

        assert!(result.is_ok());
    }

    #[test]
    fn run_cache_stats_wraps_repository_error() {
        let repository = MockChallengeRepository::with_stats_error("stats unavailable");

        let result = run_cache_command(&CacheCommands::Stats, &repository);

        assert!(matches!(
            result,
            Err(GitTypeError::TerminalError(message))
                if message == "Terminal error: stats unavailable"
        ));
    }
}
