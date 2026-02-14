use crate::domain::models::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Elixir;

impl Language for Elixir {
    fn name(&self) -> &'static str {
        "elixir"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["ex", "exs"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["ex", "exs"]
    }
    fn display_name(&self) -> &'static str {
        "Elixir"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        node.kind() == "comment"
    }
}
