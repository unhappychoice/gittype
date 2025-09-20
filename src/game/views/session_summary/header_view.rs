use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io::{stdout, Write};

pub struct HeaderView;

impl HeaderView {
    pub fn render(center_col: u16, start_row: u16) -> Result<()> {
        let mut stdout = stdout();

        let session_title = "=== SESSION COMPLETE ===";
        let title_col = center_col.saturating_sub(session_title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, start_row.saturating_sub(4)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Colors::to_crossterm(Colors::info()))
        )?;
        execute!(stdout, Print(session_title))?;
        execute!(stdout, ResetColor)?;

        let youre_label = "YOU'RE:";
        let youre_col = center_col.saturating_sub(youre_label.len() as u16 / 2);
        execute!(stdout, MoveTo(youre_col, start_row.saturating_sub(1)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Colors::to_crossterm(Colors::info()))
        )?;
        execute!(stdout, Print(youre_label))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }
}
