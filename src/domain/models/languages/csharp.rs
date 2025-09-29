use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CSharp;

impl Language for CSharp {
    fn name(&self) -> &'static str {
        "csharp"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["cs", "csx"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["cs", "c#"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_csharp()
    }

    fn display_name(&self) -> &'static str {
        "C#"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        let node_kind = node.kind();
        node_kind == "comment"
    }
}
