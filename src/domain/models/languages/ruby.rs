use crate::domain::models::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ruby;

impl Language for Ruby {
    fn name(&self) -> &'static str {
        "ruby"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["rb"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["rb"]
    }
    fn display_name(&self) -> &'static str {
        "Ruby"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment"
    }
}
