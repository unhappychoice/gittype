use crate::extractor::models::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Php;

impl Language for Php {
    fn name(&self) -> &'static str {
        "php"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["php", "phtml", "php3", "php4", "php5"]
    }

    fn color(&self) -> ratatui::style::Color {
        use crate::presentation::ui::Colors;
        Colors::lang_php()
    }

    fn display_name(&self) -> &'static str {
        "PHP"
    }
}
