use super::{
    super::{
        stage_renderer::StageRenderer,
        typing_core::{InputResult, ProcessingOptions, TypingCore},
    },
    CountdownScreen,
};
use crate::models::Challenge;
use crate::models::StageResult;
use crate::scoring::engine::ScoringEngine;
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
    scoring_engine: ScoringEngine,
    skips_remaining: usize,
    dialog_shown: bool,
    repo_info: Option<GitRepository>,
}

pub enum SessionState {
    Continue,
    Complete,
    Exit,
    Skip,
    Failed,
    ShowDialog,
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
        let mut scoring_engine = ScoringEngine::new(typing_core.text_to_type().to_string());
        scoring_engine.start();

        Ok(Self {
            challenge,
            typing_core,
            start_time: std::time::Instant::now(),
            renderer,
            scoring_engine,
            skips_remaining: 3,
            dialog_shown: false,
            repo_info,
        })
    }

    pub fn start_session(&mut self) -> Result<StageResult> {
        terminal::enable_raw_mode().map_err(|e| {
            crate::error::GitTypeError::TerminalError(format!("Failed to enable raw mode: {}", e))
        })?;

        CountdownScreen::show_with_challenge_and_repo(self.challenge.as_ref(), &self.repo_info)?;
        self.start_time = std::time::Instant::now();

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
        self.scoring_engine.finish();
        
        Ok((self.calculate_result_with_state(&final_state), final_state))
    }

    fn run_session_loop(&mut self) -> Result<StageResult> {
        self.update_display()?;
        self.event_loop()?;
        self.scoring_engine.finish();
        Ok(self.calculate_result())
    }

    fn event_loop(&mut self) -> Result<SessionState> {
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match self.handle_key(key_event)? {
                        SessionState::Continue | SessionState::ShowDialog => {
                            self.update_display()?;
                        }
                        state @ (SessionState::Complete
                        | SessionState::Exit
                        | SessionState::Skip
                        | SessionState::Failed) => {
                            return Ok(state);
                        }
                    }
                }
            } else {
                self.update_display()?;
            }
        }
    }

    fn handle_key(&mut self, key_event: KeyEvent) -> Result<SessionState> {
        if !matches!(key_event.kind, KeyEventKind::Press) {
            return Ok(SessionState::Continue);
        }

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
            let ch = if key_event.code == KeyCode::Char('S') { 'S' } else { 's' };
            self.handle_character_input(ch)
        }
    }

    fn handle_q_key(&mut self, key_event: KeyEvent) -> Result<SessionState> {
        if self.dialog_shown {
            self.close_dialog();
            Ok(SessionState::Failed)
        } else {
            let ch = if key_event.code == KeyCode::Char('Q') { 'Q' } else { 'q' };
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
        let result = self.typing_core.process_tab_input();
        self.scoring_engine.record_keystroke('\t', self.typing_core.current_position_to_type());
        self.handle_input_result(result)
    }

    fn handle_enter_key(&mut self) -> Result<SessionState> {
        let result = self.typing_core.process_enter_input();
        self.scoring_engine.record_keystroke('\n', self.typing_core.current_position_to_type());
        self.handle_input_result(result)
    }

    fn handle_character_input(&mut self, ch: char) -> Result<SessionState> {
        let result = self.typing_core.process_character_input(ch);
        self.scoring_engine.record_keystroke(ch, self.typing_core.current_position_to_type());
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
        self.scoring_engine.pause();
    }

    fn close_dialog(&mut self) {
        self.dialog_shown = false;
        self.scoring_engine.resume();
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
            &self.scoring_engine,
            &self.repo_info,
            &display_comment_ranges,
        )
    }

    fn calculate_result(&self) -> StageResult {
        let was_skipped = !self.typing_core.is_completed();
        self.scoring_engine
            .calculate_result_with_status(was_skipped, false)
            .unwrap()
    }

    pub fn calculate_result_with_state(&self, state: &SessionState) -> StageResult {
        let was_skipped = matches!(state, SessionState::Skip);
        let was_failed = matches!(state, SessionState::Failed);
        self.scoring_engine
            .calculate_result_with_status(was_skipped, was_failed)
            .unwrap()
    }

    // Public getters for external use
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
        !self.typing_core.is_completed()
    }
}