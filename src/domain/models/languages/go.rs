use crate::domain::models::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Go;

impl Language for Go {
    fn name(&self) -> &'static str {
        "go"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["go"]
    }
    fn display_name(&self) -> &'static str {
        "Go"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment"
    }
}
