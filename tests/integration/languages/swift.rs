use crate::integration::test_extraction_options;
use gittype::extractor::{CodeExtractor};
use gittype::models::{ChunkType};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_swift_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.swift");

    let swift_code = r#"
class Calculator {
    private var value: Int = 0
    
    init() {
        self.value = 0
    }
    
    deinit {
        print("Calculator deallocated")
    }
    
    func add(_ number: Int) -> Int {
        value += number
        return value
    }
    
    func multiply(_ number: Int) -> Int {
        value *= number
        return value
    }
}

class Person {
    let name: String
    let age: Int
    
    init(name: String, age: Int) {
        self.name = name
        self.age = age
    }
    
    func greet() -> String {
        return "Hello, I'm \(name)!"
    }
}
"#;
    fs::write(&file_path, swift_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find 2 classes + methods
    assert!(chunks.len() >= 2);

    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 2);

    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"Calculator".to_string()));
    assert!(class_names.contains(&&"Person".to_string()));

    for chunk in &chunks {
        assert_eq!(chunk.language, "swift".to_string());
    }
}

#[test]
fn test_swift_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.swift");

    let swift_code = r#"
func greet(name: String) -> String {
    return "Hello, \(name)!"
}

func calculateSum(a: Int, b: Int) -> Int {
    return a + b
}

func processData(items: [String]) {
    for item in items {
        print(item)
    }
}
"#;
    fs::write(&file_path, swift_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 3);

    let function_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"greet".to_string()));
    assert!(function_names.contains(&&"calculateSum".to_string()));
    assert!(function_names.contains(&&"processData".to_string()));

    for chunk in &chunks {
        assert!(matches!(chunk.chunk_type, ChunkType::Function));
        assert_eq!(chunk.language, "swift".to_string());
    }
}

#[test]
fn test_swift_protocol_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.swift");

    let swift_code = r#"
protocol Drawable {
    func draw()
    func area() -> Double
}

protocol Comparable {
    func isGreaterThan(_ other: Self) -> Bool
}
"#;
    fs::write(&file_path, swift_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find 2 protocols
    assert_eq!(chunks.len(), 2);

    let protocol_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Interface))
        .collect();
    assert_eq!(protocol_chunks.len(), 2);

    let protocol_names: Vec<&String> = protocol_chunks.iter().map(|c| &c.name).collect();
    assert!(protocol_names.contains(&&"Drawable".to_string()));
    assert!(protocol_names.contains(&&"Comparable".to_string()));

    for chunk in &chunks {
        assert_eq!(chunk.language, "swift".to_string());
    }
}

#[test]
fn test_swift_struct_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.swift");

    let swift_code = r#"
struct Point {
    let x: Double
    let y: Double
    
    func distance(to other: Point) -> Double {
        return sqrt(pow(x - other.x, 2) + pow(y - other.y, 2))
    }
}

struct Rectangle {
    let width: Double
    let height: Double
    
    func area() -> Double {
        return width * height
    }
}
"#;
    fs::write(&file_path, swift_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find 2 structs + methods
    assert!(chunks.len() >= 2);

    let struct_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Struct))
        .collect();
    assert_eq!(struct_chunks.len(), 2);

    let struct_names: Vec<&String> = struct_chunks.iter().map(|c| &c.name).collect();
    assert!(struct_names.contains(&&"Point".to_string()));
    assert!(struct_names.contains(&&"Rectangle".to_string()));

    for chunk in &chunks {
        assert_eq!(chunk.language, "swift".to_string());
    }
}

#[test]
fn test_swift_enum_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.swift");

    let swift_code = r#"
enum Direction {
    case north, south, east, west
    
    func opposite() -> Direction {
        switch self {
        case .north: return .south
        case .south: return .north
        case .east: return .west
        case .west: return .east
        }
    }
}

enum Status {
    case pending
    case completed
    case failed
}
"#;
    fs::write(&file_path, swift_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find 2 enums + methods
    assert!(chunks.len() >= 2);

    let enum_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Enum))
        .collect();
    assert_eq!(enum_chunks.len(), 2);

    let enum_names: Vec<&String> = enum_chunks.iter().map(|c| &c.name).collect();
    assert!(enum_names.contains(&&"Direction".to_string()));
    assert!(enum_names.contains(&&"Status".to_string()));

    for chunk in &chunks {
        assert_eq!(chunk.language, "swift".to_string());
    }
}
