use super::super::language::Language;
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
        use crate::ui::Colors;
        Colors::LANG_DART
    }

    fn display_name(&self) -> &'static str {
        "Dart"
    }
}
