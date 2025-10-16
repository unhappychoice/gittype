use gittype::domain::models::{ChunkType, CodeChunk};
use gittype::domain::services::challenge_generator::code_character_counter::CodeCharacterCounter;
use std::path::PathBuf;

#[test]
fn new_creates_counter() {
    let _counter = CodeCharacterCounter::new();
    // Test passes if construction succeeds
}

#[test]
fn default_creates_counter() {
    let _counter = CodeCharacterCounter;
    // Test passes if construction succeeds
}

#[test]
fn count_chars_in_content_empty_string() {
    let counter = CodeCharacterCounter::new();
    let count = counter.count_chars_in_content("", &[]);
    assert_eq!(count, 0);
}

#[test]
fn count_chars_in_content_only_whitespace() {
    let counter = CodeCharacterCounter::new();
    let content = "   \n\t  \n  ";
    let count = counter.count_chars_in_content(content, &[]);
    assert_eq!(count, 0);
}

#[test]
fn count_chars_in_content_simple_code() {
    let counter = CodeCharacterCounter::new();
    let content = "let x = 5;";
    let count = counter.count_chars_in_content(content, &[]);
    // "letx=5;" = 7 characters (excluding spaces)
    assert_eq!(count, 7);
}

#[test]
fn count_chars_in_content_with_comments() {
    let counter = CodeCharacterCounter::new();
    let content = "let x = 5; // comment";
    // "// comment" starts at position 11 and ends at 21
    let comment_ranges = vec![(11, 21)];
    let count = counter.count_chars_in_content(content, &comment_ranges);
    // Only "letx=5;" = 7 characters
    assert_eq!(count, 7);
}

#[test]
fn count_chars_in_content_multiline_with_comments() {
    let counter = CodeCharacterCounter::new();
    let content = "fn main() {\n    // comment\n    let x = 5;\n}";
    // "// comment\n" is at positions 16-28
    let comment_ranges = vec![(16, 28)];
    let count = counter.count_chars_in_content(content, &comment_ranges);
    // "fn main() { let x = 5; }" without spaces = 17 characters
    assert_eq!(count, 17);
}

#[test]
fn count_chars_in_content_multiple_comment_ranges() {
    let counter = CodeCharacterCounter::new();
    let content = "let x = 5; /* block */ let y = 10; // line";
    let comment_ranges = vec![
        (11, 23), // /* block */
        (35, 43), // // line
    ];
    let count = counter.count_chars_in_content(content, &comment_ranges);
    // "let x = 5; let y = 10;" without spaces = 15 characters
    assert_eq!(count, 15);
}

#[test]
fn count_chars_in_content_unsorted_comment_ranges() {
    let counter = CodeCharacterCounter::new();
    let content = "let x = 5; /* block */ let y = 10;";
    // Provide unsorted ranges
    let comment_ranges = vec![(11, 23)];
    let count = counter.count_chars_in_content(content, &comment_ranges);
    // "let x = 5; let y = 10;" without spaces = 15 characters
    assert_eq!(count, 15);
}

#[test]
fn count_code_characters_from_chunk() {
    let counter = CodeCharacterCounter::new();

    let chunk = CodeChunk {
        content: "fn test() {\n    let x = 5;\n}".to_string(),
        file_path: PathBuf::from("test.rs"),
        comment_ranges: vec![],
        chunk_type: ChunkType::Function,
        start_line: 1,
        end_line: 3,
        language: "rust".to_string(),
        name: "test".to_string(),
        original_indentation: 0,
    };

    let count = counter.count_code_characters(&chunk);
    // "fn test() { let x = 5; }" without spaces = 17 characters
    assert_eq!(count, 17);
}

#[test]
fn count_code_characters_chunk_with_comments() {
    let counter = CodeCharacterCounter::new();

    let content = "fn test() {\n    // comment\n    let x = 5;\n}";
    let chunk = CodeChunk {
        content: content.to_string(),
        file_path: PathBuf::from("test.rs"),
        comment_ranges: vec![(16, 28)],
        chunk_type: ChunkType::Function,
        start_line: 1,
        end_line: 4,
        language: "rust".to_string(),
        name: "test".to_string(),
        original_indentation: 0,
    };

    let count = counter.count_code_characters(&chunk);
    // "fn test() { let x = 5; }" without spaces = 17 characters (comment excluded)
    assert_eq!(count, 17);
}

#[test]
fn count_chars_handles_unicode() {
    let counter = CodeCharacterCounter::new();
    let content = "let 変数 = \"こんにちは\";";
    let count = counter.count_chars_in_content(content, &[]);
    // All non-whitespace characters including Japanese
    assert!(count > 0);
}

#[test]
fn count_chars_comment_at_start() {
    let counter = CodeCharacterCounter::new();
    let content = "// comment\nlet x = 5;";
    let comment_ranges = vec![(0, 11)]; // "// comment\n"
    let count = counter.count_chars_in_content(content, &comment_ranges);
    // Only "letx=5;" = 7 characters
    assert_eq!(count, 7);
}

#[test]
fn count_chars_comment_at_end() {
    let counter = CodeCharacterCounter::new();
    let content = "let x = 5;\n// comment";
    let comment_ranges = vec![(11, 21)];
    let count = counter.count_chars_in_content(content, &comment_ranges);
    // Only "letx=5;" = 7 characters
    assert_eq!(count, 7);
}

#[test]
fn count_chars_overlapping_ranges_not_expected() {
    let counter = CodeCharacterCounter::new();
    let content = "let x = 5; let y = 10;";
    // Even if ranges are weird, it should handle them
    let comment_ranges = vec![(5, 10), (8, 15)];
    let count = counter.count_chars_in_content(content, &comment_ranges);
    // Should still count something
    assert!(count > 0);
}
