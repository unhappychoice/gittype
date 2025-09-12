use super::super::{
    context_loader::{self, CodeContext},
    typing_core::{InputResult, ProcessingOptions, TypingCore},
};
use crate::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::game::{game_data::GameData, views::TypingView};
use crate::game::{ScreenType, SessionManager};
use crate::models::{Challenge, Countdown};
use crate::scoring::StageInput;
use crate::{models::GitRepository, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::Stdout;
use std::time::Duration;

pub struct TypingScreen {
    countdown: Countdown,
    git_repository: Option<GitRepository>,
    typing_core: TypingCore,
    challenge: Option<Challenge>,
    code_context: CodeContext,
    waiting_to_start: bool,
    dialog_shown: bool,
    typing_view: TypingView,
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
    pub fn new() -> Result<Self> {
        Ok(Self {
            countdown: Countdown::new(),
            git_repository: GameData::instance()
                .lock()
                .ok()
                .and_then(|game_data| game_data.git_repository.clone()),
            typing_core: TypingCore::default(),
            challenge: None,
            code_context: CodeContext::empty(),
            waiting_to_start: true,
            dialog_shown: false,
            typing_view: TypingView::new(),
        })
    }

    /// Load the current challenge from global SessionManager
    pub fn load_current_challenge(&mut self) -> Result<bool> {
        if let Some(challenge) = SessionManager::get_global_current_challenge()? {
            let comment_ranges = &challenge.comment_ranges;
            let options = ProcessingOptions {
                preserve_empty_lines: true,
                ..Default::default()
            };

            self.countdown = Countdown::new();
            self.typing_core = TypingCore::new(&challenge.code_content, comment_ranges, options);
            self.challenge = Some(challenge.clone());
            self.code_context = context_loader::load_context_for_challenge(&challenge, 4)?;
            // Update git_repository from GameData
            self.git_repository = GameData::get_git_repository();
            self.waiting_to_start = true;
            self.dialog_shown = false;

            // Update stage tracker in SessionManager
            let _ = SessionManager::init_global_stage_tracker(
                self.typing_core.text_to_type().to_string(),
                challenge.source_file_path.clone(),
            );

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn handle_key(&mut self, key_event: KeyEvent) -> Result<SessionState> {
        if !matches!(key_event.kind, KeyEventKind::Press) {
            return Ok(SessionState::Continue);
        }

        match (self.waiting_to_start, self.countdown.is_active()) {
            (true, _) => match key_event.code {
                KeyCode::Char(' ') => {
                    self.waiting_to_start = false;
                    self.countdown.start_countdown();
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
                        let result = self.handle_skip_action()?;
                        match result {
                            SessionState::Skip => Ok(SessionState::Skip),
                            _ => Ok(SessionState::WaitingToStart),
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
            },
            (false, true) => match key_event.code {
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
                        let result = self.handle_skip_action()?;
                        match result {
                            SessionState::Skip => Ok(SessionState::Skip),
                            _ => Ok(SessionState::Countdown),
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
            },
            (false, false) => match key_event.code {
                KeyCode::Esc => {
                    if self.dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Continue)
                    } else {
                        self.open_dialog();
                        Ok(SessionState::ShowDialog)
                    }
                }
                KeyCode::Char('s' | 'S') => {
                    if self.dialog_shown {
                        self.handle_skip_action()
                    } else {
                        let ch = if key_event.code == KeyCode::Char('S') {
                            'S'
                        } else {
                            's'
                        };
                        self.handle_character_input(ch)
                    }
                }
                KeyCode::Char('q' | 'Q') => {
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
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    Ok(SessionState::Exit)
                }
                KeyCode::Char(ch) => {
                    if self.dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Continue)
                    } else {
                        self.handle_character_input(ch)
                    }
                }
                KeyCode::Tab => {
                    if self.dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Continue)
                    } else {
                        self.handle_tab_key()
                    }
                }
                KeyCode::Enter => {
                    if self.dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Continue)
                    } else {
                        self.handle_enter_key()
                    }
                }
                _ => {
                    if self.dialog_shown {
                        self.close_dialog();
                    }
                    Ok(SessionState::Continue)
                }
            },
        }
    }

    fn handle_skip_action(&mut self) -> Result<SessionState> {
        self.close_dialog();
        let skips_remaining = SessionManager::get_global_skips_remaining().unwrap_or(0);
        if skips_remaining > 0 {
            Ok(SessionState::Skip)
        } else {
            Ok(SessionState::Continue)
        }
    }

    fn handle_tab_key(&mut self) -> Result<SessionState> {
        let _ = SessionManager::record_global_stage_input(StageInput::Keystroke {
            ch: '\t',
            position: self.typing_core.current_position_to_type(),
        });
        let result = self.typing_core.process_tab_input();
        self.handle_input_result(result)
    }

    fn handle_enter_key(&mut self) -> Result<SessionState> {
        let _ = SessionManager::record_global_stage_input(StageInput::Keystroke {
            ch: '\n',
            position: self.typing_core.current_position_to_type(),
        });
        let result = self.typing_core.process_enter_input();
        self.handle_input_result(result)
    }

    fn handle_character_input(&mut self, ch: char) -> Result<SessionState> {
        let _ = SessionManager::record_global_stage_input(StageInput::Keystroke {
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
        let _ = SessionManager::record_global_stage_input(StageInput::Pause);

        self.countdown.pause();
    }

    fn close_dialog(&mut self) {
        self.dialog_shown = false;
        let _ = SessionManager::record_global_stage_input(StageInput::Resume);

        self.countdown.resume();
    }

    fn handle_countdown_logic(&mut self) {
        if !self.countdown.is_active() {
            return;
        }

        if self.dialog_shown {
            return;
        }

        // Update countdown and check if typing should start
        if let Some(typing_start_time) = self.countdown.update_state() {
            // Set the start time in SessionManager and record start
            let _ = SessionManager::set_global_stage_start_time(typing_start_time);
            // Record Start event when countdown finishes
            let _ = SessionManager::record_global_stage_input(StageInput::Start);
        }
    }
}

impl Screen for TypingScreen {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<ScreenTransition> {
        self.handle_countdown_logic();

        let session_state = self.handle_key(key_event)?;

        match session_state {
            SessionState::Complete => {
                let _ = SessionManager::finalize_global_stage()?;
                Ok(ScreenTransition::Replace(ScreenType::StageSummary))
            }
            SessionState::Exit => Ok(ScreenTransition::PopTo(ScreenType::Title)),
            SessionState::Skip => {
                let _ = SessionManager::skip_global_stage()?;
                Ok(ScreenTransition::Replace(ScreenType::StageSummary))
            }
            SessionState::Failed => Ok(ScreenTransition::Replace(ScreenType::SessionFailure)),
            SessionState::ShowDialog => Ok(ScreenTransition::None),
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut Stdout,
        _session_result: Option<&crate::models::SessionResult>,
        _total_result: Option<&crate::scoring::TotalResult>,
    ) -> Result<()> {
        Ok(())
    }

    fn render_ratatui(&mut self, frame: &mut ratatui::Frame) -> Result<()> {
        self.handle_countdown_logic();

        let chars: Vec<char> = self.typing_core.text_to_display().chars().collect();
        let skips_remaining = SessionManager::get_global_skips_remaining().unwrap_or(0);

        self.typing_view.render(
            frame,
            self.challenge.as_ref(),
            self.git_repository.as_ref(),
            &self.typing_core,
            &chars,
            &self.code_context,
            self.waiting_to_start,
            self.countdown.get_current_count(),
            skips_remaining,
            self.dialog_shown,
        );

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        if self.countdown.is_active() {
            UpdateStrategy::Hybrid {
                interval: Duration::from_millis(50),
                input_priority: true,
            }
        } else if self.waiting_to_start {
            UpdateStrategy::InputOnly
        } else {
            UpdateStrategy::Hybrid {
                interval: Duration::from_millis(16), // ~60 FPS for smooth typing display
                input_priority: true,
            }
        }
    }

    fn update(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
