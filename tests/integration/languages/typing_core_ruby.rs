use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    ruby_method_with_comment,
    "ruby",
    r#"def add(a, b)
  a + b # Return sum
end"#
);

typing_core_test_with_parser!(
    ruby_class_with_comments,
    "ruby",
    r#"class Calculator
  def initialize
    @result = 0 # Initialize result
  end
  
  def add(value) # Add method
    @result += value
    self
  end
  
  def result
    @result # Return current result
  end
end"#
);

typing_core_test_with_parser!(
    ruby_module_with_comments,
    "ruby",
    r#"module MathUtils
  # Mathematical constants
  PI = 3.14159
  E = 2.71828
  
  # Calculate circle area
  def self.circle_area(radius)
    PI * radius * radius
  end
  
  # Calculate factorial
  def self.factorial(n)
    return 1 if n <= 1
    n * factorial(n - 1) # Recursive call
  end
end"#
);

typing_core_test_with_parser!(
    ruby_block_with_comments,
    "ruby",
    r#"def process_numbers(numbers)
  # Filter and map in chain
  numbers.select { |n| n.even? }    # Keep only even numbers
         .map { |n| n * 2 }          # Double each number
         .reduce(0) { |sum, n| sum + n } # Sum all numbers
end"#
);

typing_core_test_with_parser!(
    ruby_attr_with_comments,
    "ruby",
    r#"class Person
  attr_reader :name   # Read-only name
  attr_writer :age    # Write-only age
  attr_accessor :email # Read-write email
  
  def initialize(name)
    @name = name # Set name
    @age = 0     # Default age
  end
  
  def greet
    puts "Hello, I'm #{@name}" # Print greeting
  end
end"#
);
