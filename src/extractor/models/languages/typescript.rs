use super::super::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeScript;

impl Language for TypeScript {
    fn name(&self) -> &'static str {
        "typescript"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["ts", "tsx"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["ts"]
    }

    fn color(&self) -> ratatui::style::Color {
        use crate::presentation::ui::Colors;
        Colors::lang_typescript()
    }

    fn display_name(&self) -> &'static str {
        "TypeScript"
    }
}
