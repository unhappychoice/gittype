use gittype::domain::models::Challenge;
use gittype::domain::services::context_loader::{load_context_for_challenge, load_context_lines};
use gittype::infrastructure::storage::file_storage::FileStorage;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_load_context_lines() {
    let content =
        "line1\nline2\nline3\nTARGET_START\nTARGET_CONTENT\nTARGET_END\nline7\nline8\nline9";
    let temp_file = NamedTempFile::new().unwrap();
    fs::write(&temp_file, content).unwrap();

    let mut file_storage = FileStorage::new();
    file_storage.set_file_content(temp_file.path().to_path_buf(), content.to_string());

    let result = load_context_lines(&file_storage, temp_file.path(), 4, 6, 2).unwrap();

    assert_eq!(result.pre_context, vec!["line2", "line3"]);
    assert_eq!(result.post_context, vec!["line7", "line8"]);
}

#[test]
fn test_load_context_at_file_boundaries() {
    let content = "line1\nTARGET\nline3";
    let temp_file = NamedTempFile::new().unwrap();
    fs::write(&temp_file, content).unwrap();

    let mut file_storage = FileStorage::new();
    file_storage.set_file_content(temp_file.path().to_path_buf(), content.to_string());

    let result = load_context_lines(&file_storage, temp_file.path(), 2, 2, 5).unwrap();

    assert_eq!(result.pre_context, vec!["line1"]);
    assert_eq!(result.post_context, vec!["line3"]);
}

#[test]
fn load_context_for_challenge_returns_empty_context_without_source_info() {
    let challenge_without_source = Challenge::new("no-source".to_string(), "target".to_string());
    let context = load_context_for_challenge(&challenge_without_source, 2, None).unwrap();
    assert!(context.pre_context.is_empty());
    assert!(context.post_context.is_empty());

    let challenge_without_start = Challenge {
        source_file_path: Some("src/lib.rs".to_string()),
        ..Challenge::new("no-start".to_string(), "target".to_string())
    };
    let context = load_context_for_challenge(&challenge_without_start, 2, None).unwrap();
    assert!(context.pre_context.is_empty());
    assert!(context.post_context.is_empty());

    let challenge_without_end = Challenge {
        source_file_path: Some("src/lib.rs".to_string()),
        start_line: Some(3),
        ..Challenge::new("no-end".to_string(), "target".to_string())
    };
    let context = load_context_for_challenge(&challenge_without_end, 2, None).unwrap();
    assert!(context.pre_context.is_empty());
    assert!(context.post_context.is_empty());
}

#[test]
fn load_context_for_challenge_returns_empty_context_for_missing_relative_file() {
    let challenge = Challenge::new("missing-file".to_string(), "target".to_string())
        .with_source_info("src/missing.rs".to_string(), 2, 4);

    let context = load_context_for_challenge(&challenge, 2, None).unwrap();

    assert!(context.pre_context.is_empty());
    assert!(context.post_context.is_empty());
}
