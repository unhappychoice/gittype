use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    java_method_with_comment,
    "java",
    r#"public void greet(String name) {
    System.out.println("Hello " + name); // Print greeting
}"#
);

typing_core_test_with_parser!(
    java_class_with_comments,
    "java",
    r#"public class Calculator {
    private int result = 0; // Initialize result
    
    public Calculator add(int value) { // Add method
        this.result += value;
        return this;
    }
}"#
);

typing_core_test_with_parser!(
    java_method_with_javadoc,
    "java",
    r#"/**
 * Calculates the factorial of a number
 * @param n the number
 * @return the factorial
 */
public int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1); // Recursive call
}"#
);

typing_core_test_with_parser!(
    java_interface_with_comments,
    "java",
    r#"public interface Drawable {
    // Draw the object
    void draw();
    
    // Get the area
    double getArea();
}"#
);

typing_core_test_with_parser!(
    java_generic_class,
    "java",
    r#"public class Container<T> {
    private T item; // The contained item
    
    public void setItem(T item) {
        this.item = item; // Store the item
    }
    
    public T getItem() {
        return item; // Return the item
    }
}"#
);
