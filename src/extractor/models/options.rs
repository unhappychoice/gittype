#[derive(Debug, Clone)]
pub struct ExtractionOptions {
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub languages: Option<Vec<String>>,
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            include_patterns: super::language::LanguageRegistry::all_file_patterns(),
            exclude_patterns: vec![
                // Build directories (common across multiple languages)
                "**/target/**".to_string(), // Rust, Maven (Java)
                "**/build/**".to_string(),  // JS/TS, Java (Gradle), C/C++, Kotlin, Dart
                "**/dist/**".to_string(),   // JS/TS build output
                "**/bin/**".to_string(),    // Java, C#
                "**/obj/**".to_string(),    // C#/.NET
                // Package/dependency managers
                "**/node_modules/**".to_string(), // JavaScript/TypeScript
                "**/vendor/**".to_string(),       // Go, PHP
                "**/packages/**".to_string(),     // C#/.NET NuGet
                // Python
                "**/__pycache__/**".to_string(),
                "**/*.pyc".to_string(),
                "**/venv/**".to_string(),
                "**/.venv/**".to_string(),
                "**/env/**".to_string(),
                // JavaScript/TypeScript frameworks
                "**/.next/**".to_string(), // Next.js
                // Java compiled files
                "**/*.class".to_string(),
                // Swift/iOS
                "**/.build/**".to_string(),
                "**/DerivedData/**".to_string(),
                // C/C++ compiled files
                "**/*.o".to_string(),
                "**/*.so".to_string(),
                "**/*.a".to_string(),
                // Dart/Flutter
                "**/.dart_tool/**".to_string(),
                // Haskell
                "**/.stack-work/**".to_string(),
                "**/dist-newstyle/**".to_string(),
                // Version control & general
                "**/.git/**".to_string(),
                "**/tmp/**".to_string(),
                "**/temp/**".to_string(),
                "**/*.tmp".to_string(),
                "**/cache/**".to_string(),
                "**/.cache/**".to_string(),
                "**/logs/**".to_string(),
                "**/*.log".to_string(),
            ],
            languages: None,
        }
    }
}

impl ExtractionOptions {
    pub fn apply_language_filter(&mut self) {
        if let Some(ref languages) = self.languages {
            let registry = super::language::LanguageRegistry::all_languages();
            self.include_patterns = registry
                .into_iter()
                .filter(|lang| {
                    languages.iter().any(|name| {
                        let name_lower = name.to_lowercase();
                        name_lower == lang.name() || lang.aliases().contains(&name_lower.as_str())
                    })
                })
                .flat_map(|lang| lang.file_patterns())
                .collect();
        }
    }
}
