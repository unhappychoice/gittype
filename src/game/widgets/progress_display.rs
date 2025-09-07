use crate::game::utils::TerminalUtils;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{Stdout, Write};

/// Progress bar widget for loading screens
pub struct ProgressDisplayWidget {
    width: u16,
    height: u16,
    title_color: Color,
    progress_color: Color,
    background_color: Color,
}

impl Default for ProgressDisplayWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressDisplayWidget {
    pub fn new() -> Self {
        Self {
            width: 60,
            height: 8,
            title_color: Color::Cyan,
            progress_color: Color::Green,
            background_color: Color::DarkGrey,
        }
    }

    pub fn dimensions(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn colors(mut self, title: Color, progress: Color, background: Color) -> Self {
        self.title_color = title;
        self.progress_color = progress;
        self.background_color = background;
        self
    }

    /// Draw progress display with title, progress bar, and status
    pub fn draw_progress(
        &self,
        stdout: &mut Stdout,
        title: &str,
        progress: f64, // 0.0 to 1.0
        status_text: &str,
    ) -> Result<()> {
        let (center_col, center_row) = TerminalUtils::center_position(self.width, self.height)?;

        // Clear area
        for i in 0..self.height {
            execute!(stdout, MoveTo(center_col, center_row + i))?;
            execute!(stdout, Print(" ".repeat(self.width as usize)))?;
        }

        // Draw title
        execute!(stdout, MoveTo(center_col, center_row))?;
        execute!(stdout, SetForegroundColor(self.title_color))?;
        execute!(stdout, Print(title))?;

        // Draw progress bar
        let bar_width = self.width.saturating_sub(4);
        let filled_width = (bar_width as f64 * progress.clamp(0.0, 1.0)) as u16;
        let empty_width = bar_width.saturating_sub(filled_width);

        execute!(stdout, MoveTo(center_col, center_row + 2))?;
        execute!(stdout, Print("["))?;

        // Filled portion
        execute!(stdout, SetForegroundColor(self.progress_color))?;
        execute!(stdout, Print("█".repeat(filled_width as usize)))?;

        // Empty portion
        execute!(stdout, SetForegroundColor(self.background_color))?;
        execute!(stdout, Print("░".repeat(empty_width as usize)))?;

        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print("]"))?;

        // Progress percentage
        execute!(stdout, MoveTo(center_col, center_row + 3))?;
        execute!(stdout, Print(format!("{:.1}%", progress * 100.0)))?;

        // Status text
        execute!(stdout, MoveTo(center_col, center_row + 5))?;
        execute!(stdout, SetForegroundColor(Color::Yellow))?;
        execute!(stdout, Print(status_text))?;

        execute!(stdout, ResetColor)?;
        stdout.flush()?;
        Ok(())
    }
}
