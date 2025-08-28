pub struct CommentParser;

impl CommentParser {
    pub fn detect_comments(text: &str) -> Vec<(usize, usize)> {
        let mut comment_ranges = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            // Single line comment //
            if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
                let start = i;
                // Find end of line
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                comment_ranges.push((start, i));
            }
            // Multi-line comment /* */
            else if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '*' {
                let start = i;
                i += 2;
                // Find end of comment
                while i + 1 < chars.len() {
                    if chars[i] == '*' && chars[i + 1] == '/' {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
                comment_ranges.push((start, i));
            }
            // Hash comment #
            else if chars[i] == '#' {
                let start = i;
                // Find end of line
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                comment_ranges.push((start, i));
            }
            else {
                i += 1;
            }
        }
        
        comment_ranges
    }
    
    pub fn is_position_in_comment(position: usize, comment_ranges: &[(usize, usize)]) -> bool {
        comment_ranges.iter().any(|&(start, end)| position >= start && position < end)
    }
    
    pub fn is_whitespace_before_comment(text: &str, position: usize, comment_ranges: &[(usize, usize)]) -> bool {
        let chars: Vec<char> = text.chars().collect();
        
        // Check if current position is whitespace
        if position >= chars.len() || !chars[position].is_whitespace() || chars[position] == '\n' {
            return false;
        }
        
        // Find the line this position belongs to
        let mut line_start = position;
        while line_start > 0 && chars.get(line_start - 1).map_or(false, |&c| c != '\n') {
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
    
    pub fn is_newline_after_comment_only_line(text: &str, position: usize, comment_ranges: &[(usize, usize)]) -> bool {
        let chars: Vec<char> = text.chars().collect();
        
        // Check if current character is newline
        if position >= chars.len() || chars[position] != '\n' {
            return false;
        }
        
        // Find the start of current line
        let mut line_start = position;
        while line_start > 0 && chars[line_start - 1] != '\n' {
            line_start = line_start.saturating_sub(1);
        }
        
        // Check if everything from line_start to position is whitespace or comment
        for i in line_start..position {
            let ch = chars[i];
            if !ch.is_whitespace() {
                // Found non-whitespace, check if it's part of a comment
                if !comment_ranges.iter().any(|&(start, end)| i >= start && i < end) {
                    // Non-whitespace that's not a comment = this is not a comment-only line
                    return false;
                }
            }
        }
        
        // This line contains only whitespace and/or comments
        true
    }
}