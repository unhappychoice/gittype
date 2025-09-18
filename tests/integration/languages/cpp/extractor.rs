use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_cpp_function_extraction,
    language: "cpp",
    extension: "cpp",
    source: r#"
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
"#,
    total_chunks: 5,
    chunk_counts: {
        Function: 4,
    }
}

test_language_extractor! {
    name: test_cpp_class_extraction,
    language: "cpp",
    extension: "cpp",
    source: r#"
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
"#,
    total_chunks: 15,
    chunk_counts: {
        Function: 8,
        Struct: 2,
        Variable: 4,
    }
}

test_language_extractor! {
    name: test_cpp_namespace_extraction,
    language: "cpp",
    extension: "cpp",
    source: r#"
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
"#,
    total_chunks: 7,
    chunk_counts: {
        Function: 4,
        Variable: 2,
    }
}

test_language_extractor! {
    name: test_cpp_template_extraction,
    language: "cpp",
    extension: "cpp",
    source: r#"
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
"#,
    total_chunks: 14,
    chunk_counts: {
        CodeBlock: 1,
        Conditional: 1,
        Function: 6,
        Loop: 1,
        Struct: 2,
        Variable: 2,
    }
}

test_language_extractor! {
    name: test_cpp_constructor_destructor_extraction,
    language: "cpp",
    extension: "cpp",
    source: r#"
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
"#,
    total_chunks: 16,
    chunk_counts: {
        CodeBlock: 3,
        Conditional: 1,
        Function: 4,
        Loop: 2,
        Struct: 1,
        Variable: 4,
    }
}

test_language_extractor! {
    name: test_cpp_struct_extraction,
    language: "cpp",
    extension: "cpp",
    source: r#"
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
"#,
    total_chunks: 11,
    chunk_counts: {
        Function: 5,
        Struct: 2,
        Variable: 3,
    }
}

test_language_extractor! {
    name: test_cpp_complex_algorithm_extraction,
    language: "cpp",
    extension: "cpp",
    source: r#"
#include <vector>
#include <map>
#include <string>
#include <algorithm>
#include <memory>

template<typename T>
class DataProcessor {
private:
    std::map<std::string, T> cache;
    std::vector<T> processing_log;
    int threshold;

public:
    DataProcessor(int thresh) : threshold(thresh) {}

    std::vector<T> processComplexData(const std::vector<T>& input) {
        std::vector<T> results;
        results.reserve(input.size());
        int processed_count = 0;

        // Main processing algorithm - extractable middle chunk
        for (size_t i = 0; i < input.size(); ++i) {
            const T& value = input[i];
            std::string cache_key = "item_" + std::to_string(i);

            auto cache_it = cache.find(cache_key);
            if (cache_it != cache.end()) {
                results.push_back(cache_it->second);
                continue;
            }

            T processed_value;
            if (value > static_cast<T>(threshold)) {
                processed_value = value * static_cast<T>(2);
                processed_count++;

                // Additional processing for high values
                if (processed_value > static_cast<T>(threshold * 3)) {
                    processed_value += static_cast<T>(10); // bonus
                }
            } else if (value > static_cast<T>(0)) {
                processed_value = value + static_cast<T>(threshold);
            } else {
                continue; // skip negative values
            }

            cache[cache_key] = processed_value;
            processing_log.push_back(processed_value);
            results.push_back(processed_value);
        }

        // Finalization logic
        if (processed_count > 0) {
            T total = std::accumulate(results.begin(), results.end(), static_cast<T>(0));
            T average = total / static_cast<T>(results.size());

            // Add average to log for analysis
            processing_log.push_back(average);
        }

        return results;
    }

    std::map<std::string, int> analyzePatterns(const std::vector<T>& data) {
        std::map<std::string, int> analysis;
        std::map<std::string, std::vector<T>> categories;

        // Pattern analysis logic - extractable middle chunk
        for (const auto& item : data) {
            std::string category;

            if (item > static_cast<T>(threshold * 2)) {
                category = "HIGH";
            } else if (item > static_cast<T>(threshold)) {
                category = "MEDIUM";
            } else {
                category = "LOW";
            }

            categories[category].push_back(item);

            // Additional pattern detection
            if (item > static_cast<T>(1000)) {
                categories["PREMIUM"].push_back(item);
            }
        }

        // Calculate statistics for each category
        for (const auto& [cat_name, cat_data] : categories) {
            analysis[cat_name + "_count"] = static_cast<int>(cat_data.size());

            if (!cat_data.empty()) {
                T sum = std::accumulate(cat_data.begin(), cat_data.end(), static_cast<T>(0));
                analysis[cat_name + "_average"] = static_cast<int>(sum / static_cast<T>(cat_data.size()));
                analysis[cat_name + "_max"] = static_cast<int>(*std::max_element(cat_data.begin(), cat_data.end()));
            }
        }

        return analysis;
    }
};

// Specialized function for string processing
class StringProcessor {
public:
    static std::vector<std::string> processTextData(const std::vector<std::string>& input, const std::string& pattern) {
        std::vector<std::string> results;

        // Text processing algorithm - extractable middle chunk
        for (const auto& text : input) {
            std::string processed = text;

            // Pattern matching and transformation
            size_t pos = 0;
            while ((pos = processed.find(pattern, pos)) != std::string::npos) {
                // Replace pattern with uppercase version
                std::string replacement = pattern;
                std::transform(replacement.begin(), replacement.end(), replacement.begin(), ::toupper);
                processed.replace(pos, pattern.length(), replacement);
                pos += replacement.length();
            }

            // Additional text transformations
            if (processed.length() > 50) {
                processed = processed.substr(0, 47) + "...";
            }

            if (!processed.empty()) {
                results.push_back(processed);
            }
        }

        return results;
    }
};
"#,
    total_chunks: 40,
    chunk_counts: {
        Function: 4,
        Struct: 3,
    }
}
