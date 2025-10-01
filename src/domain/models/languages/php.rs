use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Php;

impl Language for Php {
    fn name(&self) -> &'static str {
        "php"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["php", "phtml", "php3", "php4", "php5"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_php()
    }

    fn display_name(&self) -> &'static str {
        "PHP"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment" || node_kind == "shell_comment_line"
    }
}
