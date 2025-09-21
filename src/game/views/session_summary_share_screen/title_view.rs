use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io::{stdout, Write};

pub struct TitleView;

impl TitleView {
    pub fn render(center_col: u16, center_row: u16) -> Result<()> {
        let mut stdout = stdout();

        let title = "=== SHARE YOUR RESULT ===";
        let lines: Vec<&str> = title.split('\n').collect();

        for (i, line) in lines.iter().enumerate() {
            let title_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(
                stdout,
                MoveTo(title_col, center_row.saturating_sub(10) + i as u16)
            )?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Colors::to_crossterm(Colors::info()))
            )?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;
        Ok(())
    }
}
