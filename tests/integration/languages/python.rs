use gittype::extractor::{ChunkType, CodeExtractor, ExtractionOptions};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_python_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.py");

    let python_code = r#"
def hello_world():
    print("Hello, world!")

def calculate_sum(a, b):
    return a + b
    
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)
"#;
    fs::write(&file_path, python_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 3);

    let function_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"hello_world".to_string()));
    assert!(function_names.contains(&&"calculate_sum".to_string()));
    assert!(function_names.contains(&&"fibonacci".to_string()));

    for chunk in &chunks {
        assert!(matches!(chunk.chunk_type, ChunkType::Function));
        assert_eq!(chunk.language, gittype::extractor::Language::Python);
    }
}

#[test]
fn test_python_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.py");

    let python_code = r#"
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
    
    def greet(self):
        return f"Hello, I'm {self.name}!"
    
    def get_age(self):
        return self.age

class Calculator:
    def add(self, a, b):
        return a + b
    
    def multiply(self, a, b):
        return a * b
"#;
    fs::write(&file_path, python_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Should find 2 classes + 5 methods
    assert!(chunks.len() >= 2);

    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 2);

    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"Person".to_string()));
    assert!(class_names.contains(&&"Calculator".to_string()));
}

#[test]
fn test_python_combined_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.py");

    let python_code = r#"
# Module-level function
def calculate_total(items):
    return sum(items)

# Class definition
class User:
    def __init__(self, name, email):
        self.name = name
        self.email = email
    
    def get_display_name(self):
        return f"{self.name} ({self.email})"

# Another class
class Database:
    def __init__(self):
        self.connections = []
    
    def connect(self):
        return "Connected"
    
    def disconnect(self):
        print("Disconnected")

# Another function
def process_data(data):
    return [item.upper() for item in data]
"#;
    fs::write(&file_path, python_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Should find 2 classes + 6 functions = 8 total minimum
    assert!(
        chunks.len() >= 8,
        "Should find at least 8 chunks, got {}",
        chunks.len()
    );

    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();

    assert_eq!(class_chunks.len(), 2);
    assert!(function_chunks.len() >= 6);

    // Verify class names
    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"User".to_string()));
    assert!(class_names.contains(&&"Database".to_string()));

    // Verify some function names
    let function_names: Vec<&String> = function_chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"calculate_total".to_string()));
    assert!(function_names.contains(&&"process_data".to_string()));
}
