use crate::typing_core_test_with_parser;
use gittype::game::typing_core::ProcessingOptions;

typing_core_test_with_parser!(
    rust_function_with_comment,
    "rust",
    r#"fn main() {
    println!("Hello"); // This is a comment
}"#
);

typing_core_test_with_parser!(
    rust_struct_with_line_comments,
    "rust",
    r#"struct Person {
    name: String, // Person's name
    age: u32,     // Person's age in years
}"#
);

typing_core_test_with_parser!(
    rust_function_with_block_comment,
    "rust",
    r#"fn calculate(x: i32) -> i32 {
    /* This is a block comment
       spanning multiple lines */
    x * 2
}"#
);

typing_core_test_with_parser!(
    rust_empty_line_preservation_enabled,
    "rust",
    r#"fn main() {
    let x = 1;

    let y = 2;
    
    println!("{} {}", x, y);
}"#,
    ProcessingOptions {
        preserve_empty_lines: true,
        ..ProcessingOptions::default()
    }
);

typing_core_test_with_parser!(
    rust_empty_line_preservation_disabled,
    "rust",
    r#"fn main() {
    let x = 1;

    let y = 2;
    
    println!("{} {}", x, y);
}"#,
    ProcessingOptions {
        preserve_empty_lines: false,
        ..ProcessingOptions::default()
    }
);
