use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct C;

impl Language for C {
    fn name(&self) -> &'static str {
        "c"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["c", "h"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_c()
    }

    fn display_name(&self) -> &'static str {
        "C"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment"
    }
}
