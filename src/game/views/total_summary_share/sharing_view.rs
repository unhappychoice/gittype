use crate::models::TotalResult;
use crate::sharing::SharingPlatform;
use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io::{Stdout, Write};

pub struct SharingView;

impl SharingView {
    pub fn render_menu(
        stdout: &mut Stdout,
        total_summary: &TotalResult,
        center_col: u16,
        center_row: u16,
    ) -> Result<()> {
        // Title
        let title = "Share Your Total Results";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(8)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Colors::to_crossterm(Colors::info()))
        )?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;

        // Show preview of what will be shared with individual color coding
        let preview_text = format!(
            "Score: {:.0}, CPM: {:.0}, Keystrokes: {}, Sessions: {}/{}, Time: {:.1}min",
            total_summary.total_score,
            total_summary.overall_cpm,
            total_summary.total_keystrokes,
            total_summary.total_sessions_completed,
            total_summary.total_sessions_attempted,
            total_summary.total_duration.as_secs_f64() / 60.0
        );
        let preview_row = center_row.saturating_sub(5);
        let preview_col = center_col.saturating_sub(preview_text.len() as u16 / 2);
        execute!(stdout, MoveTo(preview_col, preview_row))?;

        // Score label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::score()))
        )?;
        execute!(stdout, Print("Score: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{:.0}", total_summary.total_score)))?;
        execute!(stdout, Print(", "))?;

        // CPM label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::cpm_wpm()))
        )?;
        execute!(stdout, Print("CPM: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{:.0}", total_summary.overall_cpm)))?;
        execute!(stdout, Print(", "))?;

        // Keystrokes label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::stage_info()))
        )?;
        execute!(stdout, Print("Keystrokes: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{}", total_summary.total_keystrokes)))?;
        execute!(stdout, Print(", "))?;

        // Sessions label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::info()))
        )?;
        execute!(stdout, Print("Sessions: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(
            stdout,
            Print(format!(
                "{}/{}",
                total_summary.total_sessions_completed, total_summary.total_sessions_attempted
            ))
        )?;
        execute!(stdout, Print(", "))?;

        // Time label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::duration()))
        )?;
        execute!(stdout, Print("Time: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(
            stdout,
            Print(format!(
                "{:.1}min",
                total_summary.total_duration.as_secs_f64() / 60.0
            ))
        )?;
        execute!(stdout, ResetColor)?;

        // Platform options
        let platforms = SharingPlatform::all();
        let start_row = center_row.saturating_sub(2);

        for (i, platform) in platforms.iter().enumerate() {
            let key = format!("[{}]", i + 1);
            let platform_name = format!(" {}", platform.name());
            let full_len = key.len() + platform_name.len();
            let option_col = center_col.saturating_sub(full_len as u16 / 2);
            execute!(stdout, MoveTo(option_col, start_row + i as u16))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::success()))
            )?;
            execute!(stdout, Print(&key))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::text()))
            )?;
            execute!(stdout, Print(&platform_name))?;
            execute!(stdout, ResetColor)?;
        }

        // Back option with color coding
        let back_key = "[ESC]";
        let back_label = " Back to Exit Screen";
        let full_back_len = back_key.len() + back_label.len();
        let back_col = center_col.saturating_sub(full_back_len as u16 / 2);
        execute!(
            stdout,
            MoveTo(back_col, start_row + platforms.len() as u16 + 2)
        )?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::error()))
        )?;
        execute!(stdout, Print(back_key))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(back_label))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }

    pub fn render_fallback_url(
        stdout: &mut Stdout,
        url: &str,
        platform: &SharingPlatform,
        center_col: u16,
        center_row: u16,
    ) -> Result<()> {
        // Title
        let title = format!("Could not open {} automatically", platform.name());
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(6)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Colors::to_crossterm(Colors::warning()))
        )?;
        execute!(stdout, Print(&title))?;
        execute!(stdout, ResetColor)?;

        // Instructions
        let instruction = "Please copy the URL below and open it in your browser:";
        let instruction_col = center_col.saturating_sub(instruction.len() as u16 / 2);
        execute!(
            stdout,
            MoveTo(instruction_col, center_row.saturating_sub(4))
        )?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(instruction))?;
        execute!(stdout, ResetColor)?;

        // URL display box
        let url_display = url.to_string();
        let url_col = center_col.saturating_sub(url_display.len() as u16 / 2);
        execute!(stdout, MoveTo(url_col, center_row.saturating_sub(1)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Colors::to_crossterm(Colors::info()))
        )?;
        execute!(stdout, Print(&url_display))?;
        execute!(stdout, ResetColor)?;

        // Continue prompt with color coding
        let exit_key = "[ESC]";
        let exit_label = " Exit";
        let total_exit_len = exit_key.len() + exit_label.len();
        let continue_col = center_col.saturating_sub(total_exit_len as u16 / 2);
        execute!(stdout, MoveTo(continue_col, center_row + 4))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::error()))
        )?;
        execute!(stdout, Print(exit_key))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(exit_label))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }

    pub fn render_exit_options(
        stdout: &mut Stdout,
        center_col: u16,
        options_start: u16,
    ) -> Result<()> {
        // Show thanks message
        let thanks_message = "Thanks for playing GitType!";
        let thanks_col = center_col.saturating_sub(thanks_message.len() as u16 / 2);
        execute!(stdout, MoveTo(thanks_col, options_start))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Colors::to_crossterm(Colors::success()))
        )?;
        execute!(stdout, Print(thanks_message))?;
        execute!(stdout, ResetColor)?;

        // Show GitHub link and star message
        let github_message = "⭐ Star us on GitHub: https://github.com/unhappychoice/gittype";
        let github_col = center_col.saturating_sub(github_message.len() as u16 / 2);
        execute!(stdout, MoveTo(github_col, options_start + 1))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::warning()))
        )?;
        execute!(stdout, Print(github_message))?;
        execute!(stdout, ResetColor)?;

        let options_data = [
            (
                "[S]",
                " Share Result",
                Colors::to_crossterm(Colors::success()),
            ),
            ("[ESC]", " Exit", Colors::to_crossterm(Colors::error())),
        ];

        for (i, (key, label, key_color)) in options_data.iter().enumerate() {
            let full_text_len = key.len() + label.len();
            let option_col = center_col.saturating_sub(full_text_len as u16 / 2);
            execute!(stdout, MoveTo(option_col, options_start + 3 + i as u16))?;
            execute!(stdout, SetForegroundColor(*key_color))?;
            execute!(stdout, Print(key))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::text()))
            )?;
            execute!(stdout, Print(label))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;
        Ok(())
    }
}
