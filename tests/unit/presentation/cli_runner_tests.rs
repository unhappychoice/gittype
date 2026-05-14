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
fn run_cli_executes_repo_list_command_without_tty() {
    if atty::is(atty::Stream::Stdout) {
        return;
    }

    let result = run_cli(make_cli(Commands::Repo {
        repo_command: RepoCommands::List,
    }));

    assert!(matches!(
        result,
        Err(GitTypeError::TerminalError(message))
            if message.contains("Not running in a terminal environment")
    ));
}

#[test]
fn run_cli_executes_repo_play_command_without_tty() {
    if atty::is(atty::Stream::Stdout) {
        return;
    }

    let result = run_cli(make_cli(Commands::Repo {
        repo_command: RepoCommands::Play,
    }));

    assert!(matches!(
        result,
        Err(GitTypeError::TerminalError(message))
            if message.contains("Not running in a terminal environment")
    ));
}

#[test]
fn run_cli_without_command_returns_error_without_tty() {
    if atty::is(atty::Stream::Stdout) {
        return;
    }

    let result = run_cli(Cli {
        repo_path: None,
        repo: None,
        langs: None,
        command: None,
    });

    assert!(matches!(
        result,
        Err(GitTypeError::IoError(_)) | Err(GitTypeError::TerminalError(_))
    ));
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
fn export_command_without_output_exits_with_failure_status() {
    let output = Command::new(env!("CARGO_BIN_EXE_gittype"))
        .args(["export", "--format", "json"])
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

#[derive(Debug)]
struct FailingChallengeRepository;

#[derive(Debug)]
struct SuccessfulChallengeRepository {
    stats: (usize, u64),
    cache_keys: Vec<String>,
}

impl gittype::domain::repositories::challenge_repository::ChallengeRepositoryInterface
    for FailingChallengeRepository
{
    fn save_challenges(
        &self,
        _repo: &gittype::domain::models::GitRepository,
        _challenges: &[gittype::domain::models::Challenge],
        _reporter: Option<
            &dyn gittype::presentation::tui::screens::loading_screen::ProgressReporter,
        >,
    ) -> gittype::Result<()> {
        Ok(())
    }

    fn load_challenges_with_progress(
        &self,
        _repo: &gittype::domain::models::GitRepository,
        _reporter: Option<
            &dyn gittype::presentation::tui::screens::loading_screen::ProgressReporter,
        >,
    ) -> gittype::Result<Option<Vec<gittype::domain::models::Challenge>>> {
        Ok(None)
    }

    fn get_cache_stats(&self) -> gittype::Result<(usize, u64)> {
        Err(GitTypeError::ExtractionFailed("stats failed".to_string()))
    }

    fn clear_cache(&self) -> gittype::Result<()> {
        Err(GitTypeError::ExtractionFailed("clear failed".to_string()))
    }

    fn invalidate_repository(
        &self,
        _repo: &gittype::domain::models::GitRepository,
    ) -> gittype::Result<bool> {
        Ok(false)
    }

    fn list_cache_keys(&self) -> gittype::Result<Vec<String>> {
        Err(GitTypeError::ExtractionFailed("list failed".to_string()))
    }
}

impl gittype::domain::repositories::challenge_repository::ChallengeRepositoryInterface
    for SuccessfulChallengeRepository
{
    fn save_challenges(
        &self,
        _repo: &gittype::domain::models::GitRepository,
        _challenges: &[gittype::domain::models::Challenge],
        _reporter: Option<
            &dyn gittype::presentation::tui::screens::loading_screen::ProgressReporter,
        >,
    ) -> gittype::Result<()> {
        Ok(())
    }

    fn load_challenges_with_progress(
        &self,
        _repo: &gittype::domain::models::GitRepository,
        _reporter: Option<
            &dyn gittype::presentation::tui::screens::loading_screen::ProgressReporter,
        >,
    ) -> gittype::Result<Option<Vec<gittype::domain::models::Challenge>>> {
        Ok(None)
    }

    fn get_cache_stats(&self) -> gittype::Result<(usize, u64)> {
        Ok(self.stats)
    }

    fn clear_cache(&self) -> gittype::Result<()> {
        Ok(())
    }

    fn invalidate_repository(
        &self,
        _repo: &gittype::domain::models::GitRepository,
    ) -> gittype::Result<bool> {
        Ok(false)
    }

    fn list_cache_keys(&self) -> gittype::Result<Vec<String>> {
        Ok(self.cache_keys.clone())
    }
}

#[test]
fn run_cache_command_wraps_repository_errors_as_terminal_errors() {
    let repository = FailingChallengeRepository;

    [
        (CacheCommands::Stats, "stats failed"),
        (CacheCommands::Clear, "clear failed"),
        (CacheCommands::List, "list failed"),
    ]
    .into_iter()
    .for_each(|(command, expected_message)| {
        let result =
            gittype::presentation::cli::runner::run_cache_command_for_test(&command, &repository);

        assert!(matches!(
            result,
            Err(GitTypeError::TerminalError(message))
                if message.contains(expected_message)
        ));
    });
}

#[test]
fn run_cache_command_handles_successful_stats_and_list_outputs() {
    [0u64, 512, 2048, 2 * 1024 * 1024, 2 * 1024 * 1024 * 1024]
        .into_iter()
        .for_each(|total_bytes| {
            let repository = SuccessfulChallengeRepository {
                stats: (2, total_bytes),
                cache_keys: vec!["owner/repo/main".to_string()],
            };

            assert!(
                gittype::presentation::cli::runner::run_cache_command_for_test(
                    &CacheCommands::Stats,
                    &repository,
                )
                .is_ok()
            );
        });

    let repository = SuccessfulChallengeRepository {
        stats: (1, 0),
        cache_keys: vec!["owner/repo/main".to_string()],
    };

    assert!(
        gittype::presentation::cli::runner::run_cache_command_for_test(
            &CacheCommands::Clear,
            &repository,
        )
        .is_ok()
    );
    assert!(
        gittype::presentation::cli::runner::run_cache_command_for_test(
            &CacheCommands::List,
            &repository,
        )
        .is_ok()
    );
}
