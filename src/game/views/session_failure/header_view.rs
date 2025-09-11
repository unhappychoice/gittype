use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io::Stdout;

pub fn render_header(stdout: &mut Stdout, terminal_width: u16, center_y: u16) -> Result<()> {
    let header_text = "=== SESSION FAILED ===";
    let header_x = (terminal_width - header_text.len() as u16) / 2;
    execute!(stdout, MoveTo(header_x, center_y.saturating_sub(6)))?;
    execute!(
        stdout,
        SetForegroundColor(Colors::to_crossterm(Colors::ERROR)),
        SetAttribute(Attribute::Bold)
    )?;
    execute!(stdout, Print(header_text))?;
    execute!(stdout, ResetColor)?;
    Ok(())
}
