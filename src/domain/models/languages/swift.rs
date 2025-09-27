use crate::domain::models::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Swift;

impl Language for Swift {
    fn name(&self) -> &'static str {
        "swift"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["swift"]
    }

    fn color(&self) -> ratatui::style::Color {
        use crate::presentation::ui::Colors;
        Colors::lang_swift()
    }

    fn display_name(&self) -> &'static str {
        "Swift"
    }
}
