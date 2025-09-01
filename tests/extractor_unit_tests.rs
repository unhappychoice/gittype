use gittype::extractor::{
    ChallengeConverter, ChunkType, CodeExtractor, ExtractionOptions, Language, RepositoryLoader,
};
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
    assert_eq!(Language::from_extension("rs"), Some(Language::Rust));
    assert_eq!(Language::from_extension("ts"), Some(Language::TypeScript));
    assert_eq!(Language::from_extension("tsx"), Some(Language::TypeScript));
    assert_eq!(Language::from_extension("py"), Some(Language::Python));
    assert_eq!(Language::from_extension("rb"), Some(Language::Ruby));
    assert_eq!(Language::from_extension("go"), Some(Language::Go));
    assert_eq!(Language::from_extension("unknown"), None);
}

#[test]
fn test_code_extractor_creation() {
    let extractor = CodeExtractor::new();
    assert!(extractor.is_ok());
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
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
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
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].name, "Person");
    assert_eq!(chunks[1].name, "Config");
    assert!(matches!(chunks[0].chunk_type, ChunkType::Struct));
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

    let rust_code = r#"fn test() {}"#;
    fs::write(&src_file, rust_code).unwrap();
    fs::write(&target_file, rust_code).unwrap();
    fs::write(&log_file, rust_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Should only find src/main.rs
    assert_eq!(chunks.len(), 1);
    assert!(chunks[0]
        .file_path
        .to_string_lossy()
        .contains("src/main.rs"));
    assert!(!chunks[0].file_path.to_string_lossy().contains("target"));

    for chunk in &chunks {
        assert!(!chunk.file_path.to_string_lossy().contains(".log"));
    }
}

// Challenge converter tests
#[test]
fn test_convert_chunk_to_challenge() {
    use gittype::extractor::CodeChunk;

    let converter = ChallengeConverter::new();
    let chunk = CodeChunk {
        content: "fn test() {\n    println!(\"test\");\n}".to_string(),
        file_path: PathBuf::from("src/main.rs"),
        start_line: 10,
        end_line: 12,
        language: Language::Rust,
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
    let challenges = loader
        .load_challenges_from_repository(temp_dir.path(), None)
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
    let options = ExtractionOptions::default();

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

#[test]
fn test_ruby_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rb");

    let ruby_code = r#"
def hello_world
  puts "Hello, world!"
end

def calculate_sum(a, b)
  a + b
end
"#;
    fs::write(&file_path, ruby_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].name, "hello_world");
    assert_eq!(chunks[1].name, "calculate_sum");
    assert!(matches!(chunks[0].chunk_type, ChunkType::Method));
    assert!(matches!(chunks[1].chunk_type, ChunkType::Method));
}

#[test]
fn test_ruby_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rb");

    let ruby_code = r#"
class Person
  attr_accessor :name, :age
  
  def initialize(name, age)
    @name = name
    @age = age
  end
  
  def greet
    puts "Hello, I'm #{@name}!"
  end
end
"#;
    fs::write(&file_path, ruby_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 3); // class + 2 methods

    // Find class chunk
    let class_chunk = chunks
        .iter()
        .find(|c| matches!(c.chunk_type, ChunkType::Class))
        .unwrap();
    assert_eq!(class_chunk.name, "Person");

    // Find method chunks
    let method_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Method))
        .collect();
    assert_eq!(method_chunks.len(), 2);

    let method_names: Vec<&String> = method_chunks.iter().map(|c| &c.name).collect();
    assert!(method_names.contains(&&"initialize".to_string()));
    assert!(method_names.contains(&&"greet".to_string()));
}

#[test]
fn test_ruby_module_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rb");

    let ruby_code = r#"
module Authentication
  def login(username, password)
    puts "Logging in #{username}"
    true
  end
  
  def logout
    puts "Logged out"
  end
end
"#;
    fs::write(&file_path, ruby_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 3); // module + 2 methods

    // Find module chunk
    let module_chunk = chunks
        .iter()
        .find(|c| matches!(c.chunk_type, ChunkType::Module))
        .unwrap();
    assert_eq!(module_chunk.name, "Authentication");

    // Find method chunks
    let method_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Method))
        .collect();
    assert_eq!(method_chunks.len(), 2);

    let method_names: Vec<&String> = method_chunks.iter().map(|c| &c.name).collect();
    assert!(method_names.contains(&&"login".to_string()));
    assert!(method_names.contains(&&"logout".to_string()));
}

#[test]
fn test_go_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.go");

    let go_code = r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, world!")
}

func add(a, b int) int {
    return a + b
}

func multiply(x int, y int) int {
    return x * y
}
"#;
    fs::write(&file_path, go_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 3);

    let function_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"main".to_string()));
    assert!(function_names.contains(&&"add".to_string()));
    assert!(function_names.contains(&&"multiply".to_string()));

    for chunk in &chunks {
        assert!(matches!(chunk.chunk_type, ChunkType::Function));
        assert_eq!(chunk.language, Language::Go);
    }
}

#[test]
fn test_go_struct_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.go");

    let go_code = r#"package main

type Person struct {
    Name string
    Age  int
}

type Address struct {
    Street string
    City   string
    Zip    string
}

func (p Person) GetName() string {
    return p.Name
}

func (a *Address) GetFullAddress() string {
    return a.Street + ", " + a.City + " " + a.Zip
}
"#;
    fs::write(&file_path, go_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 4); // 2 structs + 2 methods

    // Find struct chunks
    let struct_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Struct))
        .collect();
    assert_eq!(struct_chunks.len(), 2);

    let struct_names: Vec<&String> = struct_chunks.iter().map(|c| &c.name).collect();
    assert!(struct_names.contains(&&"Person".to_string()));
    assert!(struct_names.contains(&&"Address".to_string()));

    // Find method chunks
    let method_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Method))
        .collect();
    assert_eq!(method_chunks.len(), 2);

    let method_names: Vec<&String> = method_chunks.iter().map(|c| &c.name).collect();
    assert!(method_names.contains(&&"GetName".to_string()));
    assert!(method_names.contains(&&"GetFullAddress".to_string()));
}

#[test]
fn test_go_interface_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.go");

    let go_code = r#"package main

type Writer interface {
    Write([]byte) (int, error)
}

type Reader interface {
    Read([]byte) (int, error)
}

type ReadWriter interface {
    Reader
    Writer
}

func process(rw ReadWriter) {
    // Implementation here
}
"#;
    fs::write(&file_path, go_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 4); // 3 interfaces + 1 function

    // Find interface chunks
    let interface_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Interface))
        .collect();
    assert_eq!(interface_chunks.len(), 3);

    let interface_names: Vec<&String> = interface_chunks.iter().map(|c| &c.name).collect();
    assert!(interface_names.contains(&&"Writer".to_string()));
    assert!(interface_names.contains(&&"Reader".to_string()));
    assert!(interface_names.contains(&&"ReadWriter".to_string()));

    // Find function chunk
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();
    assert_eq!(function_chunks.len(), 1);
    assert_eq!(function_chunks[0].name, "process");
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
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
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
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
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
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
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
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
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
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
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

// TypeScript new language construct tests
#[test]
fn test_typescript_interface_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    let ts_code = r#"
interface User {
    id: number;
    name: string;
    email?: string;
}

interface Admin extends User {
    permissions: string[];
}
"#;
    fs::write(&file_path, ts_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 2);

    let interface_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Interface))
        .collect();
    assert_eq!(interface_chunks.len(), 2);

    let interface_names: Vec<&String> = interface_chunks.iter().map(|c| &c.name).collect();
    assert!(interface_names.contains(&&"User".to_string()));
    assert!(interface_names.contains(&&"Admin".to_string()));
}

#[test]
fn test_typescript_type_alias_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    let ts_code = r#"
type Status = 'pending' | 'completed' | 'failed';
type UserId = number;
type ApiResponse<T> = {
    data: T;
    error?: string;
};
"#;
    fs::write(&file_path, ts_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 3);

    let type_alias_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::TypeAlias))
        .collect();
    assert_eq!(type_alias_chunks.len(), 3);

    let type_names: Vec<&String> = type_alias_chunks.iter().map(|c| &c.name).collect();
    assert!(type_names.contains(&&"Status".to_string()));
    assert!(type_names.contains(&&"UserId".to_string()));
    assert!(type_names.contains(&&"ApiResponse".to_string()));
}

#[test]
fn test_typescript_enum_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    let ts_code = r#"
enum Color {
    Red = '#ff0000',
    Green = '#00ff00',
    Blue = '#0000ff'
}

enum Status {
    Pending,
    Completed,
    Failed
}
"#;
    fs::write(&file_path, ts_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 2);

    let enum_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Enum))
        .collect();
    assert_eq!(enum_chunks.len(), 2);

    let enum_names: Vec<&String> = enum_chunks.iter().map(|c| &c.name).collect();
    assert!(enum_names.contains(&&"Color".to_string()));
    assert!(enum_names.contains(&&"Status".to_string()));
}

#[test]
fn test_typescript_namespace_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    let ts_code = r#"
namespace Utils {
    export function formatDate(date: Date): string {
        return date.toISOString();
    }

    export function calculateAge(birthDate: Date): number {
        const today = new Date();
        return today.getFullYear() - birthDate.getFullYear();
    }
}

namespace Api {
    export interface Response<T> {
        data: T;
        status: number;
    }
}
"#;
    fs::write(&file_path, ts_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Should find 2 namespaces + 2 functions + 1 interface = 5 total
    assert!(chunks.len() >= 2, "Should find at least 2 namespace chunks");

    let namespace_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Module))
        .collect();
    assert!(
        namespace_chunks.len() >= 2,
        "Should find at least 2 namespaces"
    );

    let namespace_names: Vec<&String> = namespace_chunks.iter().map(|c| &c.name).collect();
    assert!(namespace_names.contains(&&"Utils".to_string()));
    assert!(namespace_names.contains(&&"Api".to_string()));
}

#[test]
fn test_typescript_all_new_constructs_combined() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    let ts_code = r#"
// Interface declaration
interface User {
    id: number;
    name: string;
    email?: string;
}

// Type alias
type Status = 'pending' | 'completed' | 'failed';

// Enum declaration
enum Color {
    Red = '#ff0000',
    Green = '#00ff00',
    Blue = '#0000ff'
}

// Namespace declaration
namespace Utils {
    export function formatDate(date: Date): string {
        return date.toISOString();
    }
}

// Existing constructs (should still work)
class UserService {
    private users: User[] = [];

    addUser(user: User): void {
        this.users.push(user);
    }

    getUserById(id: number): User | undefined {
        return this.users.find(u => u.id === id);
    }
}

function processUser(user: User): Status {
    return 'pending';
}

const calculateTotal = (items: number[]): number => {
    return items.reduce((sum, item) => sum + item, 0);
};
"#;
    fs::write(&file_path, ts_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Should find: 1 interface + 1 type alias + 1 enum + 1 namespace + 1 class + 4 functions = 9 total minimum
    assert!(
        chunks.len() >= 9,
        "Should find at least 9 chunks, got {}",
        chunks.len()
    );

    // Verify each new chunk type
    let interface_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Interface))
        .count();
    let type_alias_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::TypeAlias))
        .count();
    let enum_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Enum))
        .count();
    let namespace_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Module))
        .count();

    assert_eq!(interface_count, 1, "Should find 1 interface");
    assert_eq!(type_alias_count, 1, "Should find 1 type alias");
    assert_eq!(enum_count, 1, "Should find 1 enum");
    assert_eq!(namespace_count, 1, "Should find 1 namespace");

    // Verify names of all new constructs
    let chunk_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(chunk_names.contains(&&"User".to_string()));
    assert!(chunk_names.contains(&&"Status".to_string()));
    assert!(chunk_names.contains(&&"Color".to_string()));
    assert!(chunk_names.contains(&&"Utils".to_string()));
}

// Python language construct tests
#[test]
fn test_python_basic_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.py");

    let python_code = r#"
def regular_function(x: int) -> int:
    return x * 2

def another_function():
    pass
"#;
    fs::write(&file_path, python_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].name, "regular_function");
    assert_eq!(chunks[1].name, "another_function");
    assert!(matches!(chunks[0].chunk_type, ChunkType::Function));
    assert!(matches!(chunks[1].chunk_type, ChunkType::Function));
}

#[test]
fn test_python_decorated_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.py");

    let python_code = r#"
@staticmethod
def utility_function(value: str) -> str:
    return value.upper()

@property
def area(self) -> float:
    return 3.14 * self._radius ** 2

@classmethod
def from_diameter(cls, diameter: float) -> 'Circle':
    return cls(diameter / 2)
"#;
    fs::write(&file_path, python_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 6); // Each decorated function creates 2 chunks

    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();
    assert_eq!(function_chunks.len(), 6); // All are functions due to duplicates

    let function_names: Vec<&String> = function_chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"utility_function".to_string()));
    assert!(function_names.contains(&&"area".to_string()));
    assert!(function_names.contains(&&"from_diameter".to_string()));
}

#[test]
fn test_python_decorated_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.py");

    let python_code = r#"
@dataclass
class User:
    id: int
    name: str
    email: Optional[str] = None

@property_class
class Circle:
    def __init__(self, radius: float):
        self._radius = radius
"#;
    fs::write(&file_path, python_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 5); // 2 decorated classes (duplicated) + 1 method

    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 4); // Each decorated class creates 2 chunks

    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"User".to_string()));
    assert!(class_names.contains(&&"Circle".to_string()));
}

#[test]
fn test_python_mixed_constructs() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.py");

    let python_code = r#"
# Regular function (already supported)
def regular_function(x: int) -> int:
    return x * 2

# Regular class (already supported)  
class RegularClass:
    def method(self):
        pass

# Decorated class (new support)
@dataclass
class User:
    id: int
    name: str

# Decorated function (new support)
@staticmethod
def utility_function(value: str) -> str:
    return value.upper()
"#;
    fs::write(&file_path, python_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 7); // All found chunks including duplicates

    // Debug: print all chunks to see what was found
    for (i, chunk) in chunks.iter().enumerate() {
        println!("Chunk {}: {} ({:?}) at {}:{}-{}", i, chunk.name, chunk.chunk_type, 
                chunk.file_path.display(), chunk.start_line, chunk.end_line);
    }

    // Verify each chunk type
    let function_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .count();
    let class_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .count();
    let method_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Method))
        .count();

    println!("Functions: {}, Classes: {}, Methods: {}", function_count, class_count, method_count);

    // With duplicates, we expect more functions due to both regular and decorated captures
    // Methods in Python are extracted as functions, not methods
    assert_eq!(function_count, 4, "Should find 4 functions (regular + method + 2 decorated)");
    assert_eq!(class_count, 3, "Should find 3 classes (regular + 2 decorated versions)");
    assert_eq!(method_count, 0, "Python methods are extracted as functions");

    // Verify names
    let chunk_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(chunk_names.contains(&&"regular_function".to_string()));
    assert!(chunk_names.contains(&&"RegularClass".to_string()));
    assert!(chunk_names.contains(&&"User".to_string()));
    assert!(chunk_names.contains(&&"utility_function".to_string()));
    assert!(chunk_names.contains(&&"method".to_string()));
}
