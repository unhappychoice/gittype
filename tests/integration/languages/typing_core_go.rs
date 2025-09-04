use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    go_function_with_comment,
    "go",
    r#"func add(a, b int) int {
    return a + b // Return sum
}"#
);

typing_core_test_with_parser!(
    go_struct_with_comments,
    "go",
    r#"type Person struct {
    Name string // Person's name
    Age  int    // Person's age
}

func (p *Person) Greet() {
    fmt.Printf("Hello, I'm %s", p.Name) // Print greeting
}"#
);

typing_core_test_with_parser!(
    go_function_with_block_comment,
    "go",
    r#"func factorial(n int) int {
    /* Calculate factorial
       using recursion */
    if n <= 1 {
        return 1
    }
    return n * factorial(n-1)
}"#
);

typing_core_test_with_parser!(
    go_interface_with_comments,
    "go",
    r#"type Shape interface {
    // Get the area
    Area() float64
    // Get the perimeter
    Perimeter() float64
}"#
);

typing_core_test_with_parser!(
    go_goroutine_with_comments,
    "go",
    r#"func processData(data []int, ch chan int) {
    result := 0
    for _, v := range data {
        result += v // Add each value
    }
    ch <- result // Send result
}

func main() {
    ch := make(chan int)
    go processData([]int{1, 2, 3}, ch) // Start goroutine
    result := <-ch // Receive result
    fmt.Println(result)
}"#
);
