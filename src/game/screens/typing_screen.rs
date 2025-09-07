use super::super::{
    context_loader::{self, CodeContext},
    stage_renderer::StageRenderer,
    typing_core::{InputResult, ProcessingOptions, TypingCore},
};
use crate::models::Challenge;
use crate::models::StageResult;
use crate::scoring::{StageInput, StageTracker};
use crate::{models::GitRepository, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal,
};

pub struct TypingScreen {
    challenge: Option<Challenge>,
    typing_core: TypingCore,
    start_time: std::time::Instant,
    renderer: StageRenderer,
    stage_tracker: StageTracker,
    skips_remaining: usize,
    dialog_shown: bool,
    repo_info: Option<GitRepository>,
    code_context: CodeContext,
    waiting_to_start: bool,
    countdown_active: bool,
    countdown_number: Option<u8>,
    countdown_start_time: Option<std::time::Instant>,
}

pub enum SessionState {
    Continue,
    Complete,
    Exit,
    Skip,
    Failed,
    ShowDialog,
    WaitingToStart,
    Countdown,
}

impl TypingScreen {
    pub fn new(challenge_text: String, repo_info: Option<GitRepository>) -> Result<Self> {
        let comment_ranges = vec![];
        let options = ProcessingOptions::default();
        Self::create_typing_screen(None, &challenge_text, &comment_ranges, options, repo_info)
    }

    pub fn new_with_challenge(
        challenge: &Challenge,
        repo_info: Option<GitRepository>,
    ) -> Result<Self> {
        let options = ProcessingOptions {
            preserve_empty_lines: true,
            ..Default::default()
        };
        Self::create_typing_screen(
            Some(challenge.clone()),
            &challenge.code_content,
            &challenge.comment_ranges,
            options,
            repo_info,
        )
    }

    fn create_typing_screen(
        challenge: Option<Challenge>,
        code_content: &str,
        comment_ranges: &[(usize, usize)],
        options: ProcessingOptions,
        repo_info: Option<GitRepository>,
    ) -> Result<Self> {
        let typing_core = TypingCore::new(code_content, comment_ranges, options);
        let renderer = StageRenderer::new(typing_core.text_to_display())?;
        let challenge_path = challenge
            .as_ref()
            .and_then(|c| c.source_file_path.clone())
            .unwrap_or_default();
        let stage_tracker =
            StageTracker::new_with_path(typing_core.text_to_type().to_string(), challenge_path);
        // Start event will be recorded when typing actually begins

        // Load context lines if challenge has source file info
        let code_context = if let Some(ref challenge) = challenge {
            context_loader::load_context_for_challenge(challenge, 4)?
        } else {
            CodeContext::empty()
        };

        Ok(Self {
            challenge,
            typing_core,
            start_time: std::time::Instant::now(), // This will be reset when typing actually starts
            renderer,
            stage_tracker,
            skips_remaining: 3,
            dialog_shown: false,
            repo_info,
            code_context,
            waiting_to_start: true,
            countdown_active: false,
            countdown_number: None,
            countdown_start_time: None,
        })
    }

    pub fn start_session(&mut self) -> Result<StageResult> {
        terminal::enable_raw_mode().map_err(|e| {
            crate::error::GitTypeError::TerminalError(format!("Failed to enable raw mode: {}", e))
        })?;

        let result = self.run_session_loop()?;

        self.renderer.cleanup()?;
        crate::game::stage_manager::cleanup_terminal();

        Ok(result)
    }

    pub fn show(&mut self) -> Result<StageResult> {
        self.start_time = std::time::Instant::now();
        self.run_session_loop()
    }

    pub fn show_with_state(&mut self) -> Result<(StageResult, SessionState)> {
        self.start_time = std::time::Instant::now();

        self.update_display()?;

        let final_state = self.event_loop()?;
        self.stage_tracker.record(StageInput::Finish);

        Ok((self.calculate_result_with_state(&final_state), final_state))
    }

    fn run_session_loop(&mut self) -> Result<StageResult> {
        self.update_display()?;
        self.event_loop()?;
        self.stage_tracker.record(StageInput::Finish);
        Ok(self.calculate_result())
    }

    fn event_loop(&mut self) -> Result<SessionState> {
        let mut last_update = std::time::Instant::now();

        loop {
            let poll_timeout = if self.countdown_active {
                // More responsive during countdown for precise timing
                std::time::Duration::from_millis(16) // ~60fps equivalent
            } else {
                // Normal responsiveness during typing
                std::time::Duration::from_millis(33) // ~30fps equivalent
            };

            let should_update_display = if event::poll(poll_timeout)? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key(key_event)? {
                        SessionState::Continue
                        | SessionState::ShowDialog
                        | SessionState::WaitingToStart
                        | SessionState::Countdown => true,
                        state @ (SessionState::Complete
                        | SessionState::Exit
                        | SessionState::Skip
                        | SessionState::Failed) => {
                            return Ok(state);
                        }
                    }
                } else {
                    false
                }
            } else {
                // Update display periodically for timer and countdown
                let update_interval = if self.countdown_active {
                    // Smooth countdown updates
                    std::time::Duration::from_millis(50)
                } else {
                    // Normal typing updates - more frequent for better time display accuracy
                    std::time::Duration::from_millis(100)
                };
                last_update.elapsed() >= update_interval
            };

            // Handle countdown timing
            if self.countdown_active {
                let countdown_finished = self.update_countdown();
                if countdown_finished {
                    // Countdown finished, normal typing can now begin
                    // No need to do anything special, just continue the loop
                }
            }

            if should_update_display {
                self.update_display()?;
                last_update = std::time::Instant::now();
            }
        }
    }

    fn handle_key(&mut self, key_event: KeyEvent) -> Result<SessionState> {
        if !matches!(key_event.kind, KeyEventKind::Press) {
            return Ok(SessionState::Continue);
        }

        // Handle waiting to start state
        if self.waiting_to_start {
            return self.handle_waiting_key(key_event);
        }

        // During countdown, allow ESC for dialog and Ctrl+C for exit
        if self.countdown_active {
            match key_event.code {
                KeyCode::Esc => {
                    if self.dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Countdown)
                    } else {
                        self.open_dialog();
                        Ok(SessionState::ShowDialog)
                    }
                }
                KeyCode::Char('s' | 'S') => {
                    if self.dialog_shown {
                        self.close_dialog();
                        if self.skips_remaining > 0 {
                            self.skips_remaining -= 1;
                            Ok(SessionState::Skip)
                        } else {
                            Ok(SessionState::Countdown)
                        }
                    } else {
                        Ok(SessionState::Countdown)
                    }
                }
                KeyCode::Char('q' | 'Q') => {
                    if self.dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Failed)
                    } else {
                        Ok(SessionState::Countdown)
                    }
                }
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    Ok(SessionState::Exit)
                }
                _ => {
                    if self.dialog_shown {
                        self.close_dialog();
                    }
                    Ok(SessionState::Countdown)
                }
            }
        } else {
            match key_event.code {
                KeyCode::Esc => self.handle_escape_key(),
                KeyCode::Char('s' | 'S') => self.handle_s_key(key_event),
                KeyCode::Char('q' | 'Q') => self.handle_q_key(key_event),
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    Ok(SessionState::Exit)
                }
                KeyCode::Char(ch) => self.handle_char_key(ch),
                KeyCode::Tab => self.handle_tab_key(),
                KeyCode::Enter => self.handle_enter_key(),
                _ => {
                    if self.dialog_shown {
                        self.close_dialog();
                    }
                    Ok(SessionState::Continue)
                }
            }
        }
    }

    fn handle_waiting_key(&mut self, key_event: KeyEvent) -> Result<SessionState> {
        match key_event.code {
            KeyCode::Char(' ') => {
                self.waiting_to_start = false;
                self.countdown_active = true;
                self.countdown_number = Some(3);
                self.countdown_start_time = Some(std::time::Instant::now());
                Ok(SessionState::Countdown)
            }
            KeyCode::Esc => {
                if self.dialog_shown {
                    self.close_dialog();
                    Ok(SessionState::WaitingToStart)
                } else {
                    self.open_dialog();
                    Ok(SessionState::ShowDialog)
                }
            }
            KeyCode::Char('s' | 'S') => {
                if self.dialog_shown {
                    self.close_dialog();
                    if self.skips_remaining > 0 {
                        self.skips_remaining -= 1;
                        Ok(SessionState::Skip)
                    } else {
                        Ok(SessionState::WaitingToStart)
                    }
                } else {
                    Ok(SessionState::WaitingToStart)
                }
            }
            KeyCode::Char('q' | 'Q') => {
                if self.dialog_shown {
                    self.close_dialog();
                    Ok(SessionState::Failed)
                } else {
                    Ok(SessionState::WaitingToStart)
                }
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                Ok(SessionState::Exit)
            }
            _ => {
                if self.dialog_shown {
                    self.close_dialog();
                }
                Ok(SessionState::WaitingToStart)
            }
        }
    }

    fn update_countdown(&mut self) -> bool {
        if let (Some(start_time), Some(current_num)) =
            (self.countdown_start_time, self.countdown_number)
        {
            let elapsed = start_time.elapsed();

            let required_duration = if current_num == 0 {
                // GO! shows for 400ms (shorter duration)
                std::time::Duration::from_millis(400)
            } else {
                // Numbers 3, 2, 1 show for 600ms each
                std::time::Duration::from_millis(600)
            };

            if elapsed >= required_duration {
                if current_num > 1 {
                    // Move to next countdown number
                    self.countdown_number = Some(current_num - 1);
                    self.countdown_start_time = Some(std::time::Instant::now());
                } else if current_num == 1 {
                    // Show "GO!" for a brief moment
                    self.countdown_number = Some(0); // 0 represents "GO!"
                    self.countdown_start_time = Some(std::time::Instant::now());
                } else {
                    // Countdown finished, start typing
                    self.countdown_active = false;
                    self.countdown_number = None;
                    self.countdown_start_time = None;

                    // Use the same timestamp for both to ensure accuracy
                    let now = std::time::Instant::now();
                    self.start_time = now;

                    // Manually set the stage_tracker start time to match exactly
                    self.stage_tracker.set_start_time(now);
                    self.stage_tracker.record(StageInput::Start); // This will not overwrite the start_time we just set
                    return true; // Countdown finished
                }
            }
        }
        false // Countdown still active
    }

    fn handle_escape_key(&mut self) -> Result<SessionState> {
        if self.dialog_shown {
            self.close_dialog();
            Ok(SessionState::Continue)
        } else {
            self.open_dialog();
            Ok(SessionState::ShowDialog)
        }
    }

    fn handle_s_key(&mut self, key_event: KeyEvent) -> Result<SessionState> {
        if self.dialog_shown {
            self.close_dialog();
            if self.skips_remaining > 0 {
                self.skips_remaining -= 1;
                Ok(SessionState::Skip)
            } else {
                Ok(SessionState::Continue)
            }
        } else {
            let ch = if key_event.code == KeyCode::Char('S') {
                'S'
            } else {
                's'
            };
            self.handle_character_input(ch)
        }
    }

    fn handle_q_key(&mut self, key_event: KeyEvent) -> Result<SessionState> {
        if self.dialog_shown {
            self.close_dialog();
            Ok(SessionState::Failed)
        } else {
            let ch = if key_event.code == KeyCode::Char('Q') {
                'Q'
            } else {
                'q'
            };
            self.handle_character_input(ch)
        }
    }

    fn handle_char_key(&mut self, ch: char) -> Result<SessionState> {
        if self.dialog_shown {
            self.close_dialog();
            Ok(SessionState::Continue)
        } else {
            self.handle_character_input(ch)
        }
    }

    fn handle_tab_key(&mut self) -> Result<SessionState> {
        self.stage_tracker.record(StageInput::Keystroke {
            ch: '\t',
            position: self.typing_core.current_position_to_type(),
        });
        let result = self.typing_core.process_tab_input();
        self.handle_input_result(result)
    }

    fn handle_enter_key(&mut self) -> Result<SessionState> {
        self.stage_tracker.record(StageInput::Keystroke {
            ch: '\n',
            position: self.typing_core.current_position_to_type(),
        });
        let result = self.typing_core.process_enter_input();
        self.handle_input_result(result)
    }

    fn handle_character_input(&mut self, ch: char) -> Result<SessionState> {
        self.stage_tracker.record(StageInput::Keystroke {
            ch,
            position: self.typing_core.current_position_to_type(),
        });
        let result = self.typing_core.process_character_input(ch);
        self.handle_input_result(result)
    }

    fn handle_input_result(&mut self, result: InputResult) -> Result<SessionState> {
        match result {
            InputResult::Correct => Ok(SessionState::Continue),
            InputResult::Incorrect => Ok(SessionState::Continue),
            InputResult::Completed => Ok(SessionState::Complete),
            InputResult::NoAction => Ok(SessionState::Continue),
        }
    }

    fn open_dialog(&mut self) {
        self.dialog_shown = true;
        self.stage_tracker.record(StageInput::Pause);
    }

    fn close_dialog(&mut self) {
        self.dialog_shown = false;
        self.stage_tracker.record(StageInput::Resume);
    }

    fn update_display(&mut self) -> Result<()> {
        let display_comment_ranges = self.typing_core.display_comment_ranges();
        self.renderer.display_challenge_with_info(
            self.typing_core.text_to_display(),
            self.typing_core.current_position_to_display(),
            self.typing_core.current_line_to_display(),
            self.typing_core.mistakes(),
            self.challenge.as_ref(),
            self.typing_core.current_mistake_position(),
            self.skips_remaining,
            self.dialog_shown,
            &self.stage_tracker,
            &self.repo_info,
            &display_comment_ranges,
            &self.code_context,
            self.waiting_to_start,
            self.countdown_number,
        )
    }

    fn calculate_result(&self) -> StageResult {
        crate::scoring::StageCalculator::calculate(&self.stage_tracker)
    }

    pub fn calculate_result_with_state(&self, _state: &SessionState) -> StageResult {
        crate::scoring::StageCalculator::calculate(&self.stage_tracker)
    }

    // Public getters for external use
    pub fn get_stage_tracker(&self) -> &StageTracker {
        &self.stage_tracker
    }

    pub fn get_skips_remaining(&self) -> usize {
        self.skips_remaining
    }

    pub fn set_skips_remaining(&mut self, skips: usize) {
        self.skips_remaining = skips;
    }

    pub fn was_skipped(&self) -> bool {
        !self.typing_core.is_completed()
    }
}
