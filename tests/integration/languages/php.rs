use crate::integration::test_extraction_options;
use gittype::extractor::{CodeExtractor};
use gittype::models::{ChunkType};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_php_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.php");

    let php_code = r#"<?php

function hello_world() {
    echo "Hello, world!";
}

function calculate_sum($a, $b) {
    return $a + $b;
}

function fibonacci($n) {
    if ($n <= 1) {
        return $n;
    }
    return fibonacci($n - 1) + fibonacci($n - 2);
}

?>"#;
    fs::write(&file_path, php_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 3);

    let function_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"hello_world".to_string()));
    assert!(function_names.contains(&&"calculate_sum".to_string()));
    assert!(function_names.contains(&&"fibonacci".to_string()));

    for chunk in &chunks {
        assert!(matches!(chunk.chunk_type, ChunkType::Function));
        assert_eq!(chunk.language, "php".to_string());
    }
}

#[test]
fn test_php_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.php");

    let php_code = r#"<?php

class Person {
    private $name;
    private $age;
    
    public function __construct($name, $age) {
        $this->name = $name;
        $this->age = $age;
    }
    
    public function greet() {
        return "Hello, I'm " . $this->name . "!";
    }
    
    public function getAge() {
        return $this->age;
    }
}

class Calculator {
    public function add($a, $b) {
        return $a + $b;
    }
    
    public function multiply($a, $b) {
        return $a * $b;
    }
}

?>"#;
    fs::write(&file_path, php_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
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
fn test_php_namespace_and_use_statements() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.php");

    let php_code = r#"<?php

namespace App\Services;

use App\Models\User;
use Exception;

class UserService {
    private $database;
    
    public function __construct(DatabaseConnection $database) {
        $this->database = $database;
    }
    
    public function createUser(array $userData): User {
        try {
            $user = new User([
                'name' => $userData['name'],
                'email' => $userData['email'],
                'password' => password_hash($userData['password'], PASSWORD_DEFAULT)
            ]);
            
            $userId = $this->database->insert('users', $user->toArray());
            $user->setId($userId);
            
            return $user;
        } catch (Exception $e) {
            throw new UserCreationException("Failed to create user: " . $e->getMessage());
        }
    }
    
    public function findUsersByRole(string $role): array {
        $query = "SELECT * FROM users WHERE role = ?";
        return $this->database->query($query, [$role]);
    }
}

?>"#;
    fs::write(&file_path, php_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find at least the namespace and class
    assert!(chunks.len() >= 2);

    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();

    // Should find the UserService class
    assert!(!class_chunks.is_empty());

    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"UserService".to_string()));
}

#[test]
fn test_php_traits_and_interfaces() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.php");

    let php_code = r#"<?php

interface Drawable {
    public function draw();
}

trait Timestampable {
    private $created_at;
    private $updated_at;
    
    public function touch() {
        $this->updated_at = time();
    }
}

class Shape implements Drawable {
    use Timestampable;
    
    protected $color;
    
    public function __construct($color) {
        $this->color = $color;
        $this->created_at = time();
        $this->updated_at = time();
    }
    
    public function draw() {
        return "Drawing a shape in " . $this->color;
    }
}

?>"#;
    fs::write(&file_path, php_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find interface, trait, and class
    assert!(chunks.len() >= 3);

    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();

    // Should find the interface, trait, and class
    assert!(class_chunks.len() >= 3);

    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"Drawable".to_string()));
    assert!(class_names.contains(&&"Timestampable".to_string()));
    assert!(class_names.contains(&&"Shape".to_string()));
}
