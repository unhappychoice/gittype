use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Scala;

impl Language for Scala {
    fn name(&self) -> &'static str {
        "scala"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["sc", "scala"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["sc"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_scala()
    }

    fn display_name(&self) -> &'static str {
        "Scala"
    }
}
