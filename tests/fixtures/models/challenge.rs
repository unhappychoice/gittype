//! Test fixtures for Challenge model

use gittype::domain::models::{Challenge, DifficultyLevel};

/// Creates a default test Challenge
pub fn build() -> Challenge {
    Challenge::new(
        "test-challenge-id".to_string(),
        "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
    )
    .with_language("rust".to_string())
    .with_difficulty_level(DifficultyLevel::Normal)
}

/// Creates a Challenge with custom ID
pub fn build_with_id(id: &str) -> Challenge {
    Challenge::new(
        id.to_string(),
        "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
    )
    .with_language("rust".to_string())
    .with_difficulty_level(DifficultyLevel::Normal)
}

/// Creates a Challenge with custom ID and code content
pub fn build_with_id_and_code(id: &str, code: &str) -> Challenge {
    Challenge::new(id.to_string(), code.to_string())
        .with_language("rust".to_string())
        .with_difficulty_level(DifficultyLevel::Normal)
}

/// Creates a Challenge with source information
pub fn build_with_source_info(path: &str, start_line: usize, end_line: usize) -> Challenge {
    Challenge::new(
        "test-challenge-id".to_string(),
        "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
    )
    .with_source_info(path.to_string(), start_line, end_line)
    .with_language("rust".to_string())
    .with_difficulty_level(DifficultyLevel::Normal)
}

/// Creates a Challenge with specific difficulty level
pub fn build_with_difficulty(difficulty: DifficultyLevel) -> Challenge {
    Challenge::new(
        "test-challenge-id".to_string(),
        "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
    )
    .with_language("rust".to_string())
    .with_difficulty_level(difficulty)
}

/// Creates an easy Challenge
pub fn build_easy() -> Challenge {
    build_with_difficulty(DifficultyLevel::Easy)
}

/// Creates a hard Challenge
pub fn build_hard() -> Challenge {
    build_with_difficulty(DifficultyLevel::Hard)
}
