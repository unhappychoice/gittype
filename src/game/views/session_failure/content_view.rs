use crate::scoring::StageTracker;
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, SetForegroundColor},
};
use std::io::Stdout;

pub fn render_stage_progress(
    stdout: &mut Stdout,
    terminal_width: u16,
    center_y: u16,
    total_stages: usize,
    completed_stages: usize,
) -> Result<()> {
    let stage_text = format!("Stages: {}/{}", completed_stages, total_stages);
    let stage_x = (terminal_width - stage_text.len() as u16) / 2;
    execute!(stdout, MoveTo(stage_x, center_y.saturating_sub(2)))?;
    execute!(
        stdout,
        SetForegroundColor(Colors::to_crossterm(Colors::info()))
    )?;
    execute!(stdout, Print(stage_text))?;
    Ok(())
}

pub fn render_metrics(
    stdout: &mut Stdout,
    terminal_width: u16,
    center_y: u16,
    stage_trackers: &[(String, StageTracker)],
) -> Result<()> {
    if !stage_trackers.is_empty() {
        let (_last_stage_name, last_tracker) = stage_trackers.last().unwrap();
        let mut tracker = last_tracker.clone();
        tracker.record(crate::scoring::StageInput::Fail);
        let metrics = crate::scoring::StageCalculator::calculate(&tracker);

        execute!(stdout, MoveTo(0, center_y))?;

        let cpm_text = "CPM: ";
        let wpm_text = " | WPM: ";
        let accuracy_text = " | Accuracy: ";
        let cpm_value = format!("{:.0}", metrics.cpm);
        let wpm_value = format!("{:.0}", metrics.wpm);
        let accuracy_value = format!("{:.0}%", metrics.accuracy);

        let total_width = cpm_text.len()
            + cpm_value.len()
            + wpm_text.len()
            + wpm_value.len()
            + accuracy_text.len()
            + accuracy_value.len();
        let metrics_x = (terminal_width - total_width as u16) / 2;
        execute!(stdout, MoveTo(metrics_x, center_y))?;

        // CPM label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::cpm_wpm()))
        )?;
        execute!(stdout, Print(cpm_text))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(cpm_value))?;

        // WPM label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::cpm_wpm()))
        )?;
        execute!(stdout, Print(wpm_text))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(wpm_value))?;

        // Accuracy label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::accuracy()))
        )?;
        execute!(stdout, Print(accuracy_text))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(accuracy_value))?;
    }
    Ok(())
}

pub fn render_failure_message(
    stdout: &mut Stdout,
    terminal_width: u16,
    center_y: u16,
) -> Result<()> {
    let fail_text = "Challenge failed. Better luck next time!";
    let fail_x = (terminal_width - fail_text.len() as u16) / 2;
    execute!(stdout, MoveTo(fail_x, center_y + 2))?;
    execute!(
        stdout,
        SetForegroundColor(Colors::to_crossterm(Colors::error()))
    )?;
    execute!(stdout, Print(fail_text))?;
    Ok(())
}
