use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    scala_function_with_comment,
    "scala",
    r#"def main(): Unit = {
    println("Hello") // This is a comment
}"#
);

typing_core_test_with_parser!(
    scala_class_with_comments,
    "scala",
    r#"class Person(val name: String, val age: Int) {
    // Person's greeting method
    def greet(): String = s"Hello, I'm $name"
    
    /* Check if person is adult
       Returns true if age >= 18 */
    def isAdult: Boolean = age >= 18
}"#
);

typing_core_test_with_parser!(
    scala_object_with_comments,
    "scala",
    r#"object Calculator {
    // Add two integers
    def add(a: Int, b: Int): Int = {
        a + b // Simple addition
    }
    
    /* Multiply two integers
       Returns the product */
    def multiply(x: Int, y: Int): Int = x * y
}"#
);

typing_core_test_with_parser!(
    scala_case_class_with_comments,
    "scala",
    r#"case class Point(x: Double, y: Double) {
    // Calculate distance to another point
    def distance(other: Point): Double = {
        /* Use Euclidean distance formula:
           sqrt((x2-x1)² + (y2-y1)²) */
        math.sqrt(math.pow(x - other.x, 2) + math.pow(y - other.y, 2))
    }
}"#
);

typing_core_test_with_parser!(
    scala_trait_with_comments,
    "scala",
    r#"trait Animal {
    // Abstract method for animal sound
    def speak(): String
    
    // Default implementation for movement
    def move(): Unit = {
        println("Moving...") // Print movement message
    }
    
    /* Method to get animal info
       Returns a formatted string */
    def info(): String = s"Animal that says: ${speak()}"
}"#
);

typing_core_test_with_parser!(
    scala_enum_with_comments,
    "scala",
    "enum Color {
    case Red, Green, Blue // Primary colors
    case RGB(r: Int, g: Int, b: Int) // Custom RGB color
    
    // Convert color to hex string
    def toHex(): String = this match {
        case Red => \"#FF0000\"    // Red hex
        case Green => \"#00FF00\"  // Green hex
        case Blue => \"#0000FF\"   // Blue hex
        /* Custom RGB to hex conversion
           Format: #RRGGBB */
        case RGB(r, g, b) => f\"#$r%02X$g%02X$b%02X\"
    }
}"
);

typing_core_test_with_parser!(
    scala_pattern_matching_with_comments,
    "scala",
    "def processValue(value: Any): String = {
    // Pattern matching on different types
    value match {
        case s: String => s\"String: $s\"     // Handle strings
        case i: Int => s\"Integer: $i\"       // Handle integers
        case d: Double => s\"Double: $d\"     // Handle doubles
        /* Default case for unknown types
           Returns a generic message */
        case _ => \"Unknown type\"
    }
}"
);

typing_core_test_with_parser!(
    scala_for_comprehension_with_comments,
    "scala",
    "def processNumbers(numbers: List[Int]): List[String] = {
    // Use for comprehension to transform numbers
    for {
        n <- numbers          // Extract each number
        if n > 0             // Filter positive numbers
        doubled = n * 2      // Double the value
        /* Convert to string with prefix
           Format: \"Value: XX\" */
    } yield s\"Value: $doubled\"
}"
);

typing_core_test_with_parser!(
    scala_implicit_with_comments,
    "scala",
    "object StringExtensions {
    // Implicit class to extend String functionality
    implicit class RichString(s: String) {
        // Check if string is palindrome
        def isPalindrome: Boolean = {
            val cleaned = s.toLowerCase.replaceAll(\"[^a-z]\", \"\")
            /* Compare string with its reverse
               Returns true if they match */
            cleaned == cleaned.reverse
        }
    }
}"
);

typing_core_test_with_parser!(
    scala_higher_order_functions,
    "scala",
    "def processData[T](data: List[T], processor: T => String): List[String] = {
    // Apply processor function to each element
    data.map { item =>
        /* Process each item individually
           Using the provided processor function */
        processor(item)
    }.filter(_.nonEmpty) // Filter out empty results
}"
);
