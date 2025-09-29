use super::{
    chunk_splitter::ChunkSplitter,
    code_character_counter::CodeCharacterCounter,
    progress_tracker::ProgressTracker,
};
use crate::domain::models::{Challenge, CodeChunk, DifficultyLevel};
use crate::presentation::game::screens::loading_screen::ProgressReporter;
use rayon::prelude::*;

/// Main orchestrator for converting CodeChunks into Challenges
pub struct ChallengeGenerator {
    chunk_splitter: ChunkSplitter,
    character_counter: CodeCharacterCounter,
}

impl Default for ChallengeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ChallengeGenerator {
    pub fn new() -> Self {
        Self {
            chunk_splitter: ChunkSplitter::new(),
            character_counter: CodeCharacterCounter::new(),
        }
    }

    pub fn convert_with_progress(
        &self,
        chunks: Vec<CodeChunk>,
        progress: &dyn ProgressReporter,
    ) -> Vec<Challenge> {
        if chunks.is_empty() {
            return Vec::new();
        }

        // Filter and sort valid chunks first
        let mut valid_chunks: Vec<_> = chunks
            .into_iter()
            .filter(|chunk| {
                !chunk.content.trim().is_empty() &&
                chunk.start_line > 0 &&
                chunk.end_line > 0 &&
                chunk.start_line <= chunk.end_line
            })
            .collect();

        valid_chunks.sort_by(|a, b| b.content.len().cmp(&a.content.len()));

        let total_chunks = valid_chunks.len();
        let progress_tracker = ProgressTracker::new(total_chunks);

        progress_tracker.initialize(progress);

        let chunk_challenges: Vec<Challenge> = valid_chunks
            .par_iter()
            .inspect(|_| { progress_tracker.increment_and_report(progress); })
            .flat_map(|chunk| {
                let code_char_count = self.character_counter.count_code_characters(chunk);

                DifficultyLevel::applicable_difficulties(chunk, code_char_count)
                    .into_par_iter()
                    .flat_map(move |difficulty| {
                        self.process_chunk_for_difficulty(chunk, &difficulty, code_char_count)
                    })
            })
            .collect();

        progress_tracker.finalize(progress);

        chunk_challenges
    }

    fn process_chunk_for_difficulty(
        &self,
        chunk: &CodeChunk,
        difficulty: &DifficultyLevel,
        code_char_count: usize,
    ) -> Vec<Challenge> {
        let (_, max_chars) = difficulty.char_limits();
        
        match (difficulty, code_char_count > max_chars) {
            (DifficultyLevel::Zen | DifficultyLevel::Wild, _) | (_, false) => {
                Challenge::from_chunk(chunk, Some(*difficulty))
                    .map(|challenge| vec![challenge])
                    .unwrap_or_else(Vec::new)
            }
            (_, true) => {
                self.chunk_splitter.split(chunk, difficulty)
                    .map(|(truncated_content, adjusted_comment_ranges, end_line)| {
                        vec![Challenge::from_content_and_chunk(
                            truncated_content,
                            chunk,
                            chunk.start_line,
                            end_line,
                            &adjusted_comment_ranges,
                            Some(*difficulty),
                        )]
                    })
                    .unwrap_or_else(Vec::new)
            }
        }
    }
}
