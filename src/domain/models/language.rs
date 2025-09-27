use crate::presentation::ui::Colors;
use std::hash::{Hash, Hasher};

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

    /// Returns the UI color for this language
    fn color(&self) -> ratatui::style::Color {
        Colors::lang_default()
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