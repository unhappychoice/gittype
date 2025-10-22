use gittype::domain::error::GitTypeError;
use std::path::PathBuf;

#[test]
fn database_error_wraps_message() {
    let error = GitTypeError::database_error("failure".into());

    if let GitTypeError::DatabaseError(inner) = error {
        assert!(inner.to_string().contains("failure"));
    } else {
        panic!("expected database error variant");
    }
}

#[test]
fn repository_not_found_displays_path() {
    let path = PathBuf::from("/tmp/nonexistent");
    let error = GitTypeError::RepositoryNotFound(path.clone());
    assert!(error.to_string().contains("/tmp/nonexistent"));
}

#[test]
fn no_supported_files_error_message() {
    let error = GitTypeError::NoSupportedFiles;
    assert_eq!(error.to_string(), "No supported files found in repository");
}

#[test]
fn extraction_failed_displays_message() {
    let error = GitTypeError::ExtractionFailed("parse error".to_string());
    assert!(error.to_string().contains("parse error"));
}

#[test]
fn terminal_error_displays_message() {
    let error = GitTypeError::TerminalError("terminal init failed".to_string());
    assert!(error.to_string().contains("terminal init failed"));
}

#[test]
fn screen_initialization_error_displays_message() {
    let error = GitTypeError::ScreenInitializationError("screen init failed".to_string());
    assert!(error.to_string().contains("screen init failed"));
}

#[test]
fn invalid_repository_format_displays_message() {
    let error = GitTypeError::InvalidRepositoryFormat("bad format".to_string());
    assert!(error.to_string().contains("bad format"));
}

#[test]
fn panic_error_displays_message() {
    let error = GitTypeError::PanicError("panic occurred".to_string());
    assert!(error.to_string().contains("panic occurred"));
}

#[test]
fn api_error_displays_message() {
    let error = GitTypeError::ApiError("API failed".to_string());
    assert!(error.to_string().contains("API failed"));
}

#[test]
fn validation_error_displays_message() {
    let error = GitTypeError::ValidationError("validation failed".to_string());
    assert!(error.to_string().contains("validation failed"));
}

#[test]
fn io_error_conversion() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error: GitTypeError = io_error.into();
    assert!(matches!(error, GitTypeError::IoError(_)));
}

#[test]
fn serde_error_conversion() {
    let json_error = serde_json::from_str::<Vec<i32>>("invalid json").unwrap_err();
    let error: GitTypeError = json_error.into();
    assert!(matches!(error, GitTypeError::SerializationError(_)));
}
