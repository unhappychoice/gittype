use super::super::{
    context_loader::{self, CodeContext},
    typing_core::{InputResult, ProcessingOptions, TypingCore},
};
use crate::domain::events::domain_events::DomainEvent;
use crate::domain::events::EventBus;
use crate::domain::models::{Challenge, Countdown};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::{
    game_data::GameData, views::TypingView, Screen, ScreenDataProvider, UpdateStrategy,
};
use crate::presentation::game::{ScreenType, SessionManager};
use crate::{domain::models::GitRepository, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::sync::{Arc, Mutex};
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
    event_bus: EventBus,
    session_manager: Arc<Mutex<SessionManager>>,
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
    pub fn new(event_bus: EventBus) -> Self {
        Self {
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
            event_bus,
            session_manager: SessionManager::instance(),
        }
    }

    /// Load the current challenge from global SessionManager
    pub fn load_current_challenge(&mut self) -> Result<bool> {
        let challenge = if let Ok(session_manager) = self.session_manager.lock() {
            session_manager.get_current_challenge()?
        } else {
            None
        };

        if let Some(challenge) = challenge {
            let comment_ranges = &challenge.comment_ranges;
            let options = ProcessingOptions {
                preserve_empty_lines: true,
                ..Default::default()
            };

            self.typing_core = TypingCore::new(&challenge.code_content, comment_ranges, options);
            self.code_context = context_loader::load_context_for_challenge(&challenge, 4)?;

            self.countdown = Countdown::new();
            self.challenge = Some(challenge.clone());
            // Update git_repository from GameData
            self.git_repository = if let Ok(game_data) = GameData::instance().lock() {
                game_data.repository()
            } else {
                None
            };
            self.waiting_to_start = true;
            self.dialog_shown = false;

            // Publish ChallengeLoaded event
            self.event_bus.publish(DomainEvent::ChallengeLoaded {
                text: self.typing_core.text_to_type().to_string(),
                source_path: challenge.source_file_path.clone().unwrap_or_default(),
            });

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
        let skips_remaining = if let Ok(session_manager) = self.session_manager.lock() {
            session_manager.get_skips_remaining().unwrap_or(0)
        } else {
            0
        };
        if skips_remaining > 0 {
            Ok(SessionState::Skip)
        } else {
            Ok(SessionState::Continue)
        }
    }

    fn handle_tab_key(&mut self) -> Result<SessionState> {
        // Publish KeyPressed event
        self.event_bus.publish(DomainEvent::KeyPressed {
            key: '\t',
            position: self.typing_core.current_position_to_type(),
        });

        let result = self.typing_core.process_tab_input();
        self.handle_input_result(result)
    }

    fn handle_enter_key(&mut self) -> Result<SessionState> {
        // Publish KeyPressed event
        self.event_bus.publish(DomainEvent::KeyPressed {
            key: '\n',
            position: self.typing_core.current_position_to_type(),
        });

        let result = self.typing_core.process_enter_input();
        self.handle_input_result(result)
    }

    fn handle_character_input(&mut self, ch: char) -> Result<SessionState> {
        // Publish KeyPressed event
        self.event_bus.publish(DomainEvent::KeyPressed {
            key: ch,
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

        // Publish StagePaused event
        self.event_bus.publish(DomainEvent::StagePaused);

        self.countdown.pause();
    }

    fn close_dialog(&mut self) {
        self.dialog_shown = false;

        // Publish StageResumed event
        self.event_bus.publish(DomainEvent::StageResumed);

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
            // Publish StageStarted event with start time
            self.event_bus.publish(DomainEvent::StageStarted {
                start_time: typing_start_time,
            });
        }
    }
}

pub struct TypingScreenDataProvider;

impl ScreenDataProvider for TypingScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

impl Screen for TypingScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::Typing
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(TypingScreenDataProvider)
    }

    fn init_with_data(&mut self, _data: Box<dyn std::any::Any>) -> Result<()> {
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        self.handle_countdown_logic();

        let session_state = self.handle_key(key_event)?;

        match session_state {
            SessionState::Complete => {
                // Publish StageFinalized event
                self.event_bus.publish(DomainEvent::StageFinalized);
                // Publish NavigateTo event
                self.event_bus
                    .publish(NavigateTo::Replace(ScreenType::StageSummary));
                Ok(())
            }
            SessionState::Exit => {
                // Publish NavigateTo event
                self.event_bus.publish(NavigateTo::PopTo(ScreenType::Title));
                Ok(())
            }
            SessionState::Skip => {
                // Publish StageSkipped event
                self.event_bus.publish(DomainEvent::StageSkipped);
                // Publish NavigateTo event
                self.event_bus
                    .publish(NavigateTo::Replace(ScreenType::StageSummary));
                Ok(())
            }
            SessionState::Failed => {
                // Publish NavigateTo event
                self.event_bus
                    .publish(NavigateTo::Replace(ScreenType::SessionFailure));
                Ok(())
            }
            SessionState::ShowDialog => Ok(()),
            _ => Ok(()),
        }
    }

    fn render_ratatui(&mut self, frame: &mut ratatui::Frame) -> Result<()> {
        self.handle_countdown_logic();

        let chars: Vec<char> = self.typing_core.text_to_display().chars().collect();
        let skips_remaining = if let Ok(session_manager) = self.session_manager.lock() {
            session_manager.get_skips_remaining().unwrap_or(0)
        } else {
            0
        };

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
                interval: Duration::from_millis(33), // ~30 FPS for efficient typing display
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
