use gittype::domain::error::GitTypeError;
use std::any::Any;
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

#[test]
fn boxed_any_send_conversion() {
    let boxed: Box<dyn Any + Send> = Box::new(42_u8);
    let error: GitTypeError = boxed.into();
    assert!(matches!(
        error,
        GitTypeError::ScreenInitializationError(message) if message == "Data type mismatch"
    ));
}

#[test]
fn boxed_any_conversion() {
    let boxed: Box<dyn Any> = Box::new("screen");
    let error: GitTypeError = boxed.into();
    assert!(matches!(
        error,
        GitTypeError::ScreenInitializationError(message) if message == "Data type mismatch"
    ));
}

#[test]
fn glob_pattern_error_conversion() {
    let pattern_error = glob::Pattern::new("[").unwrap_err();
    let error: GitTypeError = pattern_error.into();
    assert!(matches!(error, GitTypeError::GlobPatternError(_)));
}

#[test]
fn walkdir_error_conversion() {
    let temp_dir = tempfile::tempdir().unwrap();
    let missing_path = temp_dir.path().join("missing");
    let walkdir_error = walkdir::WalkDir::new(&missing_path)
        .into_iter()
        .next()
        .unwrap()
        .unwrap_err();
    let error: GitTypeError = walkdir_error.into();
    assert!(matches!(error, GitTypeError::WalkDirError(_)));
}

#[test]
fn git2_error_conversion() {
    let git2_error = git2::Error::from_str("clone failed");
    let error: GitTypeError = git2_error.into();
    assert!(matches!(error, GitTypeError::RepositoryCloneError(_)));
}

#[test]
fn reqwest_error_conversion() {
    let http_error = reqwest::Client::new().get("::").build().unwrap_err();
    let error: GitTypeError = http_error.into();
    assert!(matches!(error, GitTypeError::HttpError(_)));
}
