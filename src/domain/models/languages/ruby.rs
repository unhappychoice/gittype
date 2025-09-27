use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ruby;

impl Language for Ruby {
    fn name(&self) -> &'static str {
        "ruby"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["rb"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["rb"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_ruby()
    }

    fn display_name(&self) -> &'static str {
        "Ruby"
    }
}
