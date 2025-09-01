use gittype::extractor::{ExtractionOptions, RepositoryLoader};
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_swift_code_extraction() {
    let swift_code = r#"import Foundation

// A simple Swift class
class Calculator {
    private var value: Int = 0
    
    // Initialize calculator
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
    
    /* 
     * Multiply operation
     */
    func multiply(_ number: Int) -> Int {
        value *= number
        return value
    }
}

// Protocol definition
protocol Drawable {
    func draw()
}

struct Point {
    let x: Double
    let y: Double
    
    init(x: Double, y: Double) {
        self.x = x
        self.y = y
    }
    
    func distance(to other: Point) -> Double {
        return sqrt(pow(x - other.x, 2) + pow(y - other.y, 2))
    }
}

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

extension Point: Drawable {
    func draw() {
        print("Drawing point at (\(x), \(y))")
    }
}
"#;

    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path().with_extension("swift");
    fs::write(&temp_path, swift_code).expect("Failed to write test file");

    let mut loader = RepositoryLoader::new().expect("Failed to create loader");
    let options = ExtractionOptions::default();

    let challenges = loader
        .load_challenges_from_repository(&temp_path, Some(options))
        .expect("Failed to load challenges");

    println!("Found {} Swift challenges", challenges.len());

    for (i, challenge) in challenges.iter().enumerate() {
        println!("\n=== Swift Challenge {} ===", i + 1);
        println!("ID: {}", challenge.id);
        println!("Language: {:?}", challenge.language);
        println!("Content:");
        for (line_num, line) in challenge.code_content.lines().enumerate() {
            println!("  {}: '{}'", line_num + 1, line);
        }
        println!("Comment ranges: {:?}", challenge.comment_ranges);
    }

    assert!(
        !challenges.is_empty(),
        "Expected at least one Swift challenge"
    );

    // Verify we extract Swift structures
    let has_class = challenges
        .iter()
        .any(|c| c.code_content.contains("class Calculator"));
    let has_struct = challenges.iter().any(|c| {
        c.code_content.contains("struct Point") || c.code_content.contains("let x: Double")
    });
    let has_functions = challenges
        .iter()
        .any(|c| c.code_content.contains("func add"));
    let has_protocol = challenges
        .iter()
        .any(|c| c.code_content.contains("protocol Drawable"));
    let has_extension_block = challenges.iter().any(|c| {
        c.code_content.contains("extension Point: Drawable {")
            && c.code_content.contains("func draw()")
            && c.code_content.contains("Drawing point")
    });
    let has_enum = challenges.iter().any(|c| {
        c.code_content.contains("enum Direction {") && c.code_content.contains("case north")
    });
    let has_init = challenges
        .iter()
        .any(|c| c.code_content.contains("init(") && c.code_content.contains("self.x = x"));
    let has_deinit = challenges
        .iter()
        .any(|c| c.code_content.contains("deinit {") && c.code_content.contains("deallocated"));

    assert!(has_class, "Should extract Swift class");
    assert!(has_struct, "Should extract Swift struct");
    assert!(has_functions, "Should extract Swift functions");
    assert!(has_protocol, "Should extract Swift protocol");
    assert!(
        has_extension_block,
        "Should extract entire Swift extension blocks"
    );
    assert!(has_enum, "Should extract Swift enums");
    assert!(has_init, "Should extract Swift initializers");
    assert!(has_deinit, "Should extract Swift deinitializers");

    let _ = fs::remove_file(&temp_path);
}
