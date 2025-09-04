use crate::models::Challenge;

#[derive(Debug, Clone)]
pub struct TypingCore {
    // Original source code with comments and formatting
    text_original: String,

    // Text for typing logic (comments and empty lines removed)
    text_to_type: String,
    current_position_to_type: usize,
    mapping_to_type: Vec<usize>,

    // Text for display with improved formatting and visual hints
    text_to_display: String,
    current_position_to_display: usize,
    mapping_to_display: Vec<usize>,

    // Metadata
    comment_ranges: Vec<(usize, usize)>,
    line_starts: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessingOptions {
    pub preserve_empty_lines: bool,
    pub normalize_indentation: bool,
    pub add_newline_symbols: bool,
    pub highlight_special_chars: bool,
}

impl Default for ProcessingOptions {
    fn default() -> Self {
        Self {
            preserve_empty_lines: true,
            normalize_indentation: false,
            add_newline_symbols: true,
            highlight_special_chars: true,
        }
    }
}

impl TypingCore {
    pub fn new(
        original_text: &str,
        comment_ranges: &[(usize, usize)],
        options: ProcessingOptions,
    ) -> Self {
        let (text_to_type, text_mapping_to_type) =
            Self::create_typing_text(original_text, comment_ranges, &options);

        let (text_to_display, text_mapping_to_display) =
            Self::create_display_text(original_text, comment_ranges, &options);

        let line_starts = Self::calculate_line_starts(&text_to_display);

        let initial_position_to_type = text_to_type
            .char_indices()
            .find(|(_, ch)| !ch.is_whitespace() || *ch == '\n')
            .map(|(pos, _)| pos)
            .unwrap_or(0);

        // Find corresponding display position for initial typing position
        let initial_position_to_display = if initial_position_to_type < text_mapping_to_type.len() {
            let original_pos = text_mapping_to_type[initial_position_to_type];
            text_mapping_to_display
                .iter()
                .position(|&pos| pos >= original_pos)
                .unwrap_or(0)
        } else {
            0
        };

        Self {
            text_original: original_text.to_string(),
            text_to_type,
            current_position_to_type: initial_position_to_type,
            mapping_to_type: text_mapping_to_type,
            text_to_display,
            current_position_to_display: initial_position_to_display,
            mapping_to_display: text_mapping_to_display,
            comment_ranges: comment_ranges.to_vec(),
            line_starts,
        }
    }

    pub fn from_challenge(challenge: &Challenge, options: Option<ProcessingOptions>) -> Self {
        let options = options.unwrap_or_default();
        Self::new(&challenge.code_content, &challenge.comment_ranges, options)
    }

    // Getters for the three text representations
    pub fn text_original(&self) -> &str {
        &self.text_original
    }

    // text_to_type
    pub fn text_to_type(&self) -> &str {
        &self.text_to_type
    }

    pub fn current_position_to_type(&self) -> usize {
        self.current_position_to_type
    }

    pub fn current_char_to_type(&self) -> Option<char> {
        self.text_to_type.chars().nth(self.current_position_to_type)
    }

    // text_to_display
    pub fn text_to_display(&self) -> &str {
        &self.text_to_display
    }

    pub fn current_position_to_display(&self) -> usize {
        self.current_position_to_display
    }

    pub fn current_char_to_display(&self) -> Option<char> {
        self.text_to_display
            .chars()
            .nth(self.current_position_to_display)
    }

    pub fn current_line_to_display(&self) -> usize {
        // Count newlines up to current position to determine line number
        self.text_to_display
            .chars()
            .take(self.current_position_to_display)
            .filter(|&ch| ch == '\n')
            .count()
    }

    // Others
    pub fn line_starts(&self) -> &[usize] {
        &self.line_starts
    }

    pub fn display_comment_ranges(&self) -> Vec<(usize, usize)> {
        let mut display_ranges = Vec::new();

        for &(start, end) in &self.comment_ranges {
            // Find display positions that map to original positions in the comment range
            let mut display_start = None;
            let mut display_end = None;

            for (display_pos, &original_pos) in self.mapping_to_display.iter().enumerate() {
                if original_pos >= start && original_pos < end {
                    if display_start.is_none() {
                        display_start = Some(display_pos);
                    }
                    display_end = Some(display_pos + 1); // +1 for exclusive end
                }
            }

            if let (Some(start_pos), Some(end_pos)) = (display_start, display_end) {
                if start_pos < end_pos && end_pos <= self.text_to_display.len() {
                    display_ranges.push((start_pos, end_pos));
                }
            }
        }

        display_ranges
    }

    // Text processing methods
    fn create_typing_text(
        original: &str,
        comment_ranges: &[(usize, usize)],
        _options: &ProcessingOptions,
    ) -> (String, Vec<usize>) {
        let lines: Vec<&str> = original.lines().collect();
        let mut processed_lines = Vec::new();
        let mut position_mapping = Vec::new();
        let mut original_pos = 0;
        let mut _processed_pos = 0;

        for (line_idx, line) in lines.iter().enumerate() {
            let line_start_pos = original_pos;

            // Process this line, removing comments
            let mut line_result = String::new();
            let mut line_mapping = Vec::new();

            for (char_idx, ch) in line.char_indices() {
                let char_pos = line_start_pos + char_idx;

                // Check if this character is in a comment
                let in_comment = comment_ranges
                    .iter()
                    .any(|&(start, end)| char_pos >= start && char_pos < end);

                if in_comment {
                    continue;
                } else if ch.is_whitespace() && ch != '\n' {
                    // Check if this is leading whitespace
                    let is_leading = line_result.chars().all(|c| c.is_whitespace());

                    // Check if this is trailing whitespace or followed by comment
                    let remaining_line = &line[char_idx..];
                    let is_trailing = remaining_line.chars().all(|c| {
                        c.is_whitespace()
                            || comment_ranges.iter().any(|&(start, end)| {
                                let pos =
                                    line_start_pos + char_idx + remaining_line.find(c).unwrap_or(0);
                                pos >= start && pos < end
                            })
                    });

                    // Skip leading and trailing whitespace, preserve internal spaces
                    if !is_leading && !is_trailing {
                        line_result.push(ch);
                        line_mapping.push(char_pos);
                        _processed_pos += 1;
                    }
                } else {
                    line_result.push(ch);
                    line_mapping.push(char_pos);
                    _processed_pos += 1;
                }
            }

            // Skip empty lines (lines with only comments or whitespace)
            let is_empty_line = line_result.is_empty();
            if !is_empty_line {
                processed_lines.push(line_result);
            }

            // Add line mappings to main mapping
            position_mapping.extend(line_mapping);
            original_pos += line.len();

            // Handle newline character if not the last line
            if line_idx < lines.len() - 1 {
                if !is_empty_line {
                    position_mapping.push(line_start_pos + line.len());
                    _processed_pos += 1;
                }
                original_pos += 1; // Account for \n
            }
        }

        let processed_text = processed_lines.join("\n");
        (processed_text, position_mapping)
    }

    fn create_display_text(
        original_text: &str,
        comment_ranges: &[(usize, usize)],
        options: &ProcessingOptions,
    ) -> (String, Vec<usize>) {
        let mut display_text = String::new();
        let mut position_mapping = Vec::new();

        let lines: Vec<&str> = original_text.lines().collect();
        let mut original_pos = 0;

        for (line_idx, line) in lines.iter().enumerate() {
            let line_start = original_pos;

            // Check if this line has any content that needs typing (not just comments)
            let line_has_typeable_content = {
                let mut has_content = false;
                for (char_idx, line_ch) in line.char_indices() {
                    let absolute_pos = line_start + char_idx;
                    let in_comment = comment_ranges
                        .iter()
                        .any(|&(start, end)| absolute_pos >= start && absolute_pos < end);

                    if !in_comment && !line_ch.is_whitespace() {
                        has_content = true;
                        break;
                    }
                }
                has_content
            };

            // Find the position of the last typeable character in this line
            let last_typeable_pos = if line_has_typeable_content {
                let mut last_pos = None;
                for (char_idx, line_ch) in line.char_indices() {
                    let absolute_pos = line_start + char_idx;
                    let in_comment = comment_ranges
                        .iter()
                        .any(|&(start, end)| absolute_pos >= start && absolute_pos < end);

                    if !in_comment && !line_ch.is_whitespace() {
                        last_pos = Some(char_idx);
                    }
                }
                last_pos
            } else {
                None
            };

            // Process each character in the line
            for (char_idx, ch) in line.char_indices() {
                let char_original_pos = line_start + char_idx;
                position_mapping.push(char_original_pos);

                if options.highlight_special_chars && ch == '\t' {
                    display_text.push_str("→   ");
                    // Add mapping for the extra characters in tab representation
                    for _ in 0..3 {
                        position_mapping.push(char_original_pos);
                    }
                } else {
                    display_text.push(ch);
                }

                // Insert ↵ right after the last typeable character
                if options.add_newline_symbols && Some(char_idx) == last_typeable_pos {
                    position_mapping.push(line_start + line.len()); // Position for ↵
                    display_text.push('↵');
                }
            }

            // Handle newline
            if line_idx < lines.len() - 1 {
                position_mapping.push(line_start + line.len()); // Position of \n
                display_text.push('\n');

                original_pos += line.len() + 1; // +1 for \n
            } else {
                original_pos += line.len();
            }
        }

        (display_text, position_mapping)
    }

    fn calculate_line_starts(text: &str) -> Vec<usize> {
        let mut line_starts = vec![0];
        for (i, ch) in text.chars().enumerate() {
            if ch == '\n' && i + 1 < text.len() {
                line_starts.push(i + 1);
            }
        }
        line_starts
    }

    fn update_display_position(&mut self) {
        // Use the mapping arrays to find the correct display position
        if self.current_position_to_type < self.mapping_to_type.len() {
            let target_original_pos = self.mapping_to_type[self.current_position_to_type];

            // Find the first display position that maps to this original position or later
            for (display_char_pos, &mapped_original_pos) in
                self.mapping_to_display.iter().enumerate()
            {
                if mapped_original_pos >= target_original_pos {
                    self.current_position_to_display = display_char_pos;
                    return;
                }
            }

            // If not found, set to the end
            self.current_position_to_display = self.text_to_display.chars().count();
        }
    }

    // Helper methods for typing logic
    pub fn is_position_at_line_end(&self, type_pos: usize) -> bool {
        if type_pos >= self.text_to_type.len() {
            return true;
        }

        // Check if current position is newline or if all following chars on line are whitespace
        let chars: Vec<char> = self.text_to_type.chars().collect();
        if chars.get(type_pos) == Some(&'\n') {
            return true;
        }

        // Check if rest of line contains only whitespace
        for &ch in chars.iter().skip(type_pos) {
            if ch == '\n' {
                return true;
            }
            if !ch.is_whitespace() {
                return false;
            }
        }

        true
    }

    pub fn advance_to_next_character(&mut self) {
        self.current_position_to_type += 1;
        self.update_display_position();
    }

    pub fn reset_position(&mut self) {
        self.current_position_to_type = self
            .text_to_type
            .char_indices()
            .find(|(_, ch)| !ch.is_whitespace() || *ch == '\n')
            .map(|(pos, _)| pos)
            .unwrap_or(0);
        self.update_display_position();
    }

    // Helper methods for typing logic
    pub fn is_completed(&self) -> bool {
        self.current_position_to_type >= self.text_to_type.len()
    }

    pub fn can_accept_input(&self) -> bool {
        self.current_position_to_type < self.text_to_type.len()
    }

    pub fn check_character_match(&self, input_char: char) -> bool {
        if let Some(expected_char) = self.current_char_to_type() {
            input_char == expected_char
        } else {
            false
        }
    }

    pub fn is_at_line_end_for_enter(&self) -> bool {
        self.is_position_at_line_end(self.current_position_to_type)
    }

    pub fn handle_newline_advance(&mut self) {
        // Skip current position if it's a newline
        if let Some(ch) = self.current_char_to_type() {
            if ch == '\n' {
                self.advance_to_next_character();
            }
        }
    }

    // High-level input processing methods
    pub fn process_character_input(&mut self, input_char: char) -> InputResult {
        if !self.can_accept_input() {
            return InputResult::NoAction;
        }

        if self.check_character_match(input_char) {
            self.advance_to_next_character();
            if self.is_completed() {
                InputResult::Completed
            } else {
                InputResult::Correct
            }
        } else {
            InputResult::Incorrect
        }
    }

    pub fn process_enter_input(&mut self) -> InputResult {
        if !self.can_accept_input() {
            return InputResult::NoAction;
        }

        if self.is_at_line_end_for_enter() {
            self.handle_newline_advance();
            if self.is_completed() {
                InputResult::Completed
            } else {
                InputResult::Correct
            }
        } else {
            InputResult::Incorrect
        }
    }

    pub fn process_tab_input(&mut self) -> InputResult {
        if !self.can_accept_input() {
            return InputResult::NoAction;
        }

        if self.check_character_match('\t') {
            self.advance_to_next_character();
            if self.is_completed() {
                InputResult::Completed
            } else {
                InputResult::Correct
            }
        } else {
            InputResult::Incorrect
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputResult {
    Correct,      // Input was correct, continue
    Incorrect,    // Input was incorrect (mistake)
    Completed,    // Input was correct and typing is complete
    NoAction,     // No input accepted (already completed)
}
