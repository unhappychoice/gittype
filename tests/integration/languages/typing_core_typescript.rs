use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    typescript_function_with_comment,
    "typescript",
    r#"function add(a: number, b: number): number {
    return a + b; // Return sum
}"#
);

typing_core_test_with_parser!(
    typescript_class_with_comments,
    "typescript",
    r#"class Calculator {
    private result: number = 0; // Store result
    
    public add(value: number): Calculator { // Add method
        this.result += value;
        return this;
    }
    
    public getResult(): number {
        return this.result; // Return current result
    }
}"#
);

typing_core_test_with_parser!(
    typescript_interface_with_comments,
    "typescript",
    r#"interface User {
    id: number; // Unique identifier
    name: string; // User's name
    email?: string; // Optional email
}

function createUser(data: Partial<User>): User {
    // Create user with default values
    return {
        id: Math.random(),
        name: data.name || "Unknown",
        ...data
    };
}"#
);

typing_core_test_with_parser!(
    typescript_generic_function,
    "typescript",
    r#"function identity<T>(arg: T): T {
    /* Generic identity function
       returns the same type as input */
    return arg;
}"#
);

typing_core_test_with_parser!(
    typescript_async_function,
    "typescript",
    r#"async function fetchUser(id: number): Promise<User | null> {
    try {
        const response = await fetch(`/api/users/${id}`); // Fetch user
        return await response.json();
    } catch (error) {
        console.error(error); // Log error
        return null;
    }
}"#
);
