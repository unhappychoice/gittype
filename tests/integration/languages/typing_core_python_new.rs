use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    python_function_with_comment,
    "python",
    r#"def greet(name):
    print(f"Hello {name}")  # Print greeting
    return name"#
);

typing_core_test_with_parser!(
    python_class_with_comments,
    "python",
    r#"class Calculator:
    def __init__(self):
        self.result = 0  # Initialize result
    
    def add(self, value):  # Add method
        self.result += value
        return self"#
);

typing_core_test_with_parser!(
    python_list_comprehension,
    "python",
    r#"def process_numbers(numbers):
    # Filter even numbers
    evens = [n for n in numbers if n % 2 == 0]
    # Square them
    squared = [n ** 2 for n in evens]
    return squared"#
);

typing_core_test_with_parser!(
    python_function_with_docstring,
    "python",
    r#"def factorial(n):
    """Calculate factorial of n"""
    if n <= 1:
        return 1
    return n * factorial(n - 1)  # Recursive call"#
);

typing_core_test_with_parser!(
    python_decorator,
    "python",
    r#"@property
def value(self):
    # Return the stored value
    return self._value

@value.setter
def value(self, val):
    self._value = val  # Store the value"#
);
