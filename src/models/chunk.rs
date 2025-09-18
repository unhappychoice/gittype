use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    // New chunk types for middle implementations
    Loop,          // for/while/loop constructs
    Conditional,   // if/switch/match statements
    ErrorHandling, // try/catch/error handling blocks
    FunctionCall,  // function/method calls
    Lambda,        // closures, lambdas, arrow functions
    SpecialBlock,  // language-specific blocks (with, defer, etc.)
    Comprehension, // list/dict comprehensions
    CodeBlock,     // generic code blocks
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
    pub original_indentation: usize, // Indentation width in characters (Extractor-normalized)
}
