use super::super::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cpp;

impl Language for Cpp {
    fn name(&self) -> &'static str {
        "cpp"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["cpp", "cc", "cxx", "hpp"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["c++"]
    }

    fn color(&self) -> ratatui::style::Color {
        use crate::ui::Colors;
        Colors::LANG_CPP
    }

    fn display_name(&self) -> &'static str {
        "C++"
    }
}
