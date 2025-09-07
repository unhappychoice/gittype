use crate::game::ascii_digits::get_digit_patterns;
use crate::models::TotalResult;
use crate::sharing::SharingPlatform;
use crate::Result;
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
    fn create_total_share_text(total_summary: &TotalResult) -> String {
        format!(
            "Just demolished {} keystrokes total in gittype! 🔥 Total Score: {:.0}, CPM: {:.0}, Sessions: {}/{}, Time: {:.1}min 💪\n\nYour turn to abuse your keyboard! https://github.com/unhappychoice/gittype\n\n#gittype #typing #coding #keyboardwarrior",
            total_summary.total_keystrokes,
            total_summary.total_score,
            total_summary.overall_cpm,
            total_summary.total_sessions_completed,
            total_summary.total_sessions_attempted,
            total_summary.total_duration.as_secs_f64() / 60.0
        )
    }

    fn total_summary_to_typing_metrics(total_summary: &TotalResult) -> crate::scoring::StageResult {
        use crate::scoring::StageResult;

        // Create a StageResult from Total TotalResult data
        let rank_name = crate::scoring::Rank::for_score(total_summary.total_score)
            .name()
            .to_string();

        // Calculate tier info manually
        let all_ranks = crate::scoring::Rank::all_ranks();
        let current_rank = crate::scoring::Rank::for_score(total_summary.total_score);
        let same_tier_ranks: Vec<_> = all_ranks
            .iter()
            .filter(|rank| rank.tier() == current_rank.tier())
            .collect();

        let tier_name = match current_rank.tier() {
            crate::scoring::RankTier::Beginner => "Beginner",
            crate::scoring::RankTier::Intermediate => "Intermediate",
            crate::scoring::RankTier::Advanced => "Advanced",
            crate::scoring::RankTier::Expert => "Expert",
            crate::scoring::RankTier::Legendary => "Legendary",
        }
        .to_string();

        let tier_position = same_tier_ranks
            .iter()
            .rev()
            .position(|rank| rank.name() == current_rank.name())
            .map(|pos| pos + 1)
            .unwrap_or(1);

        let tier_total = same_tier_ranks.len();

        let overall_position = all_ranks
            .iter()
            .rev()
            .position(|rank| rank.name() == current_rank.name())
            .map(|pos| pos + 1)
            .unwrap_or(1);

        let overall_total = all_ranks.len();
        let (tier_name, tier_position, tier_total, overall_position, overall_total) = (
            tier_name,
            tier_position,
            tier_total,
            overall_position,
            overall_total,
        );

        StageResult {
            cpm: total_summary.overall_cpm,
            wpm: total_summary.overall_wpm,
            accuracy: total_summary.overall_accuracy,
            keystrokes: total_summary.total_keystrokes,
            mistakes: total_summary.total_mistakes,
            consistency_streaks: vec![], // Not available in total summary
            completion_time: total_summary.total_duration,
            challenge_score: total_summary.total_score,
            rank_name,
            tier_name,
            tier_position,
            tier_total,
            overall_position,
            overall_total,
            was_skipped: false,
            was_failed: false,
            challenge_path: String::new(),
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

    pub fn show(total_summary: &TotalResult) -> Result<ExitAction> {
        let mut stdout = stdout();

        // Comprehensive screen reset
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        // Short delay to ensure terminal state is reset
        std::thread::sleep(std::time::Duration::from_millis(10));

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        let title = "=== TOTAL SUMMARY ===";
        let lines: Vec<&str> = title.split('\n').collect();

        for (i, line) in lines.iter().enumerate() {
            let title_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(
                stdout,
                MoveTo(title_col, center_row.saturating_sub(14) + i as u16)
            )?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Cyan)
            )?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show total duration
        let duration_text = format!(
            "Total Duration: {:.1} minutes",
            total_summary.total_duration.as_secs_f64() / 60.0
        );
        let duration_col = center_col.saturating_sub(duration_text.len() as u16 / 2);
        execute!(stdout, MoveTo(duration_col, center_row.saturating_sub(10)))?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(&duration_text))?;
        execute!(stdout, ResetColor)?;

        // Show completion status
        let completion_status = total_summary.get_completion_status();
        let status_col = center_col.saturating_sub(completion_status.len() as u16 / 2);
        execute!(stdout, MoveTo(status_col, center_row.saturating_sub(9)))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(&completion_status))?;
        execute!(stdout, ResetColor)?;

        // Show total score as large ASCII
        let score_label = "TOTAL SESSION SCORE";
        let score_label_col = center_col.saturating_sub(score_label.len() as u16 / 2);
        execute!(
            stdout,
            MoveTo(score_label_col, center_row.saturating_sub(7))
        )?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(score_label))?;
        execute!(stdout, ResetColor)?;

        let score_value = format!("{:.0}", total_summary.total_score);
        let ascii_numbers = Self::create_ascii_numbers(&score_value);
        let score_start_row = center_row.saturating_sub(6);

        for (row_index, line) in ascii_numbers.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, score_start_row + row_index as u16))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Green)
            )?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show total statistics
        let stats_start_row = center_row.saturating_sub(1);

        let mut stats_lines = vec![
            format!(
                "Overall CPM: {:.1} | WPM: {:.1} | Accuracy: {:.1}%",
                total_summary.overall_cpm,
                total_summary.overall_wpm,
                total_summary.overall_accuracy
            ),
            format!(
                "Total Keystrokes: {} | Mistakes: {} | Stages: {}/{}",
                total_summary.total_keystrokes,
                total_summary.total_mistakes,
                total_summary.total_stages_completed,
                total_summary.total_stages_attempted
            ),
        ];

        if total_summary.total_stages_skipped > 0 {
            stats_lines.push(format!(
                "Stages Skipped: {}",
                total_summary.total_stages_skipped
            ));
        }

        for (i, line) in stats_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, stats_start_row + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show best/worst performance (always show, even if zero)
        let performance_start_row = stats_start_row + stats_lines.len() as u16 + 1;

        let performance_lines = [
            format!(
                "Best Session: {:.1} WPM, {:.1}% accuracy",
                total_summary.best_session_wpm, total_summary.best_session_accuracy
            ),
            format!(
                "Worst Session: {:.1} WPM, {:.1}% accuracy",
                total_summary.worst_session_wpm, total_summary.worst_session_accuracy
            ),
        ];

        for (i, line) in performance_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, performance_start_row + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::Grey))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show exit options
        let options_start = stats_start_row + stats_lines.len() as u16 + 4;
        // Show thanks message
        let thanks_message = "Thanks for playing GitType!";
        let thanks_col = center_col.saturating_sub(thanks_message.len() as u16 / 2);
        execute!(stdout, MoveTo(thanks_col, options_start))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Green)
        )?;
        execute!(stdout, Print(thanks_message))?;
        execute!(stdout, ResetColor)?;

        // Show GitHub link and star message
        let github_message = "⭐ Star us on GitHub: https://github.com/unhappychoice/gittype";
        let github_col = center_col.saturating_sub(github_message.len() as u16 / 2);
        execute!(stdout, MoveTo(github_col, options_start + 1))?;
        execute!(stdout, SetForegroundColor(Color::Yellow))?;
        execute!(stdout, Print(github_message))?;
        execute!(stdout, ResetColor)?;

        let options_data = [
            ("[S]", " Share Result", Color::Cyan),
            ("[ESC]", " Exit", Color::Red),
        ];

        for (i, (key, label, key_color)) in options_data.iter().enumerate() {
            let full_text_len = key.len() + label.len();
            let option_col = center_col.saturating_sub(full_text_len as u16 / 2);
            execute!(stdout, MoveTo(option_col, options_start + 3 + i as u16))?;
            execute!(
                stdout,
                SetForegroundColor(*key_color),
                SetAttribute(Attribute::Dim)
            )?;
            execute!(stdout, Print(key))?;
            execute!(stdout, SetAttribute(Attribute::Reset))?;
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            execute!(stdout, Print(label))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;

        // Wait for user input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            let _ = Self::show_sharing_menu_total(total_summary);
                            // Redraw exit screen after sharing
                            return Self::show(total_summary);
                        }
                        KeyCode::Esc => {
                            return Ok(ExitAction::Exit);
                        }
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            return Ok(ExitAction::Exit);
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
        }
    }

    pub fn show_sharing_menu_total(total_summary: &TotalResult) -> Result<()> {
        let _metrics = Self::total_summary_to_typing_metrics(total_summary);

        // Raw mode should already be enabled from the parent function

        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Title
        let title = "Share Your Total Results";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(8)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;

        // Show preview of what will be shared
        let preview_text = format!(
            "Score: {:.0}, CPM: {:.0}, Keystrokes: {}, Sessions: {}/{}, Time: {:.1}min",
            total_summary.total_score,
            total_summary.overall_cpm,
            total_summary.total_keystrokes,
            total_summary.total_sessions_completed,
            total_summary.total_sessions_attempted,
            total_summary.total_duration.as_secs_f64() / 60.0
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
            SetForegroundColor(Color::Green),
            SetAttribute(Attribute::Dim)
        )?;
        execute!(stdout, Print(back_key))?;
        execute!(stdout, SetAttribute(Attribute::Reset))?;
        execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
        execute!(stdout, Print(back_label))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;

        // Handle input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('1') => {
                            let _ = Self::share_total_result(total_summary, SharingPlatform::X);
                            break;
                        }
                        KeyCode::Char('2') => {
                            let _ =
                                Self::share_total_result(total_summary, SharingPlatform::Reddit);
                            break;
                        }
                        KeyCode::Char('3') => {
                            let _ =
                                Self::share_total_result(total_summary, SharingPlatform::LinkedIn);
                            break;
                        }
                        KeyCode::Char('4') => {
                            let _ =
                                Self::share_total_result(total_summary, SharingPlatform::Facebook);
                            break;
                        }
                        KeyCode::Esc => break,
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            return Ok(());
                        }
                        _ => continue,
                    }
                }
            }
        }

        Ok(())
    }

    fn share_total_result(
        total_summary: &TotalResult,
        platform: SharingPlatform,
    ) -> crate::Result<()> {
        let text = Self::create_total_share_text(total_summary);
        let url = Self::generate_total_share_url(&text, &platform, total_summary);

        match Self::open_browser(&url) {
            Ok(()) => Ok(()),
            Err(_) => Self::display_url_fallback(&url, &platform),
        }
    }

    fn generate_total_share_url(
        text: &str,
        platform: &SharingPlatform,
        total_summary: &TotalResult,
    ) -> String {
        match platform {
            SharingPlatform::X => {
                format!(
                    "https://x.com/intent/tweet?text={}",
                    urlencoding::encode(text)
                )
            }
            SharingPlatform::Reddit => {
                let title = format!(
                    "Just demolished {} keystrokes total in gittype! Score: {:.0}, CPM: {:.0}",
                    total_summary.total_keystrokes,
                    total_summary.total_score,
                    total_summary.overall_cpm
                );
                format!(
                    "https://www.reddit.com/submit?title={}&selftext=true&text={}",
                    urlencoding::encode(&title),
                    urlencoding::encode(text)
                )
            }
            SharingPlatform::LinkedIn => {
                format!(
                    "https://www.linkedin.com/feed/?shareActive=true&mini=true&text={}",
                    urlencoding::encode(text)
                )
            }
            SharingPlatform::Facebook => {
                format!(
                    "https://www.facebook.com/sharer/sharer.php?u={}&quote={}",
                    urlencoding::encode("https://github.com/unhappychoice/gittype"),
                    urlencoding::encode(text)
                )
            }
        }
    }

    fn open_browser(url: &str) -> crate::Result<()> {
        open::that(url).map_err(|e| {
            crate::error::GitTypeError::TerminalError(format!("Failed to open browser: {}", e))
        })
    }

    fn display_url_fallback(url: &str, platform: &SharingPlatform) -> crate::Result<()> {
        use crossterm::{
            cursor::MoveTo,
            event::{self, Event},
            execute,
            style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
            terminal::{self, ClearType},
        };
        use std::io::{stdout, Write};

        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Title
        let title = format!("Could not open {} automatically", platform.name());
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(6)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
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
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(instruction))?;
        execute!(stdout, ResetColor)?;

        // URL display box
        let url_display = url.to_string();
        let url_col = center_col.saturating_sub(url_display.len() as u16 / 2);
        execute!(stdout, MoveTo(url_col, center_row.saturating_sub(1)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
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
            SetForegroundColor(Color::Red),
            SetAttribute(Attribute::Dim)
        )?;
        execute!(stdout, Print(exit_key))?;
        execute!(stdout, SetAttribute(Attribute::Reset))?;
        execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
        execute!(stdout, Print(exit_label))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;

        // Wait for user input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.code == KeyCode::Esc {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn show_total(total_summary: &TotalResult) -> Result<ExitAction> {
        let mut stdout = stdout();

        // Comprehensive screen reset
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        // Short delay to ensure terminal state is reset
        std::thread::sleep(std::time::Duration::from_millis(10));

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        let title = "=== TOTAL SUMMARY ===";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(8)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;

        // Show total score
        let score_value = format!("{:.0}", total_summary.total_score);
        let ascii_numbers = Self::create_ascii_numbers(&score_value);
        let score_start_row = center_row.saturating_sub(6);

        for (row_index, line) in ascii_numbers.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, score_start_row + row_index as u16))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Green)
            )?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show total statistics
        let stats_start_row = center_row.saturating_sub(1);

        let mut stats_lines = vec![
            format!(
                "Overall CPM: {:.1} | WPM: {:.1} | Accuracy: {:.1}%",
                total_summary.overall_cpm,
                total_summary.overall_wpm,
                total_summary.overall_accuracy
            ),
            format!(
                "Total Sessions: {} | Completed: {} | Stages: {}/{}",
                total_summary.total_sessions_attempted,
                total_summary.total_sessions_completed,
                total_summary.total_stages_completed,
                total_summary.total_stages_attempted
            ),
            format!(
                "Total Keystrokes: {} | Mistakes: {} | Skipped: {}",
                total_summary.total_keystrokes,
                total_summary.total_mistakes,
                total_summary.total_stages_skipped
            ),
        ];

        stats_lines.push(format!(
            "Best Session: {:.1} WPM, {:.1}% | Worst: {:.1} WPM, {:.1}%",
            total_summary.best_session_wpm,
            total_summary.best_session_accuracy,
            total_summary.worst_session_wpm,
            total_summary.worst_session_accuracy
        ));

        for (i, line) in stats_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, stats_start_row + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Navigation instructions
        let nav_start_row = stats_start_row + stats_lines.len() as u16 + 2;
        let nav_text = "[ESC] Exit";
        let nav_col = center_col.saturating_sub(nav_text.len() as u16 / 2);
        execute!(stdout, MoveTo(nav_col, nav_start_row))?;
        execute!(stdout, SetForegroundColor(Color::Red))?;
        execute!(stdout, Print(nav_text))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;

        // Wait for user input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Esc => {
                            return Ok(ExitAction::Exit);
                        }
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            return Ok(ExitAction::Exit);
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
}
