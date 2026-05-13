use gittype::presentation::cli::screen_runner::{run_screen, ScreenRunnerContext};
use gittype::presentation::tui::screens::RepoListScreen;
use gittype::presentation::tui::ScreenType;
use gittype::GitTypeError;

fn assert_non_tty_terminal_error(result: gittype::Result<impl Sized>) {
    assert!(matches!(
        result,
        Err(GitTypeError::TerminalError(message))
            if message.contains("Not running in a terminal environment")
    ));
}

#[test]
fn run_screen_returns_terminal_error_without_tty() {
    if atty::is(atty::Stream::Stdout) {
        return;
    }

    let result = run_screen::<RepoListScreen, (), (), fn(&RepoListScreen) -> Option<()>>(
        ScreenType::RepoList,
        None::<()>,
        None,
    );

    assert_non_tty_terminal_error(result);
}

#[test]
fn screen_runner_context_new_returns_terminal_error_without_tty() {
    if atty::is(atty::Stream::Stdout) {
        return;
    }

    let result = ScreenRunnerContext::new();

    assert_non_tty_terminal_error(result);
}

#[test]
fn cleanup_succeeds_when_terminal_is_inactive() {
    let context = ScreenRunnerContext::new_for_test(false);

    assert!(context.cleanup().is_ok());
}

#[test]
fn cleanup_succeeds_when_terminal_is_active() {
    let context = ScreenRunnerContext::new_for_test(true);

    assert!(context.cleanup().is_ok());
}
