use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    csharp_class_with_comment,
    "csharp",
    r#"public class Greeter
{
    // Method to greet
    public string SayHello(string name)
    {
        return $"Hello, {name}!"; // Returns a greeting
    }
}"#
);
