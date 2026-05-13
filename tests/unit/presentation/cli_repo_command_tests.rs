use gittype::presentation::cli::commands::{run_repo_clear, run_repo_list, run_repo_play};
use gittype::{GitTypeError, Result};

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
fn run_repo_list_returns_terminal_error_without_tty() {
    assert_non_tty_terminal_error(run_repo_list());
}

#[test]
fn run_repo_play_returns_terminal_error_without_tty() {
    assert_non_tty_terminal_error(run_repo_play());
}

#[test]
fn run_repo_clear_with_force_returns_ok_when_no_cached_repos_dir() {
    let result = run_repo_clear(true);

    assert!(
        result.is_ok(),
        "run_repo_clear should succeed and short-circuit when the test-mocks FileStorage \
         reports no repos directory, got {:?}",
        result
    );
}

#[test]
fn run_repo_clear_without_force_returns_ok_when_no_cached_repos_dir() {
    let result = run_repo_clear(false);

    assert!(
        result.is_ok(),
        "run_repo_clear should succeed and short-circuit when the test-mocks FileStorage \
         reports no repos directory, got {:?}",
        result
    );
}
