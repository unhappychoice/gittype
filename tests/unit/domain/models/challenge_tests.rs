use crate::fixtures::models::git_repository;
use gittype::domain::models::{Challenge, DifficultyLevel};

#[test]
fn test_new_challenge_basic() {
    let challenge = Challenge::new("test-id".to_string(), "fn main() {}".to_string());

    assert_eq!(challenge.id, "test-id");
    assert_eq!(challenge.code_content, "fn main() {}");
    assert!(challenge.source_file_path.is_none());
    assert!(challenge.start_line.is_none());
    assert!(challenge.end_line.is_none());
    assert!(challenge.language.is_none());
    assert!(challenge.comment_ranges.is_empty());
    assert!(challenge.difficulty_level.is_none());
}

#[test]
fn test_with_source_info() {
    let challenge = Challenge::new("test-id".to_string(), "code".to_string()).with_source_info(
        "/path/to/file.rs".to_string(),
        10,
        20,
    );

    assert_eq!(
        challenge.source_file_path,
        Some("/path/to/file.rs".to_string())
    );
    assert_eq!(challenge.start_line, Some(10));
    assert_eq!(challenge.end_line, Some(20));
}

#[test]
fn test_with_language() {
    let challenge =
        Challenge::new("test-id".to_string(), "code".to_string()).with_language("rust".to_string());

    assert_eq!(challenge.language, Some("rust".to_string()));
}

#[test]
fn test_with_comment_ranges() {
    let ranges = vec![(0, 10), (20, 30)];
    let challenge = Challenge::new("test-id".to_string(), "code".to_string())
        .with_comment_ranges(ranges.clone());

    assert_eq!(challenge.comment_ranges, ranges);
}

#[test]
fn test_with_difficulty_level() {
    let challenge = Challenge::new("test-id".to_string(), "code".to_string())
        .with_difficulty_level(DifficultyLevel::Easy);

    assert_eq!(challenge.difficulty_level, Some(DifficultyLevel::Easy));
}

#[test]
fn test_builder_pattern_chaining() {
    let challenge = Challenge::new("test-id".to_string(), "fn main() {}".to_string())
        .with_source_info("main.rs".to_string(), 1, 5)
        .with_language("rust".to_string())
        .with_comment_ranges(vec![(0, 5)])
        .with_difficulty_level(DifficultyLevel::Normal);

    assert_eq!(challenge.id, "test-id");
    assert_eq!(challenge.code_content, "fn main() {}");
    assert_eq!(challenge.source_file_path, Some("main.rs".to_string()));
    assert_eq!(challenge.start_line, Some(1));
    assert_eq!(challenge.end_line, Some(5));
    assert_eq!(challenge.language, Some("rust".to_string()));
    assert_eq!(challenge.comment_ranges, vec![(0, 5)]);
    assert_eq!(challenge.difficulty_level, Some(DifficultyLevel::Normal));
}

#[test]
fn test_get_display_title_without_source_info() {
    let challenge = Challenge::new("test-123".to_string(), "code".to_string());
    let title = challenge.get_display_title();

    assert_eq!(title, "Challenge test-123");
}

#[test]
fn test_get_display_title_with_source_info() {
    let challenge = Challenge::new("test-id".to_string(), "code".to_string()).with_source_info(
        "/home/user/project/src/main.rs".to_string(),
        10,
        20,
    );

    let title = challenge.get_display_title();

    // Should show relative path with line numbers
    assert!(title.contains("main.rs"));
    assert!(title.contains("10-20"));
}

#[test]
fn test_get_display_title_without_line_numbers() {
    let mut challenge = Challenge::new("test-id".to_string(), "code".to_string());
    challenge.source_file_path = Some("/path/to/file.rs".to_string());

    let title = challenge.get_display_title();

    // Should show file name without line numbers
    assert!(title.contains("file.rs"));
    assert!(!title.contains(":"));
}

#[test]
fn test_get_display_title_with_repo() {
    let challenge = Challenge::new("test-id".to_string(), "code".to_string()).with_source_info(
        "/home/user/project/src/main.rs".to_string(),
        10,
        20,
    );

    let repo = git_repository::build_with_names("octocat", "Hello-World");

    let title = challenge.get_display_title_with_repo(&Some(repo));

    assert!(title.contains("[octocat/Hello-World]"));
    assert!(title.contains("main.rs"));
    assert!(title.contains("10-20"));
}

#[test]
fn test_get_display_title_with_repo_none() {
    let challenge = Challenge::new("test-id".to_string(), "code".to_string()).with_source_info(
        "/path/to/file.rs".to_string(),
        5,
        15,
    );

    let title = challenge.get_display_title_with_repo(&None);

    // Should not include repo info
    assert!(!title.contains("["));
    assert!(title.contains("file.rs"));
    assert!(title.contains("5-15"));
}

#[test]
fn test_get_relative_path_with_parent() {
    let challenge = Challenge::new("test-id".to_string(), "code".to_string()).with_source_info(
        "/home/user/project/src/lib/module.rs".to_string(),
        1,
        10,
    );

    let title = challenge.get_display_title();

    // Should show lib/module.rs (parent/filename)
    assert!(title.contains("lib/module.rs") || title.contains("module.rs"));
}

#[test]
fn test_clone_challenge() {
    let original = Challenge::new("test-id".to_string(), "code".to_string())
        .with_language("rust".to_string())
        .with_difficulty_level(DifficultyLevel::Hard);

    let cloned = original.clone();

    assert_eq!(original.id, cloned.id);
    assert_eq!(original.code_content, cloned.code_content);
    assert_eq!(original.language, cloned.language);
    assert_eq!(original.difficulty_level, cloned.difficulty_level);
}

#[test]
fn test_partial_eq() {
    let challenge1 =
        Challenge::new("test-id".to_string(), "code".to_string()).with_language("rust".to_string());

    let challenge2 =
        Challenge::new("test-id".to_string(), "code".to_string()).with_language("rust".to_string());

    let challenge3 = Challenge::new("different-id".to_string(), "code".to_string())
        .with_language("rust".to_string());

    assert_eq!(challenge1, challenge2);
    assert_ne!(challenge1, challenge3);
}

#[test]
fn test_empty_code_content() {
    let challenge = Challenge::new("test-id".to_string(), "".to_string());

    assert_eq!(challenge.code_content, "");
    // Should still be a valid challenge object
    assert_eq!(challenge.id, "test-id");
}

#[test]
fn test_multiline_code_content() {
    let code = "fn main() {\n    println!(\"Hello\");\n}";
    let challenge = Challenge::new("test-id".to_string(), code.to_string());

    assert_eq!(challenge.code_content, code);
}

#[test]
fn test_special_characters_in_path() {
    let challenge = Challenge::new("test-id".to_string(), "code".to_string()).with_source_info(
        "/path/to/file-with-dashes_and_underscores.rs".to_string(),
        1,
        10,
    );

    let title = challenge.get_display_title();

    // Should handle special characters in filename
    assert!(title.contains("file-with-dashes_and_underscores.rs"));
}

#[test]
fn test_difficulty_levels() {
    let easy = Challenge::new("id1".to_string(), "code".to_string())
        .with_difficulty_level(DifficultyLevel::Easy);
    let normal = Challenge::new("id2".to_string(), "code".to_string())
        .with_difficulty_level(DifficultyLevel::Normal);
    let hard = Challenge::new("id3".to_string(), "code".to_string())
        .with_difficulty_level(DifficultyLevel::Hard);
    let wild = Challenge::new("id4".to_string(), "code".to_string())
        .with_difficulty_level(DifficultyLevel::Wild);

    assert_eq!(easy.difficulty_level, Some(DifficultyLevel::Easy));
    assert_eq!(normal.difficulty_level, Some(DifficultyLevel::Normal));
    assert_eq!(hard.difficulty_level, Some(DifficultyLevel::Hard));
    assert_eq!(wild.difficulty_level, Some(DifficultyLevel::Wild));
}
