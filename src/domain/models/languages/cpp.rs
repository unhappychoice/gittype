use crate::domain::models::Language;
use crate::presentation::ui::Colors;
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
        Colors::lang_cpp()
    }

    fn display_name(&self) -> &'static str {
        "C++"
    }
}
