use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
use std::io::{stdout, Write};

pub struct OptionsView;

impl OptionsView {
    pub fn render(center_col: u16, options_start: u16) -> Result<()> {
        let mut stdout = stdout();

        let row1_options = [
            ("[D]", " Show Detail", Colors::to_crossterm(Colors::info())),
            ("[S]", " Share Result", Colors::to_crossterm(Colors::info())),
        ];

        let row2_options = [
            ("[R]", " Retry", Colors::to_crossterm(Colors::success())),
            (
                "[T]",
                " Back to Title",
                Colors::to_crossterm(Colors::success()),
            ),
            ("[ESC]", " Quit", Colors::to_crossterm(Colors::error())),
        ];

        let mut row1_text = String::new();
        for (i, (key, label, _)) in row1_options.iter().enumerate() {
            if i > 0 {
                row1_text.push_str("  ");
            }
            row1_text.push_str(key);
            row1_text.push_str(label);
        }
        let row1_col = center_col.saturating_sub(row1_text.len() as u16 / 2);
        execute!(stdout, MoveTo(row1_col, options_start))?;

        for (i, (key, label, key_color)) in row1_options.iter().enumerate() {
            if i > 0 {
                execute!(stdout, Print("  "))?;
            }
            execute!(stdout, SetForegroundColor(*key_color))?;
            execute!(stdout, Print(key))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::text()))
            )?;
            execute!(stdout, Print(label))?;
        }
        execute!(stdout, ResetColor)?;

        let mut row2_text = String::new();
        for (i, (key, label, _)) in row2_options.iter().enumerate() {
            if i > 0 {
                row2_text.push_str("  ");
            }
            row2_text.push_str(key);
            row2_text.push_str(label);
        }
        let row2_col = center_col.saturating_sub(row2_text.len() as u16 / 2);
        execute!(stdout, MoveTo(row2_col, options_start + 1))?;

        for (i, (key, label, key_color)) in row2_options.iter().enumerate() {
            if i > 0 {
                execute!(stdout, Print("  "))?;
            }
            execute!(stdout, SetForegroundColor(*key_color))?;
            execute!(stdout, Print(key))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::text()))
            )?;
            execute!(stdout, Print(label))?;
        }
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }
}
