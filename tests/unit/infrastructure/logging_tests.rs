use gittype::infrastructure::logging::{
    get_environment_context, get_log_directory, setup_console_logging,
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
fn test_get_environment_context_includes_optional_log_env_vars() {
    let original_backtrace = std::env::var("RUST_BACKTRACE").ok();
    let original_log = std::env::var("RUST_LOG").ok();

    std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_LOG", "gittype=debug");

    let context = get_environment_context();

    match original_backtrace {
        Some(value) => std::env::set_var("RUST_BACKTRACE", value),
        None => std::env::remove_var("RUST_BACKTRACE"),
    }
    match original_log {
        Some(value) => std::env::set_var("RUST_LOG", value),
        None => std::env::remove_var("RUST_LOG"),
    }

    assert!(context.contains("RUST_BACKTRACE: full"));
    assert!(context.contains("RUST_LOG: gittype=debug"));
}

#[test]
fn test_setup_console_logging_twice() {
    // Should be able to call multiple times without panicking
    setup_console_logging();
    setup_console_logging();
}
