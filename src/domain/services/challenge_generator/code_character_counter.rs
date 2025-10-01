use crate::domain::models::CodeChunk;

/// Counts non-whitespace, non-comment characters in code content
pub struct CodeCharacterCounter;

impl CodeCharacterCounter {
    pub fn new() -> Self {
        Self
    }

    /// Counts non-whitespace, non-comment characters in the given chunk
    pub fn count_code_characters(&self, chunk: &CodeChunk) -> usize {
        self.count_chars_in_content(&chunk.content, &chunk.comment_ranges)
    }

    /// Helper method to count characters in arbitrary content with comment ranges
    pub fn count_chars_in_content(
        &self,
        content: &str,
        comment_ranges: &[(usize, usize)],
    ) -> usize {
        if content.is_empty() {
            return 0;
        }

        // Sort comment ranges once for binary search efficiency
        let mut sorted_ranges = comment_ranges.to_vec();
        sorted_ranges.sort_by_key(|&(start, _)| start);

        let mut count = 0;
        let mut range_idx = 0;

        for (i, ch) in content.char_indices() {
            if ch.is_whitespace() {
                continue;
            }

            // Advance to current relevant range
            while range_idx < sorted_ranges.len() && sorted_ranges[range_idx].1 <= i {
                range_idx += 1;
            }

            // Check if current position is in comment
            let in_comment = range_idx < sorted_ranges.len()
                && i >= sorted_ranges[range_idx].0
                && i < sorted_ranges[range_idx].1;

            if !in_comment {
                count += 1;
            }
        }

        count
    }
}

impl Default for CodeCharacterCounter {
    fn default() -> Self {
        Self::new()
    }
}
