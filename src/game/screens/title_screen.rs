use crate::game::screens::{InfoAction, InfoDialog};
use crate::game::stage_builder::DifficultyLevel;
use crate::models::GitRepository;
use crate::Result;
use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub enum TitleAction {
    Start(DifficultyLevel),
    History,
    Analytics,
    Quit,
}

pub struct TitleScreen;

impl TitleScreen {
    pub fn show() -> Result<TitleAction> {
        // Use default challenge counts when none provided
        let default_counts = [0, 0, 0, 0, 0];
        Self::show_with_challenge_counts(&default_counts)
    }

    pub fn show_with_challenge_counts(challenge_counts: &[usize; 5]) -> Result<TitleAction> {
        Self::show_with_challenge_counts_and_git_repository(challenge_counts, None)
    }

    pub fn show_with_challenge_counts_and_git_repository(
        challenge_counts: &[usize; 5],
        git_repository: Option<&GitRepository>,
    ) -> Result<TitleAction> {
        let mut selected_difficulty = 1; // Start with Normal (index 1)
        let difficulties = [
            ("Easy", DifficultyLevel::Easy),
            ("Normal", DifficultyLevel::Normal),
            ("Hard", DifficultyLevel::Hard),
            ("Wild", DifficultyLevel::Wild),
            ("Zen", DifficultyLevel::Zen),
        ];

        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Draw static elements once
        Self::draw_static_elements(&mut stdout, center_row, center_col, git_repository)?;

        let mut last_difficulty = selected_difficulty;
        // Draw initial difficulty selection
        Self::draw_difficulty_selection(
            &mut stdout,
            center_row,
            center_col,
            &difficulties,
            selected_difficulty,
            challenge_counts,
        )?;
        stdout.flush()?;

        loop {
            // Only redraw difficulty selection if it changed
            if selected_difficulty != last_difficulty {
                Self::draw_difficulty_selection(
                    &mut stdout,
                    center_row,
                    center_col,
                    &difficulties,
                    selected_difficulty,
                    challenge_counts,
                )?;
                last_difficulty = selected_difficulty;
                stdout.flush()?;
            }

            // Wait for user input
            if let Ok(true) = event::poll(std::time::Duration::from_millis(50)) {
                if let Ok(Event::Key(key_event)) = event::read() {
                    match key_event.code {
                        KeyCode::Char(' ') => {
                            return Ok(TitleAction::Start(
                                difficulties[selected_difficulty].1.clone(),
                            ));
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            selected_difficulty = if selected_difficulty == 0 {
                                difficulties.len() - 1
                            } else {
                                selected_difficulty - 1
                            };
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            selected_difficulty = (selected_difficulty + 1) % difficulties.len();
                        }
                        KeyCode::Esc => return Ok(TitleAction::Quit),
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            return Ok(TitleAction::Quit);
                        }
                        KeyCode::Char('i') | KeyCode::Char('?') => {
                            Self::handle_info_dialog()?;
                            // Redraw the entire screen after closing the dialog
                            execute!(stdout, terminal::Clear(ClearType::All))?;
                            Self::draw_static_elements(
                                &mut stdout,
                                center_row,
                                center_col,
                                git_repository,
                            )?;
                            last_difficulty = selected_difficulty + 1; // Force redraw of difficulty selection
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            return Ok(TitleAction::History);
                        }
                        KeyCode::Char('a') | KeyCode::Char('A') => {
                            return Ok(TitleAction::Analytics);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw_static_elements(
        stdout: &mut std::io::Stdout,
        center_row: u16,
        center_col: u16,
        git_repository: Option<&GitRepository>,
    ) -> Result<()> {
        execute!(stdout, Hide)?;
        // ASCII logo lines from oh-my-logo "GitType" purple
        let logo_lines = [
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m",
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m/\x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m(\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;74m)\x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m \x1b[39m",
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m|\x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m|\x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m|\x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m'\x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m\\\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m/\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m\\\x1b[39m",
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m|\x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m|\x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;68m|\x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m|\x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;33m)\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m|\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m/\x1b[39m",
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m\\\x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;104m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;68m|\x1b[39m\x1b[38;5;68m_\x1b[39m\x1b[38;5;74m|\x1b[39m\x1b[38;5;74m\\\x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;74m_\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m\\\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m,\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;32m.\x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m/\x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m\\\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m_\x1b[39m\x1b[38;5;33m|\x1b[39m",
            "\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;104m \x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;68m \x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;74m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m \x1b[39m\x1b[38;5;38m|\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;38m_\x1b[39m\x1b[38;5;32m/\x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m_\x1b[39m\x1b[38;5;32m|\x1b[39m\x1b[38;5;32m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m\x1b[38;5;33m \x1b[39m",
        ];

        // Display ASCII logo
        // GitType logo width is approximately 47 visible characters
        let logo_width = 47u16;
        let logo_start_col = center_col.saturating_sub(logo_width / 2) + 5;
        let logo_start_row = center_row.saturating_sub(8);

        for (i, line) in logo_lines.iter().enumerate() {
            execute!(stdout, MoveTo(logo_start_col, logo_start_row + i as u16))?;
            execute!(stdout, Print(line))?;
        }

        // Display subtitle
        let subtitle = "Code Typing Challenge";
        let subtitle_col = center_col.saturating_sub(subtitle.len() as u16 / 2);
        execute!(stdout, MoveTo(subtitle_col, center_row - 1))?;
        execute!(stdout, SetForegroundColor(Color::Grey))?;
        execute!(stdout, Print(subtitle))?;
        execute!(stdout, ResetColor)?;

        // Display instructions in organized 3-tier structure
        let instructions_start_row = center_row + 6;

        // Tier 1: Change Difficulty (Difficulty selection controls)
        let change_display_width = 26u16; // "[←→/HL] Change Difficulty"
        let change_col = center_col.saturating_sub(change_display_width / 2) + 2;
        execute!(stdout, MoveTo(change_col, instructions_start_row))?;
        execute!(stdout, SetForegroundColor(Color::Blue))?;
        execute!(stdout, Print("[←→/HL]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Change Difficulty"))?;
        execute!(stdout, ResetColor)?;

        // Tier 2: Secondary actions (history analytics info)
        let secondary_display_width = 38u16; // "[R] history  [A] analytics  [I/?] info"
        let secondary_col = center_col.saturating_sub(secondary_display_width / 2) + 2;
        execute!(stdout, MoveTo(secondary_col, instructions_start_row + 1))?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print("[R]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" History  "))?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print("[A]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Analytics  "))?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print("[I/?]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Info"))?;
        execute!(stdout, ResetColor)?;

        // Tier 3: Primary actions (Start Quit)
        let primary_display_width = 22u16; // "[SPACE] Start  [ESC] Quit"
        let primary_col = center_col.saturating_sub(primary_display_width / 2) + 2;
        execute!(stdout, MoveTo(primary_col, instructions_start_row + 2))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print("[SPACE]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Start  "))?;
        execute!(stdout, SetForegroundColor(Color::Red))?;
        execute!(stdout, Print("[ESC]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Quit"))?;
        execute!(stdout, ResetColor)?;

        // Display git info at bottom
        Self::draw_git_repository(stdout, git_repository)?;

        Ok(())
    }

    fn draw_difficulty_selection(
        stdout: &mut std::io::Stdout,
        center_row: u16,
        center_col: u16,
        difficulties: &[(&str, DifficultyLevel); 5],
        selected_difficulty: usize,
        challenge_counts: &[usize; 5],
    ) -> Result<()> {
        let start_row = center_row + 1;
        let (name, difficulty_level) = &difficulties[selected_difficulty];
        let count = challenge_counts[selected_difficulty];

        // Clear previous difficulty display (multiple lines)
        for i in 0..4 {
            execute!(stdout, MoveTo(0, start_row + i))?;
            execute!(stdout, Print(" ".repeat(120)))?;
        }

        // Line 1: Difficulty selection
        let difficulty_text = format!("Difficulty: ← {} →", name);
        let difficulty_col = center_col.saturating_sub(difficulty_text.chars().count() as u16 / 2);

        execute!(stdout, MoveTo(difficulty_col, start_row))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print("Difficulty: "))?;
        execute!(stdout, SetForegroundColor(Color::Yellow))?;
        execute!(stdout, Print("← "))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::White)
        )?;
        execute!(stdout, Print(name))?;
        execute!(stdout, ResetColor, SetForegroundColor(Color::Yellow))?;
        execute!(stdout, Print(" →"))?;
        execute!(stdout, ResetColor)?;

        // Line 2: Challenge count
        let count_text = if count > 0 {
            format!("{} challenges available", count)
        } else {
            "Challenge count will be displayed after loading".to_string()
        };
        let count_col = center_col.saturating_sub(count_text.chars().count() as u16 / 2);

        execute!(stdout, MoveTo(count_col, start_row + 1))?;
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            SetAttribute(Attribute::Dim)
        )?;
        execute!(stdout, Print(count_text))?;
        execute!(stdout, ResetColor)?;

        // Line 3 & 4: Description lines
        let descriptions = [difficulty_level.description(), difficulty_level.subtitle()];
        for (i, description) in descriptions.iter().enumerate() {
            let desc_col = center_col.saturating_sub(description.chars().count() as u16 / 2);
            execute!(stdout, MoveTo(desc_col, start_row + 2 + i as u16))?;
            execute!(
                stdout,
                SetForegroundColor(Color::White),
                SetAttribute(Attribute::Dim)
            )?;
            execute!(stdout, Print(description))?;
            execute!(stdout, ResetColor)?;
        }

        Ok(())
    }

    fn draw_git_repository(
        stdout: &mut std::io::Stdout,
        git_repository: Option<&GitRepository>,
    ) -> Result<()> {
        if let Some(info) = git_repository {
            let (terminal_width, terminal_height) = terminal::size()?;
            let bottom_row = terminal_height - 1;

            // Build git info string
            let mut parts = vec![format!("📁 {}/{}", info.user_name, info.repository_name)];

            if let Some(ref branch) = info.branch {
                parts.push(format!("🌿 {}", branch));
            }

            if let Some(ref commit) = info.commit_hash {
                parts.push(format!("📝 {}", &commit[..8]));
            }

            let status_symbol = if info.is_dirty { "⚠️" } else { "✓" };
            parts.push(status_symbol.to_string());

            let git_text = parts.join(" • ");

            // Calculate approximate display width considering emoji width
            // Each emoji takes about 2 characters worth of width
            let emoji_count = git_text.chars().filter(|c| *c as u32 > 127).count();
            let approximate_width = git_text.chars().count() + emoji_count;

            // Center the text using approximate width
            let git_col = terminal_width.saturating_sub(approximate_width as u16) / 2;

            execute!(stdout, MoveTo(git_col, bottom_row))?;
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            execute!(stdout, Print(&git_text))?;
            execute!(stdout, ResetColor)?;
        }
        Ok(())
    }

    fn handle_info_dialog() -> Result<()> {
        match InfoDialog::show()? {
            InfoAction::OpenGithub => {
                InfoDialog::open_github()?;
            }
            InfoAction::OpenX => {
                InfoDialog::open_x()?;
            }
            InfoAction::Close => {
                // Do nothing, just close the dialog
            }
        }
        Ok(())
    }
}
