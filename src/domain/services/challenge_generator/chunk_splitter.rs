use std::borrow::Cow;
use crate::domain::models::{CodeChunk, DifficultyLevel};
use super::code_character_counter::CodeCharacterCounter;

/// Handles splitting and validation of code chunks based on difficulty requirements
pub struct ChunkSplitter {
    character_counter: CodeCharacterCounter,
}

impl ChunkSplitter {
    pub fn new() -> Self {
        Self {
            character_counter: CodeCharacterCounter::new(),
        }
    }

    /// Splits content and validates it meets character requirements
    /// Returns (content, adjusted_comment_ranges, end_line) if successful
    pub fn split<'a>(
        &self,
        chunk: &'a CodeChunk,
        difficulty: &DifficultyLevel,
    ) -> Option<(Cow<'a, str>, Vec<(usize, usize)>, usize)> {
        let (min_chars, max_chars) = difficulty.char_limits();

        // Check if content already fits within limits (common case)
        let code_char_count = self.character_counter.count_code_characters(chunk);

        if code_char_count <= max_chars && code_char_count >= min_chars {
            // No splitting needed - return original content
            let end_line = chunk.start_line + chunk.content.lines().count().saturating_sub(1);
            return Some((Cow::Borrowed(&chunk.content), chunk.comment_ranges.to_vec(), end_line));
        }

        if code_char_count < min_chars {
            return None;
        }

        // Content exceeds max_chars, need to split
        let break_point = self.find_optimal_break_point(&chunk.content, &chunk.comment_ranges, max_chars);

        if break_point == 0 {
            return None;
        }

        // Create truncated content
        let truncated_content = self.truncate_content_to_line(&chunk.content, break_point)?;

        // Adjust comment ranges for the truncated content
        let adjusted_comment_ranges = self.adjust_comment_ranges_for_truncation(
            &chunk.comment_ranges,
            truncated_content.chars().count(),
        );

        // Verify the truncated content meets minimum requirements
        let truncated_code_chars = self.character_counter.count_chars_in_content(&truncated_content, &adjusted_comment_ranges);

        if truncated_code_chars < min_chars {
            return None;
        }

        let end_line = chunk.start_line + break_point - 1;
        Some((truncated_content, adjusted_comment_ranges, end_line))
    }

    /// Finds the optimal break point for splitting content to stay within target character count
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

    /// Checks if a line represents a natural break point for code splitting
    fn is_natural_boundary(&self, line: &str) -> bool {
        let trimmed = line.trim();
        trimmed.is_empty()
            || trimmed.ends_with('}')
            || trimmed.ends_with(']')
            || trimmed.ends_with(')')
            || trimmed.ends_with(';')
    }

    /// Adjusts comment ranges when content is truncated
    fn adjust_comment_ranges_for_truncation(
        &self,
        original_ranges: &[(usize, usize)],
        new_length: usize,
    ) -> Vec<(usize, usize)> {
        original_ranges
            .iter()
            .filter(|&(start, _)| start < &new_length)
            .filter_map(|&(start, end)| {
                // Only include ranges that fall within the truncated content
                let adjusted_end = end.min(new_length);
                if adjusted_end > start {
                    Some((start, adjusted_end))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Creates a truncated version of content up to the specified line break point
    fn truncate_content_to_line<'a>(&self, content: &'a str, break_point: usize) -> Option<Cow<'a, str>> {
        let lines: Vec<&str> = content.lines().collect();

        if break_point > lines.len() {
            return None;
        }

        let selected_lines: String = lines
            .iter()
            .take(break_point)
            .map(|l| format!("{}\n", l))
            .collect();

        if selected_lines.trim().is_empty() {
            None
        } else {
            Some(Cow::Owned(selected_lines.trim_end().to_string()))
        }
    }
}

impl Default for ChunkSplitter {
    fn default() -> Self {
        Self::new()
    }
}
