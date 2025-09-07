use crate::game::ascii_digits::get_digit_patterns;

/// Widget for displaying ASCII art numbers
pub struct AsciiNumbersWidget;

impl AsciiNumbersWidget {
    /// Create ASCII art representation of a numeric string
    pub fn create_ascii_numbers(score: &str) -> Vec<String> {
        let digit_patterns = get_digit_patterns();
        let max_height = 4;
        let mut result = vec![String::new(); max_height];

        for ch in score.chars() {
            if let Some(digit) = ch.to_digit(10) {
                let pattern = &digit_patterns[digit as usize];
                for (i, line) in pattern.iter().enumerate() {
                    result[i].push_str(line);
                    result[i].push(' '); // Add space between digits
                }
            }
        }

        result
    }

    /// Calculate display width without ANSI escape sequences
    pub fn calculate_display_width(text: &str) -> u16 {
        let mut width = 0;
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Skip ANSI escape sequence
                if chars.peek() == Some(&'[') {
                    chars.next(); // consume '['
                    for seq_ch in chars.by_ref() {
                        if seq_ch.is_ascii_alphabetic() {
                            break; // End of escape sequence
                        }
                    }
                }
            } else if !ch.is_control() {
                width += 1;
            }
        }

        width as u16
    }
}
