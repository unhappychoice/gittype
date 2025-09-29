use crate::domain::models::{ChunkType, CodeChunk};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DifficultyLevel {
    Easy,   // ~100 characters
    Normal, // ~200 characters
    Hard,   // ~500 characters
    Wild,   // Entire chunks, unpredictable length
    Zen,    // Entire file
}

impl DifficultyLevel {
    pub fn char_limits(&self) -> (usize, usize) {
        match self {
            DifficultyLevel::Easy => (20, 100),
            DifficultyLevel::Normal => (80, 200),
            DifficultyLevel::Hard => (180, 500),
            DifficultyLevel::Wild => (0, usize::MAX), // No limits - full chunks
            DifficultyLevel::Zen => (0, usize::MAX),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DifficultyLevel::Easy => "~100 characters",
            DifficultyLevel::Normal => "~200 characters",
            DifficultyLevel::Hard => "~500 characters",
            DifficultyLevel::Wild => "Full chunks",
            DifficultyLevel::Zen => "Entire files",
        }
    }

    pub fn subtitle(&self) -> &'static str {
        match self {
            DifficultyLevel::Easy => "Short code snippets",
            DifficultyLevel::Normal => "Medium functions",
            DifficultyLevel::Hard => "Long functions or classes",
            DifficultyLevel::Wild => "Unpredictable length chunks",
            DifficultyLevel::Zen => "Complete files as challenges",
        }
    }

    /// Returns all applicable difficulty levels for a chunk with given character count and type
    pub fn applicable_difficulties(chunk: &CodeChunk, code_char_count: usize) -> Vec<DifficultyLevel> {
        [
            DifficultyLevel::Easy,
            DifficultyLevel::Normal,
            DifficultyLevel::Hard,
            DifficultyLevel::Wild,
            DifficultyLevel::Zen,
        ]
            .iter()
            .filter(|&difficulty| {
                match difficulty {
                    DifficultyLevel::Zen => matches!(chunk.chunk_type, ChunkType::File),
                    DifficultyLevel::Wild => true,
                    _ => {
                        let (min_chars, _) = difficulty.char_limits();
                        code_char_count >= min_chars
                    }
                }
            })
            .copied()
            .collect()
    }
}
