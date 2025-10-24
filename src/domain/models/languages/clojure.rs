use crate::domain::models::Language;
use crate::presentation::ui::Colors;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clojure;

impl Language for Clojure {
    fn name(&self) -> &'static str {
        "clojure"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["clj", "cljs", "cljc"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["clojure", "clj", "cljs"]
    }

    fn color(&self) -> ratatui::style::Color {
        Colors::lang_clojure()
    }

    fn display_name(&self) -> &'static str {
        "Clojure"
    }

    fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
        node.kind() == "comment"
    }
}
