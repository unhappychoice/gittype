use super::ProgressReporter;
use crate::game::models::StepType;
use crate::game::DifficultyLevel;
use crate::models::{Challenge, CodeChunk};
use rayon::prelude::*;
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

    pub fn convert_chunk_to_challenge(&self, chunk: CodeChunk) -> Challenge {
        let id = Uuid::new_v4().to_string();
        let language = chunk.language.to_string();
        let file_path = chunk.file_path.to_string_lossy().to_string();

        Challenge::new(id, chunk.content)
            .with_source_info(file_path, chunk.start_line, chunk.end_line)
            .with_language(language)
            .with_comment_ranges(chunk.comment_ranges)
    }

    fn convert_chunk_to_challenge_ref(&self, chunk: &CodeChunk) -> Challenge {
        let id = Uuid::new_v4().to_string();
        let language = chunk.language.to_string();
        let file_path = chunk.file_path.to_string_lossy().to_string();

        Challenge::new(id, chunk.content.clone())
            .with_source_info(file_path, chunk.start_line, chunk.end_line)
            .with_language(language)
            .with_comment_ranges(chunk.comment_ranges.clone())
    }

    pub fn convert_chunks_and_files_to_challenges_with_progress(
        &self,
        chunks: Vec<CodeChunk>,
        file_paths: Vec<std::path::PathBuf>,
        git_root: Option<&std::path::Path>,
        progress: &dyn ProgressReporter,
    ) -> Vec<Challenge> {
        let total_chunks = chunks.len();
        let total_files = file_paths.len();
        let total_work = total_chunks + total_files;

        let mut all_challenges = Vec::new();

        progress.set_file_counts(StepType::Generating, 0, total_work, None);

        // Global progress tracking for both phases
        let processed_total = Arc::new(AtomicUsize::new(0));

        // Phase 1: Convert chunks to regular challenges
        if !chunks.is_empty() {
            let difficulties = [
                DifficultyLevel::Easy,
                DifficultyLevel::Normal,
                DifficultyLevel::Hard,
                DifficultyLevel::Wild,
            ];

            // Sort chunks by content length (largest first) for better loading progress perception
            let mut sorted_chunks = chunks;
            sorted_chunks.sort_by(|a, b| b.content.len().cmp(&a.content.len()));

            let processed_total_clone = processed_total.clone();

            let chunk_challenges: Vec<Challenge> = sorted_chunks
                .into_par_iter()
                .map(|chunk| {
                    let mut local = Vec::new();
                    for difficulty in &difficulties {
                        let split = self.split_chunk_by_difficulty(&chunk, difficulty);
                        local.extend(split);
                    }

                    // Update progress atomically - always increment for every chunk
                    let current = processed_total_clone.fetch_add(1, Ordering::Relaxed) + 1;
                    if current % 5 == 0 || current == total_work {
                        progress.set_file_counts(StepType::Generating, current, total_work, None);
                    }

                    local
                })
                .flatten()
                .collect();

            all_challenges.extend(chunk_challenges);
        }

        // Phase 2: Convert files to zen challenges
        if !file_paths.is_empty() {
            // Sort files by size (largest first) for better loading progress perception
            // Cache metadata to avoid repeated filesystem calls
            let mut files_with_sizes: Vec<_> = file_paths
                .into_iter()
                .map(|path| {
                    let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                    (path, size)
                })
                .collect();

            files_with_sizes.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by size (largest first)

            let sorted_file_paths: Vec<_> = files_with_sizes
                .into_iter()
                .map(|(path, _size)| path)
                .collect();

            let processed_total_clone = processed_total.clone();

            let file_challenges: Vec<Challenge> = sorted_file_paths
                .into_par_iter()
                .map(|file_path| {
                    // Always update progress, regardless of file read success
                    let current = processed_total_clone.fetch_add(1, Ordering::Relaxed) + 1;
                    if current % 5 == 0 || current == total_work {
                        progress.set_file_counts(StepType::Generating, current, total_work, None);
                    }

                    match std::fs::read_to_string(&file_path) {
                        Ok(content) => {
                            let id = Uuid::new_v4().to_string();
                            let language =
                                super::models::language::LanguageRegistry::detect_from_path(
                                    &file_path,
                                );

                            let relative_file_path = if let Some(root) = git_root {
                                if let Ok(relative) = file_path.strip_prefix(root) {
                                    relative.to_path_buf()
                                } else {
                                    file_path.clone()
                                }
                            } else {
                                file_path.clone()
                            };

                            let file_path_str = relative_file_path.to_string_lossy().to_string();
                            let line_count = content.lines().count();
                            Some(
                                Challenge::new(id, content)
                                    .with_source_info(file_path_str, 1, line_count)
                                    .with_language(language)
                                    .with_difficulty_level(DifficultyLevel::Zen),
                            )
                        }
                        Err(e) => {
                            eprintln!(
                                "Warning: Failed to read file {:?} for Zen challenge: {}",
                                file_path, e
                            );
                            None
                        }
                    }
                })
                .filter_map(|result| result)
                .collect();

            all_challenges.extend(file_challenges);
        }

        // Ensure final progress is 100%
        progress.set_file_counts(StepType::Generating, total_work, total_work, None);

        all_challenges
    }

    fn split_chunk_by_difficulty(
        &self,
        chunk: &CodeChunk,
        difficulty: &crate::game::DifficultyLevel,
    ) -> Vec<Challenge> {
        use crate::game::DifficultyLevel;

        if matches!(difficulty, DifficultyLevel::Zen) {
            return vec![self.convert_chunk_to_challenge_ref(chunk)];
        }

        // Wild difficulty uses the full chunk as-is
        if matches!(difficulty, DifficultyLevel::Wild) {
            let mut challenge = self.convert_chunk_to_challenge_ref(chunk);
            challenge.difficulty_level = Some(*difficulty);
            return vec![challenge];
        }

        let (min_chars, max_chars) = difficulty.char_limits();

        let content = &chunk.content;
        let lines: Vec<&str> = content.lines().collect();

        // Calculate actual code characters (excluding comments) using AST data
        let code_char_count = self.count_code_characters(content, &chunk.comment_ranges);

        // Skip if chunk doesn't meet minimum size for this difficulty
        if code_char_count < min_chars {
            return vec![]; // Don't generate challenge for this difficulty
        }

        // If the chunk is within the target range, return as-is
        if code_char_count <= max_chars {
            let mut challenge = self.convert_chunk_to_challenge_ref(chunk);
            challenge.difficulty_level = Some(*difficulty);

            return vec![challenge];
        }

        // Find the best natural break point that keeps us under max_chars
        let break_point = self.find_optimal_break_point(content, &chunk.comment_ranges, max_chars);

        if break_point > 0 && break_point < lines.len() {
            // Create single challenge from beginning to break point only
            // Don't create meaningless fragments from the remainder
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
                    let id = Uuid::new_v4().to_string();
                    let language = chunk.language.to_string();
                    // file_path is already relative to git project root from CodeChunk
                    let file_path = chunk.file_path.to_string_lossy().to_string();

                    let challenge = Challenge::new(id, truncated_content.to_string())
                        .with_source_info(
                            file_path,
                            chunk.start_line,
                            chunk.start_line + break_point - 1,
                        )
                        .with_language(language)
                        .with_comment_ranges(adjusted_comment_ranges)
                        .with_difficulty_level(*difficulty);

                    return vec![challenge];
                }
            }
        }

        // Don't use fallback for difficulty-based splitting - if we can't fit within the target range, don't generate a challenge
        vec![] // Don't generate challenge if it doesn't fit within the difficulty range
    }

    fn count_code_characters(&self, content: &str, comment_ranges: &[(usize, usize)]) -> usize {
        let chars: Vec<char> = content.chars().collect();
        let mut code_char_count = 0;

        for (i, ch) in chars.iter().enumerate() {
            // Skip whitespace-only characters
            if ch.is_whitespace() {
                continue;
            }

            // Check if this character is inside a comment range
            let in_comment = comment_ranges
                .iter()
                .any(|&(start, end)| i >= start && i < end);

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
