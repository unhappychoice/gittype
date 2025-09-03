use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ChunkType {
    Function,
    Class,
    Method,
    Struct,
    Enum,
    Trait,
    TypeAlias,
    Interface,
    Module,
    Const,
    Variable,
    Component, // JSX/TSX React components
    Namespace, // For C# namespaces
}

#[derive(Debug, Clone)]
pub struct CodeChunk {
    pub content: String,
    pub file_path: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
    pub language: String,
    pub chunk_type: ChunkType,
    pub name: String,
    pub comment_ranges: Vec<(usize, usize)>, // Character-based ranges for comments relative to content
    pub original_indentation: usize,         // Column position of the first character in source
}