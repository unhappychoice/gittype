use gittype::presentation::cli::args::{CacheCommands, RepoCommands};
use gittype::presentation::cli::{run_cli, Cli, Commands};
use gittype::GitTypeError;
use std::process::Command;

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
fn repo_clear_force_keeps_empty_repository_cache_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let repos_dir = temp_dir.path().join(".config").join("repos");
    let placeholder_dir = repos_dir.join("not-a-git-repository").join("nested");
    std::fs::create_dir_all(&placeholder_dir).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_gittype"))
        .args(["repo", "clear", "--force"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(repos_dir.exists());
    assert!(placeholder_dir.exists());
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

#[test]
fn export_command_exits_with_failure_status() {
    let output = Command::new(env!("CARGO_BIN_EXE_gittype"))
        .args(["export", "--format", "csv", "--output", "sessions.csv"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn stats_command_exits_with_not_implemented_status() {
    let output = Command::new(env!("CARGO_BIN_EXE_gittype"))
        .arg("stats")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn history_command_exits_with_not_implemented_status() {
    let output = Command::new(env!("CARGO_BIN_EXE_gittype"))
        .arg("history")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
}
