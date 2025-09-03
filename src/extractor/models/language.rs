#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    TypeScript,
    JavaScript,
    Python,
    Ruby,
    Go,
    Swift,
    Kotlin,
    Java,
    Php,
    CSharp,
    C,
    Cpp,
    Haskell,
    Dart,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::Rust => "rust",
            Language::TypeScript => "typescript",
            Language::JavaScript => "javascript",
            Language::Python => "python",
            Language::Ruby => "ruby",
            Language::Go => "go",
            Language::Swift => "swift",
            Language::Kotlin => "kotlin",
            Language::Java => "java",
            Language::Php => "php",
            Language::CSharp => "csharp",
            Language::C => "c",
            Language::Cpp => "cpp",
            Language::Haskell => "haskell",
            Language::Dart => "dart",
        };
        write!(f, "{}", s)
    }
}

impl Language {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Some(Language::Rust),
            "ts" | "tsx" => Some(Language::TypeScript),
            "js" | "jsx" | "mjs" | "cjs" => Some(Language::JavaScript),
            "py" => Some(Language::Python),
            "rb" => Some(Language::Ruby),
            "go" => Some(Language::Go),
            "swift" => Some(Language::Swift),
            "kt" | "kts" => Some(Language::Kotlin),
            "java" => Some(Language::Java),
            "php" | "phtml" | "php3" | "php4" | "php5" => Some(Language::Php),
            "cs" | "csx" => Some(Language::CSharp),
            "c" | "h" => Some(Language::C),
            "cpp" | "cc" | "cxx" | "hpp" => Some(Language::Cpp),
            "hs" | "lhs" => Some(Language::Haskell),
            "dart" => Some(Language::Dart),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::TypeScript => "ts",
            Language::JavaScript => "js",
            Language::Python => "py",
            Language::Ruby => "rb",
            Language::Go => "go",
            Language::Swift => "swift",
            Language::Kotlin => "kt",
            Language::Java => "java",
            Language::Php => "php",
            Language::CSharp => "cs",
            Language::C => "c",
            Language::Cpp => "cpp",
            Language::Haskell => "hs",
            Language::Dart => "dart",
        }
    }

    pub fn detect_from_path(path: &std::path::Path) -> String {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => "rust".to_string(),
            Some("ts") | Some("tsx") => "typescript".to_string(),
            Some("js") | Some("jsx") | Some("mjs") | Some("cjs") => "javascript".to_string(),
            Some("py") => "python".to_string(),
            Some("go") => "go".to_string(),
            Some("rb") => "ruby".to_string(),
            Some("swift") => "swift".to_string(),
            Some("kt") | Some("kts") => "kotlin".to_string(),
            Some("java") => "java".to_string(),
            Some("php") | Some("phtml") | Some("php3") | Some("php4") | Some("php5") => {
                "php".to_string()
            }
            Some("cs") | Some("csx") => "csharp".to_string(),
            Some("c") | Some("h") => "c".to_string(),
            Some("cpp") | Some("cc") | Some("cxx") | Some("hpp") => "cpp".to_string(),
            Some("hs") | Some("lhs") => "haskell".to_string(),
            Some("dart") => "dart".to_string(),
            _ => "text".to_string(),
        }
    }
}
