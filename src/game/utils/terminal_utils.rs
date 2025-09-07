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

    /// Display centered header text with color
    pub fn display_header(
        stdout: &mut Stdout,
        text: &str,
        color: crossterm::style::Color,
        row: u16,
    ) -> Result<()> {
        use crossterm::{
            execute,
            style::{SetAttribute, SetForegroundColor, Attribute, Print, ResetColor},
            cursor::MoveTo,
        };

        let (terminal_width, _) = Self::size()?;
        let text_col = terminal_width.saturating_sub(text.len() as u16) / 2;
        
        execute!(stdout, MoveTo(text_col, row))?;
        execute!(stdout, SetForegroundColor(color))?;
        execute!(stdout, SetAttribute(Attribute::Bold))?;
        execute!(stdout, Print(text))?;
        execute!(stdout, ResetColor)?;
        
        Ok(())
    }

    /// Display keyboard controls with color coding
    pub fn display_controls(
        stdout: &mut Stdout,
        controls: &[(&str, &str, crossterm::style::Color)], // (key, description, color)
        row: u16,
    ) -> Result<()> {
        use crossterm::{
            execute,
            style::{SetForegroundColor, Print, ResetColor},
            cursor::MoveTo,
        };

        // Calculate total text length for centering
        let total_len: usize = controls.iter()
            .map(|(key, desc, _)| key.len() + desc.len() + if controls.len() > 1 { 3 } else { 0 }) // +3 for " | "
            .sum::<usize>().saturating_sub(3); // Remove last " | "

        let (terminal_width, _) = Self::size()?;
        let start_col = terminal_width.saturating_sub(total_len as u16) / 2;
        
        execute!(stdout, MoveTo(start_col, row))?;
        
        for (i, (key, description, key_color)) in controls.iter().enumerate() {
            if i > 0 {
                execute!(stdout, SetForegroundColor(crossterm::style::Color::White))?;
                execute!(stdout, Print(" | "))?;
            }
            execute!(stdout, SetForegroundColor(*key_color))?;
            execute!(stdout, Print(key))?;
            execute!(stdout, SetForegroundColor(crossterm::style::Color::White))?;
            execute!(stdout, Print(description))?;
        }
        
        execute!(stdout, ResetColor)?;
        Ok(())
    }
}
