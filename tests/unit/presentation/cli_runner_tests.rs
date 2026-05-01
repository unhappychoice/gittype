use gittype::presentation::cli::args::{CacheCommands, RepoCommands};
use gittype::presentation::cli::{run_cli, Cli, Commands};
use gittype::GitTypeError;

fn make_cli(command: Commands) -> Cli {
    Cli {
        repo_path: None,
        repo: None,
        langs: None,
        command: Some(command),
    }
}

#[test]
fn run_cli_executes_cache_stats_command() {
    let result = run_cli(make_cli(Commands::Cache {
        cache_command: CacheCommands::Stats,
    }));

    assert!(result.is_ok());
}

#[test]
fn run_cli_executes_cache_list_command() {
    let result = run_cli(make_cli(Commands::Cache {
        cache_command: CacheCommands::List,
    }));

    assert!(result.is_ok());
}

#[test]
fn run_cli_executes_cache_clear_command() {
    let result = run_cli(make_cli(Commands::Cache {
        cache_command: CacheCommands::Clear,
    }));

    assert!(result.is_ok());
}

#[test]
fn run_cli_executes_repo_clear_force_command() {
    let result = run_cli(make_cli(Commands::Repo {
        repo_command: RepoCommands::Clear { force: true },
    }));

    assert!(result.is_ok());
}

#[test]
fn run_cli_returns_validation_error_for_invalid_trending_language() {
    let result = run_cli(make_cli(Commands::Trending {
        language: Some("invalid-language".to_string()),
        repo_name: None,
        period: "daily".to_string(),
    }));

    assert!(matches!(
        result,
        Err(GitTypeError::ValidationError(message))
        if message == "Unsupported language: invalid-language"
    ));
}
