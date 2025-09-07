use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::ResetColor,
    terminal::{self, ClearType},
};
use std::io::{stdout, Stdout, Write};

/// Utility functions for terminal operations
pub struct TerminalUtils;

impl TerminalUtils {
    /// Get terminal size as (width, height)
    pub fn size() -> Result<(u16, u16)> {
        Ok(terminal::size()?)
    }

    /// Calculate center position for given content size
    pub fn center_position(content_width: u16, content_height: u16) -> Result<(u16, u16)> {
        let (terminal_width, terminal_height) = Self::size()?;
        let center_col = terminal_width.saturating_sub(content_width) / 2;
        let center_row = terminal_height.saturating_sub(content_height) / 2;
        Ok((center_col, center_row))
    }

    /// Clear screen and reset cursor to top-left
    pub fn clear_screen(stdout: &mut Stdout) -> Result<()> {
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;
        Ok(())
    }

    /// Clear screen with default stdout
    pub fn clear_screen_default() -> Result<()> {
        Self::clear_screen(&mut stdout())
    }
}
