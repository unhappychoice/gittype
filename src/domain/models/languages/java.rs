use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Java;

impl Language for Java {
    fn name(&self) -> &'static str {
        "java"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["java"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_java()
    }

    fn display_name(&self) -> &'static str {
        "Java"
    }
}
