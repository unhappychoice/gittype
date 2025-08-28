use crate::game::Challenge;
use super::{CodeChunk, ChunkType};
use uuid::Uuid;

pub struct ChallengeConverter;

impl ChallengeConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_chunk_to_challenge(&self, chunk: CodeChunk) -> Challenge {
        let id = Uuid::new_v4().to_string();
        let language = self.language_to_string(&chunk.language);
        let file_path = chunk.file_path.to_string_lossy().to_string();
        
        Challenge::new(id, chunk.content)
            .with_source_info(file_path, chunk.start_line, chunk.end_line)
            .with_language(language)
            .with_comment_ranges(chunk.comment_ranges)
    }

    pub fn convert_chunks_to_challenges(&self, chunks: Vec<CodeChunk>) -> Vec<Challenge> {
        chunks.into_iter()
            .map(|chunk| self.convert_chunk_to_challenge(chunk))
            .collect()
    }

    pub fn convert_with_filter<F>(&self, chunks: Vec<CodeChunk>, filter: F) -> Vec<Challenge>
    where
        F: Fn(&CodeChunk) -> bool,
    {
        chunks.into_iter()
            .filter(filter)
            .map(|chunk| self.convert_chunk_to_challenge(chunk))
            .collect()
    }

    pub fn convert_functions_only(&self, chunks: Vec<CodeChunk>) -> Vec<Challenge> {
        self.convert_with_filter(chunks, |chunk| {
            matches!(chunk.chunk_type, ChunkType::Function | ChunkType::Method)
        })
    }

    pub fn convert_classes_only(&self, chunks: Vec<CodeChunk>) -> Vec<Challenge> {
        self.convert_with_filter(chunks, |chunk| {
            matches!(chunk.chunk_type, ChunkType::Class | ChunkType::Struct)
        })
    }

    fn language_to_string(&self, language: &super::Language) -> String {
        match language {
            super::Language::Rust => "rust".to_string(),
            super::Language::TypeScript => "typescript".to_string(),
            super::Language::Python => "python".to_string(),
        }
    }
}

