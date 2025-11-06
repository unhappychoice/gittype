pub struct TextProcessor;

impl TextProcessor {
    pub fn process_challenge_text(text: &str) -> String {
        text.lines()
            .map(|line| line.trim_end()) // Remove trailing whitespace
            .filter(|line| !line.trim().is_empty()) // Skip empty lines
            .collect::<Vec<_>>()
            .join("\n")
            .trim_end() // Remove trailing whitespace only (preserve leading if needed)
            .to_string()
    }

    pub fn process_challenge_text_with_comment_mapping(
        text: &str,
        comment_ranges: &[(usize, usize)],
    ) -> (String, Vec<(usize, usize)>) {
        Self::process_challenge_text_with_comment_mapping_preserve_empty(
            text,
            comment_ranges,
            false,
        )
    }

    pub fn process_challenge_text_with_comment_mapping_preserve_empty(
        text: &str,
        comment_ranges: &[(usize, usize)],
        preserve_empty_lines: bool,
    ) -> (String, Vec<(usize, usize)>) {
        // Create character position mapping from original to processed text
        let mut position_mapping = Vec::new();
        let mut original_pos = 0;
        let mut processed_pos = 0;

        let lines: Vec<&str> = text.lines().collect();
        let mut processed_lines = Vec::new();

        for line in &lines {
            let trimmed_line = line.trim_end();

            // Skip empty lines only if preserve_empty_lines is false
            if !preserve_empty_lines && trimmed_line.trim().is_empty() {
                // Record that all characters in this line are skipped
                for _ in 0..line.len() {
                    position_mapping.push(None); // None means this character was removed
                    original_pos += 1;
                }
                // Account for newline
                if original_pos < text.len() {
                    position_mapping.push(None);
                    original_pos += 1;
                }
            } else {
                // Keep this line, record character mappings
                for _ch in trimmed_line.chars() {
                    position_mapping.push(Some(processed_pos));
                    processed_pos += 1;
                    original_pos += 1;
                }

                // Skip any trailing whitespace that was trimmed
                while original_pos < text.len()
                    && text.chars().nth(original_pos).unwrap_or('\n') != '\n'
                {
                    position_mapping.push(None);
                    original_pos += 1;
                }

                // Add newline (except for last line)
                if original_pos < text.len() {
                    position_mapping.push(Some(processed_pos));
                    processed_pos += 1;
                    original_pos += 1;
                }

                processed_lines.push(trimmed_line);
            }
        }

        let processed_text = processed_lines.join("\n");

        // Map comment ranges from original positions to processed positions
        let mapped_comment_ranges = comment_ranges
            .iter()
            .filter_map(|&(start, end)| {
                // Find mapped positions for start and end
                let mapped_start = if start < position_mapping.len() {
                    position_mapping[start]
                } else {
                    None
                };

                let mapped_end = if end <= position_mapping.len() {
                    (0..end)
                        .rev()
                        .find_map(|i| position_mapping.get(i).and_then(|&pos| pos))
                        .map(|pos| pos + 1)
                } else {
                    None
                };

                match (mapped_start, mapped_end) {
                    (Some(s), Some(e)) if s < e => Some((s, e)),
                    _ => None, // Comment was in a removed section
                }
            })
            .collect();

        (processed_text, mapped_comment_ranges)
    }

    pub fn calculate_line_starts(text: &str) -> Vec<usize> {
        let mut line_starts = vec![0];
        for (i, ch) in text.chars().enumerate() {
            if ch == '\n' && i + 1 < text.len() {
                line_starts.push(i + 1);
            }
        }
        line_starts
    }

    pub fn should_skip_final_newline(text: &str, position: usize) -> bool {
        // Skip newlines that are at the very end of the text
        let chars: Vec<char> = text.chars().collect();
        if position >= chars.len() {
            return false;
        }

        // Check if this is a trailing newline (at the end of text)
        if chars[position] == '\n' && position == chars.len() - 1 {
            return true;
        }

        false
    }

    pub fn should_skip_character(
        text: &str,
        position: usize,
        _line_starts: &[usize],
        comment_ranges: &[(usize, usize)],
    ) -> bool {
        let chars: Vec<char> = text.chars().collect();
        if position >= chars.len() {
            return false;
        }

        // Skip newlines only in specific cases, but keep them typeable
        if chars[position] == '\n' {
            // Skip newlines only at the very end of text
            if Self::should_skip_final_newline(text, position) {
                return true;
            }
            // Keep newlines typeable by default
            return false;
        }

        // Leading whitespace is now preserved since we have context lines

        // Check if this position is within a comment
        if Self::is_position_in_comment(position, comment_ranges) {
            return true;
        }

        // Check if this position is leading whitespace before a comment on the same line
        Self::is_whitespace_before_comment(text, position, comment_ranges)
    }

    pub fn is_at_end_of_line_content(
        text: &str,
        current_position: usize,
        line_starts: &[usize],
        comment_ranges: &[(usize, usize)],
    ) -> bool {
        let chars: Vec<char> = text.chars().collect();
        if current_position >= chars.len() {
            return false;
        }

        let current_char = chars[current_position];

        // If we're at a newline, we're at end of line
        if current_char == '\n' {
            return true;
        }

        // Check if we've reached the end of the actual code content on this line
        // (i.e., next characters until newline are only whitespace or comments)
        for (i, &ch) in chars.iter().enumerate().skip(current_position) {
            if ch == '\n' {
                return true; // Everything until newline was skippable
            }
            if !Self::should_skip_character(text, i, line_starts, comment_ranges) {
                return false; // Found non-skippable character
            }
        }

        true // Reached end of text
    }

    fn is_position_in_comment(position: usize, comment_ranges: &[(usize, usize)]) -> bool {
        comment_ranges
            .iter()
            .any(|&(start, end)| position >= start && position < end)
    }

    fn is_whitespace_before_comment(
        text: &str,
        position: usize,
        comment_ranges: &[(usize, usize)],
    ) -> bool {
        let chars: Vec<char> = text.chars().collect();

        // Check if current position is whitespace
        if position >= chars.len() || !chars[position].is_whitespace() || chars[position] == '\n' {
            return false;
        }

        // Find the line this position belongs to
        let mut line_start = position;
        while line_start > 0 && chars.get(line_start - 1).is_some_and(|&c| c != '\n') {
            line_start = line_start.saturating_sub(1);
        }

        // Look forward from current position to see if we hit a comment before any non-whitespace
        let mut i = position;
        while i < chars.len() && chars[i] != '\n' {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Found non-whitespace, check if it's start of a comment
            return comment_ranges.iter().any(|&(start, _)| start == i);
        }

        false
    }

    pub fn is_rest_of_line_comment_only(
        text: &str,
        position: usize,
        comment_ranges: &[(usize, usize)],
    ) -> bool {
        let chars: Vec<char> = text.chars().collect();
        if position >= chars.len() {
            return false;
        }

        // Find the next newline or end of text
        let mut end_pos = position;
        while end_pos < chars.len() && chars[end_pos] != '\n' {
            end_pos += 1;
        }

        // Check if everything from current position to end of line is comment or whitespace
        for (i, &ch) in chars.iter().enumerate().take(end_pos).skip(position) {
            // Skip whitespace
            if ch.is_whitespace() {
                continue;
            }

            // Check if this character is part of a comment
            let is_in_comment = comment_ranges
                .iter()
                .any(|&(start, end)| i >= start && i < end);

            if !is_in_comment {
                return false; // Found non-comment, non-whitespace content
            }
        }

        // Everything from current position to end of line is comment or whitespace
        true
    }
}
