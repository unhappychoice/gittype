use super::super::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ruby;

impl Language for Ruby {
    fn name(&self) -> &'static str {
        "ruby"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["rb"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["rb"]
    }
}
