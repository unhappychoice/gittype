use crate::models::TotalResult;
use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
use std::io::{Stdout, Write};

pub struct StatisticsView;

impl StatisticsView {
    pub fn render(
        stdout: &mut Stdout,
        total_summary: &TotalResult,
        center_col: u16,
        stats_start_row: u16,
    ) -> Result<()> {
        // Line 1: Overall CPM, WPM, Accuracy with colors
        let line1_text = format!(
            "Overall CPM: {:.1} | WPM: {:.1} | Accuracy: {:.1}%",
            total_summary.overall_cpm, total_summary.overall_wpm, total_summary.overall_accuracy
        );
        let line1_col = center_col.saturating_sub(line1_text.len() as u16 / 2);
        execute!(stdout, MoveTo(line1_col, stats_start_row))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print("Overall "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::CPM_WPM))
        )?;
        execute!(stdout, Print("CPM: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(format!("{:.1}", total_summary.overall_cpm)))?;
        execute!(stdout, Print(" | "))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::CPM_WPM))
        )?;
        execute!(stdout, Print("WPM: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(format!("{:.1}", total_summary.overall_wpm)))?;
        execute!(stdout, Print(" | "))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::ACCURACY))
        )?;
        execute!(stdout, Print("Accuracy: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(
            stdout,
            Print(format!("{:.1}%", total_summary.overall_accuracy))
        )?;
        execute!(stdout, ResetColor)?;

        // Line 2: Sessions and Stages
        let line2_text = format!(
            "Total Sessions: {} | Completed: {} | Stages: {}/{}",
            total_summary.total_sessions_attempted,
            total_summary.total_sessions_completed,
            total_summary.total_stages_completed,
            total_summary.total_stages_attempted
        );
        let line2_col = center_col.saturating_sub(line2_text.len() as u16 / 2);
        execute!(stdout, MoveTo(line2_col, stats_start_row + 1))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print("Total "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::STAGE_INFO))
        )?;
        execute!(stdout, Print("Sessions: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(
            stdout,
            Print(format!("{}", total_summary.total_sessions_attempted))
        )?;
        execute!(stdout, Print(" | "))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::SUCCESS))
        )?;
        execute!(stdout, Print("Completed: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(
            stdout,
            Print(format!("{}", total_summary.total_sessions_completed))
        )?;
        execute!(stdout, Print(" | "))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::STAGE_INFO))
        )?;
        execute!(stdout, Print("Stages: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(
            stdout,
            Print(format!(
                "{}/{}",
                total_summary.total_stages_completed, total_summary.total_stages_attempted
            ))
        )?;
        execute!(stdout, ResetColor)?;

        // Line 3: Keystrokes, Mistakes, Skipped
        let line3_text = format!(
            "Total Keystrokes: {} | Mistakes: {} | Skipped: {}",
            total_summary.total_keystrokes,
            total_summary.total_mistakes,
            total_summary.total_stages_skipped
        );
        let line3_col = center_col.saturating_sub(line3_text.len() as u16 / 2);
        execute!(stdout, MoveTo(line3_col, stats_start_row + 2))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print("Total "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::STAGE_INFO))
        )?;
        execute!(stdout, Print("Keystrokes: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(format!("{}", total_summary.total_keystrokes)))?;
        execute!(stdout, Print(" | "))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::ERROR))
        )?;
        execute!(stdout, Print("Mistakes: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(format!("{}", total_summary.total_mistakes)))?;
        execute!(stdout, Print(" | "))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::WARNING))
        )?;
        execute!(stdout, Print("Skipped: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(
            stdout,
            Print(format!("{}", total_summary.total_stages_skipped))
        )?;
        execute!(stdout, ResetColor)?;

        // Line 4: Best/Worst sessions
        let line4_text = format!(
            "Best Session: {:.1} WPM, {:.1}% | Worst: {:.1} WPM, {:.1}%",
            total_summary.best_session_wpm,
            total_summary.best_session_accuracy,
            total_summary.worst_session_wpm,
            total_summary.worst_session_accuracy
        );
        let line4_col = center_col.saturating_sub(line4_text.len() as u16 / 2);
        execute!(stdout, MoveTo(line4_col, stats_start_row + 3))?;

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print("Best Session: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::CPM_WPM))
        )?;
        execute!(
            stdout,
            Print(format!("{:.1} WPM", total_summary.best_session_wpm))
        )?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(", "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::ACCURACY))
        )?;
        execute!(
            stdout,
            Print(format!("{:.1}%", total_summary.best_session_accuracy))
        )?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(" | Worst: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::CPM_WPM))
        )?;
        execute!(
            stdout,
            Print(format!("{:.1} WPM", total_summary.worst_session_wpm))
        )?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(", "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::ACCURACY))
        )?;
        execute!(
            stdout,
            Print(format!("{:.1}%", total_summary.worst_session_accuracy))
        )?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }
}
