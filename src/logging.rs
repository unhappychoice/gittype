use crate::{error::GitTypeError, Result};
use chrono;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::path::PathBuf;

pub fn setup_logging() -> Result<()> {
    let log_dir = get_log_directory()?;
    std::fs::create_dir_all(&log_dir)?;

    // Create timestamp-based log filename
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let log_file = log_dir.join(format!("gittype_{}.log", timestamp));

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

/// Get the log directory path (project/logs/ in dev, ~/.gittype/logs/ in release)
fn get_log_directory() -> Result<PathBuf> {
    if cfg!(debug_assertions) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_log_directory() {
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
}
