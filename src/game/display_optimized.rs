use super::{challenge::Challenge, text_processor::TextProcessor};
use crate::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

#[derive(Debug, Clone, PartialEq, Default)]
struct StyleState {
    foreground: Option<Color>,
    background: Option<Color>,
    attribute: Option<Attribute>,
}

pub struct GameDisplayOptimized {
    chars: Vec<char>,
    last_position: usize,
    last_terminal_size: (u16, u16),
}

impl GameDisplayOptimized {
    pub fn new(challenge_text: &str) -> Self {
        Self {
            chars: challenge_text.chars().collect(),
            last_position: 0,
            last_terminal_size: (0, 0),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn display_challenge_with_info(
        &mut self,
        challenge_text: &str,
        current_position: usize,
        mistakes: usize,
        start_time: &std::time::Instant,
        line_starts: &[usize],
        comment_ranges: &[(usize, usize)],
        challenge: Option<&Challenge>,
        current_mistake_position: Option<usize>,
        skips_remaining: usize,
    ) -> Result<()> {
        let mut stdout = stdout();
        let (terminal_width, terminal_height) = terminal::size()?;

        // Check if we need to rebuild character cache
        if self.chars.len() != challenge_text.chars().count() {
            self.chars = challenge_text.chars().collect();
        }

        // Only do full redraw on size change or large position jump
        let needs_full_redraw = (terminal_width, terminal_height) != self.last_terminal_size
            || current_position.abs_diff(self.last_position) > 50;

        // Hide cursor to prevent flickering
        queue!(stdout, Hide)?;

        if needs_full_redraw {
            queue!(stdout, terminal::Clear(ClearType::All))?;
            self.last_terminal_size = (terminal_width, terminal_height);
        }

        queue!(stdout, MoveTo(0, 0))?;

        // Update header (always needs update for progress)
        self.display_header(
            &mut stdout,
            challenge,
            current_position,
            challenge_text.len(),
            terminal_width,
        )?;

        // Calculate display window
        let display_start_row = 3u16;
        let available_rows = terminal_height.saturating_sub(5);

        // Always show full text, but optimize rendering
        // Only redraw content if position changed or full redraw needed
        if needs_full_redraw || current_position != self.last_position {
            self.display_content(
                &mut stdout,
                0,
                self.chars.len(),
                current_position,
                line_starts,
                comment_ranges,
                current_mistake_position,
                display_start_row,
                available_rows,
                terminal_width,
            )?;
        }

        // Update metrics at bottom
        self.display_metrics(
            &mut stdout,
            current_position,
            mistakes,
            start_time,
            line_starts,
            terminal_height,
            skips_remaining,
        )?;

        queue!(stdout, Show)?;
        stdout.flush()?;

        self.last_position = current_position;
        Ok(())
    }

    fn display_header(
        &self,
        stdout: &mut std::io::Stdout,
        challenge: Option<&Challenge>,
        current_position: usize,
        total_len: usize,
        terminal_width: u16,
    ) -> Result<()> {
        let progress = if total_len > 0 {
            (current_position as f32 / total_len as f32 * 100.0) as u32
        } else {
            0
        };

        queue!(
            stdout,
            MoveTo(0, 0),
            terminal::Clear(ClearType::CurrentLine)
        )?;
        if let Some(challenge) = challenge {
            queue!(
                stdout,
                Print(format!(
                    "[{}] - Progress: {}%",
                    challenge.get_display_title(),
                    progress
                ))
            )?;
        } else {
            queue!(
                stdout,
                Print(format!("[Challenge] - Progress: {}%", progress))
            )?;
        }

        queue!(
            stdout,
            MoveTo(0, 1),
            terminal::Clear(ClearType::CurrentLine)
        )?;
        queue!(
            stdout,
            Print("Press ESC to skip challenge or Ctrl+ESC to fail")
        )?;

        queue!(
            stdout,
            MoveTo(0, 2),
            terminal::Clear(ClearType::CurrentLine)
        )?;
        queue!(stdout, Print("─".repeat(terminal_width as usize)))?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn display_content(
        &self,
        stdout: &mut std::io::Stdout,
        _visible_start: usize, // Keep for API compatibility
        _visible_end: usize,   // Keep for API compatibility
        current_position: usize,
        line_starts: &[usize],
        comment_ranges: &[(usize, usize)],
        current_mistake_position: Option<usize>,
        start_row: u16,
        available_rows: u16,
        terminal_width: u16,
    ) -> Result<()> {
        // Smart scrolling: calculate which line contains current position
        let current_line = self.find_line_for_position(current_position, line_starts);

        // Calculate scroll offset to keep current line in view
        let visible_lines = available_rows as usize;
        let scroll_offset = if current_line > visible_lines / 2 {
            current_line.saturating_sub(visible_lines / 2)
        } else {
            0
        };

        // Pre-calculate string conversion (do once)
        let text_str = self.chars.iter().collect::<String>();

        let mut current_col = 0u16;
        let mut current_row = start_row;
        let mut display_line = 0usize;

        // Clear only the rows we'll use
        for row in start_row..start_row + available_rows {
            queue!(
                stdout,
                MoveTo(0, row),
                terminal::Clear(ClearType::CurrentLine)
            )?;
        }

        queue!(stdout, MoveTo(0, current_row))?;

        // Batch character processing with optimized styling
        let mut last_style = StyleState::default();

        for (i, &ch) in self.chars.iter().enumerate() {
            // Handle scrolling
            let char_line = self.find_line_for_position(i, line_starts);
            if char_line < scroll_offset {
                continue; // Skip lines before scroll window
            }
            if display_line >= visible_lines {
                break; // Stop when we've filled available rows
            }

            // Handle newlines
            if ch == '\n' {
                display_line += 1;
                current_row += 1;
                current_col = 0;
                if current_row >= start_row + available_rows {
                    break;
                }
                queue!(stdout, MoveTo(current_col, current_row))?;
                continue;
            }

            // Line wrapping
            if current_col >= terminal_width - 1 {
                current_row += 1;
                current_col = 0;
                if current_row >= start_row + available_rows {
                    break;
                }
                queue!(stdout, MoveTo(current_col, current_row))?;
            }

            // Calculate character state
            let should_skip =
                TextProcessor::should_skip_character(&text_str, i, line_starts, comment_ranges);
            let is_comment = Self::is_position_in_comment(i, comment_ranges);

            // Optimized rendering with style batching
            self.render_character_optimized(
                stdout,
                ch,
                i,
                current_position,
                should_skip,
                is_comment,
                current_mistake_position,
                &mut last_style,
            )?;

            current_col += 1;
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn render_character_optimized(
        &self,
        stdout: &mut std::io::Stdout,
        ch: char,
        position: usize,
        current_position: usize,
        should_skip: bool,
        is_comment: bool,
        current_mistake_position: Option<usize>,
        last_style: &mut StyleState,
    ) -> Result<()> {
        // Determine the new style needed
        let new_style = if position < current_position || should_skip {
            if is_comment {
                StyleState {
                    foreground: Some(Color::Green),
                    background: None,
                    attribute: Some(Attribute::Dim),
                }
            } else if should_skip {
                StyleState {
                    foreground: Some(Color::DarkGrey),
                    background: None,
                    attribute: Some(Attribute::Dim),
                }
            } else {
                StyleState {
                    foreground: Some(Color::White),
                    background: None,
                    attribute: Some(Attribute::Bold),
                }
            }
        } else if position == current_position {
            if let Some(mistake_pos) = current_mistake_position {
                if position == mistake_pos {
                    StyleState {
                        foreground: Some(Color::White),
                        background: Some(Color::Red),
                        attribute: None,
                    }
                } else {
                    StyleState {
                        foreground: Some(Color::Black),
                        background: Some(Color::Yellow),
                        attribute: None,
                    }
                }
            } else {
                StyleState {
                    foreground: Some(Color::Black),
                    background: Some(Color::Yellow),
                    attribute: None,
                }
            }
        } else if is_comment {
            StyleState {
                foreground: Some(Color::Green),
                background: None,
                attribute: Some(Attribute::Dim),
            }
        } else {
            StyleState {
                foreground: Some(Color::White),
                background: None,
                attribute: Some(Attribute::Dim),
            }
        };

        // Only apply style changes if different from last style
        if new_style != *last_style {
            queue!(stdout, ResetColor)?;

            if let Some(fg) = new_style.foreground {
                queue!(stdout, SetForegroundColor(fg))?;
            }
            if let Some(bg) = new_style.background {
                queue!(stdout, SetBackgroundColor(bg))?;
            }
            if let Some(attr) = new_style.attribute {
                queue!(stdout, SetAttribute(attr))?;
            }

            *last_style = new_style;
        }

        self.print_visible_char(ch, stdout)?;

        // Reset after cursor position
        if position == current_position && last_style.background.is_some() {
            queue!(stdout, ResetColor)?;
            *last_style = StyleState::default();
        }

        Ok(())
    }

    fn print_visible_char(&self, ch: char, stdout: &mut std::io::Stdout) -> Result<()> {
        match ch {
            ' ' => queue!(stdout, Print(' ')),
            '\t' => queue!(stdout, Print("    ")),
            '\n' => queue!(stdout, Print('↵')),
            c if c.is_control() => queue!(stdout, Print('?')),
            c => queue!(stdout, Print(c)),
        }?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn display_metrics(
        &self,
        stdout: &mut std::io::Stdout,
        current_position: usize,
        mistakes: usize,
        start_time: &std::time::Instant,
        line_starts: &[usize],
        terminal_height: u16,
        skips_remaining: usize,
    ) -> Result<()> {
        let metrics = crate::scoring::engine::ScoringEngine::calculate_real_time_metrics(
            current_position,
            mistakes,
            start_time,
        );
        let _current_line = self.find_line_for_position(current_position, line_starts);
        let metrics_row = terminal_height.saturating_sub(2);

        queue!(
            stdout,
            MoveTo(0, metrics_row),
            terminal::Clear(ClearType::CurrentLine)
        )?;
        let elapsed_secs = start_time.elapsed().as_secs();
        let total_chars = self.chars.len();
        let progress_percent = if total_chars > 0 {
            (current_position as f32 / total_chars as f32 * 100.0) as u8
        } else {
            0
        };

        queue!(stdout, ResetColor, Print(format!(
            "CPM: {:.0} | WPM: {:.0} | Accuracy: {:.0}% | Mistakes: {} | Progress: {}/{}({:.0}%) | Time: {}s | Title: {} | Skips: {} | [ESC=skip, Ctrl+ESC=fail]",
            metrics.cpm, metrics.wpm, metrics.accuracy, metrics.mistakes,
            current_position, total_chars, progress_percent, elapsed_secs,
            metrics.ranking_title, skips_remaining
        )))?;

        Ok(())
    }

    fn find_line_for_position(&self, position: usize, line_starts: &[usize]) -> usize {
        for (line_num, &line_start) in line_starts.iter().enumerate() {
            if position < line_start {
                return line_num.saturating_sub(1);
            }
        }
        line_starts.len().saturating_sub(1)
    }

    fn is_position_in_comment(position: usize, comment_ranges: &[(usize, usize)]) -> bool {
        comment_ranges
            .iter()
            .any(|&(start, end)| position >= start && position < end)
    }
}
