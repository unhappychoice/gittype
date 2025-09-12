use crate::integration::{extract_chunks_for_test, test_extraction_options};
use gittype::extractor::CodeChunkExtractor;
use gittype::models::ChunkType;
use std::fs;
use tempfile::TempDir;

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

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    assert_eq!(chunks.len(), 3);

    let function_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"main".to_string()));
    assert!(function_names.contains(&&"add".to_string()));
    assert!(function_names.contains(&&"multiply".to_string()));

    for chunk in &chunks {
        assert!(matches!(chunk.chunk_type, ChunkType::Function));
        assert_eq!(chunk.language, "go".to_string());
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

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
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

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
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
fn test_go_const_var_type_alias_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.go");

    let go_code = r#"package main

import "errors"

// Const block test
const (
    StatusOK = 200
    StatusNotFound = 404
    StatusError = 500
)

// Single const
const MaxRetries = 3

// Var block test
var (
    ErrNotFound = errors.New("not found")
    ErrTimeout = errors.New("timeout")
)

// Single var
var GlobalCounter int

// Type alias tests
type UserID int64
type Handler func(string, string)
type Point struct {
    X, Y int
}

func main() {}
"#;
    fs::write(&file_path, go_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    // Should find: 2 const blocks + 2 var blocks + 2 type aliases + 1 function + 1 struct = 8 total
    assert_eq!(chunks.len(), 8);

    // Find const chunks
    let const_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Const))
        .collect();
    assert!(
        const_chunks.len() >= 2,
        "Should find at least 2 const blocks"
    );

    // Find var chunks
    let var_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Variable))
        .collect();
    assert!(var_chunks.len() >= 2, "Should find at least 2 var blocks");

    // Find type alias chunks (should include UserID, Handler)
    let type_alias_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::TypeAlias))
        .collect();
    assert!(
        type_alias_chunks.len() >= 2,
        "Should find at least 2 type aliases"
    );

    let type_alias_names: Vec<&String> = type_alias_chunks.iter().map(|c| &c.name).collect();
    assert!(type_alias_names.contains(&&"UserID".to_string()));
    assert!(type_alias_names.contains(&&"Handler".to_string()));

    // Verify we still find struct and function
    let struct_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Struct))
        .collect();
    assert_eq!(struct_chunks.len(), 1);
    assert_eq!(struct_chunks[0].name, "Point");

    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();
    assert!(!function_chunks.is_empty());

    let function_names: Vec<&String> = function_chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"main".to_string()));
}
