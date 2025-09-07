use crate::integration::test_extraction_options;
use gittype::extractor::{ChallengeConverter, CodeExtractor};
use gittype::models::{ChunkType, CodeChunk};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_haskell_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.hs");

    let haskell_code = r#"
factorial :: Integer -> Integer
factorial 0 = 1
factorial n = n * factorial (n - 1)

quicksort :: [a] -> [a]
quicksort [] = []
quicksort (x:xs) = quicksort [y | y <- xs, y < x] ++ [x] ++ quicksort [y | y <- xs, y >= x]
"#;
    fs::write(&file_path, haskell_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();

    assert!(
        function_chunks.len() >= 4,
        "Should find at least 4 function-related chunks"
    );

    // Check that factorial functions are found
    let has_factorial = function_chunks.iter().any(|c| c.name == "factorial");
    let has_quicksort = function_chunks.iter().any(|c| c.name == "quicksort");

    assert!(has_factorial, "Should find factorial function");
    assert!(has_quicksort, "Should find quicksort function");
}

#[test]
fn test_haskell_data_type_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.hs");

    let haskell_code = r#"
data Maybe a = Nothing | Just a

data Tree a = Leaf a | Branch (Tree a) (Tree a)

newtype UserId = UserId Int
"#;
    fs::write(&file_path, haskell_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find data type related chunks
    assert!(!chunks.is_empty(), "Should extract chunks from data types");

    // Check for constructor chunks
    let constructor_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Struct))
        .collect();

    assert!(
        !constructor_chunks.is_empty(),
        "Should find constructor chunks"
    );
}

#[test]
fn test_haskell_type_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.hs");

    let haskell_code = r#"
class Eq a where
    (==) :: a -> a -> Bool
    (/=) :: a -> a -> Bool

instance Eq Int where
    x == y = x `eqInt` y
    x /= y = not (x == y)
"#;
    fs::write(&file_path, haskell_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert!(
        !chunks.is_empty(),
        "Should extract chunks from type classes"
    );

    // Should find function chunks for type class methods
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();

    assert!(
        !function_chunks.is_empty(),
        "Should find type class related functions"
    );
}

#[test]
fn test_haskell_module_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.hs");

    let haskell_code = r#"
module Data.List.Utils where

import Data.List
import qualified Data.Set as Set

head' :: [a] -> a
head' [] = error "Empty list"
head' (x:_) = x
"#;
    fs::write(&file_path, haskell_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert!(!chunks.is_empty(), "Should extract chunks from module");

    // Should find module chunks
    let module_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Module))
        .collect();

    assert!(!module_chunks.is_empty(), "Should find module chunks");
}

#[test]
fn test_haskell_comprehensive_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.hs");

    let haskell_code = r#"
module TestModule where

-- Function definition with type signature
factorial :: Integer -> Integer
factorial 0 = 1
factorial n = n * factorial (n - 1)

-- Data type declaration
data Maybe a = Nothing | Just a

-- Type class definition
class Eq a where
    (==) :: a -> a -> Bool

-- Type alias
type Name = String

-- Instance declaration
instance Eq Int where
    x == y = x `eqInt` y
"#;
    fs::write(&file_path, haskell_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert!(
        !chunks.is_empty(),
        "Should extract chunks from comprehensive Haskell code"
    );

    // Check that we have chunks for different types
    let has_function = chunks
        .iter()
        .any(|c| matches!(c.chunk_type, ChunkType::Function));
    let has_struct = chunks
        .iter()
        .any(|c| matches!(c.chunk_type, ChunkType::Struct));
    let has_module = chunks
        .iter()
        .any(|c| matches!(c.chunk_type, ChunkType::Module));
    let has_type_alias = chunks
        .iter()
        .any(|c| matches!(c.chunk_type, ChunkType::TypeAlias));

    assert!(has_function, "Should extract functions");
    assert!(has_module, "Should extract module declaration");

    // At least some of these should be present
    assert!(
        has_struct || has_type_alias,
        "Should extract data types or type aliases"
    );

    println!("Extracted {} chunks total", chunks.len());
    for chunk in &chunks {
        println!(
            "  {:?} {}: {}",
            chunk.chunk_type,
            chunk.name,
            chunk.content.lines().next().unwrap_or("").trim()
        );
    }
}

#[test]
fn test_haskell_converter() {
    // CodeChunk is now imported from models at the top

    let converter = ChallengeConverter::new();
    let chunk = CodeChunk {
        content:
            "factorial :: Integer -> Integer\nfactorial 0 = 1\nfactorial n = n * factorial (n - 1)"
                .to_string(),
        file_path: PathBuf::from("src/Math.hs"),
        start_line: 5,
        end_line: 7,
        language: "haskell".to_string(),
        chunk_type: ChunkType::Function,
        name: "factorial".to_string(),
        comment_ranges: vec![],
        original_indentation: 0,
    };

    let challenge = converter.convert_chunk_to_challenge(chunk);

    assert_eq!(challenge.language, Some("haskell".to_string()));
    assert_eq!(challenge.source_file_path, Some("src/Math.hs".to_string()));
    assert!(challenge.code_content.contains("factorial"));
    assert!(!challenge.id.is_empty());
}
