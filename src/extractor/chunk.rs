use std::path::PathBuf;
use super::Language;

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
}