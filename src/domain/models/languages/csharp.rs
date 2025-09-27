use crate::extractor::models::language::Language;
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
        use crate::presentation::ui::Colors;
        Colors::lang_csharp()
    }

    fn display_name(&self) -> &'static str {
        "C#"
    }
}
