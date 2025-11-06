use crate::domain::models::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Zig;

impl Language for Zig {
    fn name(&self) -> &'static str {
        "zig"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["zig"]
    }
    fn display_name(&self) -> &'static str {
        "Zig"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment"
    }
}
