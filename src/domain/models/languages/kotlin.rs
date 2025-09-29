use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Kotlin;

impl Language for Kotlin {
    fn name(&self) -> &'static str {
        "kotlin"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["kt", "kts"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["kt"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_kotlin()
    }

    fn display_name(&self) -> &'static str {
        "Kotlin"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "line_comment" || node_kind == "multiline_comment"
    }
}
