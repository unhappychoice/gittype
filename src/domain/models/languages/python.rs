use crate::domain::models::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Python;

impl Language for Python {
    fn name(&self) -> &'static str {
        "python"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["py"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["py"]
    }
    fn display_name(&self) -> &'static str {
        "Python"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment"
    }
}
