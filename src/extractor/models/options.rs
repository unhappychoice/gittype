#[derive(Debug, Clone)]
pub struct ExtractionOptions {
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            include_patterns: vec![
                "**/*.rs".to_string(),
                "**/*.ts".to_string(),
                "**/*.tsx".to_string(),
                "**/*.js".to_string(),
                "**/*.jsx".to_string(),
                "**/*.mjs".to_string(),
                "**/*.cjs".to_string(),
                "**/*.py".to_string(),
                "**/*.rb".to_string(),
                "**/*.go".to_string(),
                "**/*.swift".to_string(),
                "**/*.kt".to_string(),
                "**/*.kts".to_string(),
                "**/*.java".to_string(),
                "**/*.php".to_string(),
                "**/*.phtml".to_string(),
                "**/*.php3".to_string(),
                "**/*.php4".to_string(),
                "**/*.php5".to_string(),
                "**/*.cs".to_string(),
                "**/*.csx".to_string(),
                "**/*.c".to_string(),
                "**/*.h".to_string(),
                "**/*.cpp".to_string(),
                "**/*.cc".to_string(),
                "**/*.cxx".to_string(),
                "**/*.hpp".to_string(),
                "**/*.hs".to_string(),
                "**/*.lhs".to_string(),
                "**/*.dart".to_string(),
            ],
            exclude_patterns: vec![
                "**/target/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/__pycache__/**".to_string(),
            ],
        }
    }
}
