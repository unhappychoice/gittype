use super::ProgressReporter;
use crate::game::models::StepType;
use crate::game::DifficultyLevel;
use crate::models::{Challenge, CodeChunk};
use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use uuid::Uuid;

pub struct ChallengeConverter;

impl Default for ChallengeConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl ChallengeConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_chunk_to_challenge(&self, chunk: CodeChunk) -> Option<Challenge> {
        // Skip empty or whitespace-only chunks
        if chunk.content.trim().is_empty() {
            return None;
        }

        // Skip invalid line numbers
        if chunk.start_line == 0 || chunk.end_line == 0 || chunk.start_line > chunk.end_line {
            return None;
        }

        let id = Uuid::new_v4().to_string();
        // Reuse chunk's existing strings instead of converting
        let language = chunk.language;
        let file_path = chunk.file_path.to_string_lossy().into_owned();

        Some(
            Challenge::new(id, chunk.content)
                .with_source_info(file_path, chunk.start_line, chunk.end_line)
                .with_language(language)
                .with_comment_ranges(chunk.comment_ranges),
        )
    }

    pub fn convert_chunks_and_files_to_challenges_with_progress(
        &self,
        chunks: Vec<CodeChunk>,
        progress: &dyn ProgressReporter,
    ) -> Vec<Challenge> {
        let total_chunks = chunks.len();
        let total_work = total_chunks;

        let mut all_challenges = Vec::new();

        progress.set_file_counts(StepType::Generating, 0, total_work, None);

        // Global progress tracking
        let processed_total = Arc::new(AtomicUsize::new(0));

        // Convert chunks to challenges for all difficulty levels
        if !chunks.is_empty() {
            let difficulties = [
                DifficultyLevel::Easy,
                DifficultyLevel::Normal,
                DifficultyLevel::Hard,
                DifficultyLevel::Wild,
                DifficultyLevel::Zen,
            ];

            // Sort chunks by content length (largest first) for better loading progress perception
            let mut sorted_chunks = chunks;
            sorted_chunks.sort_by(|a, b| b.content.len().cmp(&a.content.len()));

            // Pre-compute code character counts for reuse across difficulty levels
            let code_char_cache: HashMap<usize, usize> = sorted_chunks
                .par_iter()
                .enumerate()
                .map(|(idx, chunk)| {
                    let code_chars =
                        self.count_code_characters(&chunk.content, &chunk.comment_ranges);
                    (idx, code_chars)
                })
                .collect();

            // Use par_iter to maintain sort order (unlike into_par_iter)
            let chunk_challenges: Vec<Challenge> = sorted_chunks
                .par_iter()
                .enumerate()
                .flat_map(|(idx, chunk)| {
                    let code_char_count = code_char_cache[&idx];

                    // Pre-filter applicable difficulties to reduce iterations
                    let applicable_difficulties: Vec<_> = difficulties
                        .iter()
                        .filter(|&difficulty| {
                            self.is_difficulty_applicable(chunk, difficulty, code_char_count)
                        })
                        .collect();

                    let mut local = Vec::with_capacity(applicable_difficulties.len());

                    for difficulty in applicable_difficulties {
                        let split = self.split_chunk_by_difficulty_cached(
                            chunk,
                            difficulty,
                            code_char_count,
                        );
                        local.extend(split);
                    }

                    // Update progress atomically - reduce frequency of updates
                    let current = processed_total.fetch_add(1, Ordering::Relaxed) + 1;
                    if current.is_multiple_of(10) || current == total_work {
                        progress.set_file_counts(StepType::Generating, current, total_work, None);
                    }

                    local
                })
                .collect();

            all_challenges.extend(chunk_challenges);
        }

        // Ensure final progress is 100%
        progress.set_file_counts(StepType::Generating, total_work, total_work, None);

        all_challenges
    }

    fn is_difficulty_applicable(
        &self,
        chunk: &CodeChunk,
        difficulty: &DifficultyLevel,
        code_char_count: usize,
    ) -> bool {
        use crate::game::DifficultyLevel;

        // Skip empty or whitespace-only chunks early
        if chunk.content.trim().is_empty() {
            return false;
        }

        // Skip invalid line numbers early
        if chunk.start_line == 0 || chunk.end_line == 0 || chunk.start_line > chunk.end_line {
            return false;
        }

        match difficulty {
            DifficultyLevel::Zen => {
                // Only File chunks for Zen mode
                matches!(chunk.chunk_type, crate::models::ChunkType::File)
            }
            DifficultyLevel::Wild => {
                // Wild difficulty accepts any valid chunk
                true
            }
            _ => {
                // For other difficulties, check if chunk meets minimum size requirements
                let (min_chars, _) = difficulty.char_limits();
                code_char_count >= min_chars
            }
        }
    }

    fn split_chunk_by_difficulty_cached(
        &self,
        chunk: &CodeChunk,
        difficulty: &DifficultyLevel,
        code_char_count: usize,
    ) -> Vec<Challenge> {
        use crate::game::DifficultyLevel;

        // Helper function to create challenge with string borrowing optimization
        let create_challenge = |content: Cow<'_, str>,
                                start_line: usize,
                                end_line: usize,
                                comment_ranges: &[(usize, usize)]|
         -> Challenge {
            let id = Uuid::new_v4().to_string();
            let language = &chunk.language;
            let file_path = chunk.file_path.to_string_lossy();

            Challenge::new(id, content.into_owned())
                .with_source_info(file_path.into_owned(), start_line, end_line)
                .with_language(language.clone())
                .with_comment_ranges(comment_ranges.to_vec())
                .with_difficulty_level(*difficulty)
        };

        // Handle Zen mode only for File chunks
        if matches!(difficulty, DifficultyLevel::Zen) {
            let challenge = create_challenge(
                Cow::Borrowed(&chunk.content),
                chunk.start_line,
                chunk.end_line,
                &chunk.comment_ranges,
            );
            return vec![challenge];
        }

        // Wild difficulty uses the full chunk as-is
        if matches!(difficulty, DifficultyLevel::Wild) {
            let challenge = create_challenge(
                Cow::Borrowed(&chunk.content),
                chunk.start_line,
                chunk.end_line,
                &chunk.comment_ranges,
            );
            return vec![challenge];
        }

        let (min_chars, max_chars) = difficulty.char_limits();

        // If the chunk is within the target range, return as-is
        if code_char_count <= max_chars {
            let challenge = create_challenge(
                Cow::Borrowed(&chunk.content),
                chunk.start_line,
                chunk.end_line,
                &chunk.comment_ranges,
            );
            return vec![challenge];
        }

        // Find the best natural break point that keeps us under max_chars
        let break_point =
            self.find_optimal_break_point(&chunk.content, &chunk.comment_ranges, max_chars);

        if break_point > 0 {
            let lines: Vec<&str> = chunk.content.lines().collect();
            if break_point < lines.len() {
                // Create single challenge from beginning to break point only
                let selected_lines: String = lines
                    .iter()
                    .take(break_point)
                    .map(|l| format!("{}\n", l))
                    .collect();

                if !selected_lines.trim().is_empty() {
                    let truncated_content = selected_lines.trim_end();

                    // Check if truncated content meets minimum requirements
                    let adjusted_comment_ranges = self.adjust_comment_ranges_for_truncation(
                        &chunk.comment_ranges,
                        truncated_content.chars().count(),
                    );
                    let truncated_code_chars =
                        self.count_code_characters(truncated_content, &adjusted_comment_ranges);

                    // Only create challenge if it meets minimum size for this difficulty
                    if truncated_code_chars >= min_chars {
                        let challenge = create_challenge(
                            Cow::Owned(truncated_content.to_string()),
                            chunk.start_line,
                            chunk.start_line + break_point - 1,
                            &adjusted_comment_ranges,
                        );
                        return vec![challenge];
                    }
                }
            }
        }

        // Don't generate challenge if it doesn't fit within the difficulty range
        vec![]
    }

    fn count_code_characters(&self, content: &str, comment_ranges: &[(usize, usize)]) -> usize {
        // Early return for empty content
        if content.is_empty() {
            return 0;
        }

        // Pre-sort comment ranges for binary search optimization
        let mut sorted_ranges = comment_ranges.to_vec();
        sorted_ranges.sort_by_key(|&(start, _)| start);

        let mut code_char_count = 0;
        let mut current_range_idx = 0;

        for (i, ch) in content.char_indices() {
            // Skip whitespace-only characters
            if ch.is_whitespace() {
                continue;
            }

            // Optimized comment range checking using sorted ranges
            let mut in_comment = false;
            while current_range_idx < sorted_ranges.len() {
                let (start, end) = sorted_ranges[current_range_idx];
                if i < start {
                    break; // No more relevant ranges
                } else if i >= start && i < end {
                    in_comment = true;
                    break;
                } else if i >= end {
                    current_range_idx += 1; // Move to next range
                } else {
                    break;
                }
            }

            if !in_comment {
                code_char_count += 1;
            }
        }

        code_char_count
    }

    fn find_optimal_break_point(
        &self,
        content: &str,
        comment_ranges: &[(usize, usize)],
        target_chars: usize,
    ) -> usize {
        let lines: Vec<&str> = content.lines().collect();
        let mut current_pos = 0;
        let mut code_char_count = 0;
        let mut last_good_break = 0;

        for (line_idx, line) in lines.iter().enumerate() {
            let line_end = current_pos + line.len();

            // Count non-comment, non-whitespace characters in this line
            for (char_idx, ch) in line.chars().enumerate() {
                let char_pos = current_pos + char_idx;

                if ch.is_whitespace() {
                    continue;
                }

                // Check if this character is in a comment
                let in_comment = comment_ranges
                    .iter()
                    .any(|&(start, end)| char_pos >= start && char_pos < end);

                if !in_comment {
                    code_char_count += 1;
                }
            }

            // Check if we've exceeded the target
            if code_char_count > target_chars {
                // Return the last good break point
                return last_good_break.max(1);
            }

            // Update last good break point if this is a natural boundary
            if self.is_natural_boundary(line) {
                last_good_break = line_idx + 1;
            }

            // Move to next line (add 1 for newline character)
            current_pos = line_end + 1;
        }

        // If we never exceeded the target, return the full length
        lines.len()
    }

    fn is_natural_boundary(&self, line: &str) -> bool {
        let trimmed = line.trim();

        // Empty lines are natural boundaries
        if trimmed.is_empty() {
            return true;
        }

        // Lines ending with closing braces/brackets (end of blocks/scopes)
        if trimmed.ends_with('}') || trimmed.ends_with(']') || trimmed.ends_with(')') {
            return true;
        }

        // Lines ending with semicolons (end of statements)
        if trimmed.ends_with(';') {
            return true;
        }

        false
    }

    fn adjust_comment_ranges_for_truncation(
        &self,
        original_ranges: &[(usize, usize)],
        new_length: usize,
    ) -> Vec<(usize, usize)> {
        original_ranges
            .iter()
            .filter_map(|&(start, end)| {
                // Only include ranges that fall within the truncated content
                if start < new_length {
                    let adjusted_end = end.min(new_length);
                    if adjusted_end > start {
                        Some((start, adjusted_end))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}
