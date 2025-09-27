use crate::extractor::models::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct C;

impl Language for C {
    fn name(&self) -> &'static str {
        "c"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["c", "h"]
    }

    fn color(&self) -> ratatui::style::Color {
        use crate::presentation::ui::Colors;
        Colors::lang_c()
    }

    fn display_name(&self) -> &'static str {
        "C"
    }
}
