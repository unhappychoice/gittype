use std::process::{Command, ExitStatus};

fn run_gittype(args: &[&str]) -> ExitStatus {
    Command::new(env!("CARGO_BIN_EXE_gittype"))
        .args(args)
        .status()
        .unwrap()
}

#[test]
fn history_command_exits_with_failure_status() {
    let status = run_gittype(&["history"]);

    assert!(!status.success());
    assert_eq!(status.code(), Some(1));
}

#[test]
fn stats_command_exits_with_failure_status() {
    let status = run_gittype(&["stats"]);

    assert!(!status.success());
    assert_eq!(status.code(), Some(1));
}

#[test]
fn export_command_with_options_exits_with_failure_status() {
    let status = run_gittype(&["export", "--format", "csv", "--output", "sessions.csv"]);

    assert!(!status.success());
    assert_eq!(status.code(), Some(1));
}

#[test]
fn invalid_language_filter_exits_with_failure_status() {
    let status = run_gittype(&["--langs", "invalid-language"]);

    assert!(!status.success());
    assert_eq!(status.code(), Some(1));
}
