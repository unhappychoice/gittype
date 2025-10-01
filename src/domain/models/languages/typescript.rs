use crate::domain::models::Language;
use crate::presentation::ui::Colors;
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
        Colors::lang_typescript()
    }

    fn display_name(&self) -> &'static str {
        "TypeScript"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment"
    }
}
