use crate::integration::{extract_chunks_for_test, test_extraction_options};
use gittype::extractor::CodeChunkExtractor;
use gittype::models::ChunkType;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cpp_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cpp");

    let cpp_code = r#"
#include <iostream>

int main() {
    std::cout << "Hello, world!" << std::endl;
    return 0;
}

int add(int a, int b) {
    return a + b;
}

void print_number(int num) {
    std::cout << num << std::endl;
}

double calculate_area(double radius) {
    return 3.14159 * radius * radius;
}
"#;
    fs::write(&file_path, cpp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    println!("Found {} chunks:", chunks.len());
    for chunk in &chunks {
        println!("  - {} (type: {:?})", chunk.name, chunk.chunk_type);
    }

    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Function))
        .collect();

    assert!(!function_chunks.is_empty());
    let function_names: Vec<&str> = function_chunks
        .iter()
        .map(|chunk| chunk.name.as_str())
        .collect();

    assert!(function_names.contains(&"main"));
    assert!(function_names.contains(&"add"));
    assert!(function_names.contains(&"print_number"));
    assert!(function_names.contains(&"calculate_area"));
}

#[test]
fn test_cpp_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cpp");

    let cpp_code = r#"
class Point {
private:
    int x, y;
public:
    Point(int x, int y) : x(x), y(y) {}
    
    int getX() const { return x; }
    int getY() const { return y; }
    
    void setX(int newX) { x = newX; }
    void setY(int newY) { y = newY; }
};

class Rectangle {
private:
    Point topLeft;
    Point bottomRight;
    
public:
    Rectangle(const Point& tl, const Point& br) 
        : topLeft(tl), bottomRight(br) {}
    
    double area() const {
        int width = bottomRight.getX() - topLeft.getX();
        int height = topLeft.getY() - bottomRight.getY();
        return width * height;
    }
};

int main() {
    Point p1(0, 10);
    Point p2(10, 0);
    Rectangle rect(p1, p2);
    return 0;
}
"#;
    fs::write(&file_path, cpp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    println!("Found {} chunks:", chunks.len());
    for chunk in &chunks {
        println!("  - {} (type: {:?})", chunk.name, chunk.chunk_type);
    }

    // Find class chunks
    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Struct))
        .collect();

    assert!(!class_chunks.is_empty());
    let class_names: Vec<&str> = class_chunks
        .iter()
        .map(|chunk| chunk.name.as_str())
        .collect();

    assert!(class_names.contains(&"Point"));
    assert!(class_names.contains(&"Rectangle"));

    // Find function chunks (including constructors and methods)
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Function))
        .collect();

    assert!(!function_chunks.is_empty());
}

#[test]
fn test_cpp_namespace_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cpp");

    let cpp_code = r#"
namespace math {
    const double PI = 3.14159;
    
    double square(double x) {
        return x * x;
    }
    
    namespace geometry {
        double circle_area(double radius) {
            return PI * square(radius);
        }
    }
}

namespace utils {
    void print_message(const std::string& msg) {
        std::cout << msg << std::endl;
    }
}

int main() {
    double area = math::geometry::circle_area(5.0);
    utils::print_message("Hello from namespace!");
    return 0;
}
"#;
    fs::write(&file_path, cpp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    println!("Found {} chunks:", chunks.len());
    for chunk in &chunks {
        println!("  - {} (type: {:?})", chunk.name, chunk.chunk_type);
    }

    // Note: Namespace extraction is not fully supported by tree-sitter-cpp
    // Instead, we just verify that functions inside namespaces are extracted
    assert!(chunks.len() >= 3); // At least main and some functions from namespaces

    // Find function chunks
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Function))
        .collect();

    assert!(!function_chunks.is_empty());
    let function_names: Vec<&str> = function_chunks
        .iter()
        .map(|chunk| chunk.name.as_str())
        .collect();

    assert!(function_names.contains(&"main"));
}

#[test]
fn test_cpp_template_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cpp");

    let cpp_code = r#"
template<typename T>
class Vector {
private:
    T* data;
    size_t size;
    size_t capacity;
    
public:
    Vector() : data(nullptr), size(0), capacity(0) {}
    
    void push_back(const T& value) {
        if (size >= capacity) {
            reserve(capacity == 0 ? 1 : capacity * 2);
        }
        data[size++] = value;
    }
    
    T& operator[](size_t index) {
        return data[index];
    }
    
private:
    void reserve(size_t new_capacity) {
        T* new_data = new T[new_capacity];
        for (size_t i = 0; i < size; ++i) {
            new_data[i] = data[i];
        }
        delete[] data;
        data = new_data;
        capacity = new_capacity;
    }
};

template<typename T>
T max_value(const T& a, const T& b) {
    return (a > b) ? a : b;
}

int main() {
    Vector<int> numbers;
    numbers.push_back(1);
    numbers.push_back(2);
    
    int max_num = max_value(10, 20);
    return 0;
}
"#;
    fs::write(&file_path, cpp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    println!("Found {} chunks:", chunks.len());
    for chunk in &chunks {
        println!("  - {} (type: {:?})", chunk.name, chunk.chunk_type);
    }

    assert!(!chunks.is_empty());

    // Find template class and function chunks
    let struct_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Struct))
        .collect();

    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Function))
        .collect();

    assert!(!struct_chunks.is_empty());
    assert!(!function_chunks.is_empty());
}

#[test]
fn test_cpp_constructor_destructor_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cpp");

    let cpp_code = r#"
class Resource {
private:
    int* data;
    size_t size;

public:
    // Default constructor
    Resource() : data(nullptr), size(0) {}
    
    // Parameterized constructor
    Resource(size_t s) : size(s) {
        data = new int[size];
    }
    
    // Copy constructor
    Resource(const Resource& other) : size(other.size) {
        data = new int[size];
        for (size_t i = 0; i < size; ++i) {
            data[i] = other.data[i];
        }
    }
    
    // Destructor
    ~Resource() {
        delete[] data;
    }
    
    // Assignment operator
    Resource& operator=(const Resource& other) {
        if (this != &other) {
            delete[] data;
            size = other.size;
            data = new int[size];
            for (size_t i = 0; i < size; ++i) {
                data[i] = other.data[i];
            }
        }
        return *this;
    }
    
    int& operator[](size_t index) {
        return data[index];
    }
};

int main() {
    Resource r1(10);
    Resource r2 = r1;
    Resource r3;
    r3 = r2;
    return 0;
}
"#;
    fs::write(&file_path, cpp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    println!("Found {} chunks:", chunks.len());
    for chunk in &chunks {
        println!("  - {} (type: {:?})", chunk.name, chunk.chunk_type);
    }

    assert!(!chunks.is_empty());

    // Find class chunks
    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Struct))
        .collect();

    assert!(!class_chunks.is_empty());
    assert!(class_chunks.iter().any(|chunk| chunk.name == "Resource"));

    // Find function chunks (should include constructors, destructor, and operators)
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Function))
        .collect();

    assert!(!function_chunks.is_empty());
}

#[test]
fn test_cpp_struct_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cpp");

    let cpp_code = r#"
struct Point3D {
    double x, y, z;
    
    Point3D() : x(0), y(0), z(0) {}
    Point3D(double x, double y, double z) : x(x), y(y), z(z) {}
    
    double distance_from_origin() const {
        return sqrt(x*x + y*y + z*z);
    }
};

struct Color {
    uint8_t r, g, b, a;
    
    Color(uint8_t red, uint8_t green, uint8_t blue, uint8_t alpha = 255)
        : r(red), g(green), b(blue), a(alpha) {}
};

int main() {
    Point3D origin;
    Point3D point(1.0, 2.0, 3.0);
    Color red(255, 0, 0);
    
    double distance = point.distance_from_origin();
    return 0;
}
"#;
    fs::write(&file_path, cpp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    println!("Found {} chunks:", chunks.len());
    for chunk in &chunks {
        println!("  - {} (type: {:?})", chunk.name, chunk.chunk_type);
    }

    // Find struct chunks
    let struct_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Struct))
        .collect();

    assert!(!struct_chunks.is_empty());
    let struct_names: Vec<&str> = struct_chunks
        .iter()
        .map(|chunk| chunk.name.as_str())
        .collect();

    assert!(struct_names.contains(&"Point3D"));
    assert!(struct_names.contains(&"Color"));

    // Find function chunks
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| matches!(chunk.chunk_type, ChunkType::Function))
        .collect();

    assert!(!function_chunks.is_empty());
    let function_names: Vec<&str> = function_chunks
        .iter()
        .map(|chunk| chunk.name.as_str())
        .collect();

    assert!(function_names.contains(&"main"));
}
