use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
use std::io::{stdout, Write};

pub struct BackOptionView;

impl BackOptionView {
    pub fn render(center_col: u16, row: u16) -> Result<()> {
        let mut stdout = stdout();

        let back_key = "[ESC]";
        let back_label = " Back to Results";
        let full_back_len = back_key.len() + back_label.len();
        let back_col = center_col.saturating_sub(full_back_len as u16 / 2);

        execute!(stdout, MoveTo(back_col, row))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::SUCCESS))
        )?;
        execute!(stdout, Print(back_key))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(back_label))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }
}
