use super::super::language::Language;
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
}
