use crate::integration::test_extraction_options;
use gittype::extractor::CodeExtractor;
use gittype::models::ChunkType;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_c_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.c");

    let c_code = r#"
int main(void) {
    printf("Hello, world!");
    return 0;
}

int add(int a, int b) {
    return a + b;
}

void print_number(int num) {
    printf("%d\n", num);
}
"#;
    fs::write(&file_path, c_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    println!("Found {} chunks:", chunks.len());
    for chunk in &chunks {
        println!("  - {} (type: {:?})", chunk.name, chunk.chunk_type);
    }

    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0].name, "main");
    assert_eq!(chunks[1].name, "add");
    assert_eq!(chunks[2].name, "print_number");
    assert!(matches!(chunks[0].chunk_type, ChunkType::Function));
    assert!(matches!(chunks[1].chunk_type, ChunkType::Function));
    assert!(matches!(chunks[2].chunk_type, ChunkType::Function));
}

#[test]
fn test_c_struct_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.c");

    let c_code = r#"
struct Point {
    int x;
    int y;
};

struct Person {
    char name[50];
    int age;
    struct Point location;
};

int main() {
    struct Point p = {10, 20};
    return 0;
}
"#;
    fs::write(&file_path, c_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Find struct chunks
    let struct_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Struct))
        .collect();

    assert_eq!(struct_chunks.len(), 2);
    assert!(struct_chunks.iter().any(|chunk| chunk.name == "Point"));
    assert!(struct_chunks.iter().any(|chunk| chunk.name == "Person"));
}

#[test]
fn test_c_variable_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.c");

    let c_code = r#"
int global_counter = 0;
char *buffer = NULL;
static int static_var = 42;

int main() {
    int local_var = 10;
    char arr[100];
    float *ptr = malloc(sizeof(float));
    return 0;
}
"#;
    fs::write(&file_path, c_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Find variable chunks
    let variable_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Variable))
        .collect();

    assert!(!variable_chunks.is_empty());
    // Check that we can extract at least some variable names
    let variable_names: Vec<&str> = variable_chunks
        .iter()
        .map(|chunk| chunk.name.as_str())
        .collect();
    assert!(variable_names.contains(&"global_counter") || variable_names.contains(&"buffer"));
}

#[test]
fn test_c_function_declarations() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.h");

    let c_code = r#"
#ifndef TEST_H
#define TEST_H

// Function declarations
int calculate_sum(int a, int b);
void print_message(const char *msg);
float compute_average(int *values, int count);

// Function definitions
static inline int max(int a, int b) {
    return (a > b) ? a : b;
}

#endif
"#;
    fs::write(&file_path, c_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Find function chunks (both declarations and definitions)
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Function))
        .collect();

    assert!(!function_chunks.is_empty());
    let function_names: Vec<&str> = function_chunks
        .iter()
        .map(|chunk| chunk.name.as_str())
        .collect();

    // Should include both declarations and the inline definition
    assert!(
        function_names.contains(&"calculate_sum")
            || function_names.contains(&"print_message")
            || function_names.contains(&"max")
    );
}

#[test]
fn test_c_complex_types() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.c");

    let c_code = r#"
typedef struct {
    int id;
    char name[32];
} User;

typedef union {
    int i;
    float f;
    char c[4];
} Value;

enum Status {
    SUCCESS,
    ERROR,
    PENDING
};

int process_user(User *user, enum Status *status) {
    if (!user || !status) return -1;
    *status = SUCCESS;
    return user->id;
}
"#;
    fs::write(&file_path, c_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert!(!chunks.is_empty());

    // Should find the function
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Function))
        .collect();
    assert!(!function_chunks.is_empty());
    assert!(function_chunks
        .iter()
        .any(|chunk| chunk.name == "process_user"));
}
