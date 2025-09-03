use super::super::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Java;

impl Language for Java {
    fn name(&self) -> &'static str {
        "java"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["java"]
    }
}
