use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, SetForegroundColor},
};
use std::io::Stdout;

pub fn render_navigation(stdout: &mut Stdout, terminal_width: u16, center_y: u16) -> Result<()> {
    let full_text_len = "[R] Retry | [T] Back to Title | [ESC] Session Summary & Exit".len();
    let nav_x = (terminal_width - full_text_len as u16) / 2;
    execute!(stdout, MoveTo(nav_x, center_y + 4))?;

    // [R] Retry
    execute!(
        stdout,
        SetForegroundColor(Colors::to_crossterm(Colors::success()))
    )?;
    execute!(stdout, Print("[R]"))?;
    execute!(
        stdout,
        SetForegroundColor(Colors::to_crossterm(Colors::text()))
    )?;
    execute!(stdout, Print(" Retry | "))?;

    // [T] Back to Title
    execute!(
        stdout,
        SetForegroundColor(Colors::to_crossterm(Colors::success()))
    )?;
    execute!(stdout, Print("[T]"))?;
    execute!(
        stdout,
        SetForegroundColor(Colors::to_crossterm(Colors::text()))
    )?;
    execute!(stdout, Print(" Back to Title | "))?;

    // [ESC] Session Summary & Exit
    execute!(
        stdout,
        SetForegroundColor(Colors::to_crossterm(Colors::error()))
    )?;
    execute!(stdout, Print("[ESC]"))?;
    execute!(
        stdout,
        SetForegroundColor(Colors::to_crossterm(Colors::text()))
    )?;
    execute!(stdout, Print(" Session Summary & Exit"))?;

    Ok(())
}
