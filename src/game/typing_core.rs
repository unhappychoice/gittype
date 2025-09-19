use crate::models::Challenge;

#[derive(Debug, Clone)]
pub struct TypingCore {
    // Text for typing logic (comments and empty lines removed)
    text_to_type: String,
    current_position_to_type: usize,
    mapping_to_type: Vec<usize>,

    // Text for display with improved formatting and visual hints
    text_to_display: String,
    current_position_to_display: usize,
    mapping_to_display: Vec<usize>,

    // Original text and metadata
    original_text: String,
    comment_ranges: Vec<(usize, usize)>,

    // Mistake tracking
    mistakes: usize,
    current_mistake_position: Option<usize>, // display position for highlighting
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessingOptions {
    pub preserve_empty_lines: bool,
    pub add_newline_symbols: bool,
    pub highlight_special_chars: bool,
}

impl Default for ProcessingOptions {
    fn default() -> Self {
        Self {
            preserve_empty_lines: true,
            add_newline_symbols: true,
            highlight_special_chars: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputResult {
    Correct,   // Input was correct, continue
    Incorrect, // Input was incorrect (mistake)
    Completed, // Input was correct and typing is complete
    NoAction,  // No input accepted (already completed)
}

impl TypingCore {
    pub fn new(
        original_text: &str,
        comment_ranges: &[(usize, usize)],
        options: ProcessingOptions,
    ) -> Self {
        // Normalize incoming comment ranges to character-based positions.
        // Some tests or callers may still provide byte-based ranges using str::find.
        let total_chars = original_text.chars().count();
        let normalized_ranges: Vec<(usize, usize)> = comment_ranges
            .iter()
            .map(|&(s, e)| {
                // Heuristic: Prefer byte->char conversion when the byte-sliced substring
                // clearly points to a comment start but the char-indexed slice does not.
                let within_bytes = s <= original_text.len() && e <= original_text.len() && s < e;
                let within_chars = s <= total_chars && e <= total_chars && s < e;

                let bytes_sub = if within_bytes
                    && original_text.is_char_boundary(s)
                    && original_text.is_char_boundary(e)
                {
                    Some(&original_text[s..e])
                } else {
                    None
                };

                let char_sub = if within_chars {
                    let chars: Vec<char> = original_text.chars().collect();
                    Some(chars[s..e].iter().collect::<String>())
                } else {
                    None
                };

                let bytes_looks_like_comment = bytes_sub
                    .map(|t| t.starts_with("//") || t.starts_with("/*") || t.starts_with("#"))
                    .unwrap_or(false);
                let chars_looks_like_comment = char_sub
                    .as_ref()
                    .map(|t| t.starts_with("//") || t.starts_with("/*") || t.starts_with("#"))
                    .unwrap_or(false);

                if bytes_looks_like_comment && !chars_looks_like_comment {
                    // Convert byte offsets to char offsets
                    (
                        original_text[..s.min(original_text.len())].chars().count(),
                        original_text[..e.min(original_text.len())].chars().count(),
                    )
                } else if s > total_chars || e > total_chars {
                    // Out-of-range as char indices; must be bytes
                    (
                        original_text[..s.min(original_text.len())].chars().count(),
                        original_text[..e.min(original_text.len())].chars().count(),
                    )
                } else {
                    // Assume already char-based
                    (s, e)
                }
            })
            .collect();

        let (text_to_type, text_mapping_to_type) =
            Self::create_typing_text(original_text, &normalized_ranges, &options);

        let (text_to_display, text_mapping_to_display) =
            Self::create_display_text(original_text, &normalized_ranges, &options);

        let initial_position_to_type = text_to_type
            .chars()
            .enumerate()
            .find(|(_, ch)| !ch.is_whitespace() || *ch == '\n')
            .map(|(idx, _)| idx)
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

        let result = Self {
            text_to_type,
            current_position_to_type: initial_position_to_type,
            mapping_to_type: text_mapping_to_type,
            text_to_display,
            current_position_to_display: initial_position_to_display,
            mapping_to_display: text_mapping_to_display,
            original_text: original_text.to_string(),
            comment_ranges: normalized_ranges,
            mistakes: 0,
            current_mistake_position: None,
        };

        result
    }

    pub fn from_challenge(challenge: &Challenge, options: Option<ProcessingOptions>) -> Self {
        let options = options.unwrap_or_default();
        Self::new(&challenge.code_content, &challenge.comment_ranges, options)
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

    pub fn current_line_to_display(&self) -> usize {
        // Count newlines up to current position to determine line number
        self.text_to_display
            .chars()
            .take(self.current_position_to_display)
            .filter(|&ch| ch == '\n')
            .count()
    }

    // Mistake tracking
    pub fn mistakes(&self) -> usize {
        self.mistakes
    }

    pub fn current_mistake_position(&self) -> Option<usize> {
        self.current_mistake_position
    }

    pub fn display_comment_ranges(&self) -> Vec<(usize, usize)> {
        let mut display_ranges = Vec::new();
        let display_text = self.text_to_display();
        let original_text = &self.original_text;
        let display_chars: Vec<char> = display_text.chars().collect();

        for &(original_start, original_end) in &self.comment_ranges {
            // Extract the comment text from original
            if original_end <= original_text.chars().count() {
                let original_chars: Vec<char> = original_text.chars().collect();
                let original_comment_chars: Vec<char> =
                    original_chars[original_start..original_end].to_vec();

                // Transform original comment to match display text (tabs → arrows)
                let mut comment_chars = Vec::new();
                for ch in &original_comment_chars {
                    if *ch == '\t' {
                        comment_chars.extend("→   ".chars()); // Tab becomes arrow + 3 spaces
                    } else {
                        comment_chars.push(*ch);
                    }
                }

                let mut found_match = false;

                // For "//" comments, we need to account for possible ↵ symbols added before them
                let search_patterns = if comment_chars.len() >= 2
                    && comment_chars[0] == '/'
                    && comment_chars[1] == '/'
                {
                    vec![
                        // Try with ↵ prefix first (more specific)
                        {
                            let mut with_arrow = vec!['↵'];
                            with_arrow.extend_from_slice(&comment_chars);
                            with_arrow
                        },
                        comment_chars.clone(), // Transformed comment as fallback
                    ]
                } else {
                    vec![comment_chars.clone()]
                };

                // Search for each pattern until we find one that works
                for pattern_chars in search_patterns {
                    if found_match {
                        break;
                    }

                    let mut search_start_char = 0;
                    while let Some(relative_pos_char) = display_chars[search_start_char..]
                        .windows(pattern_chars.len())
                        .position(|window| window == pattern_chars.as_slice())
                    {
                        let display_start_char = search_start_char + relative_pos_char;
                        let display_end_char = display_start_char + pattern_chars.len();

                        // Additional validation for proper comment boundaries
                        let is_valid_comment_start = if display_start_char == 0 {
                            true // Start of text
                        } else {
                            let prev_char = display_chars[display_start_char - 1];
                            prev_char.is_whitespace() || prev_char == '\n' || prev_char == '↵'
                        };

                        if !is_valid_comment_start {
                            search_start_char = display_start_char + 1;
                            continue;
                        }

                        // Convert character positions to byte positions
                        let display_start_byte = display_text
                            .char_indices()
                            .nth(display_start_char)
                            .map(|(byte_pos, _)| byte_pos)
                            .unwrap_or(display_text.len());

                        let display_end_byte = if display_end_char < display_chars.len() {
                            display_text
                                .char_indices()
                                .nth(display_end_char)
                                .map(|(byte_pos, _)| byte_pos)
                                .unwrap_or(display_text.len())
                        } else {
                            display_text.len()
                        };

                        // Check if this range has already been used
                        let overlaps_existing = display_ranges.iter().any(|&(start, end)| {
                            !(display_end_byte <= start || display_start_byte >= end)
                        });

                        if !overlaps_existing {
                            display_ranges.push((display_start_byte, display_end_byte));
                            found_match = true;
                            break; // Found a valid match for this comment
                        }

                        search_start_char = display_start_char + 1;
                    }
                }
            }
        }

        display_ranges
    }

    // Debug helper for tests
    pub fn debug_mapping_to_display(&self) -> &Vec<usize> {
        &self.mapping_to_display
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
        let mut original_char_pos = 0;
        let mut _processed_pos = 0;

        for (line_idx, line) in lines.iter().enumerate() {
            let line_start_char_pos = original_char_pos;

            // Process this line, removing comments
            let mut line_result = String::new();
            let mut line_mapping = Vec::new();

            // Use enumerate() to get character position, not byte position
            for (char_idx_in_line, ch) in line.chars().enumerate() {
                let char_pos = line_start_char_pos + char_idx_in_line;

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
                    let remaining_chars: Vec<char> = line.chars().skip(char_idx_in_line).collect();
                    let is_trailing = remaining_chars.iter().all(|&c| {
                        c.is_whitespace() || {
                            // Calculate the absolute char position of this character
                            let check_pos = char_pos
                                + remaining_chars.iter().position(|&rc| rc == c).unwrap_or(0);
                            comment_ranges
                                .iter()
                                .any(|&(start, end)| check_pos >= start && check_pos < end)
                        }
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
            original_char_pos += line.chars().count();

            // Handle newline character if not the last line
            if line_idx < lines.len() - 1 {
                if !is_empty_line {
                    position_mapping.push(line_start_char_pos + line.chars().count());
                    _processed_pos += 1;
                }
                original_char_pos += 1; // Account for \n
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
        let mut original_char_pos = 0;

        for (line_idx, line) in lines.iter().enumerate() {
            let line_start_char_pos = original_char_pos;

            // Check if this line has any content that needs typing (not just comments)
            let line_has_typeable_content = {
                let mut has_content = false;
                for (char_idx_in_line, line_ch) in line.chars().enumerate() {
                    let absolute_char_pos = line_start_char_pos + char_idx_in_line;
                    let in_comment = comment_ranges
                        .iter()
                        .any(|&(start, end)| absolute_char_pos >= start && absolute_char_pos < end);

                    if !in_comment && !line_ch.is_whitespace() {
                        has_content = true;
                        break;
                    }
                }
                has_content
            };

            // Find the position of the last typeable character in this line
            let last_typeable_char_idx = if line_has_typeable_content {
                let mut last_pos = None;
                for (char_idx_in_line, line_ch) in line.chars().enumerate() {
                    let absolute_char_pos = line_start_char_pos + char_idx_in_line;
                    let in_comment = comment_ranges
                        .iter()
                        .any(|&(start, end)| absolute_char_pos >= start && absolute_char_pos < end);

                    if !in_comment && !line_ch.is_whitespace() {
                        last_pos = Some(char_idx_in_line);
                    }
                }
                last_pos
            } else {
                None
            };

            // Process each character in the line
            for (char_idx_in_line, ch) in line.chars().enumerate() {
                let char_original_pos = line_start_char_pos + char_idx_in_line;
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
                if options.add_newline_symbols && Some(char_idx_in_line) == last_typeable_char_idx {
                    position_mapping.push(line_start_char_pos + line.chars().count()); // Position for ↵
                    display_text.push('↵');
                }
            }

            // Handle newline
            if line_idx < lines.len() - 1 {
                position_mapping.push(line_start_char_pos + line.chars().count()); // Position of \n
                display_text.push('\n');

                original_char_pos += line.chars().count() + 1; // +1 for \n
            } else {
                original_char_pos += line.chars().count();
            }
        }

        (display_text, position_mapping)
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
        let chars: Vec<char> = self.text_to_type.chars().collect();

        if type_pos >= chars.len() {
            return true;
        }

        // Check if current position is newline or if all following chars on line are whitespace
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

    // Helper methods for typing logic
    pub fn is_completed(&self) -> bool {
        self.current_position_to_type >= self.text_to_type.chars().count()
    }

    pub fn can_accept_input(&self) -> bool {
        self.current_position_to_type < self.text_to_type.chars().count()
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

    fn record_mistake(&mut self) {
        self.mistakes += 1;
        self.current_mistake_position = Some(self.current_position_to_display);
    }

    fn clear_mistake_position(&mut self) {
        self.current_mistake_position = None;
    }

    // High-level input processing methods
    pub fn process_character_input(&mut self, input_char: char) -> InputResult {
        if !self.can_accept_input() {
            return InputResult::NoAction;
        }

        if self.check_character_match(input_char) {
            self.clear_mistake_position();
            self.advance_to_next_character();
            if self.is_completed() {
                InputResult::Completed
            } else {
                InputResult::Correct
            }
        } else {
            self.record_mistake();
            InputResult::Incorrect
        }
    }

    pub fn process_enter_input(&mut self) -> InputResult {
        if !self.can_accept_input() {
            return InputResult::NoAction;
        }

        if self.is_at_line_end_for_enter() {
            self.clear_mistake_position();
            self.handle_newline_advance();
            if self.is_completed() {
                InputResult::Completed
            } else {
                InputResult::Correct
            }
        } else {
            self.record_mistake();
            InputResult::Incorrect
        }
    }

    pub fn process_tab_input(&mut self) -> InputResult {
        if !self.can_accept_input() {
            return InputResult::NoAction;
        }

        if self.check_character_match('\t') {
            self.clear_mistake_position();
            self.advance_to_next_character();
            if self.is_completed() {
                InputResult::Completed
            } else {
                InputResult::Correct
            }
        } else {
            self.record_mistake();
            InputResult::Incorrect
        }
    }
}

impl Default for TypingCore {
    fn default() -> Self {
        Self::new(
            "// Placeholder - will be updated with challenge data",
            &[],
            ProcessingOptions::default(),
        )
    }
}
