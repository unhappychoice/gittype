use gittype::infrastructure::logging::{
    get_environment_context, get_log_directory, log_error_to_file, log_panic_to_file,
    setup_console_logging,
};
use gittype::GitTypeError;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex, MutexGuard,
};
use std::time::{SystemTime, UNIX_EPOCH};

static PANIC_HOOK_TEST_LOCK: Mutex<()> = Mutex::new(());

fn panic_hook_test_lock() -> MutexGuard<'static, ()> {
    PANIC_HOOK_TEST_LOCK
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

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

fn with_env_var<T>(key: &str, value: &str, f: impl FnOnce() -> T) -> T {
    let previous = std::env::var_os(key);
    std::env::set_var(key, value);
    let result = f();

    if let Some(previous) = previous {
        std::env::set_var(key, previous);
    } else {
        std::env::remove_var(key);
    }

    result
}

fn log_panicked_payload(f: impl FnOnce() + std::panic::UnwindSafe) -> bool {
    let _guard = panic_hook_test_lock();
    let previous_hook = std::panic::take_hook();
    let was_logged = Arc::new(AtomicBool::new(false));
    let hook_was_logged = Arc::clone(&was_logged);

    std::panic::set_hook(Box::new(move |panic_info| {
        log_panic_to_file(panic_info);
        hook_was_logged.store(true, Ordering::SeqCst);
    }));

    let result = std::panic::catch_unwind(f);
    std::panic::set_hook(previous_hook);

    assert!(result.is_err());
    was_logged.load(Ordering::SeqCst)
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
fn test_get_environment_context_contains_rust_env_vars() {
    with_env_var("RUST_BACKTRACE", "full", || {
        with_env_var("RUST_LOG", "gittype=debug", || {
            let context = get_environment_context();

            assert!(context.contains("RUST_BACKTRACE: full"));
            assert!(context.contains("RUST_LOG: gittype=debug"));
        });
    });
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

#[test]
fn test_log_panic_to_file_handles_str_payload() {
    assert!(log_panicked_payload(|| panic!("logging panic str payload")));
}

#[test]
fn test_log_panic_to_file_handles_string_payload() {
    let message = unique_log_message();

    assert!(log_panicked_payload(|| {
        std::panic::panic_any(message);
    }));
}
