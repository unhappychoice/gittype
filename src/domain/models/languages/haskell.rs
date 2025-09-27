use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Haskell;

impl Language for Haskell {
    fn name(&self) -> &'static str {
        "haskell"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["hs", "lhs"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["hs"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_haskell()
    }

    fn display_name(&self) -> &'static str {
        "Haskell"
    }
}
