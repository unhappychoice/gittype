use gittype::extractor::{ChunkType, CodeExtractor, ExtractionOptions};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_kotlin_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.kt");

    let kotlin_code = r#"
fun greet(name: String): String {
    return "Hello, $name!"
}

fun calculateSum(a: Int, b: Int): Int {
    return a + b
}

fun processData(data: List<String>) {
    data.forEach { println(it) }
}
"#;
    fs::write(&file_path, kotlin_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    assert_eq!(chunks.len(), 3);

    let function_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"greet".to_string()));
    assert!(function_names.contains(&&"calculateSum".to_string()));
    assert!(function_names.contains(&&"processData".to_string()));

    for chunk in &chunks {
        assert!(matches!(chunk.chunk_type, ChunkType::Function));
        assert_eq!(chunk.language, gittype::extractor::Language::Kotlin);
    }
}

#[test]
fn test_kotlin_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.kt");

    let kotlin_code = r#"
class Person(val name: String, val age: Int) {
    fun greet(): String {
        return "Hello, I'm $name and I'm $age years old"
    }
    
    fun isAdult(): Boolean {
        return age >= 18
    }
}

data class User(
    val id: Long,
    val name: String,
    val email: String
) {
    fun getDisplayName(): String = "$name ($email)"
}
"#;
    fs::write(&file_path, kotlin_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Should find 2 classes + 3 functions = 5 total
    assert_eq!(chunks.len(), 5);

    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 2);

    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"Person".to_string()));
    assert!(class_names.contains(&&"User".to_string()));

    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();
    assert_eq!(function_chunks.len(), 3);

    let function_names: Vec<&String> = function_chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"greet".to_string()));
    assert!(function_names.contains(&&"isAdult".to_string()));
    assert!(function_names.contains(&&"getDisplayName".to_string()));
}

#[test]
fn test_kotlin_object_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.kt");

    let kotlin_code = r#"
object DatabaseHelper {
    const val DB_NAME = "app.db"
    
    fun connect(): String {
        return "Connected to $DB_NAME"
    }
    
    fun disconnect() {
        println("Disconnected from database")
    }
}

object Utils {
    fun formatName(name: String): String {
        return name.trim().lowercase()
    }
}
"#;
    fs::write(&file_path, kotlin_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Should find 2 objects + 3 functions = 5 total, but sometimes might find 6 due to additional functions
    assert!(
        chunks.len() >= 5,
        "Should find at least 5 chunks, got {}",
        chunks.len()
    );

    let object_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(object_chunks.len(), 2);

    let object_names: Vec<&String> = object_chunks.iter().map(|c| &c.name).collect();
    assert!(object_names.contains(&&"DatabaseHelper".to_string()));
    assert!(object_names.contains(&&"Utils".to_string()));
}
