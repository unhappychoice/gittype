use super::super::language::Language;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Python;

impl Language for Python {
    fn name(&self) -> &'static str {
        "python"
    }
    fn extensions(&self) -> Vec<&'static str> {
        vec!["py"]
    }
    fn aliases(&self) -> Vec<&'static str> {
        vec!["py"]
    }
}
