use gittype::infrastructure::storage::app_data_provider::AppDataProvider;
use gittype::presentation::cli::commands::{run_repo_clear, run_repo_list, run_repo_play};
use gittype::{GitTypeError, Result};
use std::sync::{Mutex, MutexGuard};

static REPO_CACHE_LOCK: Mutex<()> = Mutex::new(());

struct TestRepoCommand;
impl AppDataProvider for TestRepoCommand {}

fn lock_repo_cache() -> MutexGuard<'static, ()> {
    REPO_CACHE_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn assert_non_tty_terminal_error(result: Result<()>) {
    if atty::is(atty::Stream::Stdout) {
        return;
    }

    assert!(matches!(
        result,
        Err(GitTypeError::TerminalError(message))
            if message.contains("Not running in a terminal environment")
    ));
}

#[test]
fn run_repo_clear_returns_ok_when_cache_directory_is_missing() {
    let _guard = lock_repo_cache();
    let repos_dir = TestRepoCommand::get_app_data_dir().unwrap().join("repos");
    let _ = std::fs::remove_dir_all(&repos_dir);

    let result = run_repo_clear(true);

    assert!(result.is_ok());
}

#[test]
fn run_repo_clear_returns_ok_when_cache_directory_has_no_git_repositories() {
    let _guard = lock_repo_cache();
    let repos_dir = TestRepoCommand::get_app_data_dir().unwrap().join("repos");
    let nested_dir = repos_dir.join("owner").join("repo").join("src");
    let _ = std::fs::remove_dir_all(&repos_dir);
    std::fs::create_dir_all(&nested_dir).unwrap();

    let result = run_repo_clear(true);

    assert!(result.is_ok());
    assert!(repos_dir.exists());

    std::fs::remove_dir_all(&repos_dir).unwrap();
}

#[test]
fn run_repo_list_returns_terminal_error_without_tty() {
    assert_non_tty_terminal_error(run_repo_list());
}

#[test]
fn run_repo_play_returns_terminal_error_without_tty() {
    assert_non_tty_terminal_error(run_repo_play());
}
