use super::super::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeScript;

impl Language for TypeScript {
    fn name(&self) -> &'static str {
        "typescript"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["ts", "tsx"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["ts"]
    }
}
