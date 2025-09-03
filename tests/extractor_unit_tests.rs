use gittype::extractor::models::language::LanguageRegistry;
use gittype::extractor::{ChallengeConverter, CodeExtractor, ExtractionOptions, RepositoryLoader};
use gittype::models::{ChunkType, CodeChunk};
use gittype::GitTypeError;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tempfile::TempDir;

// Basic extractor tests
#[test]
fn test_extraction_options_default() {
    let options = ExtractionOptions::default();
    assert!(options.include_patterns.contains(&"**/*.rs".to_string()));
    assert!(options
        .exclude_patterns
        .contains(&"**/target/**".to_string()));
}

#[test]
fn test_language_from_extension() {
    assert_eq!(
        LanguageRegistry::from_extension("rs").map(|l| l.name().to_string()),
        Some("rust".to_string())
    );
    assert_eq!(
        LanguageRegistry::from_extension("ts").map(|l| l.name().to_string()),
        Some("typescript".to_string())
    );
    assert_eq!(
        LanguageRegistry::from_extension("tsx").map(|l| l.name().to_string()),
        Some("typescript".to_string())
    );
    assert_eq!(
        LanguageRegistry::from_extension("js").map(|l| l.name().to_string()),
        Some("javascript".to_string())
    );
    assert_eq!(
        LanguageRegistry::from_extension("mjs").map(|l| l.name().to_string()),
        Some("javascript".to_string())
    );
    assert_eq!(
        LanguageRegistry::from_extension("cjs").map(|l| l.name().to_string()),
        Some("javascript".to_string())
    );
    assert_eq!(
        LanguageRegistry::from_extension("py").map(|l| l.name().to_string()),
        Some("python".to_string())
    );
    assert_eq!(
        LanguageRegistry::from_extension("rb").map(|l| l.name().to_string()),
        Some("ruby".to_string())
    );
    assert_eq!(
        LanguageRegistry::from_extension("go").map(|l| l.name().to_string()),
        Some("go".to_string())
    );
    assert_eq!(LanguageRegistry::from_extension("unknown"), None);
}

#[test]
fn test_code_extractor_creation() {
    let extractor = CodeExtractor::new();
    assert!(extractor.is_ok());
}

#[test]
fn test_gitignore_respected() {
    let temp_dir = TempDir::new().unwrap();
    fs::create_dir_all(temp_dir.path().join("src")).unwrap();
    fs::create_dir_all(temp_dir.path().join("target/debug")).unwrap();

    // Initialize git repository
    std::process::Command::new("git")
        .arg("init")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to initialize git repository");

    // Create .gitignore file
    let gitignore_content = "/target/\n*.log.*\n";
    fs::write(temp_dir.path().join(".gitignore"), gitignore_content).unwrap();

    // Create files
    let src_file = temp_dir.path().join("src/main.rs");
    let target_file = temp_dir.path().join("target/debug/main.rs");
    let log_file = temp_dir.path().join("debug.log.rs");

    let rust_code = r#"
fn main() {
    println!("Hello, world!");
}

fn test_function() {
    // This is a test function
}

struct TestStruct {
    value: i32,
}
"#;
    fs::write(&src_file, rust_code).unwrap();
    fs::write(&target_file, rust_code).unwrap();
    fs::write(&log_file, rust_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let mut options = ExtractionOptions::default();
    // Remove tmp/** pattern for this test since we're using a temp directory
    options.exclude_patterns.retain(|p| p != "**/tmp/**");

    let chunks = extractor.extract_chunks(temp_dir.path(), options).unwrap();

    // Should find multiple chunks (functions + struct) but all from src/main.rs
    assert!(
        chunks.len() >= 3,
        "Expected at least 3 chunks, found {}",
        chunks.len()
    );

    // All chunks should be from src/main.rs (not from target/ or debug.log.rs)
    for chunk in &chunks {
        assert!(
            chunk.file_path.to_string_lossy().contains("src/main.rs"),
            "Found chunk from unexpected file: {}",
            chunk.file_path.display()
        );
    }
    assert!(!chunks[0].file_path.to_string_lossy().contains("target"));

    for chunk in &chunks {
        assert!(!chunk.file_path.to_string_lossy().contains(".log"));
    }
}

// Challenge converter tests
#[test]
fn test_convert_chunk_to_challenge() {
    // CodeChunk is now imported from models at the top

    let converter = ChallengeConverter::new();
    let chunk = CodeChunk {
        content: "fn test() {\n    println!(\"test\");\n}".to_string(),
        file_path: PathBuf::from("src/main.rs"),
        start_line: 10,
        end_line: 12,
        language: "rust".to_string(),
        chunk_type: ChunkType::Function,
        name: "test".to_string(),
        comment_ranges: vec![],
        original_indentation: 0,
    };

    let challenge = converter.convert_chunk_to_challenge(chunk);

    assert_eq!(
        challenge.code_content,
        "fn test() {\n    println!(\"test\");\n}"
    );
    assert_eq!(challenge.source_file_path, Some("src/main.rs".to_string()));
    assert_eq!(challenge.start_line, Some(10));
    assert_eq!(challenge.end_line, Some(12));
    assert_eq!(challenge.language, Some("rust".to_string()));
    assert!(!challenge.id.is_empty());
}

// Repository loader tests
#[test]
fn test_load_challenges_from_repository() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let rust_code = r#"
fn hello_world() {
    println!("Hello, world!");
}

struct Person {
    name: String,
}
"#;
    fs::write(&file_path, rust_code).unwrap();

    let mut loader = RepositoryLoader::new().unwrap();
    let mut options = ExtractionOptions::default();
    // Remove tmp/** pattern for this test since we're using a temp directory
    options.exclude_patterns.retain(|p| p != "**/tmp/**");

    let challenges = loader
        .load_challenges_from_repository(temp_dir.path(), Some(options))
        .unwrap();

    // Repository loader may filter out too-small chunks by difficulty thresholds.
    // Ensure at least one challenge is produced from repository contents.
    assert!(!challenges.is_empty());
    assert!(challenges[0].source_file_path.is_some());
    assert!(challenges[0].language.is_some());
    assert!(!challenges[0].id.is_empty());
}

#[test]
fn test_repository_not_found() {
    let mut loader = RepositoryLoader::new().unwrap();
    let result = loader.load_challenges_from_repository(Path::new("/nonexistent/path"), None);

    assert!(matches!(result, Err(GitTypeError::RepositoryNotFound(_))));
}

#[test]
fn test_parallel_ast_parsing_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple test files with different languages
    for i in 0..10 {
        let rust_file = temp_dir.path().join(format!("test_{}.rs", i));
        fs::write(
            &rust_file,
            format!(
                r#"
fn function_{}() {{
    println!("Function {{}}", {});
}}

struct Struct{} {{
    field: i32,
}}

impl Struct{} {{
    fn method_{}(&self) -> i32 {{
        self.field + {}
    }}
}}
"#,
                i, i, i, i, i, i
            ),
        )
        .unwrap();

        let ts_file = temp_dir.path().join(format!("test_{}.ts", i));
        fs::write(
            &ts_file,
            format!(
                r#"
function tsFunction{}(x: number): number {{
    return x * {};
}}

class TsClass{} {{
    private value: number = {};

    public getValue(): number {{
        return this.value;
    }}
}}
"#,
                i, i, i, i
            ),
        )
        .unwrap();
    }

    let mut extractor = CodeExtractor::new().unwrap();
    let mut options = ExtractionOptions::default();
    // Remove tmp/** pattern for this test since we're using a temp directory
    options.exclude_patterns.retain(|p| p != "**/tmp/**");

    let start = Instant::now();
    let chunks = extractor.extract_chunks(temp_dir.path(), options).unwrap();
    let duration = start.elapsed();

    // Should extract functions, structs, impls, and classes from all files
    assert!(
        chunks.len() >= 40,
        "Expected at least 40 chunks, got {}",
        chunks.len()
    ); // 10 files * (1 fn + 1 struct + 1 impl + 1 ts function + 1 ts class) = 50 minimum

    println!("Parallel extraction of {} files took {:?}", 20, duration);
    println!("Found {} total code chunks", chunks.len());

    // Verify we have different types of chunks
    let function_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .count();
    let struct_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Struct))
        .count();
    let class_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .count();

    println!(
        "Types found: {} functions, {} structs, {} classes",
        function_count, struct_count, class_count
    );

    assert!(function_count >= 20, "Should find at least 20 functions");
    assert!(struct_count >= 10, "Should find at least 10 structs");
    assert!(class_count >= 10, "Should find at least 10 classes");

    // Performance test - should complete reasonably quickly
    assert!(
        duration.as_millis() < 5000,
        "Parallel parsing should complete within 5 seconds"
    );
}
