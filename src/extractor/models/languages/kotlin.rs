use super::super::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Kotlin;

impl Language for Kotlin {
    fn name(&self) -> &'static str {
        "kotlin"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["kt", "kts"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["kt"]
    }

    fn color(&self) -> ratatui::style::Color {
        use crate::ui::Colors;
        Colors::LANG_KOTLIN
    }

    fn display_name(&self) -> &'static str {
        "Kotlin"
    }
}
