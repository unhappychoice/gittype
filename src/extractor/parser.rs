use std::path::Path;
use crate::{Result, GitTypeError};
use super::{CodeChunk, Language};

pub struct CodeExtractor;

impl CodeExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_chunks(&self, repo_path: &Path) -> Result<Vec<CodeChunk>> {
        // TODO: Implement code extraction using tree-sitter
        Err(GitTypeError::ExtractionFailed("Not yet implemented".to_string()))
    }
}