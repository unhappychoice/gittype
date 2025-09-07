use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    haskell_function_with_comment,
    "haskell",
    r#"-- Function to add two numbers
add :: Int -> Int -> Int
add x y = x + y -- Returns the sum"#
);
