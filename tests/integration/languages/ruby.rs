use gittype::extractor::{ChunkType, CodeExtractor, ExtractionOptions};
use std::fs;
use tempfile::TempDir;

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

    // Debug output to see what's being extracted
    for (i, chunk) in chunks.iter().enumerate() {
        println!(
            "Chunk {}: name='{}', type={:?}",
            i, chunk.name, chunk.chunk_type
        );
    }
    assert_eq!(chunks.len(), 4); // class + 2 methods + 1 attr_accessor (name, age combined)

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
    assert_eq!(method_chunks.len(), 3); // initialize, greet + attr_accessor (name, age combined)

    let method_names: Vec<&String> = method_chunks.iter().map(|c| &c.name).collect();
    assert!(method_names.iter().any(|name| name.contains("initialize")));
    assert!(method_names.iter().any(|name| name.contains("greet")));
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
    assert!(method_names.iter().any(|name| name.contains("login")));
    assert!(method_names.iter().any(|name| name.contains("logout")));
}

#[test]
fn test_ruby_class_method_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rb");

    let ruby_code = r#"
class User
  def self.find_by_email(email)
    puts "Finding user by email: #{email}"
  end
  
  def instance_method
    "instance"
  end
end
"#;
    fs::write(&file_path, ruby_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    let class_methods: Vec<_> = chunks
        .iter()
        .filter(|chunk| chunk.name.contains("find_by_email"))
        .collect();
    assert!(
        !class_methods.is_empty(),
        "Should extract class method find_by_email"
    );

    let instance_methods: Vec<_> = chunks
        .iter()
        .filter(|chunk| chunk.name.contains("instance_method"))
        .collect();
    assert!(
        !instance_methods.is_empty(),
        "Should extract instance method"
    );
}

#[test]
fn test_ruby_attr_accessor_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rb");

    let ruby_code = r#"
class Product
  attr_accessor :name, :price
  attr_reader :id
  attr_writer :description
end
"#;
    fs::write(&file_path, ruby_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    let attr_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| {
            chunk.name.contains("name")
                || chunk.name.contains("price")
                || chunk.name.contains("id")
                || chunk.name.contains("description")
        })
        .collect();
    assert!(
        !attr_chunks.is_empty(),
        "Should extract attr_accessor, attr_reader, attr_writer"
    );
}
