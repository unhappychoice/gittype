use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    javascript_function_with_comment,
    "javascript",
    r#"function greet(name) {
    console.log("Hello " + name); // Print greeting
    return name;
}"#
);

typing_core_test_with_parser!(
    javascript_class_with_comments,
    "javascript",
    r#"class Calculator {
    constructor() {
        this.result = 0; // Initialize result
    }
    
    add(value) { // Add method
        this.result += value;
        return this;
    }
}"#
);

typing_core_test_with_parser!(
    javascript_function_with_block_comment,
    "javascript",
    r#"function calculate(x, y) {
    /* This calculates the sum
       of two numbers */
    return x + y;
}"#
);

typing_core_test_with_parser!(
    javascript_arrow_function,
    "javascript",
    r#"const multiply = (a, b) => {
    // Simple multiplication
    return a * b;
};"#
);

typing_core_test_with_parser!(
    javascript_async_function,
    "javascript",
    r#"async function fetchData(url) {
    try {
        const response = await fetch(url); // Fetch data
        return response.json();
    } catch (error) {
        console.error(error); // Log error
    }
}"#
);
