#[derive(Debug, Clone)]
pub struct ExtractionOptions {
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub languages: Option<Vec<String>>,
    /// Maximum file size in bytes to process (default: 2MB)
    pub max_file_size_bytes: u64,
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            include_patterns: crate::domain::services::extractor::LanguageRegistry::all_file_patterns(),
            exclude_patterns: vec![
                // === Common build directories ===
                "**/build/**".to_string(),
                "**/dist/**".to_string(),
                "**/target/**".to_string(),
                "**/bin/**".to_string(),
                "**/obj/**".to_string(),
                // === Dependency directories ===
                "**/node_modules/**".to_string(), // JavaScript/TypeScript
                "**/vendor/**".to_string(),       // Go, PHP, Ruby
                // === Language-specific patterns ===

                // Python
                "**/__pycache__/**".to_string(),
                "**/*.pyc".to_string(),
                "**/venv/**".to_string(),
                "**/.venv/**".to_string(),
                "**/env/**".to_string(),
                // JavaScript/TypeScript
                "**/.next/**".to_string(),       // Next.js
                "**/.nuxt/**".to_string(),       // Nuxt.js
                "**/coverage/**".to_string(),    // Test coverage
                "**/.nyc_output/**".to_string(), // NYC coverage tool
                // Java/Kotlin
                "**/*.class".to_string(),
                "**/gradle/**".to_string(),   // Gradle wrapper
                "**/.gradle/**".to_string(),  // Gradle cache
                "**/buildSrc/**".to_string(), // Gradle buildSrc
                "**/.m2/**".to_string(),      // Maven cache
                "**/.ivy2/**".to_string(),    // SBT/Ivy cache
                // Ruby
                "**/bundle/**".to_string(),  // Bundler
                "**/.bundle/**".to_string(), // Bundler cache
                // Swift/iOS
                "**/.build/**".to_string(),
                "**/DerivedData/**".to_string(),
                "**/Pods/**".to_string(),     // CocoaPods
                "**/Carthage/**".to_string(), // Carthage
                // C#/.NET
                "**/packages.config".to_string(), // NuGet packages.config
                "**/packages/**/*.dll".to_string(), // NuGet compiled libraries
                "**/packages/**/*.pdb".to_string(), // NuGet debug symbols
                "**/packages/**/*.xml".to_string(), // NuGet documentation
                // C/C++
                "**/*.o".to_string(),
                "**/*.so".to_string(),
                "**/*.a".to_string(),
                "**/CMakeFiles/**".to_string(),
                "**/cmake-build-*/**".to_string(),
                "**/.vs/**".to_string(),     // Visual Studio
                "**/x64/**".to_string(),     // VS build dirs
                "**/x86/**".to_string(),     // VS build dirs
                "**/Debug/**".to_string(),   // VS/MSBuild
                "**/Release/**".to_string(), // VS/MSBuild
                // Dart/Flutter
                "**/.dart_tool/**".to_string(),
                // Haskell
                "**/.stack-work/**".to_string(),
                "**/dist-newstyle/**".to_string(),
                // === Generated code ===
                "**/generated/**".to_string(),
                "**/.generated/**".to_string(),
                "**/gen/**".to_string(),
                "**/codegen/**".to_string(),
                "**/*_pb2.py".to_string(), // Python protobuf
                "**/*.pb.go".to_string(),  // Go protobuf
                // === Build tools and caches ===
                "**/bazel-*/**".to_string(), // Bazel
                // === System and temporary files ===
                "**/.git/**".to_string(),
                "**/tmp/**".to_string(),
                "**/temp/**".to_string(),
                "**/*.tmp".to_string(),
                "**/cache/**".to_string(),
                "**/.cache/**".to_string(),
                "**/logs/**".to_string(),
                "**/*.log".to_string(),
                // === Performance test files (large generated files) ===
                "**/colorize-fixtures/**".to_string(),
                "**/perf-tests/**".to_string(),
            ],
            languages: None,
            max_file_size_bytes: 1024 * 1024, // 1MB limit
        }
    }
}

impl ExtractionOptions {
    pub fn apply_language_filter(&mut self) {
        if let Some(ref languages) = self.languages {
            let registry = crate::domain::services::extractor::LanguageRegistry::all_languages();
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
