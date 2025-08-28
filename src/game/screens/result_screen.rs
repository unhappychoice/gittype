use crate::Result;
use crate::scoring::TypingMetrics;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};
use super::super::stage_manager::SessionMetrics;

pub enum ResultAction {
    Restart,
    BackToTitle,
    Quit,
}

pub struct ResultScreen;

impl ResultScreen {
    pub fn show(metrics: &TypingMetrics) -> Result<ResultAction> {
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Display results title
        let title = "Challenge Complete!";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(6)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;

        // Display metrics
        let wpm_text = format!("Words Per Minute: {:.0}", metrics.wpm);
        let accuracy_text = format!("Accuracy: {:.1}%", metrics.accuracy);
        let mistakes_text = format!("Mistakes: {}", metrics.mistakes);
        let score_text = format!("Score: {:.0}", metrics.challenge_score);

        let metrics_lines = vec![&wpm_text, &accuracy_text, &mistakes_text, &score_text];
        for (i, line) in metrics_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, center_row.saturating_sub(3) + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display options
        let options = vec![
            ("[R] Try Again", Color::Green),
            ("[T] Back to Title", Color::Cyan),
            ("[ESC] Quit", Color::Red),
        ];

        for (i, (msg, color)) in options.iter().enumerate() {
            let msg_col = center_col.saturating_sub(msg.len() as u16 / 2);
            execute!(stdout, MoveTo(msg_col, center_row + 3 + i as u16))?;
            execute!(stdout, SetForegroundColor(*color))?;
            execute!(stdout, Print(msg))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;

        // Wait for user input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('r') | KeyCode::Char('R') => return Ok(ResultAction::Restart),
                        KeyCode::Char('t') | KeyCode::Char('T') => return Ok(ResultAction::BackToTitle),
                        KeyCode::Esc => return Ok(ResultAction::Quit),
                        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(ResultAction::Quit);
                        },
                        _ => continue,
                    }
                }
            }
        }
    }

    pub fn show_stage_completion(metrics: &TypingMetrics, current_stage: usize, total_stages: usize, has_next_stage: bool) -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Display stage completion
        let title = format!("Stage {} Complete!", current_stage);
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(4)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(&title))?;
        execute!(stdout, ResetColor)?;

        // Display brief metrics
        let metrics_lines = vec![
            format!("WPM: {:.0}", metrics.wpm),
            format!("Accuracy: {:.1}%", metrics.accuracy),
            format!("Mistakes: {}", metrics.mistakes),
        ];

        for (i, line) in metrics_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, center_row.saturating_sub(1) + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show progress and next action
        if has_next_stage {
            let progress_text = format!("Progress: {} / {}", current_stage, total_stages);
            let progress_col = center_col.saturating_sub(progress_text.len() as u16 / 2);
            execute!(stdout, MoveTo(progress_col, center_row + 3))?;
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            execute!(stdout, Print(&progress_text))?;
            execute!(stdout, ResetColor)?;

            let next_text = "Next stage starting...";
            let next_col = center_col.saturating_sub(next_text.len() as u16 / 2);
            execute!(stdout, MoveTo(next_col, center_row + 4))?;
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            execute!(stdout, Print(next_text))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;

        // Brief pause to show results
        std::thread::sleep(std::time::Duration::from_millis(2000));
        Ok(())
    }

    pub fn show_session_summary(session_metrics: &SessionMetrics) -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Display session complete title
        let title = "Session Complete!";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(8)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;

        // Display session summary
        let summary_lines = vec![
            format!("Stages Completed: {}/{}", session_metrics.completed_stages, session_metrics.total_stages),
            format!("Average WPM: {:.1}", session_metrics.total_wpm),
            format!("Average Accuracy: {:.1}%", session_metrics.total_accuracy),
            format!("Total Mistakes: {}", session_metrics.total_mistakes),
            format!("Final Score: {:.0}", session_metrics.session_score),
        ];

        for (i, line) in summary_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, center_row.saturating_sub(4) + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display individual stage results
        if !session_metrics.stage_metrics.is_empty() {
            execute!(stdout, MoveTo(center_col.saturating_sub(10), center_row + 2))?;
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            execute!(stdout, Print("Stage Results:"))?;
            execute!(stdout, ResetColor)?;

            for (i, (stage_name, metrics)) in session_metrics.stage_metrics.iter().enumerate().take(5) {
                let result_line = format!("{}: WPM {:.0}, Acc {:.1}%", stage_name, metrics.wpm, metrics.accuracy);
                let result_col = center_col.saturating_sub(result_line.len() as u16 / 2);
                execute!(stdout, MoveTo(result_col, center_row + 3 + i as u16))?;
                execute!(stdout, SetForegroundColor(Color::Grey))?;
                execute!(stdout, Print(&result_line))?;
                execute!(stdout, ResetColor)?;
            }
        }

        // Display options
        let options = vec![
            "[T/ENTER] Back to Title",
            "[ESC] Quit",
        ];

        let options_start = center_row + 9;
        for (i, option) in options.iter().enumerate() {
            let option_col = center_col.saturating_sub(option.len() as u16 / 2);
            execute!(stdout, MoveTo(option_col, options_start + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            execute!(stdout, Print(option))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;

        // Wait for user input before returning
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Enter | KeyCode::Esc => {
                            return Ok(());
                        },
                        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            std::process::exit(0);
                        },
                        _ => continue,
                    }
                }
            }
        }
    }
}