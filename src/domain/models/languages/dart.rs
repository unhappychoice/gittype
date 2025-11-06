use crate::domain::models::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dart;

impl Language for Dart {
    fn name(&self) -> &'static str {
        "dart"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["dart"]
    }
    fn display_name(&self) -> &'static str {
        "Dart"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment" || node_kind == "documentation_comment"
    }
}
