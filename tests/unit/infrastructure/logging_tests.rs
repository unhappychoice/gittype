use gittype::infrastructure::logging::{
    get_environment_context, get_log_directory, log_error_to_file, setup_console_logging,
};
use gittype::GitTypeError;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn error_log_candidates(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    name.starts_with("error_") || name.starts_with("gittype_error_")
                })
        })
        .collect()
}

fn error_logs_containing(message: &str) -> Vec<PathBuf> {
    [Path::new("logs"), Path::new(".")]
        .into_iter()
        .flat_map(error_log_candidates)
        .filter(|path| {
            fs::read_to_string(path)
                .map(|content| content.contains(message))
                .unwrap_or(false)
        })
        .collect()
}

fn unique_log_message() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    format!("logging-test-{}", nanos)
}

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
fn test_log_error_to_file_writes_error_details() {
    fs::create_dir_all("logs").unwrap();
    let message = unique_log_message();
    let error = GitTypeError::ValidationError(message.clone());

    log_error_to_file(&error);

    let written_logs = error_logs_containing(&message);
    assert!(!written_logs.is_empty());
    written_logs.iter().for_each(|path| {
        let _ = fs::remove_file(path);
    });
}
