use super::{
    super::{display_ratatui::GameDisplayRatatui, text_processor::TextProcessor},
    CountdownScreen,
};
use crate::models::Challenge;
use crate::scoring::engine::ScoringEngine;
use crate::models::StageResult;
use crate::{models::GitRepository, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal,
};

pub struct TypingScreen {
    challenge: Option<Challenge>,
    challenge_text: String,
    challenge_chars: Vec<char>,
    current_position: usize,
    mistakes: usize,
    start_time: std::time::Instant,
    line_starts: Vec<usize>,
    comment_ranges: Vec<(usize, usize)>,
    mistake_positions: Vec<usize>,
    current_mistake_position: Option<usize>,
    display: GameDisplayRatatui,
    scoring_engine: ScoringEngine,
    skips_remaining: usize,
    #[allow(dead_code)]
    last_esc_time: Option<std::time::Instant>,
    dialog_shown: bool,
    repo_info: Option<GitRepository>,
}

pub enum SessionState {
    Continue,
    Complete,
    Exit,
    Skip,
    Failed,     // For failed state - mark as failed
    ShowDialog, // Show Skip/Quit dialog
}

impl TypingScreen {
    pub fn new(challenge_text: String, repo_info: Option<GitRepository>) -> Result<Self> {
        let processed_text = TextProcessor::process_challenge_text(&challenge_text);
        let challenge_chars: Vec<char> = processed_text.chars().collect();
        let line_starts = TextProcessor::calculate_line_starts(&processed_text);
        let comment_ranges = vec![]; // No comment info available without Challenge
        let initial_position = TextProcessor::find_first_non_whitespace_or_comment(
            &processed_text,
            0,
            &comment_ranges,
        );
        let display = GameDisplayRatatui::new(&processed_text)?;
        let mut scoring_engine = ScoringEngine::new(processed_text.clone());
        scoring_engine.start(); // Start timing immediately

        Ok(Self {
            challenge: None,
            challenge_text: processed_text,
            challenge_chars,
            current_position: initial_position,
            mistakes: 0,
            start_time: std::time::Instant::now(),
            line_starts,
            comment_ranges,
            mistake_positions: Vec::new(),
            current_mistake_position: None,
            display,
            scoring_engine,
            skips_remaining: 3,
            last_esc_time: None,
            dialog_shown: false,
            repo_info,
        })
    }

    pub fn new_with_challenge(
        challenge: &Challenge,
        repo_info: Option<GitRepository>,
    ) -> Result<Self> {
        // Apply basic text processing (remove empty lines, etc.)
        // Indentation normalization is already done in extractor
        let (processed_text, mapped_comment_ranges) =
            TextProcessor::process_challenge_text_with_comment_mapping(
                &challenge.code_content,
                &challenge.comment_ranges,
            );

        let challenge_chars: Vec<char> = processed_text.chars().collect();
        let line_starts = TextProcessor::calculate_line_starts(&processed_text);
        let initial_position = TextProcessor::find_first_non_whitespace_or_comment(
            &processed_text,
            0,
            &mapped_comment_ranges,
        );
        let display = GameDisplayRatatui::new(&processed_text)?;
        let mut scoring_engine = ScoringEngine::new(processed_text.clone());
        scoring_engine.start(); // Start timing immediately

        Ok(Self {
            challenge: Some(challenge.clone()),
            challenge_text: processed_text,
            challenge_chars,
            current_position: initial_position,
            mistakes: 0,
            start_time: std::time::Instant::now(),
            line_starts,
            comment_ranges: mapped_comment_ranges,
            mistake_positions: Vec::new(),
            current_mistake_position: None,
            display,
            scoring_engine,
            skips_remaining: 3,
            last_esc_time: None,
            dialog_shown: false,
            repo_info,
        })
    }

    pub fn start_session(&mut self) -> Result<StageResult> {
        match terminal::enable_raw_mode() {
            Ok(_) => {}
            Err(e) => {
                return Err(crate::error::GitTypeError::TerminalError(format!(
                    "Failed to enable raw mode: {}",
                    e
                )));
            }
        }

        // Show countdown with challenge info if available
        CountdownScreen::show_with_challenge_and_repo(self.challenge.as_ref(), &self.repo_info)?;

        // Reset start time after countdown
        self.start_time = std::time::Instant::now();

        self.display.display_challenge_with_info(
            &self.challenge_text,
            self.current_position,
            self.mistakes,
            &self.start_time,
            &self.line_starts,
            &self.comment_ranges,
            self.challenge.as_ref(),
            self.current_mistake_position,
            self.skips_remaining,
            self.dialog_shown,
            &self.scoring_engine,
            &self.repo_info,
        )?;

        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key(key_event)? {
                        SessionState::Continue => {
                            self.update_display()?;
                        }
                        SessionState::Complete => {
                            break;
                        }
                        SessionState::Exit => {
                            break;
                        }
                        SessionState::Skip => {
                            // Mark challenge as skipped and complete
                            break;
                        }
                        SessionState::Failed => {
                            // Mark challenge as failed and complete
                            break;
                        }
                        SessionState::ShowDialog => {
                            // Dialog was opened, update display to show dialog
                            self.update_display()?;
                        }
                    }
                }
            }
        }

        self.display.cleanup()?;

        crate::game::stage_manager::cleanup_terminal();
        self.scoring_engine.finish(); // Record final duration
        Ok(self.calculate_result())
    }

    pub fn show(&mut self) -> Result<StageResult> {
        // For stage manager - assumes raw mode is already enabled
        self.start_time = std::time::Instant::now();

        self.display.display_challenge_with_info(
            &self.challenge_text,
            self.current_position,
            self.mistakes,
            &self.start_time,
            &self.line_starts,
            &self.comment_ranges,
            self.challenge.as_ref(),
            self.current_mistake_position,
            self.skips_remaining,
            self.dialog_shown,
            &self.scoring_engine,
            &self.repo_info,
        )?;

        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key(key_event)? {
                        SessionState::Continue => {
                            self.update_display()?;
                        }
                        SessionState::Complete => {
                            break;
                        }
                        SessionState::Exit => {
                            break;
                        }
                        SessionState::Skip => {
                            // Mark challenge as skipped and complete
                            break;
                        }
                        SessionState::Failed => {
                            // Mark challenge as failed and complete
                            break;
                        }
                        SessionState::ShowDialog => {
                            // Dialog was opened, update display to show dialog
                            self.update_display()?;
                        }
                    }
                }
            }
        }

        self.scoring_engine.finish(); // Record final duration
        Ok(self.calculate_result())
    }

    pub fn show_with_state(&mut self) -> Result<(StageResult, SessionState)> {
        // For stage manager - assumes raw mode is already enabled
        self.start_time = std::time::Instant::now();

        self.display.display_challenge_with_info(
            &self.challenge_text,
            self.current_position,
            self.mistakes,
            &self.start_time,
            &self.line_starts,
            &self.comment_ranges,
            self.challenge.as_ref(),
            self.current_mistake_position,
            self.skips_remaining,
            self.dialog_shown,
            &self.scoring_engine,
            &self.repo_info,
        )?;

        let final_state = loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key(key_event)? {
                        SessionState::Continue => {
                            self.update_display()?;
                        }
                        SessionState::ShowDialog => {
                            self.update_display()?;
                        }
                        state @ (SessionState::Complete
                        | SessionState::Exit
                        | SessionState::Skip
                        | SessionState::Failed) => {
                            break state;
                        }
                    }
                }
            }
        };

        self.scoring_engine.finish(); // Record final duration
        Ok((self.calculate_result_with_state(&final_state), final_state))
    }

    fn handle_key(&mut self, key_event: KeyEvent) -> Result<SessionState> {
        // Only process key press events, ignore release/repeat
        if !matches!(key_event.kind, KeyEventKind::Press) {
            return Ok(SessionState::Continue);
        }

        match key_event.code {
            KeyCode::Esc => {
                if self.dialog_shown {
                    // Dialog is shown, Esc closes it
                    self.dialog_shown = false;
                    self.scoring_engine.resume();
                    Ok(SessionState::Continue)
                } else {
                    // No dialog, show Skip/Quit dialog
                    self.dialog_shown = true;
                    self.scoring_engine.pause();
                    Ok(SessionState::ShowDialog)
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                if self.dialog_shown {
                    self.dialog_shown = false;
                    self.scoring_engine.resume();
                    if self.skips_remaining > 0 {
                        self.skips_remaining -= 1;
                        Ok(SessionState::Skip)
                    } else {
                        Ok(SessionState::Continue)
                    }
                } else {
                    // Normal typing - handle as character input with actual character
                    let ch = if key_event.code == KeyCode::Char('S') {
                        'S'
                    } else {
                        's'
                    };
                    self.handle_character_input(ch, key_event)
                }
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                if self.dialog_shown {
                    self.dialog_shown = false;
                    self.scoring_engine.resume();
                    Ok(SessionState::Failed)
                } else {
                    // Normal typing - handle as character input with actual character
                    let ch = if key_event.code == KeyCode::Char('Q') {
                        'Q'
                    } else {
                        'q'
                    };
                    self.handle_character_input(ch, key_event)
                }
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                // Show session summary and exit
                Ok(SessionState::Exit)
            }
            KeyCode::Char(ch) => {
                if self.dialog_shown {
                    // Dialog is shown, any other char closes it
                    self.dialog_shown = false;
                    self.scoring_engine.resume();
                    Ok(SessionState::Continue)
                } else {
                    // Normal typing
                    self.handle_character_input(ch, key_event)
                }
            }
            KeyCode::Tab => {
                // Handle tab character
                if self.current_position < self.challenge_text.len() {
                    let expected_char = self
                        .challenge_text
                        .chars()
                        .nth(self.current_position)
                        .unwrap();
                    let is_correct = expected_char == '\t';

                    // Record keystroke in scoring engine
                    self.scoring_engine
                        .record_keystroke('\t', self.current_position);

                    if is_correct {
                        self.current_mistake_position = None;
                        self.current_position += 1;
                        // Skip over any non-typeable characters (comments, whitespace)
                        self.advance_to_next_typeable_character();
                        if self.current_position >= self.challenge_text.len() {
                            return Ok(SessionState::Complete);
                        }
                    } else {
                        self.mistakes += 1;
                        self.mistake_positions.push(self.current_position);
                        self.current_mistake_position = Some(self.current_position);
                    }
                }
                Ok(SessionState::Continue)
            }
            KeyCode::Enter => {
                // Auto-advance when reaching end of line (after last code character)
                if self.current_position < self.challenge_text.len() {
                    let is_correct = self.is_at_end_of_line_content();

                    // Record keystroke in scoring engine
                    self.scoring_engine
                        .record_keystroke('\n', self.current_position);

                    // Check if we're at a newline or at the end of code content on a line
                    if is_correct {
                        // Skip over the newline and any following whitespace/comments to next typeable character
                        self.current_mistake_position = None;
                        self.advance_to_next_line()?;
                        if self.current_position >= self.challenge_text.len() {
                            return Ok(SessionState::Complete);
                        }
                    } else {
                        self.mistakes += 1;
                        self.mistake_positions.push(self.current_position);
                        self.current_mistake_position = Some(self.current_position);
                    }
                }
                Ok(SessionState::Continue)
            }
            // Handle any other key
            _ => {
                if self.dialog_shown {
                    self.dialog_shown = false;
                    self.scoring_engine.resume();
                }
                Ok(SessionState::Continue)
            }
        }
    }

    fn update_display(&mut self) -> Result<()> {
        self.display.display_challenge_with_info(
            &self.challenge_text,
            self.current_position,
            self.mistakes,
            &self.start_time,
            &self.line_starts,
            &self.comment_ranges,
            self.challenge.as_ref(),
            self.current_mistake_position,
            self.skips_remaining,
            self.dialog_shown,
            &self.scoring_engine,
            &self.repo_info,
        )
    }

    fn is_at_end_of_line_content(&self) -> bool {
        TextProcessor::is_at_end_of_line_content(
            &self.challenge_text,
            self.current_position,
            &self.line_starts,
            &self.comment_ranges,
        )
    }

    fn advance_to_next_line(&mut self) -> Result<()> {
        let chars: Vec<char> = self.challenge_text.chars().collect();

        // Skip current position if it's a newline
        if self.current_position < chars.len() && chars[self.current_position] == '\n' {
            self.current_position += 1;
        }

        // Skip to next typeable character
        self.current_position = TextProcessor::find_first_non_whitespace_or_comment(
            &self.challenge_text,
            self.current_position,
            &self.comment_ranges,
        );

        Ok(())
    }

    fn calculate_result(&self) -> StageResult {
        let was_skipped = self.was_skipped();
        let was_failed = self.was_failed();
        self.scoring_engine
            .calculate_result_with_status(was_skipped, was_failed)
            .unwrap()
    }

    pub fn calculate_result_with_state(&self, state: &SessionState) -> StageResult {
        let was_skipped = matches!(state, SessionState::Skip);
        let was_failed = matches!(state, SessionState::Failed);
        self.scoring_engine
            .calculate_result_with_status(was_skipped, was_failed)
            .unwrap()
    }

    pub fn get_scoring_engine(&self) -> &ScoringEngine {
        &self.scoring_engine
    }

    pub fn get_skips_remaining(&self) -> usize {
        self.skips_remaining
    }

    pub fn set_skips_remaining(&mut self, skips: usize) {
        self.skips_remaining = skips;
    }

    pub fn was_skipped(&self) -> bool {
        // Check if we completed due to a skip (not normal completion)
        self.current_position < self.challenge_chars.len()
    }

    pub fn was_failed(&self) -> bool {
        // This will be set by stage manager when Failed state is returned
        false // For now, we'll handle this in stage manager
    }

    fn handle_character_input(&mut self, ch: char, _key_event: KeyEvent) -> Result<SessionState> {
        if self.current_position < self.challenge_chars.len() {
            let expected_char = self.challenge_chars[self.current_position];
            let is_correct = ch == expected_char;

            // Record keystroke in scoring engine
            self.scoring_engine
                .record_keystroke(ch, self.current_position);

            if is_correct {
                self.current_mistake_position = None;
                self.current_position += 1;
                // Skip over any non-typeable characters (comments, whitespace)
                self.advance_to_next_typeable_character();
                if self.current_position >= self.challenge_chars.len() {
                    return Ok(SessionState::Complete);
                }
            } else {
                self.mistakes += 1;
                self.mistake_positions.push(self.current_position);
                self.current_mistake_position = Some(self.current_position);
            }
        }
        Ok(SessionState::Continue)
    }

    fn advance_to_next_typeable_character(&mut self) {
        while self.current_position < self.challenge_chars.len() {
            // Check if current position should be skipped (comment, whitespace, etc.)
            if TextProcessor::should_skip_character(
                &self.challenge_text,
                self.current_position,
                &self.line_starts,
                &self.comment_ranges,
            ) {
                self.current_position += 1;
            } else {
                // Found a typeable character
                break;
            }
        }
    }
}
