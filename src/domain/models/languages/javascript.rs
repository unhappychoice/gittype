use crate::domain::models::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JavaScript;

impl Language for JavaScript {
    fn name(&self) -> &'static str {
        "javascript"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["js", "jsx", "mjs", "cjs"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["js"]
    }
    fn display_name(&self) -> &'static str {
        "JavaScript"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment"
    }
}
