use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    haskell_function_with_comment,
    "haskell",
    r#"-- Function to add two numbers
add :: Int -> Int -> Int
add x y = x + y -- Returns the sum"#
);

typing_core_test_with_parser!(
    haskell_data_type_with_comments,
    "haskell",
    r#"-- Represents a person with a name and age
data Person = Person
  { name :: String  -- The person's name
  , age  :: Int     -- The person's age
  } deriving (Show)"#
);

typing_core_test_with_parser!(
    haskell_typeclass_with_comments,
    "haskell",
    r#"-- A typeclass for things that can be greeted
class Greetable a where
  greet :: a -> String -- The greeting function"#
);

typing_core_test_with_parser!(
    haskell_multi_line_comment,
    "haskell",
    r#"{- A multi-line comment
   describing the function below -}
factorial :: Integer -> Integer
factorial 0 = 1
factorial n = n * factorial (n - 1)"#
);
