use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rust;

impl Language for Rust {
    fn name(&self) -> &'static str {
        "rust"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["rs"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["rs"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_rust()
    }

    fn display_name(&self) -> &'static str {
        "Rust"
    }
}
