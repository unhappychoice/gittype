use crate::domain::models::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Erlang;

impl Language for Erlang {
    fn name(&self) -> &'static str {
        "erlang"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["erl", "hrl"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["erl"]
    }
    fn display_name(&self) -> &'static str {
        "Erlang"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        node.kind() == "comment"
    }
}
