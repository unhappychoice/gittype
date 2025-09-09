use crate::game::screen_manager::{Screen, ScreenTransition, UpdateStrategy};
use std::io::Stdout;
use crate::sharing::{SharingPlatform, SharingService};
use crate::ui::Colors;
use crate::{models::GitRepository, Result};
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
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
                SetForegroundColor(Colors::to_crossterm(Colors::INFO))
            )?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Show preview of what will be shared with colors
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

        // Display with individual colors
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print("\""))?;
        execute!(stdout, SetForegroundColor(best_rank.terminal_color()))?;
        execute!(stdout, Print(best_rank.name()))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print("\" with "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::SCORE))
        )?;
        execute!(stdout, Print(format!("{:.0}pts", metrics.session_score)))?;

        if let Some(repo) = repo_info {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
            )?;
            execute!(stdout, Print(" on ["))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::INFO))
            )?;
            execute!(
                stdout,
                Print(format!("{}/{}", repo.user_name, repo.repository_name))
            )?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
            )?;
            execute!(stdout, Print("]"))?;
        }

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(" - "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::CPM_WPM))
        )?;
        execute!(stdout, Print("CPM: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(format!("{:.0}", metrics.overall_cpm)))?;
        execute!(stdout, Print(", "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::ERROR))
        )?;
        execute!(stdout, Print("Mistakes: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(
            stdout,
            Print(format!(
                "{}",
                metrics.valid_mistakes + metrics.invalid_mistakes
            ))
        )?;
        execute!(stdout, ResetColor)?;

        // Platform options
        let platforms = SharingPlatform::all();
        let start_row = center_row.saturating_sub(2);

        for (i, platform) in platforms.iter().enumerate() {
            let option_text = format!("[{}] {}", i + 1, platform.name());
            let option_col = center_col.saturating_sub(option_text.len() as u16 / 2);
            execute!(stdout, MoveTo(option_col, start_row + i as u16))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
            )?;
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
            SetForegroundColor(Colors::to_crossterm(Colors::SUCCESS))
        )?;
        execute!(stdout, Print(back_key))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
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

// Basic Screen trait implementation for ScreenManager compatibility
pub struct ScreenState {
    should_exit: bool,
}

impl ScreenState {
    pub fn new() -> Self {
        Self { should_exit: false }
    }
}

impl Screen for ScreenState {
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> crate::Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Esc => {
                self.should_exit = true;
                Ok(ScreenTransition::None)
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_exit = true;
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm(&self, _stdout: &mut Stdout) -> crate::Result<()> {
        // TODO: Use real SessionResult and GitRepository data instead of dummy
        let dummy_result = crate::models::SessionResult::default();
        let _ = SharingScreen::show_sharing_menu(&dummy_result, &None);
        Ok(())
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> crate::Result<bool> {
        Ok(false)
    }
}
