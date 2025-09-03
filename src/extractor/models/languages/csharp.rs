use super::super::language::Language;
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
}
