use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    c_function_with_comment,
    "c",
    r#"int add(int a, int b) {
    return a + b; // Return sum
}"#
);

typing_core_test_with_parser!(
    c_struct_with_comments,
    "c",
    r#"struct person {
    char name[50]; // Person's name
    int age;       // Person's age
};

void greet(struct person *p) {
    printf("Hello, %s\n", p->name); // Print greeting
}"#
);

typing_core_test_with_parser!(
    c_function_with_block_comment,
    "c",
    r#"int factorial(int n) {
    /* Calculate factorial
       using recursion */
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}"#
);

typing_core_test_with_parser!(
    c_enum_with_comments,
    "c",
    r#"enum color {
    RED,   // Red color
    GREEN, // Green color
    BLUE   // Blue color
};

void print_color(enum color c) {
    switch (c) {
        case RED:
            printf("Red\n");   // Print red
            break;
        case GREEN:
            printf("Green\n"); // Print green
            break;
        case BLUE:
            printf("Blue\n");  // Print blue
            break;
    }
}"#
);

typing_core_test_with_parser!(
    c_macros_with_comments,
    "c",
    r#"#define MAX_SIZE 100    // Maximum array size
#define PI 3.14159         // Pi constant

int main() {
    int array[MAX_SIZE];   // Declare array
    double radius = 5.0;   // Circle radius
    double area = PI * radius * radius; // Calculate area
    
    printf("Area: %.2f\n", area); // Print result
    return 0;
}"#
);
