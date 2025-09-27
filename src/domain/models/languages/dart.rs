use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dart;

impl Language for Dart {
    fn name(&self) -> &'static str {
        "dart"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["dart"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_dart()
    }

    fn display_name(&self) -> &'static str {
        "Dart"
    }
}
