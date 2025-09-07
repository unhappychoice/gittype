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

typing_core_test_with_parser!(
    csharp_struct_with_comments,
    "csharp",
    r#"public struct Point
{
    public int X; // X-coordinate
    public int Y; // Y-coordinate
}"#
);

typing_core_test_with_parser!(
    csharp_interface_with_comments,
    "csharp",
    r#"public interface ILogger
{
    void Log(string message); // Log a message
    void LogError(string message); // Log an error
}"#
);

typing_core_test_with_parser!(
    csharp_method_with_xml_doc,
    "csharp",
    r#"/// <summary>
/// Adds two integers.
/// </summary>
/// <param name="a">The first integer.</param>
/// <param name="b">The second integer.</param>
/// <returns>The sum of the two integers.</returns>
public int Add(int a, int b)
{
    return a + b;
}"#
);