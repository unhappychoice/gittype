use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_rust_function_extraction,
    language: "rust",
    extension: "rs",
    source: r#"
fn hello_world() {
    println!("Hello, world!");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        Function: 2,
        CodeBlock: 1,
    }
}

test_language_extractor! {
    name: test_rust_struct_extraction,
    language: "rust",
    extension: "rs",
    source: r#"
struct Person {
    name: String,
    age: u32,
}

pub struct Config {
    debug: bool,
}
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Struct: 2,
    }
}

test_language_extractor! {
    name: test_rust_enum_extraction,
    language: "rust",
    extension: "rs",
    source: r#"
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Color {
    Red,
    Green,
    Blue,
}
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Enum: 2,
    }
}

test_language_extractor! {
    name: test_rust_trait_extraction,
    language: "rust",
    extension: "rs",
    source: r#"
pub trait Display {
    fn fmt(&self) -> String;

    fn to_string(&self) -> String {
        self.fmt()
    }
}

trait Clone {
    fn clone(&self) -> Self;
}
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        Function: 1,
        Trait: 2,
    }
}

test_language_extractor! {
    name: test_rust_module_extraction,
    language: "rust",
    extension: "rs",
    source: r#"
pub mod utils {
    pub fn helper() -> i32 {
        42
    }

    pub struct Config {
        value: String,
    }
}

mod private_utils {
    fn internal_function() {}
}
"#,
    total_chunks: 7,
    chunk_counts: {
        File: 1,
        Function: 2,
        CodeBlock: 1,
        Module: 2,
        Struct: 1
    }
}

test_language_extractor! {
    name: test_rust_type_alias_extraction,
    language: "rust",
    extension: "rs",
    source: r#"
pub type UserId = u64;
pub type DatabaseResult<T> = Result<T, String>;
type Point = (f64, f64);
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        TypeAlias: 3,
    }
}

test_language_extractor! {
    name: test_rust_all_constructs_combined,
    language: "rust",
    extension: "rs",
    source: r#"
// Enum with variants
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Trait definition
pub trait Display {
    fn fmt(&self) -> String;
}

// Module definition
pub mod utils {
    pub fn helper() -> i32 {
        42
    }
}

// Type alias
pub type UserId = u64;

// Existing constructs (should still work)
pub struct User {
    id: UserId,
    name: String,
}

impl Display for User {
    fn fmt(&self) -> String {
        format!("User({})", self.name)
    }
}

pub fn create_user(name: String) -> User {
    User {
        id: 0,
        name,
    }
}
"#,
    total_chunks: 11,
    chunk_counts: {
        File: 1,
        Function: 3,
        TypeAlias: 1,
        CodeBlock :1,
        Struct: 1,
        Trait: 1,
        Enum: 1,
        Module: 1,
        Class: 1,
    }
}

test_language_extractor! {
    name: test_nested_and_oneline_structures,
    language: "rust",
    extension: "rs",
    source: r#"mod calculator {
    pub struct Calculator;

    impl Calculator {
        pub fn new() -> Self { Self }

        pub fn complex_calculation(&self, values: &[i32]) -> i32 {
            values.iter().sum()
        }
    }

    impl Default for Calculator {
        fn default() -> Self {
            Self::new()
        }
    }

    mod advanced {
        use super::Calculator;

        impl Calculator {
            pub fn advanced_method(&self) -> String {
                "advanced".to_string()
            }
        }
    }
}
"#,
    total_chunks: 17,
    chunk_counts: {
        File: 1,
        Function: 4,
        CodeBlock: 6,
        Module: 2,
        Struct: 1,
        Class: 3
    }
}

test_language_extractor! {
    name: test_comment_ranges_in_real_challenge,
    language: "rust",
    extension: "rs",
    source: r#"// Sample function with comments
fn calculate_sum(a: i32, b: i32) -> i32 {
    let result = a + b; // Add the numbers
    /*
     * Return the result
     */
    result
}
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Function: 1,
        CodeBlock: 1,
    }
}

test_language_extractor! {
    name: test_rust_complex_algorithm_extraction,
    language: "rust",
    extension: "rs",
    source: r#"
use std::collections::HashMap;

fn complex_data_processor(input: Vec<i32>, threshold: i32) -> HashMap<String, i32> {
    let mut result = HashMap::new();
    let mut counter = 0;

    // Main processing loop - extractable middle chunk
    for (index, value) in input.iter().enumerate() {
        let key = format!("item_{}", index);

        if *value > threshold {
            let processed = value * 2;
            result.insert(key.clone(), processed);
            counter += 1;

            // Additional processing for high values
            if processed > 100 {
                let bonus_key = format!("{}_bonus", key);
                result.insert(bonus_key, processed / 10);
            }
        } else if *value > 0 {
            let adjusted = value + threshold;
            result.insert(key, adjusted);
            counter += 1;
        }
    }

    // Finalization logic
    if counter > 0 {
        result.insert("total_processed".to_string(), counter);
        result.insert("average_processed".to_string(),
                     result.values().sum::<i32>() / counter);
    }

    result
}

pub fn advanced_string_matcher(patterns: &[&str], text: &str) -> Vec<(usize, String)> {
    let mut matches = Vec::new();

    // Pattern matching algorithm - extractable middle chunk
    for (pattern_idx, pattern) in patterns.iter().enumerate() {
        let mut search_start = 0;

        while let Some(pos) = text[search_start..].find(pattern) {
            let absolute_pos = search_start + pos;
            let context_start = absolute_pos.saturating_sub(10);
            let context_end = (absolute_pos + pattern.len() + 10).min(text.len());
            let context = text[context_start..context_end].to_string();

            matches.push((absolute_pos, context));
            search_start = absolute_pos + 1;

            // Prevent infinite loops
            if search_start >= text.len() {
                break;
            }
        }
    }

    matches.sort_by_key(|&(pos, _)| pos);
    matches.dedup_by_key(|&mut (pos, _)| pos);
    matches
}
"#,
    total_chunks: 15,
    chunk_counts: {
        File: 1,
        CodeBlock: 3,
        FunctionCall: 1,
        Function: 2,
        Loop: 3,
        Conditional: 5,
    }
}

test_language_extractor! {
    name: test_rust_struct_with_complex_impl,
    language: "rust",
    extension: "rs",
    source: r#"
#[derive(Debug, Clone)]
pub struct DataCache<T> {
    cache: HashMap<String, T>,
    max_size: usize,
    access_count: HashMap<String, usize>,
}

impl<T: Clone> DataCache<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            access_count: HashMap::new(),
        }
    }

    pub fn get_or_insert<F>(&mut self, key: &str, compute: F) -> T
    where
        F: FnOnce() -> T,
    {
        // Cache management logic - extractable middle chunk
        if let Some(value) = self.cache.get(key) {
            *self.access_count.entry(key.to_string()).or_insert(0) += 1;
            return value.clone();
        }

        // Check if cache is full and evict least used item
        if self.cache.len() >= self.max_size {
            if let Some(lru_key) = self.find_least_used_key() {
                self.cache.remove(&lru_key);
                self.access_count.remove(&lru_key);
            }
        }

        let computed_value = compute();
        self.cache.insert(key.to_string(), computed_value.clone());
        self.access_count.insert(key.to_string(), 1);
        computed_value
    }

    fn find_least_used_key(&self) -> Option<String> {
        self.access_count
            .iter()
            .min_by_key(|(_, &count)| count)
            .map(|(key, _)| key.clone())
    }

    pub fn clear_stale_entries(&mut self, max_access_count: usize) {
        // Cleanup logic - extractable middle chunk
        let stale_keys: Vec<String> = self
            .access_count
            .iter()
            .filter(|(_, &count)| count > max_access_count)
            .map(|(key, _)| key.clone())
            .collect();

        for key in stale_keys {
            self.cache.remove(&key);
            self.access_count.remove(&key);
        }
    }
}
"#,
    total_chunks: 21,
    chunk_counts: {
        File: 1,
        Struct: 1,
        FunctionCall: 7,
        Function: 4,
        Loop: 1,
        Conditional: 3,
        CodeBlock: 4,
    }
}
