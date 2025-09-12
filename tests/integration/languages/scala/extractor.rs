use gittype::extractor::{CodeChunkExtractor, RepositoryExtractor};
use gittype::models::ChunkType;
use std::collections::HashSet;
use std::fs;
use tempfile::TempDir;

use crate::integration::{
    extract_challenges_for_test, extract_chunks_for_test, test_extraction_options,
};

fn setup_git_repo(dir_path: &std::path::Path) {
    // Initialize git repository
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(dir_path)
        .output()
        .expect("Failed to init git repo");

    // Set up basic git config
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(dir_path)
        .output()
        .expect("Failed to set git user.name");

    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(dir_path)
        .output()
        .expect("Failed to set git user.email");

    // Add a remote URL to avoid "Failed to get remote URL" error
    std::process::Command::new("git")
        .args([
            "remote",
            "add",
            "origin",
            "https://github.com/test/test.git",
        ])
        .current_dir(dir_path)
        .output()
        .expect("Failed to add remote");
}

#[test]
fn test_scala_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.scala");

    let scala_code = r#"
def hello(): Unit = {
    println("Hello, Scala!")
}

def add(a: Int, b: Int): Int = {
    a + b
}

def multiply(x: Int, y: Int): Int = x * y
"#;
    fs::write(&file_path, scala_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    assert_eq!(chunks.len(), 3);
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();
    assert_eq!(function_chunks.len(), 3);

    let function_names: Vec<&String> = function_chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"hello".to_string()));
    assert!(function_names.contains(&&"add".to_string()));
    assert!(function_names.contains(&&"multiply".to_string()));
}

#[test]
fn test_scala_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.scala");

    let scala_code = r#"
class Person(val name: String, val age: Int) {
    def greet(): String = s"Hello, I'm $name"
}

case class Point(x: Double, y: Double) {
    def distance(other: Point): Double = {
        math.sqrt(math.pow(x - other.x, 2) + math.pow(y - other.y, 2))
    }
}

abstract class Animal {
    def speak(): String
}
"#;
    fs::write(&file_path, scala_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert!(class_chunks.len() >= 3);

    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"Person".to_string()));
    assert!(class_names.contains(&&"Point".to_string()));
    assert!(class_names.contains(&&"Animal".to_string()));
}

#[test]
fn test_scala_object_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.scala");

    let scala_code = r#"
object Main {
    def main(args: Array[String]): Unit = {
        println("Hello, world!")
    }
    
    val PI = 3.14159
}

object MathUtils {
    def factorial(n: Int): Int = {
        if (n <= 1) 1 else n * factorial(n - 1)
    }
}

case object Singleton {
    def process(): String = "processing"
}
"#;
    fs::write(&file_path, scala_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    let object_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class)) // Objects are treated as classes
        .collect();
    assert!(object_chunks.len() >= 3);

    let object_names: Vec<&String> = object_chunks.iter().map(|c| &c.name).collect();
    assert!(object_names.contains(&&"Main".to_string()));
    assert!(object_names.contains(&&"MathUtils".to_string()));
    assert!(object_names.contains(&&"Singleton".to_string()));
}

#[test]
fn test_scala_trait_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.scala");

    let scala_code = r#"
trait Animal {
    def speak(): String
    def move(): Unit = println("Moving...")
}

sealed trait Result[+T] {
    def map[U](f: T => U): Result[U]
}

trait Drawable {
    def draw(): Unit
}
"#;
    fs::write(&file_path, scala_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    let trait_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class)) // Traits might be treated as classes
        .collect();
    assert!(trait_chunks.len() >= 3);

    let trait_names: Vec<&String> = trait_chunks.iter().map(|c| &c.name).collect();
    assert!(trait_names.contains(&&"Animal".to_string()));
    assert!(trait_names.contains(&&"Result".to_string()));
    assert!(trait_names.contains(&&"Drawable".to_string()));
}

#[test]
fn test_scala_enum_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.scala");

    let scala_code = "
enum Color {
    case Red, Green, Blue
    case RGB(r: Int, g: Int, b: Int)
    
    def toHex(): String = this match {
        case Red => \"#FF0000\"
        case Green => \"#00FF00\"
        case Blue => \"#0000FF\"
        case RGB(r, g, b) => f\"#$r%02X$g%02X$b%02X\"
    }
}

enum Direction {
    case North, South, East, West
}
";
    fs::write(&file_path, scala_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    let enum_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Const)) // Enums might be treated as constants
        .collect();
    assert!(enum_chunks.len() >= 2);

    let enum_names: Vec<&String> = enum_chunks.iter().map(|c| &c.name).collect();
    assert!(enum_names.contains(&&"Color".to_string()));
    assert!(enum_names.contains(&&"Direction".to_string()));
}

#[test]
fn test_scala_all_constructs_combined() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.scala");

    let scala_code = r#"
// Object definition
object Calculator {
    def add(a: Int, b: Int): Int = a + b
}

// Class definition
class Person(val name: String) {
    def greet(): String = s"Hello, $name"
}

// Case class
case class Point(x: Int, y: Int) {
    def move(dx: Int, dy: Int): Point = Point(x + dx, y + dy)
}

// Trait definition
trait Drawable {
    def draw(): Unit
}

// Enum definition
enum Status {
    case Active, Inactive
}

// Function definition
def factorial(n: Int): Int = {
    if (n <= 1) 1 else n * factorial(n - 1)
}

// Type definition
type UserId = Long
"#;
    fs::write(&file_path, scala_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    assert!(chunks.len() >= 7); // At least 7 different constructs

    // Count different chunk types
    let function_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .count();
    let class_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .count();
    let const_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Const))
        .count();

    assert!(function_count >= 2, "Should find at least 2 functions");
    assert!(
        class_count >= 4,
        "Should find at least 4 classes/objects/traits"
    );
    assert!(const_count >= 1, "Should find at least 1 enum/const");

    // Verify specific names exist
    let chunk_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(chunk_names.contains(&&"Calculator".to_string()));
    assert!(chunk_names.contains(&&"Person".to_string()));
    assert!(chunk_names.contains(&&"Point".to_string()));
    assert!(chunk_names.contains(&&"Drawable".to_string()));
    assert!(chunk_names.contains(&&"factorial".to_string()));
}

#[test]
fn test_scala_comment_ranges_in_challenge() {
    let scala_code = r#"// Scala function with comments
def calculateSum(a: Int, b: Int): Int = {
    val result = a + b // Add the numbers
    /*
     * Return the result
     */
    result
}
"#;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("test.scala");
    fs::write(&file_path, scala_code).expect("Failed to write test file");

    // Setup git repository
    setup_git_repo(temp_dir.path());

    let mut loader = RepositoryExtractor::new().expect("Failed to create loader");
    let options = test_extraction_options();

    let challenges = extract_challenges_for_test(&mut loader, temp_dir.path(), options)
        .expect("Failed to load challenges");

    if challenges.is_empty() {
        println!("No challenges found - likely due to filtering. Skipping test.");
        return;
    }

    let challenge = &challenges[0];
    println!("Challenge content: '{}'", challenge.code_content);
    println!("Comment ranges: {:?}", challenge.comment_ranges);

    let chars: Vec<char> = challenge.code_content.chars().collect();

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

#[test]
fn test_scala_no_duplicates() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.scala");

    let scala_code = r#"
package com.example.test

object Calculator {
    def add(a: Int, b: Int): Int = {
        val result = a + b
        result
    }
    
    def processValue(value: Any): String = {
        value match {
            case s: String => s"String: $s"
            case i: Int => s"Int: $i" 
            case _ => "Unknown"
        }
    }
    
    val numbers = List(1, 2, 3)
    val doubled = for {
        n <- numbers
        if n > 1
        result = n * 2
    } yield result
    
    val filtered = numbers.filter(x => x > 2).map(y => y * y)
    
    val attempt = Try {
        "123".toInt
    }
}

class Person(val name: String) {
    def greet(): String = s"Hello, $name"
    
    def isLongName(): Boolean = {
        if (name.length > 5) true else false
    }
}

trait Animal {
    def speak(): String
    
    def move(): Unit = {
        println("Moving...")
    }
}

extension (s: String) {
    def isPalindrome: Boolean = s == s.reverse
}

@deprecated
def oldFunction(): Unit = {}
"#;

    fs::write(&file_path, scala_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    // Simple duplicate check
    let mut seen_content = HashSet::new();
    let mut duplicates = Vec::new();

    for (i, chunk) in chunks.iter().enumerate() {
        if !seen_content.insert(chunk.content.clone()) {
            duplicates.push(i);
        }
    }

    assert!(
        duplicates.is_empty(),
        "Found duplicate chunks at indices: {:?}",
        duplicates
    );

    // Check for position duplicates
    let mut seen_positions = HashSet::new();
    let mut position_duplicates = Vec::new();

    for (i, chunk) in chunks.iter().enumerate() {
        let pos = (chunk.start_line, chunk.end_line);
        if !seen_positions.insert(pos) {
            position_duplicates.push(i);
        }
    }

    assert!(
        position_duplicates.is_empty(),
        "Found chunks with duplicate positions at indices: {:?}",
        position_duplicates
    );

    println!(
        "✅ Duplicate check passed with {} unique chunks",
        chunks.len()
    );
}
