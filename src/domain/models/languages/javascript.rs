use crate::extractor::models::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JavaScript;

impl Language for JavaScript {
    fn name(&self) -> &'static str {
        "javascript"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["js", "jsx", "mjs", "cjs"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["js"]
    }

    fn color(&self) -> ratatui::style::Color {
        use crate::presentation::ui::Colors;
        Colors::lang_javascript()
    }

    fn display_name(&self) -> &'static str {
        "JavaScript"
    }
}
