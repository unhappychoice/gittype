use crate::Result;
use crossterm::{
    cursor::{MoveTo, Hide, Show},
    queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::io::{stdout, Write};
use crate::scoring::TypingMetrics;
use super::{text_processor::TextProcessor, challenge::Challenge};

pub struct GameDisplay;

impl GameDisplay {
    pub fn display_challenge(
        challenge_text: &str,
        current_position: usize,
        mistakes: usize,
        start_time: &std::time::Instant,
        line_starts: &[usize],
        comment_ranges: &[(usize, usize)],
    ) -> Result<()> {
        Self::display_challenge_with_info(
            challenge_text,
            current_position,
            mistakes,
            start_time,
            line_starts,
            comment_ranges,
            None,
            None,
        )
    }

    pub fn display_challenge_with_info(
        challenge_text: &str,
        current_position: usize,
        mistakes: usize,
        start_time: &std::time::Instant,
        line_starts: &[usize],
        comment_ranges: &[(usize, usize)],
        challenge: Option<&Challenge>,
        current_mistake_position: Option<usize>,
    ) -> Result<()> {
        let mut stdout = stdout();
        let (terminal_width, terminal_height) = terminal::size()?;
        
        // Hide cursor to prevent flickering
        queue!(stdout, Hide)?;
        
        // Only clear screen on first render or when needed
        // Instead of clearing all, move to top
        queue!(stdout, MoveTo(0, 0))?;
        
        // Display header with challenge info
        let progress = (current_position as f32 / challenge_text.len() as f32 * 100.0) as u32;
        if let Some(challenge) = challenge {
            queue!(stdout, Print(format!("[{}] - Progress: {}%", challenge.get_display_title(), progress)))?;
        } else {
            queue!(stdout, Print(format!("[Challenge] - Progress: {}%", progress)))?;
        }
        queue!(stdout, MoveTo(0, 1))?;
        queue!(stdout, Print("Press ESC to quit"))?;
        queue!(stdout, MoveTo(0, 2))?;
        queue!(stdout, Print("─".repeat(terminal_width as usize)))?; // Separator line
        
        // Calculate display window for scrolling
        let display_start_row = 3u16;
        let available_rows = terminal_height.saturating_sub(5); // Reserve space for header and metrics
        
        // Find the line containing current position
        let current_line = Self::find_line_for_position(current_position, line_starts);
        
        // Calculate scroll offset to keep current line visible
        let scroll_offset = if current_line > (available_rows as usize / 2) {
            current_line.saturating_sub(available_rows as usize / 2)
        } else {
            0
        };
        
        // Display challenge text with proper wrapping and scrolling
        let mut current_col = 0u16;
        let mut current_row = display_start_row;
        let mut display_line = 0usize;
        queue!(stdout, MoveTo(0, current_row))?;
        
        for (i, ch) in challenge_text.chars().enumerate() {
            // Handle newlines in the source code - show them as visual markers
            if ch == '\n' {
                // Skip lines that are before our scroll window
                if display_line < scroll_offset {
                    display_line += 1;
                    continue;
                }
                
                // Stop if we've filled the available rows
                if current_row >= display_start_row + available_rows {
                    break;
                }
                let should_skip = TextProcessor::should_skip_character(challenge_text, i, line_starts, comment_ranges);
                
                if !should_skip {
                    // Display the newline marker
                    if i < current_position {
                        queue!(stdout, ResetColor, SetAttribute(Attribute::Bold), SetForegroundColor(Color::White))?;
                        Self::print_visible_char(ch, &mut stdout)?;
                    } else if i == current_position {
                        queue!(stdout, SetForegroundColor(Color::Black), SetBackgroundColor(Color::Yellow))?;
                        Self::print_visible_char(ch, &mut stdout)?;
                        queue!(stdout, ResetColor)?;
                    } else {
                        queue!(stdout, ResetColor, SetAttribute(Attribute::Dim), SetForegroundColor(Color::White))?;
                        Self::print_visible_char(ch, &mut stdout)?;
                    }
                }
                
                display_line += 1;
                current_row += 1;
                current_col = 0;
                queue!(stdout, MoveTo(current_col, current_row))?;
                continue;
            }
            
            // Skip characters that are in lines before our scroll window
            let char_line = Self::find_line_for_position(i, line_starts);
            if char_line < scroll_offset {
                continue;
            }
            
            // Stop if we've moved beyond the visible area
            if current_row >= display_start_row + available_rows {
                break;
            }
            
            // Check if we need to wrap to next line
            if current_col >= terminal_width - 1 {
                current_row += 1;
                current_col = 0;
                queue!(stdout, MoveTo(current_col, current_row))?;
            }

            // Skip displaying leading whitespace that user doesn't need to type
            let should_skip = TextProcessor::should_skip_character(challenge_text, i, line_starts, comment_ranges);
            let is_comment = Self::is_position_in_comment(i, comment_ranges);
            
            // Color the character based on typing state
            if i < current_position || should_skip {
                // Completed characters or skipped content
                if is_comment {
                    // Comments - show in green and italic-like style
                    queue!(stdout, ResetColor, SetAttribute(Attribute::Dim), SetForegroundColor(Color::Green))?;
                } else if should_skip && !is_comment {
                    // Skipped leading whitespace - show in darker color
                    queue!(stdout, ResetColor, SetAttribute(Attribute::Dim), SetForegroundColor(Color::DarkGrey))?;
                } else {
                    // Completed characters - bold white text
                    queue!(stdout, ResetColor, SetAttribute(Attribute::Bold), SetForegroundColor(Color::White))?;
                }
                Self::print_visible_char(ch, &mut stdout)?;
            } else if i == current_position {
                // Check if current position is a mistake
                if let Some(mistake_pos) = current_mistake_position {
                    if i == mistake_pos {
                        // Current character with mistake - white text on red background
                        queue!(stdout, SetForegroundColor(Color::White), SetBackgroundColor(Color::Red))?;
                    } else {
                        // Current character - black text on yellow background (cursor effect)
                        queue!(stdout, SetForegroundColor(Color::Black), SetBackgroundColor(Color::Yellow))?;
                    }
                } else {
                    // Current character - black text on yellow background (cursor effect)
                    queue!(stdout, SetForegroundColor(Color::Black), SetBackgroundColor(Color::Yellow))?;
                }
                Self::print_visible_char(ch, &mut stdout)?;
                queue!(stdout, ResetColor)?;
            } else {
                // Untyped characters
                if is_comment {
                    // Future comments - show in dim green
                    queue!(stdout, ResetColor, SetAttribute(Attribute::Dim), SetForegroundColor(Color::Green))?;
                } else {
                    // Untyped code - medium gray with slight dim for better contrast
                    queue!(stdout, ResetColor, SetAttribute(Attribute::Dim), SetForegroundColor(Color::White))?;
                }
                Self::print_visible_char(ch, &mut stdout)?;
            }
            
            current_col += 1;
        }
        
        // Display metrics at the bottom
        let metrics = Self::calculate_metrics(current_position, mistakes, start_time);
        let metrics_row = terminal_height.saturating_sub(2);
        queue!(stdout, MoveTo(0, metrics_row))?;
        queue!(stdout, ResetColor, Print(format!(
            "WPM: {:.0}  Accuracy: {:.0}%  Mistakes: {}  Line: {}/{}  [ESC to quit]",
            metrics.wpm, metrics.accuracy, metrics.mistakes, current_line + 1, line_starts.len()
        )))?;
        
        // Show cursor and flush all queued operations at once
        queue!(stdout, Show)?;
        stdout.flush()?;
        Ok(())
    }

    fn print_visible_char(ch: char, stdout: &mut std::io::Stdout) -> Result<()> {
        match ch {
            ' ' => {
                // Show spaces as normal spaces (no special visualization)
                queue!(stdout, Print(' '))?;
            },
            '\t' => {
                // Show tabs as 4 spaces
                queue!(stdout, Print("    "))?;
            },
            '\n' => {
                // Show newlines with a visual marker (return symbol)
                queue!(stdout, Print('↵'))?;
            },
            c if c.is_control() => {
                // Skip other control characters
                queue!(stdout, Print('?'))?;
            },
            c => {
                // Normal visible character
                queue!(stdout, Print(c))?;
            }
        }
        Ok(())
    }

    fn find_line_for_position(position: usize, line_starts: &[usize]) -> usize {
        for (line_num, &line_start) in line_starts.iter().enumerate() {
            if position < line_start {
                return line_num.saturating_sub(1);
            }
        }
        line_starts.len().saturating_sub(1)
    }

    fn calculate_metrics(current_position: usize, mistakes: usize, start_time: &std::time::Instant) -> TypingMetrics {
        let elapsed = start_time.elapsed();
        let words_typed = current_position as f64 / 5.0;
        let wpm = (words_typed / elapsed.as_secs_f64()) * 60.0;
        let total_chars = current_position.max(1);
        
        // Prevent overflow when mistakes > total_chars
        let correct_chars = if mistakes > total_chars {
            0
        } else {
            total_chars - mistakes
        };
        
        let accuracy = (correct_chars as f64 / total_chars as f64) * 100.0;
        
        TypingMetrics {
            wpm,
            accuracy,
            mistakes,
            corrections: 0,
            consistency_score: accuracy,
            completion_time: elapsed,
            challenge_score: wpm * (accuracy / 100.0),
        }
    }

    fn is_position_in_comment(position: usize, comment_ranges: &[(usize, usize)]) -> bool {
        comment_ranges.iter().any(|&(start, end)| position >= start && position < end)
    }
}