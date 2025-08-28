use super::comment_parser::CommentParser;

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

    pub fn calculate_line_starts(text: &str) -> Vec<usize> {
        let mut line_starts = vec![0];
        for (i, ch) in text.chars().enumerate() {
            if ch == '\n' && i + 1 < text.len() {
                line_starts.push(i + 1);
            }
        }
        line_starts
    }

    pub fn find_first_non_whitespace(text: &str, line_start: usize) -> usize {
        text.chars()
            .enumerate()
            .skip(line_start)
            .find(|(_, ch)| !ch.is_whitespace() || *ch == '\n')
            .map(|(i, _)| i)
            .unwrap_or(line_start)
    }

    pub fn find_first_non_whitespace_or_comment(text: &str, line_start: usize, comment_ranges: &[(usize, usize)]) -> usize {
        let chars: Vec<char> = text.chars().collect();
        let mut i = line_start;
        
        while i < chars.len() {
            let ch = chars[i];
            
            // Skip whitespace except newlines
            if ch.is_whitespace() && ch != '\n' {
                i += 1;
                continue;
            }
            
            // If we hit a newline, move to next line and continue searching
            if ch == '\n' {
                i += 1;
                continue;
            }
            
            // Check if this position is within a comment
            let is_in_comment = comment_ranges.iter().any(|&(start, end)| i >= start && i < end);
            
            if is_in_comment {
                // Skip to end of comment and continue searching
                if let Some(&(_, end)) = comment_ranges.iter().find(|&&(start, end)| i >= start && i < end) {
                    i = end;
                    continue;
                }
            }
            
            // Found a typeable character
            return i;
        }
        
        // If we reach here, we've hit the end of text - return the actual end
        chars.len()
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

    pub fn should_skip_character(text: &str, position: usize, line_starts: &[usize], comment_ranges: &[(usize, usize)]) -> bool {
        let chars: Vec<char> = text.chars().collect();
        if position >= chars.len() {
            return false;
        }
        
        // Don't skip newlines - they need to be typeable
        if chars[position] == '\n' {
            // Only skip newlines that are at the end of comment-only lines or at end of text
            if CommentParser::is_newline_after_comment_only_line(text, position, comment_ranges) 
                || Self::should_skip_final_newline(text, position) {
                return true;
            }
            return false;
        }
        
        // Check if this position is before the first non-whitespace character of a line
        for &line_start in line_starts {
            if position >= line_start {
                let first_non_ws = Self::find_first_non_whitespace(text, line_start);
                if position < first_non_ws {
                    return true;
                }
            }
        }
        
        // Check if this position is within a comment
        if CommentParser::is_position_in_comment(position, comment_ranges) {
            return true;
        }
        
        // Check if this position is leading whitespace before a comment on the same line
        CommentParser::is_whitespace_before_comment(text, position, comment_ranges)
    }

    pub fn is_at_end_of_line_content(text: &str, current_position: usize, line_starts: &[usize], comment_ranges: &[(usize, usize)]) -> bool {
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
        for i in current_position..chars.len() {
            let ch = chars[i];
            if ch == '\n' {
                return true; // Everything until newline was skippable
            }
            if !Self::should_skip_character(text, i, line_starts, comment_ranges) {
                return false; // Found non-skippable character
            }
        }
        
        true // Reached end of text
    }
}