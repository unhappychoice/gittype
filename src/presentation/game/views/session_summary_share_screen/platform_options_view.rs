use crate::presentation::sharing::SharingPlatform;
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
use std::io::{stdout, Write};

pub struct PlatformOptionsView;

impl PlatformOptionsView {
    pub fn render(center_col: u16, start_row: u16) -> Result<()> {
        let mut stdout = stdout();

        let platforms = SharingPlatform::all();

        for (i, platform) in platforms.iter().enumerate() {
            let option_text = format!("[{}] {}", i + 1, platform.name());
            let option_col = center_col.saturating_sub(option_text.len() as u16 / 2);
            execute!(stdout, MoveTo(option_col, start_row + i as u16))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::text()))
            )?;
            execute!(stdout, Print(&option_text))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;
        Ok(())
    }
}
