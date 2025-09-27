use crate::domain::models::Language;
use crate::presentation::ui::Colors;
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
        Colors::lang_kotlin()
    }

    fn display_name(&self) -> &'static str {
        "Kotlin"
    }
}
