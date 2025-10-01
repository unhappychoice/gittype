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

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "line_comment" || node_kind == "block_comment"
    }
}
