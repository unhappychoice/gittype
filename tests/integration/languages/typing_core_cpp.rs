use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    cpp_function_with_comment,
    "cpp",
    r#"int add(int a, int b) {
    return a + b; // Return sum
}"#
);

typing_core_test_with_parser!(
    cpp_class_with_comments,
    "cpp",
    r#"class Calculator {
private:
    int result; // Store result
    
public:
    Calculator() : result(0) {} // Initialize
    
    void add(int value) { // Add method
        result += value;
    }
};"#
);

typing_core_test_with_parser!(
    cpp_function_with_block_comment,
    "cpp",
    r#"int factorial(int n) {
    /* Calculate factorial
       recursively */
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}"#
);

typing_core_test_with_parser!(
    cpp_template_class,
    "cpp",
    r#"template<typename T>
class Container {
private:
    T data; // The data
    
public:
    void set(const T& value) {
        data = value; // Store value
    }
    
    T get() const {
        return data; // Return data
    }
};"#
);

typing_core_test_with_parser!(
    cpp_namespace_with_comments,
    "cpp",
    r#"namespace math {
    // Constants
    const double PI = 3.14159;
    const double E = 2.71828;
    
    // Calculate circle area
    double circleArea(double radius) {
        return PI * radius * radius;
    }
}"#
);
