use crate::domain::models::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Java;

impl Language for Java {
    fn name(&self) -> &'static str {
        "java"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["java"]
    }
    fn display_name(&self) -> &'static str {
        "Java"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "line_comment" || node_kind == "block_comment"
    }
}
