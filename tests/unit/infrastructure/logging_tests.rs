use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use gittype::domain::error::GitTypeError;
use gittype::infrastructure::logging::{
    get_environment_context, get_log_directory, log_error_to_file, setup_console_logging,
};

#[test]
fn test_get_log_directory() {
    // Skip test if running in a Nix build environment
    if std::env::var("IN_NIX_SHELL").is_ok() || std::env::var("NIX_BUILD_CORES").is_ok() {
        eprintln!("Skipping test in Nix build environment.");
        return;
    }

    let log_dir = get_log_directory().unwrap();

    if cfg!(debug_assertions) {
        // In debug mode, should be project directory
        assert!(log_dir.ends_with("logs"));
    } else {
        // In release mode, should be ~/.gittype/logs
        assert!(log_dir.to_string_lossy().contains(".gittype"));
    }
}

#[test]
fn test_setup_console_logging() {
    // This should not panic
    setup_console_logging();
}

#[test]
fn test_format_panic_info() {
    // This is a simplified test since creating PanicInfo manually is complex
    let context = get_environment_context();

    assert!(context.contains("EXECUTABLE:"));
    assert!(context.contains("WORKING_DIR:"));
    assert!(context.contains("COMMAND_ARGS:"));
}

#[test]
fn test_get_environment_context() {
    let context = get_environment_context();

    assert!(context.contains("EXECUTABLE:"));
    assert!(context.contains("WORKING_DIR:"));
    assert!(context.contains("COMMAND_ARGS:"));
    assert!(context.contains("OS:"));
    assert!(context.contains("ARCH:"));
}

#[test]
fn test_get_environment_context_contains_os() {
    let context = get_environment_context();
    let os_str = std::env::consts::OS;
    assert!(context.contains(os_str));
}

#[test]
fn test_get_environment_context_contains_arch() {
    let context = get_environment_context();
    let arch_str = std::env::consts::ARCH;
    assert!(context.contains(arch_str));
}

#[test]
fn test_setup_console_logging_twice() {
    // Should be able to call multiple times without panicking
    setup_console_logging();
    setup_console_logging();
}

#[test]
fn test_log_error_to_file_writes_detailed_report() {
    let marker = unique_marker();
    let error = GitTypeError::database_error(marker.clone());

    log_error_to_file(&error);

    let matching_logs = find_error_logs(&marker);
    assert!(
        !matching_logs.is_empty(),
        "expected an error log containing the marker"
    );

    let content = fs::read_to_string(&matching_logs[0]).unwrap();
    assert!(content.contains("ERROR OCCURRED AT:"));
    assert!(content.contains("ERROR MESSAGE: Database error:"));
    assert!(content.contains(&marker));
    assert!(content.contains("CAUSED BY (level 1):"));
    assert!(content.contains("WORKING_DIR:"));

    matching_logs.into_iter().for_each(|path| {
        let _ = fs::remove_file(path);
    });
}

fn unique_marker() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("logging-test-{nanos}")
}

fn find_error_logs(marker: &str) -> Vec<PathBuf> {
    fs::read_dir("logs")
        .into_iter()
        .flat_map(|entries| entries.filter_map(|entry| entry.ok()))
        .map(|entry| entry.path())
        .filter(|path| path.file_name().and_then(|name| name.to_str()).is_some())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("error_") && name.ends_with(".log"))
                .unwrap_or(false)
        })
        .filter(|path| {
            fs::read_to_string(path)
                .map(|content| content.contains(marker))
                .unwrap_or(false)
        })
        .collect()
}
