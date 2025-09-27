use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
use std::io::{stdout, Write};

pub struct SummaryView;

impl SummaryView {
    pub fn render(
        session_result: &crate::domain::models::SessionResult,
        center_col: u16,
        summary_start_row: u16,
    ) -> Result<()> {
        let mut stdout = stdout();

        let line1_text = format!(
            "CPM: {:.0} | WPM: {:.0} | Time: {:.1}s",
            session_result.overall_cpm,
            session_result.overall_wpm,
            session_result.session_duration.as_secs_f64()
        );
        let line1_col = center_col.saturating_sub(line1_text.len() as u16 / 2);
        execute!(stdout, MoveTo(line1_col, summary_start_row))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::cpm_wpm()))
        )?;
        execute!(stdout, Print("CPM: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{:.0}", session_result.overall_cpm)))?;
        execute!(stdout, Print(" | "))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::cpm_wpm()))
        )?;
        execute!(stdout, Print("WPM: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{:.0}", session_result.overall_wpm)))?;
        execute!(stdout, Print(" | "))?;

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
                "{:.1}s",
                session_result.session_duration.as_secs_f64()
            ))
        )?;
        execute!(stdout, ResetColor)?;

        let line2_text = format!(
            "Keystrokes: {} | Mistakes: {} | Accuracy: {:.1}%",
            session_result.valid_keystrokes + session_result.invalid_keystrokes,
            session_result.valid_mistakes + session_result.invalid_mistakes,
            session_result.overall_accuracy
        );
        let line2_col = center_col.saturating_sub(line2_text.len() as u16 / 2);
        execute!(stdout, MoveTo(line2_col, summary_start_row + 1))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::stage_info()))
        )?;
        execute!(stdout, Print("Keystrokes: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(
            stdout,
            Print(format!(
                "{}",
                session_result.valid_keystrokes + session_result.invalid_keystrokes
            ))
        )?;
        execute!(stdout, Print(" | "))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::error()))
        )?;
        execute!(stdout, Print("Mistakes: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(
            stdout,
            Print(format!(
                "{}",
                session_result.valid_mistakes + session_result.invalid_mistakes
            ))
        )?;
        execute!(stdout, Print(" | "))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::accuracy()))
        )?;
        execute!(stdout, Print("Accuracy: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(
            stdout,
            Print(format!("{:.1}%", session_result.overall_accuracy))
        )?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }
}
