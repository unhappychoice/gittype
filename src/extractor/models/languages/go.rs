use super::super::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Go;

impl Language for Go {
    fn name(&self) -> &'static str {
        "go"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["go"]
    }

    fn color(&self) -> ratatui::style::Color {
        use crate::ui::Colors;
        Colors::lang_go()
    }

    fn display_name(&self) -> &'static str {
        "Go"
    }
}
