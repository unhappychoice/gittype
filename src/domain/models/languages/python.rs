use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Python;

impl Language for Python {
    fn name(&self) -> &'static str {
        "python"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["py"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["py"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_python()
    }

    fn display_name(&self) -> &'static str {
        "Python"
    }
}
