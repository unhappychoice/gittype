use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    swift_function_with_comment,
    "swift",
    r#"func add(_ a: Int, _ b: Int) -> Int {
    return a + b // Return sum
}"#
);

typing_core_test_with_parser!(
    swift_class_with_comments,
    "swift",
    r#"class Calculator {
    private var result: Int = 0 // Store result
    
    func add(_ value: Int) -> Calculator { // Add method
        result += value
        return self
    }
    
    func getResult() -> Int {
        return result // Return current result
    }
}"#
);

typing_core_test_with_parser!(
    swift_struct_with_comments,
    "swift",
    r#"struct Person {
    let name: String // Person's name
    let age: Int     // Person's age
    
    func greet() {
        print("Hello, I'm \(name)") // Print greeting
    }
}"#
);

typing_core_test_with_parser!(
    swift_enum_with_comments,
    "swift",
    r#"enum Result<T, E> {
    case success(T) // Success case
    case failure(E) // Failure case
    
    /* Check if result is successful */
    var isSuccess: Bool {
        switch self {
        case .success:
            return true
        case .failure:
            return false
        }
    }
}"#
);

typing_core_test_with_parser!(
    swift_protocol_with_comments,
    "swift",
    r#"protocol Drawable {
    // Draw the object
    func draw()
    
    // Get the area
    var area: Double { get }
}

extension Drawable {
    // Default implementation
    func description() -> String {
        return "A drawable object with area \(area)"
    }
}"#
);
