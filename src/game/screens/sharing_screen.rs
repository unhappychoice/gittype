use crate::sharing::{SharingPlatform, SharingService};
use crate::{models::GitRepository, Result};
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub struct SharingScreen;

impl SharingScreen {
    pub fn show_sharing_menu(
        metrics: &crate::models::SessionResult,
        repo_info: &Option<GitRepository>,
    ) -> Result<()> {
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

        // Title
        let title = "=== SHARE YOUR RESULT ===";
        let lines: Vec<&str> = title.split('\n').collect();

        for (i, line) in lines.iter().enumerate() {
            let title_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(
                stdout,
                MoveTo(title_col, center_row.saturating_sub(10) + i as u16)
            )?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Cyan)
            )?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show preview of what will be shared
        let best_rank = crate::scoring::Rank::for_score(metrics.session_score);
        let preview_text = if let Some(repo) = repo_info {
            format!(
                "\"{}\" with {:.0}pts on [{}/{}] - CPM: {:.0}, Mistakes: {}",
                best_rank.name(),
                metrics.session_score,
                repo.user_name,
                repo.repository_name,
                metrics.overall_cpm,
                metrics.valid_mistakes + metrics.invalid_mistakes
            )
        } else {
            format!(
                "\"{}\" with {:.0}pts - CPM: {:.0}, Mistakes: {}",
                best_rank.name(),
                metrics.session_score,
                metrics.overall_cpm,
                metrics.valid_mistakes + metrics.invalid_mistakes
            )
        };
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
        let back_label = " Back to Results";
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
                            let _ = SharingService::share_result(
                                metrics,
                                SharingPlatform::X,
                                repo_info,
                            );
                            break;
                        }
                        KeyCode::Char('2') => {
                            let _ = SharingService::share_result(
                                metrics,
                                SharingPlatform::Reddit,
                                repo_info,
                            );
                            break;
                        }
                        KeyCode::Char('3') => {
                            let _ = SharingService::share_result(
                                metrics,
                                SharingPlatform::LinkedIn,
                                repo_info,
                            );
                            break;
                        }
                        KeyCode::Char('4') => {
                            let _ = SharingService::share_result(
                                metrics,
                                SharingPlatform::Facebook,
                                repo_info,
                            );
                            break;
                        }
                        KeyCode::Esc => break,
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            crate::game::stage_manager::cleanup_terminal();
                            std::process::exit(0);
                        }
                        _ => continue,
                    }
                }
            }
        }

        Ok(())
    }
}
