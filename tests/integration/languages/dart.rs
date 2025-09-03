use crate::integration::test_extraction_options;
use gittype::extractor::CodeExtractor;
use gittype::models::ChunkType;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_dart_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.dart");

    let dart_code = r#"
class Calculator {
  int _value = 0;
  
  Calculator([int value = 0]) : _value = value;
  
  int add(int number) {
    _value += number;
    return _value;
  }
  
  int multiply(int number) {
    _value *= number;
    return _value;
  }
  
  int get value => _value;
  
  set value(int newValue) {
    _value = newValue;
  }
}

class Person {
  final String name;
  final int age;
  
  const Person(this.name, this.age);
  
  String greet() {
    return 'Hello, I\'m $name!';
  }
}
"#;
    fs::write(&file_path, dart_code).unwrap();

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
        assert_eq!(chunk.language, "dart".to_string());
    }
}

#[test]
fn test_dart_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.dart");

    let dart_code = r#"
String greet(String name) {
  return 'Hello, $name!';
}

int calculateSum(int a, int b) {
  return a + b;
}

Future<String> fetchData() async {
  await Future.delayed(Duration(seconds: 1));
  return 'Data fetched';
}

void processItems(List<String> items) {
  for (final item in items) {
    print(item);
  }
}
"#;
    fs::write(&file_path, dart_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert!(chunks.len() >= 4);

    let function_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"greet".to_string()));
    assert!(function_names.contains(&&"calculateSum".to_string()));
    assert!(function_names.contains(&&"fetchData".to_string()));
    assert!(function_names.contains(&&"processItems".to_string()));

    for chunk in &chunks {
        assert!(matches!(chunk.chunk_type, ChunkType::Function));
        assert_eq!(chunk.language, "dart".to_string());
    }
}

#[test]
fn test_dart_enum_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.dart");

    let dart_code = r#"
enum Color {
  red,
  green,
  blue;
  
  String get hex {
    switch (this) {
      case Color.red:
        return '#FF0000';
      case Color.green:
        return '#00FF00';
      case Color.blue:
        return '#0000FF';
    }
  }
}

enum Status {
  pending,
  completed,
  failed
}
"#;
    fs::write(&file_path, dart_code).unwrap();

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
    assert!(enum_names.contains(&&"Color".to_string()));
    assert!(enum_names.contains(&&"Status".to_string()));

    for chunk in &chunks {
        assert_eq!(chunk.language, "dart".to_string());
    }
}

#[test]
fn test_dart_mixin_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.dart");

    let dart_code = r#"
mixin Flyable {
  void fly() {
    print('Flying!');
  }
  
  double get altitude => 1000.0;
}

mixin Swimable {
  void swim() {
    print('Swimming!');
  }
}

class Bird with Flyable {
  String name;
  
  Bird(this.name);
  
  void chirp() {
    print('$name is chirping');
  }
}
"#;
    fs::write(&file_path, dart_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find 2 mixins + 1 class + methods
    assert!(chunks.len() >= 3);

    let mixin_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class)) // mixins are treated as classes
        .collect();
    assert!(mixin_chunks.len() >= 2); // 2 mixins + 1 class

    let chunk_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(chunk_names.contains(&&"Flyable".to_string()));
    assert!(chunk_names.contains(&&"Swimable".to_string()));
    assert!(chunk_names.contains(&&"Bird".to_string()));

    for chunk in &chunks {
        assert_eq!(chunk.language, "dart".to_string());
    }
}

#[test]
fn test_dart_variable_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.dart");

    let dart_code = r#"
final String appName = 'My App';
const int maxRetries = 3;
var isEnabled = true;
late String apiKey;

class Config {
  static const String version = '1.0.0';
  final String environment;
  
  Config(this.environment);
}
"#;
    fs::write(&file_path, dart_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find variables + class
    assert!(chunks.len() >= 4);

    let variable_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Variable))
        .collect();
    assert!(variable_chunks.len() >= 3); // appName, maxRetries, isEnabled

    let variable_names: Vec<&String> = variable_chunks.iter().map(|c| &c.name).collect();
    assert!(variable_names.contains(&&"appName".to_string()));
    assert!(variable_names.contains(&&"maxRetries".to_string()));
    assert!(variable_names.contains(&&"isEnabled".to_string()));

    for chunk in &chunks {
        assert_eq!(chunk.language, "dart".to_string());
    }
}

#[test]
fn test_dart_extension_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.dart");

    let dart_code = r#"
extension StringExtensions on String {
  bool get isEmail {
    return contains('@') && contains('.');
  }
  
  String capitalize() {
    if (isEmpty) return this;
    return '${this[0].toUpperCase()}${substring(1)}';
  }
}

extension ListExtensions<T> on List<T> {
  T? get firstOrNull {
    return isEmpty ? null : first;
  }
}
"#;
    fs::write(&file_path, dart_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find 2 extensions + methods
    assert!(chunks.len() >= 2);

    let extension_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class)) // extensions are treated as classes
        .collect();
    assert!(extension_chunks.len() >= 2);

    let extension_names: Vec<&String> = extension_chunks.iter().map(|c| &c.name).collect();
    assert!(extension_names.contains(&&"StringExtensions".to_string()));
    assert!(extension_names.contains(&&"ListExtensions".to_string()));

    for chunk in &chunks {
        assert_eq!(chunk.language, "dart".to_string());
    }
}
