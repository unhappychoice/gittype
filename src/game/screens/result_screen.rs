use crate::Result;
use crate::scoring::{TypingMetrics, ScoringEngine};
use crate::game::ascii_digits::get_digit_patterns;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub enum ResultAction {
    Restart,
    BackToTitle,
    Quit,
}

pub struct ResultScreen;

impl ResultScreen {
    pub fn create_ascii_numbers(score: &str) -> Vec<String> {
        let digit_patterns = get_digit_patterns();
        let max_height = 4;
        let mut result = vec![String::new(); max_height];

        for ch in score.chars() {
            if let Some(digit) = ch.to_digit(10) {
                let pattern = &digit_patterns[digit as usize];
                for (i, line) in pattern.iter().enumerate() {
                    result[i].push_str(line);
                    result[i].push(' ');
                }
            }
        }

        result
    }
    pub fn show(metrics: &TypingMetrics) -> Result<ResultAction> {
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Display results title
        let title = "Challenge Complete!";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(8)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;

        // Display "SCORE" label in normal text with color
        let score_label = "SCORE";
        let label_col = center_col.saturating_sub(score_label.len() as u16 / 2);
        execute!(stdout, MoveTo(label_col, center_row.saturating_sub(8)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(score_label))?;
        execute!(stdout, ResetColor)?;

        // Display large ASCII art numbers with single bold color
        let score_value = format!("{:.0}", metrics.challenge_score);
        let ascii_numbers = Self::create_ascii_numbers(&score_value);
        let score_start_row = center_row.saturating_sub(7);
        
        for (row_index, line) in ascii_numbers.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, score_start_row + row_index as u16))?;
            execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display ranking title prominently below score
        let ranking_display = format!("\"{}\"", metrics.ranking_title);
        let ranking_col = center_col.saturating_sub(ranking_display.len() as u16 / 2);
        execute!(stdout, MoveTo(ranking_col, center_row.saturating_sub(4)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(&ranking_display))?;
        execute!(stdout, ResetColor)?;

        // Display other metrics with symmetric padding around colon
        let cpm_text = format!("{:>21} : {:<21}", "Characters Per Minute", format!("{:.0}", metrics.cpm));
        let wpm_text = format!("{:>21} : {:<21}", "Words Per Minute", format!("{:.0}", metrics.wpm));
        let accuracy_text = format!("{:>21} : {:<21}", "Accuracy", format!("{:.1}%", metrics.accuracy));
        let mistakes_text = format!("{:>21} : {:<21}", "Mistakes", format!("{}", metrics.mistakes));

        let metrics_lines = vec![&cpm_text, &wpm_text, &accuracy_text, &mistakes_text];
        for (i, line) in metrics_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, center_row.saturating_sub(1) + i as u16))?;
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
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(10)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(&title))?;
        execute!(stdout, ResetColor)?;

        // Display "SCORE" label in normal text with color
        let score_label = "SCORE";
        let label_col = center_col.saturating_sub(score_label.len() as u16 / 2);
        execute!(stdout, MoveTo(label_col, center_row.saturating_sub(8)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(score_label))?;
        execute!(stdout, ResetColor)?;

        // Display large ASCII art numbers
        let score_value = format!("{:.0}", metrics.challenge_score);
        let ascii_numbers = Self::create_ascii_numbers(&score_value);
        let score_start_row = center_row.saturating_sub(7);
        
        for (row_index, line) in ascii_numbers.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, score_start_row + row_index as u16))?;
            execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display compact metrics below the score
        let metrics_lines = vec![
            format!("CPM: {:.0} | WPM: {:.0} | Accuracy: {:.1}% | Mistakes: {}", 
                metrics.cpm, metrics.wpm, metrics.accuracy, metrics.mistakes),
            format!("Title: {}", {
                if metrics.ranking_title.chars().count() > 30 {
                    let truncated: String = metrics.ranking_title.chars().take(27).collect();
                    format!("{}...", truncated)
                } else {
                    metrics.ranking_title.clone()
                }
            }),
        ];

        for (i, line) in metrics_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, center_row.saturating_sub(2) + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show progress and next action
        if has_next_stage {
            let progress_text = format!("Progress: {} / {}", current_stage, total_stages);
            let progress_col = center_col.saturating_sub(progress_text.len() as u16 / 2);
            execute!(stdout, MoveTo(progress_col, center_row + 5))?;
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            execute!(stdout, Print(&progress_text))?;
            execute!(stdout, ResetColor)?;

            let next_text = "Next stage starting...";
            let next_col = center_col.saturating_sub(next_text.len() as u16 / 2);
            execute!(stdout, MoveTo(next_col, center_row + 6))?;
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            execute!(stdout, Print(next_text))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;

        // Show results with user input to proceed
        let continue_text = "Press any key to continue...";
        let continue_col = center_col.saturating_sub(continue_text.len() as u16 / 2);
        execute!(stdout, MoveTo(continue_col, center_row + 8))?;
        execute!(stdout, SetForegroundColor(Color::Grey))?;
        execute!(stdout, Print(continue_text))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;
        
        // Wait for user input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(_) = event::read()? {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn show_session_summary(
        total_stages: usize,
        completed_stages: usize, 
        stage_engines: &[(String, ScoringEngine)],
    ) -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Display session complete title
        let title = "Session Complete!";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(15)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;

        // Calculate aggregated session metrics by combining ScoringEngines with + operator
        if stage_engines.is_empty() {
            return Ok(());
        }

        let combined_engine = stage_engines.iter()
            .map(|(_, engine)| engine.clone())
            .reduce(|acc, engine| acc + engine)
            .unwrap(); // Safe because we checked is_empty() above

        let session_metrics = match combined_engine.calculate_metrics() {
            Ok(metrics) => metrics,
            Err(_) => {
                // Fallback if calculation fails
                return Ok(());
            }
        };

        // Display "SCORE" label in normal text with color
        let score_label = "SESSION SCORE";
        let label_col = center_col.saturating_sub(score_label.len() as u16 / 2);
        execute!(stdout, MoveTo(label_col, center_row.saturating_sub(11)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(score_label))?;
        execute!(stdout, ResetColor)?;

        // Display large ASCII art session score with single bold color
        let score_value = format!("{:.0}", session_metrics.challenge_score);
        let ascii_numbers = Self::create_ascii_numbers(&score_value);
        let score_start_row = center_row.saturating_sub(10);
        
        for (row_index, line) in ascii_numbers.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, score_start_row + row_index as u16))?;
            execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }
        
        // Truncate session title if too long for 17-char field  
        let session_title = if session_metrics.ranking_title.chars().count() > 17 {
            let truncated: String = session_metrics.ranking_title.chars().take(14).collect();
            format!("{}...", truncated)
        } else {
            session_metrics.ranking_title
        };
        
        // Display session summary with symmetric padding around colon (excluding score - shown as ASCII art above)
        let summary_lines = vec![
            format!("{:>17} : {:<17}", "Stages Completed", format!("{}/{}", completed_stages, total_stages)),
            format!("{:>17} : {:<17}", "Session CPM", format!("{:.1}", session_metrics.cpm)),
            format!("{:>17} : {:<17}", "Session WPM", format!("{:.1}", session_metrics.wpm)),
            format!("{:>17} : {:<17}", "Session Accuracy", format!("{:.1}%", session_metrics.accuracy)),
            format!("{:>17} : {:<17}", "Total Keystrokes", format!("{}", stage_engines.iter().map(|(_, e)| e.total_chars()).sum::<usize>())),
            format!("{:>17} : {:<17}", "Total Mistakes", format!("{}", session_metrics.mistakes)),
            format!("{:>17} : {:<17}", "Total Time", format!("{:.1}s", session_metrics.completion_time.as_secs_f64())),
            format!("{:>17} : {:<17}", "Session Title", session_title),
        ];

        for (i, line) in summary_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, center_row.saturating_sub(4) + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display individual stage results
        if !stage_engines.is_empty() {
            execute!(stdout, MoveTo(center_col.saturating_sub(10), center_row + 6))?;
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            execute!(stdout, Print("Stage Results:"))?;
            execute!(stdout, ResetColor)?;

            for (i, (stage_name, engine)) in stage_engines.iter().enumerate().take(5) {
                let result_line = format!("{}: CPM {:.0}, WPM {:.0}, Acc {:.1}%", stage_name, engine.cpm(), engine.wpm(), engine.accuracy());
                let result_col = center_col.saturating_sub(result_line.len() as u16 / 2);
                execute!(stdout, MoveTo(result_col, center_row + 7 + i as u16))?;
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

        let options_start = center_row + 13;
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
