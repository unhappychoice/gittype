use crate::Result;
use crate::game::stage_builder::DifficultyLevel;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub enum TitleAction {
    Start(DifficultyLevel),
    Quit,
}

pub struct TitleScreen;

impl TitleScreen {
    pub fn show() -> Result<TitleAction> {
        // Use default challenge counts when none provided
        let default_counts = [0, 0, 0, 0];
        Self::show_with_challenge_counts(&default_counts)
    }
    
    pub fn show_with_challenge_counts(challenge_counts: &[usize; 4]) -> Result<TitleAction> {
        let mut selected_difficulty = 1; // Start with Normal (index 1)
        let difficulties = [
            ("Easy", DifficultyLevel::Easy, vec!["30-150 characters", "Short code snippets"]),
            ("Normal", DifficultyLevel::Normal, vec!["120-350 characters", "Medium functions"]),
            ("Hard", DifficultyLevel::Hard, vec!["300-700+ characters", "Long functions or classes"]),
            ("Zen", DifficultyLevel::Zen, vec!["Entire files", "Complete files as challenges"]),
        ];

        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Draw static elements once
        Self::draw_static_elements(&mut stdout, center_row, center_col)?;
        
        let mut last_difficulty = selected_difficulty;
        // Draw initial difficulty selection
        Self::draw_difficulty_selection(&mut stdout, center_row, center_col, &difficulties, selected_difficulty, challenge_counts)?;
        stdout.flush()?;

        loop {
            // Only redraw difficulty selection if it changed
            if selected_difficulty != last_difficulty {
                Self::draw_difficulty_selection(&mut stdout, center_row, center_col, &difficulties, selected_difficulty, challenge_counts)?;
                last_difficulty = selected_difficulty;
                stdout.flush()?;
            }

            // Wait for user input
            if let Ok(true) = event::poll(std::time::Duration::from_millis(50)) {
                if let Ok(Event::Key(key_event)) = event::read() {
                    match key_event.code {
                        KeyCode::Enter => {
                            return Ok(TitleAction::Start(difficulties[selected_difficulty].1.clone()));
                        },
                        KeyCode::Left => {
                            selected_difficulty = if selected_difficulty == 0 {
                                difficulties.len() - 1
                            } else {
                                selected_difficulty - 1
                            };
                        },
                        KeyCode::Right => {
                            selected_difficulty = (selected_difficulty + 1) % difficulties.len();
                        },
                        KeyCode::Esc => return Ok(TitleAction::Quit),
                        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(TitleAction::Quit);
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw_static_elements(stdout: &mut std::io::Stdout, center_row: u16, center_col: u16) -> Result<()> {
        // ASCII logo lines
        let logo_lines = vec![
            "─╔═══╗─╔══╗─╔════╗────╔════╗─╔╗──╔╗─╔═══╗─╔═══╗─",
            "─║╔═╗║─╚╣╠╝─║╔╗╔╗║────║╔╗╔╗║─║╚╗╔╝║─║╔═╗║─║╔══╝─",
            "─║║─╚╝──║║──╚╝║║╚╝────╚╝║║╚╝─╚╗╚╝╔╝─║╚═╝║─║╚══╗─",
            "─║║╔═╗──║║────║║────────║║────╚╗╔╝──║╔══╝─║╔══╝─",
            "─║╚╩═║─╔╣╠╗───║║────────║║─────║║───║║────║╚══╗─",
            "─╚═══╝─╚══╝───╚╝────────╚╝─────╚╝───╚╝────╚═══╝─",
        ];

        // Display ASCII logo
        let logo_width = logo_lines[0].chars().count() as u16;
        let logo_start_col = center_col.saturating_sub(logo_width / 2);
        let logo_start_row = center_row.saturating_sub(8);

        for (i, line) in logo_lines.iter().enumerate() {
            execute!(stdout, MoveTo(logo_start_col, logo_start_row + i as u16))?;
            execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Cyan))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display subtitle
        let subtitle = "Code Typing Challenge";
        let subtitle_col = center_col.saturating_sub(subtitle.len() as u16 / 2);
        execute!(stdout, MoveTo(subtitle_col, center_row - 1))?;
        execute!(stdout, SetForegroundColor(Color::Grey))?;
        execute!(stdout, Print(subtitle))?;
        execute!(stdout, ResetColor)?;


        // Display instructions (moved down to accommodate multi-line difficulty display)
        let instructions = "[←→] Change Difficulty  [ENTER] Start  [ESC] Quit";
        let instructions_col = center_col.saturating_sub(instructions.len() as u16 / 2);
        
        execute!(stdout, MoveTo(instructions_col, center_row + 6))?;
        execute!(stdout, SetForegroundColor(Color::Blue))?;
        execute!(stdout, Print("[←→] Change Difficulty  "))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print("[ENTER] Start  "))?;
        execute!(stdout, SetForegroundColor(Color::Red))?;
        execute!(stdout, Print("[ESC] Quit"))?;
        execute!(stdout, ResetColor)?;

        Ok(())
    }

    fn draw_difficulty_selection(
        stdout: &mut std::io::Stdout, 
        center_row: u16, 
        center_col: u16,
        difficulties: &[(&str, DifficultyLevel, Vec<&str>); 4],
        selected_difficulty: usize,
        challenge_counts: &[usize; 4]
    ) -> Result<()> {
        let start_row = center_row + 1;
        let (name, _, descriptions) = &difficulties[selected_difficulty];
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
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::White))?;
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
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(count_text))?;
        execute!(stdout, ResetColor)?;

        // Line 3 & 4: Description lines
        for (i, description) in descriptions.iter().enumerate() {
            let desc_col = center_col.saturating_sub(description.chars().count() as u16 / 2);
            execute!(stdout, MoveTo(desc_col, start_row + 2 + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::Grey))?;
            execute!(stdout, Print(description))?;
            execute!(stdout, ResetColor)?;
        }

        Ok(())
    }
}