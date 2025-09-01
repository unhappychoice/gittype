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
                "**/*.py".to_string(),
                "**/*.rb".to_string(),
                "**/*.go".to_string(),
                "**/*.swift".to_string(),
                "**/*.kt".to_string(),
                "**/*.kts".to_string(),
                "**/*.java".to_string(),
            ],
            exclude_patterns: vec![
                "**/target/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/__pycache__/**".to_string(),
            ],
        }
    }
}
