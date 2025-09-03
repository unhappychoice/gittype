use super::super::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Swift;

impl Language for Swift {
    fn name(&self) -> &'static str {
        "swift"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["swift"]
    }
}
