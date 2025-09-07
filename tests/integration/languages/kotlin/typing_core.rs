use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    kotlin_function_with_comment,
    "kotlin",
    r#"fun add(a: Int, b: Int): Int {
    return a + b // Return sum
}"#
);

typing_core_test_with_parser!(
    kotlin_class_with_comments,
    "kotlin",
    r#"class Calculator {
    private var result: Int = 0 // Store result
    
    fun add(value: Int): Calculator { // Add method
        result += value
        return this
    }
    
    fun getResult(): Int {
        return result // Return current result
    }
}"#
);

typing_core_test_with_parser!(
    kotlin_data_class_with_comments,
    "kotlin",
    r#"data class Person(
    val name: String, // Person's name
    val age: Int      // Person's age
) {
    fun greet() {
        println("Hello, I'm $name") // Print greeting
    }
}"#
);

typing_core_test_with_parser!(
    kotlin_sealed_class_with_comments,
    "kotlin",
    r#"sealed class Result<out T, out E> {
    /* Success case with value */
    data class Success<T>(val value: T) : Result<T, Nothing>()
    
    /* Failure case with error */
    data class Failure<E>(val error: E) : Result<Nothing, E>()
}"#
);

typing_core_test_with_parser!(
    kotlin_extension_with_comments,
    "kotlin",
    r#"fun String.isEmail(): Boolean {
    // Simple email validation
    return this.contains("@") && this.contains(".")
}

fun List<Int>.sum(): Int {
    var total = 0
    for (item in this) {
        total += item // Add each item
    }
    return total // Return sum
}"#
);
