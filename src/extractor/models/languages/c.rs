use super::super::language::Language;
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
        use crate::ui::Colors;
        Colors::LANG_C
    }

    fn display_name(&self) -> &'static str {
        "C"
    }
}
