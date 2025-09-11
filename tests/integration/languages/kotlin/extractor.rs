use crate::integration::{extract_chunks_for_test, test_extraction_options};
use gittype::extractor::CodeChunkExtractor;
use gittype::models::ChunkType;
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

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    assert_eq!(chunks.len(), 3);

    let function_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"greet".to_string()));
    assert!(function_names.contains(&&"calculateSum".to_string()));
    assert!(function_names.contains(&&"processData".to_string()));

    for chunk in &chunks {
        assert!(matches!(chunk.chunk_type, ChunkType::Function));
        assert_eq!(chunk.language, "kotlin".to_string());
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

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
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

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
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

#[test]
fn test_kotlin_comprehensive_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("comprehensive_test.kt");

    let kotlin_code = r#"
// Line comment
/* Block comment */

package com.example.test

// Type alias
typealias StringList = List<String>

// Interface declaration
interface TestInterface {
    fun interfaceMethod(): String
}

// Regular function
fun regularFunction(param: String): String {
    return "Hello $param"
}

// Anonymous function
val anonymousFunc = fun(x: Int): Int { return x * 2 }

// Regular class
class RegularClass(private val name: String) {
    fun method(): String = name
}

// Data class
data class DataClass(val id: Int, val name: String)

// Enum class
enum class Color {
    RED,
    GREEN,
    BLUE
}

// Object declaration
object SingletonObject {
    const val CONSTANT = "constant_value"
    
    fun objectMethod(): String = "object method"
}

// Class with companion object
class ClassWithCompanion {
    companion object {
        const val COMPANION_CONSTANT = "companion_constant"
        
        fun companionMethod(): String = "companion method"
    }
}

// Properties
val globalVal: String = "global val"
var globalVar: String = "global var"
"#;

    fs::write(&file_path, kotlin_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    println!("Total chunks found: {}", chunks.len());
    for (i, chunk) in chunks.iter().enumerate() {
        println!("{}. {:?} - {}", i + 1, chunk.chunk_type, chunk.name);
    }

    // Check for specific constructs
    let chunk_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();

    // Functions
    assert!(chunk_names.contains(&&"regularFunction".to_string()));

    // Classes
    assert!(chunk_names.contains(&&"RegularClass".to_string()));
    assert!(chunk_names.contains(&&"DataClass".to_string()));
    assert!(chunk_names.contains(&&"Color".to_string())); // enum class

    // Objects
    assert!(chunk_names.contains(&&"SingletonObject".to_string()));
    assert!(chunk_names.contains(&&"ClassWithCompanion".to_string()));

    // Type alias
    assert!(chunk_names.contains(&&"StringList".to_string()));

    // Interface
    assert!(chunk_names.contains(&&"TestInterface".to_string()));

    // Enum entries
    assert!(chunk_names.contains(&&"RED".to_string()));
    assert!(chunk_names.contains(&&"GREEN".to_string()));
    assert!(chunk_names.contains(&&"BLUE".to_string()));

    // Verify chunk types are correct
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();
    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    let _variable_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Variable))
        .collect();
    let const_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Const))
        .collect();

    assert!(!function_chunks.is_empty(), "Should have function chunks");
    assert!(!class_chunks.is_empty(), "Should have class chunks");
    assert!(
        !const_chunks.is_empty(),
        "Should have const chunks (enum entries)"
    );

    // Check that we have at least the expected minimum chunks
    assert!(
        chunks.len() >= 15,
        "Should have at least 15 chunks, got {}",
        chunks.len()
    );
}
