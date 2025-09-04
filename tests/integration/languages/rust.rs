use gittype::extractor::{CodeExtractor, RepositoryLoader};
use gittype::models::ChunkType;
use std::fs;
use tempfile::TempDir;

use crate::integration::test_extraction_options;

fn setup_git_repo(dir_path: &std::path::Path) {
    // Initialize git repository
    std::process::Command::new("git")
        .args(&["init"])
        .current_dir(dir_path)
        .output()
        .expect("Failed to init git repo");
    
    // Set up basic git config
    std::process::Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(dir_path)
        .output()
        .expect("Failed to set git user.name");
    
    std::process::Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(dir_path)
        .output()
        .expect("Failed to set git user.email");

    // Add a remote URL to avoid "Failed to get remote URL" error
    std::process::Command::new("git")
        .args(&["remote", "add", "origin", "https://github.com/test/test.git"])
        .current_dir(dir_path)
        .output()
        .expect("Failed to add remote");
}

#[test]
fn test_rust_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let rust_code = r#"
fn hello_world() {
    println!("Hello, world!");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;
    fs::write(&file_path, rust_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].name, "hello_world");
    assert_eq!(chunks[1].name, "add");
    assert!(matches!(chunks[0].chunk_type, ChunkType::Function));
    assert!(matches!(chunks[1].chunk_type, ChunkType::Function));
}

#[test]
fn test_rust_struct_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let rust_code = r#"
struct Person {
    name: String,
    age: u32,
}

pub struct Config {
    debug: bool,
}
"#;
    fs::write(&file_path, rust_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].name, "Person");
    assert_eq!(chunks[1].name, "Config");
    assert!(matches!(chunks[0].chunk_type, ChunkType::Struct));
}

#[test]
fn test_rust_enum_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let rust_code = r#"
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Color {
    Red,
    Green,
    Blue,
}
"#;
    fs::write(&file_path, rust_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 2);

    let enum_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Enum))
        .collect();
    assert_eq!(enum_chunks.len(), 2);

    let enum_names: Vec<&String> = enum_chunks.iter().map(|c| &c.name).collect();
    assert!(enum_names.contains(&&"Result".to_string()));
    assert!(enum_names.contains(&&"Color".to_string()));
}

#[test]
fn test_rust_trait_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let rust_code = r#"
pub trait Display {
    fn fmt(&self) -> String;
    
    fn to_string(&self) -> String {
        self.fmt()
    }
}

trait Clone {
    fn clone(&self) -> Self;
}
"#;
    fs::write(&file_path, rust_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 3); // 2 traits + 1 function from trait

    let trait_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Trait))
        .collect();
    assert_eq!(trait_chunks.len(), 2);

    let trait_names: Vec<&String> = trait_chunks.iter().map(|c| &c.name).collect();
    assert!(trait_names.contains(&&"Display".to_string()));
    assert!(trait_names.contains(&&"Clone".to_string()));
}

#[test]
fn test_rust_module_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let rust_code = r#"
pub mod utils {
    pub fn helper() -> i32 { 
        42 
    }
    
    pub struct Config {
        value: String,
    }
}

mod private_utils {
    fn internal_function() {}
}
"#;
    fs::write(&file_path, rust_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 5); // 2 modules + 1 function + 1 struct + 1 function from private module

    let module_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Module))
        .collect();
    assert_eq!(module_chunks.len(), 2);

    let module_names: Vec<&String> = module_chunks.iter().map(|c| &c.name).collect();
    assert!(module_names.contains(&&"utils".to_string()));
    assert!(module_names.contains(&&"private_utils".to_string()));
}

#[test]
fn test_rust_type_alias_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let rust_code = r#"
pub type UserId = u64;
pub type DatabaseResult<T> = Result<T, String>;
type Point = (f64, f64);
"#;
    fs::write(&file_path, rust_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 3);

    let type_alias_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::TypeAlias))
        .collect();
    assert_eq!(type_alias_chunks.len(), 3);

    let type_names: Vec<&String> = type_alias_chunks.iter().map(|c| &c.name).collect();
    assert!(type_names.contains(&&"UserId".to_string()));
    assert!(type_names.contains(&&"DatabaseResult".to_string()));
    assert!(type_names.contains(&&"Point".to_string()));
}

#[test]
fn test_rust_all_constructs_combined() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let rust_code = r#"
// Enum with variants
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Trait definition
pub trait Display {
    fn fmt(&self) -> String;
}

// Module definition
pub mod utils {
    pub fn helper() -> i32 { 
        42 
    }
}

// Type alias
pub type UserId = u64;

// Existing constructs (should still work)
pub struct User {
    id: UserId,
    name: String,
}

impl Display for User {
    fn fmt(&self) -> String {
        format!("User({})", self.name)
    }
}

pub fn create_user(name: String) -> User {
    User {
        id: 1,
        name,
    }
}
"#;
    fs::write(&file_path, rust_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 9); // 1 enum + 1 trait + 1 module + 1 type_alias + 1 struct + 1 impl + 2 functions + 1 nested function

    // Verify each chunk type
    let enum_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Enum))
        .count();
    let trait_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Trait))
        .count();
    let module_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Module))
        .count();
    let type_alias_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::TypeAlias))
        .count();
    let struct_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Struct))
        .count();
    let class_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .count(); // impl
    let function_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .count();

    assert_eq!(enum_count, 1, "Should find 1 enum");
    assert_eq!(trait_count, 1, "Should find 1 trait");
    assert_eq!(module_count, 1, "Should find 1 module");
    assert_eq!(type_alias_count, 1, "Should find 1 type alias");
    assert_eq!(struct_count, 1, "Should find 1 struct");
    assert_eq!(class_count, 1, "Should find 1 impl (class)");
    assert_eq!(function_count, 3, "Should find 3 functions");

    // Verify names
    let chunk_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(chunk_names.contains(&&"Result".to_string()));
    assert!(chunk_names.contains(&&"Display".to_string()));
    assert!(chunk_names.contains(&&"utils".to_string()));
    assert!(chunk_names.contains(&&"UserId".to_string()));
    assert!(chunk_names.contains(&&"User".to_string()));
    assert!(chunk_names.contains(&&"create_user".to_string()));
    assert!(chunk_names.contains(&&"helper".to_string()));
}

#[test]
fn test_nested_and_oneline_structures() {
    let rust_code = r#"mod calculator {
    pub struct Calculator;

    impl Calculator {
        pub fn new() -> Self { Self }

        pub fn complex_calculation(&self, values: &[i32]) -> i32 {
            values.iter().sum()
        }
    }

    impl Default for Calculator {
        fn default() -> Self {
            Self::new()
        }
    }

    mod advanced {
        use super::Calculator;

        impl Calculator {
            pub fn advanced_method(&self) -> String {
                "advanced".to_string()
            }
        }
    }
}
"#;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("test.rs");
    fs::write(&file_path, rust_code).expect("Failed to write test file");
    
    // Setup git repository
    setup_git_repo(temp_dir.path());

    let mut loader = RepositoryLoader::new().expect("Failed to create loader");
    let options = test_extraction_options();

    let challenges = loader
        .load_challenges_from_repository(temp_dir.path(), Some(options))
        .expect("Failed to load challenges");

    println!("Found {} challenges", challenges.len());

    for (i, challenge) in challenges.iter().enumerate() {
        println!("\n=== Challenge {} ===", i + 1);
        println!("Raw content:");
        for (line_num, line) in challenge.code_content.lines().enumerate() {
            println!("  {}: '{}'", line_num + 1, line);
        }

        // Apply processing (indentation normalization is now done in extractor)
        let (processed, mapped_ranges) = gittype::game::text_processor::TextProcessor::process_challenge_text_with_comment_mapping(
            &challenge.code_content,
            &challenge.comment_ranges
        );

        println!("\nFinal normalized content:");
        for (line_num, line) in processed.lines().enumerate() {
            println!("  {}: '{}'", line_num + 1, line);
        }

        println!("Comment ranges: {:?}", mapped_ranges);
    }

}

#[test]
fn test_comment_ranges_in_real_challenge() {
    let rust_code = r#"// Sample function with comments
fn calculate_sum(a: i32, b: i32) -> i32 {
    let result = a + b; // Add the numbers
    /*
     * Return the result
     */
    result
}
"#;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("test.rs");
    fs::write(&file_path, rust_code).expect("Failed to write test file");
    
    // Setup git repository
    setup_git_repo(temp_dir.path());

    let mut loader = RepositoryLoader::new().expect("Failed to create loader");
    let options = test_extraction_options();

    let challenges = loader
        .load_challenges_from_repository(temp_dir.path(), Some(options))
        .expect("Failed to load challenges");

    println!("Found {} challenges for comment test", challenges.len());
    for (i, challenge) in challenges.iter().enumerate() {
        println!(
            "Challenge {}: '{}'",
            i + 1,
            challenge.code_content.replace('\n', "\\n")
        );
    }

    // The extractor now creates both function-based and file-based challenges
    assert!(!challenges.is_empty(), "Expected at least 1 challenge");

    let challenge = &challenges[0];
    println!("Challenge content: '{}'", challenge.code_content);
    println!("Comment ranges: {:?}", challenge.comment_ranges);

    let chars: Vec<char> = challenge.code_content.chars().collect();
    println!("Content length: {} chars", chars.len());

    for (start, end) in &challenge.comment_ranges {
        if *end <= chars.len() {
            let comment_text: String = chars[*start..*end].iter().collect();
            println!("Comment at {}-{}: '{}'", start, end, comment_text);

            // Verify it's actually a comment
            assert!(
                comment_text.starts_with("//") || comment_text.starts_with("/*"),
                "Text at {}-{} should be a comment but got: '{}'",
                start,
                end,
                comment_text
            );
        } else {
            panic!(
                "Comment range {}-{} exceeds content length {}",
                start,
                end,
                chars.len()
            );
        }
    }
}
