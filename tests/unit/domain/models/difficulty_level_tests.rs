use gittype::domain::models::{ChunkType, CodeChunk, DifficultyLevel};
use std::path::PathBuf;

#[test]
fn test_easy_char_limits() {
    let (min, max) = DifficultyLevel::Easy.char_limits();
    assert_eq!(min, 20);
    assert_eq!(max, 100);
}

#[test]
fn test_normal_char_limits() {
    let (min, max) = DifficultyLevel::Normal.char_limits();
    assert_eq!(min, 80);
    assert_eq!(max, 200);
}

#[test]
fn test_hard_char_limits() {
    let (min, max) = DifficultyLevel::Hard.char_limits();
    assert_eq!(min, 180);
    assert_eq!(max, 500);
}

#[test]
fn test_wild_char_limits() {
    let (min, max) = DifficultyLevel::Wild.char_limits();
    assert_eq!(min, 0);
    assert_eq!(max, usize::MAX);
}

#[test]
fn test_zen_char_limits() {
    let (min, max) = DifficultyLevel::Zen.char_limits();
    assert_eq!(min, 0);
    assert_eq!(max, usize::MAX);
}

#[test]
fn test_easy_description() {
    assert_eq!(DifficultyLevel::Easy.description(), "~100 characters");
}

#[test]
fn test_normal_description() {
    assert_eq!(DifficultyLevel::Normal.description(), "~200 characters");
}

#[test]
fn test_hard_description() {
    assert_eq!(DifficultyLevel::Hard.description(), "~500 characters");
}

#[test]
fn test_wild_description() {
    assert_eq!(DifficultyLevel::Wild.description(), "Full chunks");
}

#[test]
fn test_zen_description() {
    assert_eq!(DifficultyLevel::Zen.description(), "Entire files");
}

#[test]
fn test_easy_subtitle() {
    assert_eq!(DifficultyLevel::Easy.subtitle(), "Short code snippets");
}

#[test]
fn test_normal_subtitle() {
    assert_eq!(DifficultyLevel::Normal.subtitle(), "Medium functions");
}

#[test]
fn test_hard_subtitle() {
    assert_eq!(
        DifficultyLevel::Hard.subtitle(),
        "Long functions or classes"
    );
}

#[test]
fn test_wild_subtitle() {
    assert_eq!(
        DifficultyLevel::Wild.subtitle(),
        "Unpredictable length chunks"
    );
}

#[test]
fn test_zen_subtitle() {
    assert_eq!(
        DifficultyLevel::Zen.subtitle(),
        "Complete files as challenges"
    );
}

#[test]
fn test_applicable_difficulties_for_short_code() {
    let chunk = CodeChunk {
        content: "fn test() {}".to_string(),
        file_path: PathBuf::from("test.rs"),
        start_line: 1,
        end_line: 1,
        language: "rust".to_string(),
        chunk_type: ChunkType::Function,
        name: "test".to_string(),
        comment_ranges: vec![],
        original_indentation: 0,
    };
    let code_char_count = 12;

    let difficulties = DifficultyLevel::applicable_difficulties(&chunk, code_char_count);

    // Short code (12 chars) should not qualify for Easy (min 20)
    assert!(!difficulties.contains(&DifficultyLevel::Easy));
    // Wild is always applicable
    assert!(difficulties.contains(&DifficultyLevel::Wild));
    // Zen only for files
    assert!(!difficulties.contains(&DifficultyLevel::Zen));
}

#[test]
fn test_applicable_difficulties_for_medium_code() {
    let chunk = CodeChunk {
        content: "a".repeat(150),
        file_path: PathBuf::from("test.rs"),
        start_line: 1,
        end_line: 5,
        language: "rust".to_string(),
        chunk_type: ChunkType::Function,
        name: "test".to_string(),
        comment_ranges: vec![],
        original_indentation: 0,
    };
    let code_char_count = 150;

    let difficulties = DifficultyLevel::applicable_difficulties(&chunk, code_char_count);

    // 150 chars should qualify for Easy and Normal
    assert!(difficulties.contains(&DifficultyLevel::Easy));
    assert!(difficulties.contains(&DifficultyLevel::Normal));
    assert!(difficulties.contains(&DifficultyLevel::Wild));
    // Not enough for Hard (min 180)
    assert!(!difficulties.contains(&DifficultyLevel::Hard));
    // Zen only for files
    assert!(!difficulties.contains(&DifficultyLevel::Zen));
}

#[test]
fn test_applicable_difficulties_for_long_code() {
    let chunk = CodeChunk {
        content: "a".repeat(400),
        file_path: PathBuf::from("test.rs"),
        start_line: 1,
        end_line: 20,
        language: "rust".to_string(),
        chunk_type: ChunkType::Function,
        name: "test".to_string(),
        comment_ranges: vec![],
        original_indentation: 0,
    };
    let code_char_count = 400;

    let difficulties = DifficultyLevel::applicable_difficulties(&chunk, code_char_count);

    // 400 chars should qualify for Easy, Normal, Hard, and Wild
    assert!(difficulties.contains(&DifficultyLevel::Easy));
    assert!(difficulties.contains(&DifficultyLevel::Normal));
    assert!(difficulties.contains(&DifficultyLevel::Hard));
    assert!(difficulties.contains(&DifficultyLevel::Wild));
    // Zen only for files
    assert!(!difficulties.contains(&DifficultyLevel::Zen));
}

#[test]
fn test_applicable_difficulties_for_file_chunk() {
    let chunk = CodeChunk {
        content: "a".repeat(1000),
        file_path: PathBuf::from("test.rs"),
        start_line: 1,
        end_line: 50,
        language: "rust".to_string(),
        chunk_type: ChunkType::File,
        name: "test.rs".to_string(),
        comment_ranges: vec![],
        original_indentation: 0,
    };
    let code_char_count = 1000;

    let difficulties = DifficultyLevel::applicable_difficulties(&chunk, code_char_count);

    // File chunks should include Zen
    assert!(difficulties.contains(&DifficultyLevel::Zen));
    // Also all others
    assert!(difficulties.contains(&DifficultyLevel::Easy));
    assert!(difficulties.contains(&DifficultyLevel::Normal));
    assert!(difficulties.contains(&DifficultyLevel::Hard));
    assert!(difficulties.contains(&DifficultyLevel::Wild));
}

#[test]
fn test_difficulty_clone() {
    let difficulty = DifficultyLevel::Normal;
    let cloned = difficulty;
    assert_eq!(difficulty, cloned);
}

#[test]
fn test_difficulty_equality() {
    assert_eq!(DifficultyLevel::Easy, DifficultyLevel::Easy);
    assert_ne!(DifficultyLevel::Easy, DifficultyLevel::Normal);
}

#[test]
fn test_difficulty_serialize_deserialize() {
    let difficulty = DifficultyLevel::Hard;
    let serialized = serde_json::to_string(&difficulty).unwrap();
    let deserialized: DifficultyLevel = serde_json::from_str(&serialized).unwrap();
    assert_eq!(difficulty, deserialized);
}
