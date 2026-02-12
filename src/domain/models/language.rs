use std::hash::{Hash, Hasher};

use crate::domain::models::languages::{
    CSharp, Clojure, Cpp, Dart, Go, Haskell, Java, JavaScript, Kotlin, Php, Python, Ruby, Rust,
    Scala, Swift, TypeScript, Zig, C,
};

/// Domain trait representing a programming language
pub trait Language: std::fmt::Debug + Send + Sync {
    /// Returns the internal name of the language
    fn name(&self) -> &'static str;

    /// Returns the display name of the language (defaults to name)
    fn display_name(&self) -> &'static str {
        self.name()
    }

    /// Returns the file extensions for this language
    fn extensions(&self) -> Vec<&'static str>;

    /// Returns alternative names/aliases for this language
    fn aliases(&self) -> Vec<&'static str> {
        vec![]
    }

    /// Returns file glob patterns for this language
    fn file_patterns(&self) -> Vec<String> {
        self.extensions()
            .into_iter()
            .map(|ext| format!("**/*.{}", ext))
            .collect()
    }

    /// Returns a unique hash key for this language
    fn as_hash_key(&self) -> &'static str {
        self.name()
    }

    /// Returns true if the tree-sitter node represents a valid comment for this language
    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool;

    /// Returns the color for this language
    fn color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self.name() {
            "rust" => Color::Red,
            "python" => Color::Blue,
            "javascript" => Color::Yellow,
            "typescript" => Color::Blue,
            "java" => Color::Red,
            "c" => Color::Blue,
            "cpp" => Color::Cyan,
            "csharp" => Color::Magenta,
            "go" => Color::Cyan,
            "ruby" => Color::Red,
            "php" => Color::Magenta,
            "swift" => Color::Red,
            "kotlin" => Color::Magenta,
            "scala" => Color::Red,
            "haskell" => Color::Magenta,
            "dart" => Color::Cyan,
            "zig" => Color::Yellow,
            _ => Color::White,
        }
    }
}

pub struct Languages;

impl Languages {
    pub fn get_language_by_name(name: &str) -> Option<Box<dyn Language>> {
        Self::get_by_name(name)
    }
}

impl Languages {
    pub fn all_languages() -> Vec<Box<dyn Language>> {
        vec![
            Box::new(Rust),
            Box::new(TypeScript),
            Box::new(JavaScript),
            Box::new(Python),
            Box::new(Ruby),
            Box::new(Go),
            Box::new(Swift),
            Box::new(Kotlin),
            Box::new(Java),
            Box::new(Php),
            Box::new(CSharp),
            Box::new(C),
            Box::new(Cpp),
            Box::new(Haskell),
            Box::new(Dart),
            Box::new(Scala),
            Box::new(Zig),
            Box::new(Clojure),
        ]
    }

    pub fn all_file_patterns() -> Vec<String> {
        Self::all_languages()
            .into_iter()
            .flat_map(|lang| lang.file_patterns())
            .collect()
    }

    pub fn get_supported_languages() -> Vec<&'static str> {
        Self::all_languages()
            .into_iter()
            .flat_map(|lang| {
                let mut names = vec![lang.name()];
                names.extend(lang.aliases());
                names
            })
            .collect()
    }

    pub fn validate_languages(languages: &[String]) -> Result<(), Vec<String>> {
        let supported = Self::get_supported_languages();
        let unsupported: Vec<String> = languages
            .iter()
            .filter(|lang| !supported.contains(&lang.to_lowercase().as_str()))
            .cloned()
            .collect();

        if unsupported.is_empty() {
            Ok(())
        } else {
            Err(unsupported)
        }
    }

    pub fn from_extension(extension: &str) -> Option<Box<dyn Language>> {
        Self::all_languages()
            .into_iter()
            .find(|lang| lang.extensions().contains(&extension))
    }

    pub fn detect_from_path(path: &std::path::Path) -> String {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => Self::from_extension(ext)
                .map(|lang| lang.name().to_string())
                .unwrap_or_else(|| "text".to_string()),
            None => "text".to_string(),
        }
    }

    pub fn get_by_name(name: &str) -> Option<Box<dyn Language>> {
        let name_lower = name.to_lowercase();
        Self::all_languages()
            .into_iter()
            .find(|lang| lang.name() == name_lower || lang.aliases().contains(&name_lower.as_str()))
    }

    pub fn get_display_name(language: Option<&str>) -> String {
        match language {
            Some(lang) => Self::get_by_name(lang)
                .map(|l| l.display_name().to_string())
                .unwrap_or_else(|| lang.to_string()),
            None => "Unknown".to_string(),
        }
    }
}

// Implement Hash for Box<dyn Language>
impl Hash for dyn Language {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_hash_key().hash(state);
    }
}

impl PartialEq for dyn Language {
    fn eq(&self, other: &Self) -> bool {
        self.as_hash_key() == other.as_hash_key()
    }
}

impl Eq for dyn Language {}
