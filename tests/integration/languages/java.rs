use gittype::extractor::{ChunkType, CodeExtractor, ExtractionOptions};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_java_class_method_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("HelloWorld.java");

    let java_code = r#"public class HelloWorld {
    private String message;
    
    public HelloWorld(String message) {
        this.message = message;
    }
    
    public void printMessage() {
        System.out.println(this.message);
    }
    
    public static void main(String[] args) {
        HelloWorld hello = new HelloWorld("Hello, World!");
        hello.printMessage();
    }
    
    private int calculateLength() {
        return this.message.length();
    }
}"#;
    fs::write(&file_path, java_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Remove debug output
    assert!(chunks.len() >= 4); // 1 class + 3+ methods (constructor, printMessage, main, calculateLength)

    // Find class chunk
    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 1);
    assert_eq!(class_chunks[0].name, "HelloWorld");

    // Find method chunks
    let method_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Method))
        .collect();
    assert!(method_chunks.len() >= 3);

    let method_names: Vec<&String> = method_chunks.iter().map(|c| &c.name).collect();
    assert!(method_names.contains(&&"printMessage".to_string()));
    assert!(method_names.contains(&&"main".to_string()));
    assert!(method_names.contains(&&"calculateLength".to_string()));

    for chunk in &chunks {
        assert_eq!(chunk.language, gittype::extractor::Language::Java);
    }
}

#[test]
fn test_java_interface_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("Drawable.java");

    let java_code = r#"public interface Drawable {
    void draw();
    void setColor(String color);
    String getColor();
}

public interface Resizable {
    void resize(int width, int height);
    int getWidth();
    int getHeight();
}

public class Circle implements Drawable, Resizable {
    private String color;
    private int radius;
    
    @Override
    public void draw() {
        System.out.println("Drawing a " + color + " circle");
    }
    
    @Override
    public void setColor(String color) {
        this.color = color;
    }
    
    @Override
    public String getColor() {
        return this.color;
    }
    
    @Override
    public void resize(int width, int height) {
        this.radius = Math.min(width, height) / 2;
    }
    
    @Override
    public int getWidth() {
        return radius * 2;
    }
    
    @Override
    public int getHeight() {
        return radius * 2;
    }
}"#;
    fs::write(&file_path, java_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Find interface chunks
    let interface_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Interface))
        .collect();
    assert_eq!(interface_chunks.len(), 2);

    let interface_names: Vec<&String> = interface_chunks.iter().map(|c| &c.name).collect();
    assert!(interface_names.contains(&&"Drawable".to_string()));
    assert!(interface_names.contains(&&"Resizable".to_string()));

    // Find class chunk
    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 1);
    assert_eq!(class_chunks[0].name, "Circle");

    // Find method chunks
    let method_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Method))
        .collect();
    assert!(method_chunks.len() >= 8); // Interface methods + class methods

    for chunk in &chunks {
        assert_eq!(chunk.language, gittype::extractor::Language::Java);
    }
}

#[test]
fn test_java_enum_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("Color.java");

    let java_code = r##"public enum Color {
    RED("red", "#FF0000"),
    GREEN("green", "#00FF00"),
    BLUE("blue", "#0000FF");
    
    private final String name;
    private final String hexCode;
    
    Color(String name, String hexCode) {
        this.name = name;
        this.hexCode = hexCode;
    }
    
    public String getName() {
        return name;
    }
    
    public String getHexCode() {
        return hexCode;
    }
}

public class ColorTest {
    public void testColor() {
        Color color = Color.RED;
        System.out.println(color.getName());
    }
}"##;
    fs::write(&file_path, java_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Find enum chunk
    let enum_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Enum))
        .collect();
    assert_eq!(enum_chunks.len(), 1);
    assert_eq!(enum_chunks[0].name, "Color");

    // Find class chunk
    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 1);
    assert_eq!(class_chunks[0].name, "ColorTest");

    // Find method chunks (constructor + getName + getHexCode + testColor)
    let method_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Method))
        .collect();
    assert!(method_chunks.len() >= 3);

    let method_names: Vec<&String> = method_chunks.iter().map(|c| &c.name).collect();
    assert!(method_names.contains(&&"getName".to_string()));
    assert!(method_names.contains(&&"getHexCode".to_string()));
    assert!(method_names.contains(&&"testColor".to_string()));

    for chunk in &chunks {
        assert_eq!(chunk.language, gittype::extractor::Language::Java);
    }
}

#[test]
fn test_java_field_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("Person.java");

    let java_code = r#"public class Person {
    private String name;
    private int age;
    private static final String DEFAULT_COUNTRY = "Unknown";
    public boolean isActive;
    
    public Person(String name, int age) {
        this.name = name;
        this.age = age;
        this.isActive = true;
    }
    
    public String getName() {
        return name;
    }
    
    public int getAge() {
        return age;
    }
}"#;
    fs::write(&file_path, java_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), ExtractionOptions::default())
        .unwrap();

    // Find class chunk
    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 1);
    assert_eq!(class_chunks[0].name, "Person");

    // Find field chunks
    let field_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Variable))
        .collect();
    assert!(field_chunks.len() >= 3);

    let field_names: Vec<&String> = field_chunks.iter().map(|c| &c.name).collect();
    assert!(field_names.contains(&&"name".to_string()));
    assert!(field_names.contains(&&"age".to_string()));
    assert!(field_names.contains(&&"isActive".to_string()));

    // Find method chunks
    let method_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Method))
        .collect();
    assert!(method_chunks.len() >= 3); // constructor + getName + getAge

    for chunk in &chunks {
        assert_eq!(chunk.language, gittype::extractor::Language::Java);
    }
}
