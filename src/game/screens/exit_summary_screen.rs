use crate::Result;
use crate::game::{SessionSummary, ascii_digits::get_digit_patterns};
use crate::sharing::{SharingService, SharingPlatform};
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

#[derive(Debug)]
pub enum ExitAction {
    Exit,
    Share,
}

pub struct ExitSummaryScreen;

impl ExitSummaryScreen {
    fn session_summary_to_typing_metrics(session_summary: &SessionSummary) -> crate::scoring::TypingMetrics {
        use crate::scoring::{TypingMetrics, ScoringEngine};
        
        // Create a TypingMetrics from SessionSummary data
        let ranking_title = ScoringEngine::get_ranking_title_for_score(session_summary.session_score).name().to_string();
        
        TypingMetrics {
            cpm: session_summary.overall_cpm,
            wpm: session_summary.overall_wpm,
            accuracy: session_summary.overall_accuracy,
            mistakes: session_summary.total_mistakes,
            consistency_streaks: vec![], // Not available in session summary
            completion_time: session_summary.total_session_time,
            challenge_score: session_summary.session_score,
            ranking_title,
            was_skipped: false,
            was_failed: false,
        }
    }
    
    fn create_ascii_numbers(score: &str) -> Vec<String> {
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

    pub fn show(session_summary: &SessionSummary) -> Result<ExitAction> {
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        let title = "🎮 SESSION SUMMARY 🎮";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(12)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Yellow))?;
        execute!(stdout, Print(&title))?;
        execute!(stdout, ResetColor)?;

        // Show session duration
        let duration_text = format!("Session Duration: {:.1} minutes", session_summary.total_session_time.as_secs_f64() / 60.0);
        let duration_col = center_col.saturating_sub(duration_text.len() as u16 / 2);
        execute!(stdout, MoveTo(duration_col, center_row.saturating_sub(10)))?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(&duration_text))?;
        execute!(stdout, ResetColor)?;

        // Show completion status
        let completion_status = session_summary.get_session_completion_status();
        let status_col = center_col.saturating_sub(completion_status.len() as u16 / 2);
        execute!(stdout, MoveTo(status_col, center_row.saturating_sub(9)))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(&completion_status))?;
        execute!(stdout, ResetColor)?;

        // Show total score as large ASCII
        let score_label = "TOTAL SESSION SCORE";
        let score_label_col = center_col.saturating_sub(score_label.len() as u16 / 2);
        execute!(stdout, MoveTo(score_label_col, center_row.saturating_sub(7)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(score_label))?;
        execute!(stdout, ResetColor)?;

        let score_value = format!("{:.0}", session_summary.session_score);
        let ascii_numbers = Self::create_ascii_numbers(&score_value);
        let score_start_row = center_row.saturating_sub(6);
        
        for (row_index, line) in ascii_numbers.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, score_start_row + row_index as u16))?;
            execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show session statistics
        let stats_start_row = center_row.saturating_sub(1);
        
        let mut stats_lines = vec![
            format!("Overall CPM: {:.1} | WPM: {:.1} | Accuracy: {:.1}%", 
                session_summary.overall_cpm, session_summary.overall_wpm, session_summary.overall_accuracy),
            format!("Total Keystrokes: {} | Mistakes: {} | Challenges: {}/{}", 
                session_summary.total_effort_keystrokes(), session_summary.total_effort_mistakes(),
                session_summary.total_challenges_completed, session_summary.total_challenges_attempted),
        ];

        if session_summary.total_skips_used > 0 {
            stats_lines.push(format!("Skips Used: {}", session_summary.total_skips_used));
        }

        for (i, line) in stats_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, stats_start_row + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show best/worst performance if we have completed challenges
        if session_summary.total_challenges_completed > 0 {
            let performance_start_row = stats_start_row + stats_lines.len() as u16 + 1;
            
            let performance_lines = vec![
                format!("Best Stage: {:.1} WPM, {:.1}% accuracy", 
                    session_summary.best_stage_wpm, session_summary.best_stage_accuracy),
                format!("Worst Stage: {:.1} WPM, {:.1}% accuracy", 
                    session_summary.worst_stage_wpm, session_summary.worst_stage_accuracy),
            ];

            for (i, line) in performance_lines.iter().enumerate() {
                let line_col = center_col.saturating_sub(line.len() as u16 / 2);
                execute!(stdout, MoveTo(line_col, performance_start_row + i as u16))?;
                execute!(stdout, SetForegroundColor(Color::Grey))?;
                execute!(stdout, Print(line))?;
                execute!(stdout, ResetColor)?;
            }
        }


        // Show exit options
        let options_start = if session_summary.total_challenges_completed > 0 {
            stats_start_row + stats_lines.len() as u16 + 4
        } else {
            stats_start_row + stats_lines.len() as u16 + 2
        };
        // Show thanks message
        let thanks_message = "Thanks for playing GitType!";
        let thanks_col = center_col.saturating_sub(thanks_message.len() as u16 / 2);
        execute!(stdout, MoveTo(thanks_col, options_start))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(thanks_message))?;
        execute!(stdout, ResetColor)?;

        let options = vec![
            "[S] Share Result",
            "[ESC] Exit",
        ];
        
        for (i, option) in options.iter().enumerate() {
            let option_col = center_col.saturating_sub(option.len() as u16 / 2);
            execute!(stdout, MoveTo(option_col, options_start + 2 + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            execute!(stdout, Print(option))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;

        // Wait for user input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            return Ok(ExitAction::Share);
                        },
                        KeyCode::Esc => {
                            return Ok(ExitAction::Exit);
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

    pub fn show_sharing_menu(session_summary: &SessionSummary) -> Result<()> {
        let metrics = Self::session_summary_to_typing_metrics(session_summary);
        
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Title
        let title = "📤 Share Your Session Result";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(8)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Yellow))?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;

        // Show preview of what will be shared
        let preview_text = format!(
            "\"{}\" - Score: {:.0}, CPM: {:.0}, Session Time: {:.1}min",
            metrics.ranking_title,
            metrics.challenge_score,
            metrics.cpm,
            metrics.completion_time.as_secs_f64() / 60.0
        );
        let preview_col = center_col.saturating_sub(preview_text.len() as u16 / 2);
        execute!(stdout, MoveTo(preview_col, center_row.saturating_sub(5)))?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(&preview_text))?;
        execute!(stdout, ResetColor)?;

        // Platform options
        let platforms = SharingPlatform::all();
        let start_row = center_row.saturating_sub(2);
        
        for (i, platform) in platforms.iter().enumerate() {
            let option_text = format!("[{}] {}", i + 1, platform.name());
            let option_col = center_col.saturating_sub(option_text.len() as u16 / 2);
            execute!(stdout, MoveTo(option_col, start_row + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(&option_text))?;
            execute!(stdout, ResetColor)?;
        }

        // Back option
        let back_text = "[ESC] Back to Exit Screen";
        let back_col = center_col.saturating_sub(back_text.len() as u16 / 2);
        execute!(stdout, MoveTo(back_col, start_row + platforms.len() as u16 + 2))?;
        execute!(stdout, SetForegroundColor(Color::Grey))?;
        execute!(stdout, Print(back_text))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;

        // Handle input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('1') => {
                            let _ = SharingService::share_result(&metrics, SharingPlatform::Twitter);
                            break;
                        },
                        KeyCode::Char('2') => {
                            let _ = SharingService::share_result(&metrics, SharingPlatform::Reddit);
                            break;
                        },
                        KeyCode::Char('3') => {
                            let _ = SharingService::share_result(&metrics, SharingPlatform::LinkedIn);
                            break;
                        },
                        KeyCode::Char('4') => {
                            let _ = SharingService::share_result(&metrics, SharingPlatform::Facebook);
                            break;
                        },
                        KeyCode::Esc => break,
                        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            std::process::exit(0);
                        },
                        _ => continue,
                    }
                }
            }
        }

        Ok(())
    }
}