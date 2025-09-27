use crate::domain::models::GitRepository;
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
use std::io::Stdout;

pub struct StaticElementsView;

impl StaticElementsView {
    pub fn draw(
        stdout: &mut Stdout,
        center_row: u16,
        center_col: u16,
        git_repository: Option<&GitRepository>,
    ) -> Result<()> {
        // ASCII logo lines from oh-my-logo "GitType" purple
        let logo_lines = [
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m",
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m/\x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m(\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;74m)\x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m \x1b[39m",
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m|\x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m|\x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m|\x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m'\x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m\\\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m/\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m\\\x1b[39m",
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m|\x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m|\x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;68m|\x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m|\x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;33m)\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m|\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m/\x1b[39m",
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m\\\x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m|\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;74m\\\x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m\\\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m,\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m.\x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m/\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m\\\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m|\x1b[39m",
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;32m/\x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m",
        ];

        // Display ASCII logo
        // GitType logo width is approximately 47 visible characters
        let logo_width = 47u16;
        let logo_start_col = center_col.saturating_sub(logo_width / 2) + 5;
        let logo_start_row = center_row.saturating_sub(8);

        for (i, line) in logo_lines.iter().enumerate() {
            execute!(stdout, MoveTo(logo_start_col, logo_start_row + i as u16))?;
            execute!(stdout, Print(line))?;
        }

        // Display subtitle
        let subtitle = "Code Typing Challenge";
        let subtitle_col = center_col.saturating_sub(subtitle.len() as u16 / 2);
        execute!(stdout, MoveTo(subtitle_col, center_row - 1))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text_secondary()))
        )?;
        execute!(stdout, Print(subtitle))?;
        execute!(stdout, ResetColor)?;

        // Display instructions in organized 3-tier structure
        let instructions_start_row = center_row + 6;

        // Tier 1: Change Difficulty (Difficulty selection controls)
        let change_display_width = 26u16; // "[←→/HL] Change Difficulty"
        let change_col = center_col.saturating_sub(change_display_width / 2) + 2;
        execute!(stdout, MoveTo(change_col, instructions_start_row))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::key_navigation()))
        )?;
        execute!(stdout, Print("[←→/HL]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(" Change Difficulty"))?;
        execute!(stdout, ResetColor)?;

        // Tier 2: Secondary actions (records analytics settings help)
        let secondary_display_width = 50u16; // "[R] Records  [A] Analytics  [S] Settings  [I/?] Help"
        let secondary_col = center_col.saturating_sub(secondary_display_width / 2) + 2;
        execute!(stdout, MoveTo(secondary_col, instructions_start_row + 1))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::info()))
        )?;
        execute!(stdout, Print("[R]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(" Records  "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::info()))
        )?;
        execute!(stdout, Print("[A]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(" Analytics  "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::info()))
        )?;
        execute!(stdout, Print("[S]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(" Settings  "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::info()))
        )?;
        execute!(stdout, Print("[I/?]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(" Help"))?;
        execute!(stdout, ResetColor)?;

        // Tier 3: Primary actions (Start Quit)
        let primary_display_width = 22u16; // "[SPACE] Start  [ESC] Quit"
        let primary_col = center_col.saturating_sub(primary_display_width / 2) + 2;
        execute!(stdout, MoveTo(primary_col, instructions_start_row + 2))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::success()))
        )?;
        execute!(stdout, Print("[SPACE]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(" Start  "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::error()))
        )?;
        execute!(stdout, Print("[ESC]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(" Quit"))?;
        execute!(stdout, ResetColor)?;

        // Display git info at bottom
        crate::presentation::game::views::title::GitRepositoryView::draw(stdout, git_repository)?;

        Ok(())
    }
}
