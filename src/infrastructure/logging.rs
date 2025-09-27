use crate::{domain::error::GitTypeError, Result};
use chrono;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};
use std::path::PathBuf;
use std::sync::OnceLock;

static CURRENT_LOG_FILE: OnceLock<String> = OnceLock::new();

pub fn setup_logging() -> Result<()> {
    let log_dir = get_log_directory()?;
    std::fs::create_dir_all(&log_dir)?;

    // Create timestamp-based log filename
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let log_file = log_dir.join(format!("gittype_{}.log", timestamp));

    // Store the log file path for later retrieval
    let _ = CURRENT_LOG_FILE.set(log_file.display().to_string());

    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} [{l}] {t} - {m}\n",
        )))
        .build(log_file)
        .map_err(|e| {
            GitTypeError::database_error(format!("Failed to create file appender: {}", e))
        })?;

    // Console appender for development/debugging (only shows warnings and errors by default)
    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{l}] {m}\n")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .logger(Logger::builder().build("globset", log::LevelFilter::Off))
        .logger(Logger::builder().build("ignore", log::LevelFilter::Off))
        .build(Root::builder().appender("file").build(get_log_level()))
        .map_err(|e| GitTypeError::database_error(format!("Failed to build log config: {}", e)))?;

    log4rs::init_config(config).map_err(|e| {
        GitTypeError::database_error(format!("Failed to initialize logging: {}", e))
    })?;

    log::info!(
        "GitType logging initialized, logs saved to: {}",
        log_dir.display()
    );
    Ok(())
}

/// Get the log directory path (temp in tests, project/logs/ in dev, ~/.gittype/logs/ in release)
pub fn get_log_directory() -> Result<PathBuf> {
    if cfg!(test) {
        // Test: use temporary directory (each test gets its own)
        use tempfile::TempDir;
        let temp_dir = TempDir::new().map_err(|e| {
            GitTypeError::ExtractionFailed(format!("Failed to create temp log directory: {}", e))
        })?;
        let path = temp_dir.path().join("logs");
        std::fs::create_dir_all(&path).map_err(|e| {
            GitTypeError::ExtractionFailed(format!("Failed to create test log directory: {}", e))
        })?;
        // Keep the temp dir alive for the test duration
        std::mem::forget(temp_dir);
        Ok(path)
    } else if cfg!(debug_assertions) {
        // Development: use project directory
        let current_dir = std::env::current_dir().map_err(|e| {
            GitTypeError::ExtractionFailed(format!("Could not get current directory: {}", e))
        })?;
        Ok(current_dir.join("logs"))
    } else {
        // Release: use home directory
        let home_dir = dirs::home_dir().ok_or_else(|| {
            GitTypeError::ExtractionFailed("Could not determine home directory".to_string())
        })?;
        Ok(home_dir.join(".gittype").join("logs"))
    }
}

/// Get the current log file path (the actual file being used)
pub fn get_current_log_file_path() -> Option<String> {
    CURRENT_LOG_FILE.get().cloned()
}

/// Get appropriate log level based on build configuration
fn get_log_level() -> log::LevelFilter {
    if cfg!(debug_assertions) {
        // Development builds: show all logs including debug
        log::LevelFilter::Debug
    } else {
        // Release builds: only show info and above
        log::LevelFilter::Info
    }
}

/// Setup minimal console-only logging (fallback if file logging fails)
pub fn setup_console_logging() {
    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{l}] {m}\n")))
        .build();

    if let Ok(config) = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .build(
            Root::builder()
                .appender("console")
                .build(log::LevelFilter::Warn), // Only warnings and errors
        )
    {
        let _ = log4rs::init_config(config);
    }
}

/// Log panic information to file with detailed context
pub fn log_panic_to_file(panic_info: &std::panic::PanicHookInfo) {
    let panic_message = format_panic_info(panic_info);

    // Try to log using the existing logger first (if available)
    if try_log_panic(&panic_message).is_err() {
        // Fallback to writing directly to panic log file
        write_panic_to_file(&panic_message);
    }
}

/// Log application error to file with detailed context
pub fn log_error_to_file(error: &GitTypeError) {
    use std::error::Error;

    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let mut error_info = String::new();

    error_info.push_str(&format!("ERROR OCCURRED AT: {}\n", timestamp));
    error_info.push_str(&format!(
        "ERROR TYPE: {:?}\n",
        std::any::type_name_of_val(error)
    ));
    error_info.push_str(&format!("ERROR MESSAGE: {}\n", error));

    // Add error chain if available
    let mut current_error = error.source();
    let mut level = 1;
    while let Some(err) = current_error {
        error_info.push_str(&format!("CAUSED BY (level {}): {}\n", level, err));
        current_error = err.source();
        level += 1;
    }

    // Add environment context
    error_info.push_str(&get_environment_context());

    // Try to log using the existing logger first
    log::error!("APPLICATION ERROR:\n{}", error_info);

    // Also write to dedicated error log file
    write_error_to_file(&error_info);
}

fn format_panic_info(panic_info: &std::panic::PanicHookInfo) -> String {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

    let mut info = String::new();
    info.push_str(&format!("PANIC OCCURRED AT: {}\n", timestamp));
    info.push_str(&format!("PANIC INFO: {}\n", panic_info));

    if let Some(location) = panic_info.location() {
        info.push_str(&format!(
            "PANIC LOCATION: {}:{}:{}\n",
            location.file(),
            location.line(),
            location.column()
        ));
    }

    if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
        info.push_str(&format!("PANIC MESSAGE: {}\n", payload));
    } else if let Some(payload) = panic_info.payload().downcast_ref::<String>() {
        info.push_str(&format!("PANIC MESSAGE: {}\n", payload));
    }

    // Add environment context
    info.push_str(&get_environment_context());

    info
}

fn get_environment_context() -> String {
    use std::env;

    let mut context = String::new();

    // Add executable info
    if let Ok(exe) = env::current_exe() {
        context.push_str(&format!("EXECUTABLE: {:?}\n", exe));
    }

    if let Ok(cwd) = env::current_dir() {
        context.push_str(&format!("WORKING_DIR: {:?}\n", cwd));
    }

    // Add command line args
    let args: Vec<String> = env::args().collect();
    context.push_str(&format!("COMMAND_ARGS: {:?}\n", args));

    // Add relevant environment variables
    if let Ok(rust_backtrace) = env::var("RUST_BACKTRACE") {
        context.push_str(&format!("RUST_BACKTRACE: {}\n", rust_backtrace));
    }

    if let Ok(rust_log) = env::var("RUST_LOG") {
        context.push_str(&format!("RUST_LOG: {}\n", rust_log));
    }

    // Add system info
    context.push_str(&format!("OS: {}\n", env::consts::OS));
    context.push_str(&format!("ARCH: {}\n", env::consts::ARCH));

    context
}

fn try_log_panic(message: &str) -> std::result::Result<(), ()> {
    log::error!("PANIC DETAILS:\n{}", message);
    Ok(())
}

fn write_panic_to_file(message: &str) {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let panic_log_path = format!("logs/panic_{}.log", timestamp);

    if write_to_log_file(&panic_log_path, message).is_err() {
        // Fallback to current directory
        let fallback_path = format!("gittype_panic_{}.log", timestamp);
        if let Ok(()) = write_to_log_file(&fallback_path, message) {
            eprintln!("💾 Panic details written to: {}", fallback_path);
        }
    } else {
        eprintln!("💾 Panic details written to: {}", panic_log_path);
    }
}

fn write_error_to_file(error_info: &str) {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let error_log_path = format!("logs/error_{}.log", timestamp);

    if write_to_log_file(&error_log_path, error_info).is_err() {
        // Fallback to current directory
        let fallback_path = format!("gittype_error_{}.log", timestamp);
        if let Ok(()) = write_to_log_file(&fallback_path, error_info) {
            eprintln!("💾 Error details written to: {}", fallback_path);
        }
    } else {
        eprintln!("💾 Error details written to: {}", error_log_path);
    }
}

fn write_to_log_file(path: &str, content: &str) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    writeln!(file, "{}", content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
