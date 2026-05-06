use gittype::presentation::cli::commands::run_trending;
use gittype::GitTypeError;

fn assert_non_tty_terminal_error(result: gittype::Result<()>) {
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
fn run_trending_rejects_unsupported_language() {
    let result = run_trending(
        Some("invalid-language".to_string()),
        None,
        "daily".to_string(),
    );

    assert!(matches!(
        result,
        Err(GitTypeError::ValidationError(message))
        if message == "Unsupported language: invalid-language"
    ));
}

#[test]
fn run_trending_accepts_supported_language_before_repo_name_validation() {
    let result = run_trending(
        Some("rUsT".to_string()),
        Some("repoonly".to_string()),
        "weekly".to_string(),
    );

    assert!(result.is_ok());
}

#[test]
fn run_trending_with_direct_repo_returns_raw_mode_io_error_without_tty() {
    if atty::is(atty::Stream::Stdout) {
        return;
    }

    let result = run_trending(
        Some("Rust".to_string()),
        Some("owner/repo".to_string()),
        "daily".to_string(),
    );

    assert!(matches!(result, Err(GitTypeError::IoError(_))));
}

#[test]
fn run_trending_with_language_returns_terminal_error_without_tty() {
    let result = run_trending(Some("Rust".to_string()), None, "daily".to_string());

    assert_non_tty_terminal_error(result);
}

#[test]
fn run_trending_without_language_returns_terminal_error_without_tty() {
    let result = run_trending(None, None, "daily".to_string());

    assert_non_tty_terminal_error(result);
}
