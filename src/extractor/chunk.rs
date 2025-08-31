use super::Language;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ChunkType {
    Function,
    Class,
    Method,
    Struct,
}

#[derive(Debug, Clone)]
pub struct CodeChunk {
    pub content: String,
    pub file_path: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
    pub language: Language,
    pub chunk_type: ChunkType,
    pub name: String,
    pub comment_ranges: Vec<(usize, usize)>, // Character-based ranges for comments relative to content
    pub original_indentation: usize,         // Column position of the first character in source
}
