use crate::domain::models::languages::{
    CSharp, Cpp, Dart, Go, Haskell, Java, JavaScript, Kotlin, Php, Python, Ruby, Rust, Scala,
    Swift, TypeScript, C,
};
use crate::domain::models::Language;
use crate::presentation::ui::Colors;

pub struct LanguageRegistry;

impl LanguageRegistry {
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

    /// Get language by name or alias
    pub fn get_by_name(name: &str) -> Option<Box<dyn Language>> {
        let name_lower = name.to_lowercase();
        Self::all_languages()
            .into_iter()
            .find(|lang| lang.name() == name_lower || lang.aliases().contains(&name_lower.as_str()))
    }

    /// Get color for a language string
    pub fn get_color(language: Option<&str>) -> ratatui::style::Color {
        match language {
            Some(lang) => Self::get_by_name(lang)
                .map(|l| l.color())
                .unwrap_or_else(Colors::lang_default),
            None => Colors::lang_default(),
        }
    }

    /// Get display name for a language string
    pub fn get_display_name(language: Option<&str>) -> String {
        match language {
            Some(lang) => Self::get_by_name(lang)
                .map(|l| l.display_name().to_string())
                .unwrap_or_else(|| lang.to_string()),
            None => "Unknown".to_string(),
        }
    }
}
