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
    fn display_name(&self) -> &'static str {
        "Swift"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment" || node_kind == "multiline_comment"
    }
}
