use crate::domain::events::domain_events::DomainEvent;
use crate::domain::events::EventBusInterface;
use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::models::typing::{CodeContext, InputResult, ProcessingOptions};
use crate::domain::models::{Challenge, Countdown};
use crate::domain::services::context_loader;
use crate::domain::services::typing_core::TypingCore;
use crate::presentation::game::{GameData, SessionManager};
use crate::presentation::tui::views::TypingView;
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::{domain::models::GitRepository, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::sync::RwLock;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub trait TypingScreenInterface: Screen {}

#[derive(Clone)]
pub struct GameDataRef(Arc<Mutex<GameData>>);

impl Default for GameDataRef {
    fn default() -> Self {
        Self(GameData::instance())
    }
}

#[derive(Clone)]
pub struct SessionManagerRef(Arc<Mutex<SessionManager>>);

impl Default for SessionManagerRef {
    fn default() -> Self {
        Self(SessionManager::instance())
    }
}

#[derive(shaku::Component)]
#[shaku(interface = TypingScreenInterface)]
pub struct TypingScreen {
    #[shaku(default)]
    countdown: RwLock<Countdown>,
    #[shaku(default)]
    git_repository: RwLock<Option<GitRepository>>,
    #[shaku(default)]
    typing_core: RwLock<TypingCore>,
    #[shaku(default)]
    challenge: RwLock<Option<Challenge>>,
    #[shaku(default)]
    code_context: RwLock<CodeContext>,
    #[shaku(default)]
    waiting_to_start: RwLock<bool>,
    #[shaku(default)]
    dialog_shown: RwLock<bool>,
    #[shaku(default)]
    typing_view: RwLock<TypingView>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
    #[shaku(default)]
    game_data: GameDataRef,
    #[shaku(default)]
    session_manager: SessionManagerRef,
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
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
        game_data: Arc<Mutex<GameData>>,
        session_manager: Arc<Mutex<SessionManager>>,
    ) -> Self {
        let git_repository = game_data
            .lock()
            .ok()
            .and_then(|data| data.git_repository.clone());

        Self {
            countdown: RwLock::new(Countdown::new()),
            git_repository: RwLock::new(git_repository),
            typing_core: RwLock::new(TypingCore::default()),
            challenge: RwLock::new(None),
            code_context: RwLock::new(CodeContext::empty()),
            waiting_to_start: RwLock::new(true),
            dialog_shown: RwLock::new(false),
            typing_view: RwLock::new(TypingView::new()),
            event_bus,
            theme_service,
            game_data: GameDataRef(game_data),
            session_manager: SessionManagerRef(session_manager),
        }
    }

    pub fn set_waiting_to_start(&self, waiting: bool) {
        *self.waiting_to_start.write().unwrap() = waiting;
    }

    pub fn skip_countdown_for_test(&self) {
        while self.countdown.read().unwrap().is_active() {
            self.countdown
                .write()
                .unwrap()
                .fast_forward_for_test(Duration::from_secs(10));
            self.countdown.write().unwrap().update_state();
        }
    }

    /// Load the current challenge from global SessionManager
    pub fn load_current_challenge(&self) -> Result<bool> {
        let challenge = if let Ok(session_manager) = self.session_manager.0.lock() {
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

            *self.typing_core.write().unwrap() =
                TypingCore::new(&challenge.code_content, comment_ranges, options);
            *self.code_context.write().unwrap() =
                context_loader::load_context_for_challenge(&challenge, 4)?;

            *self.countdown.write().unwrap() = Countdown::new();
            *self.challenge.write().unwrap() = Some(challenge.clone());
            // Update git_repository from GameData
            *self.git_repository.write().unwrap() = if let Ok(game_data) = self.game_data.0.lock() {
                game_data.repository()
            } else {
                None
            };
            *self.waiting_to_start.write().unwrap() = true;
            *self.dialog_shown.write().unwrap() = false;

            // Publish ChallengeLoaded event
            self.event_bus
                .as_event_bus()
                .publish(DomainEvent::ChallengeLoaded {
                    text: self.typing_core.read().unwrap().text_to_type().to_string(),
                    source_path: challenge.source_file_path.clone().unwrap_or_default(),
                });

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn handle_key(&self, key_event: KeyEvent) -> Result<SessionState> {
        if !matches!(key_event.kind, KeyEventKind::Press) {
            return Ok(SessionState::Continue);
        }

        let waiting_to_start = *self.waiting_to_start.read().unwrap();
        let countdown_active = self.countdown.read().unwrap().is_active();
        let dialog_shown = *self.dialog_shown.read().unwrap();

        match (waiting_to_start, countdown_active) {
            (true, _) => match key_event.code {
                KeyCode::Char(' ') => {
                    *self.waiting_to_start.write().unwrap() = false;
                    self.countdown.write().unwrap().start_countdown();
                    Ok(SessionState::Countdown)
                }
                KeyCode::Esc => {
                    if dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::WaitingToStart)
                    } else {
                        self.open_dialog();
                        Ok(SessionState::ShowDialog)
                    }
                }
                KeyCode::Char('s' | 'S') => {
                    if dialog_shown {
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
                    if dialog_shown {
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
                    if dialog_shown {
                        self.close_dialog();
                    }
                    Ok(SessionState::WaitingToStart)
                }
            },
            (false, true) => match key_event.code {
                KeyCode::Esc => {
                    if dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Countdown)
                    } else {
                        self.open_dialog();
                        Ok(SessionState::ShowDialog)
                    }
                }
                KeyCode::Char('s' | 'S') => {
                    if dialog_shown {
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
                    if dialog_shown {
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
                    if dialog_shown {
                        self.close_dialog();
                    }
                    Ok(SessionState::Countdown)
                }
            },
            (false, false) => match key_event.code {
                KeyCode::Esc => {
                    if dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Continue)
                    } else {
                        self.open_dialog();
                        Ok(SessionState::ShowDialog)
                    }
                }
                KeyCode::Char('s' | 'S') => {
                    if dialog_shown {
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
                    if dialog_shown {
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
                    if dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Continue)
                    } else {
                        self.handle_character_input(ch)
                    }
                }
                KeyCode::Tab => {
                    if dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Continue)
                    } else {
                        self.handle_tab_key()
                    }
                }
                KeyCode::Enter => {
                    if dialog_shown {
                        self.close_dialog();
                        Ok(SessionState::Continue)
                    } else {
                        self.handle_enter_key()
                    }
                }
                _ => {
                    if dialog_shown {
                        self.close_dialog();
                    }
                    Ok(SessionState::Continue)
                }
            },
        }
    }

    fn handle_skip_action(&self) -> Result<SessionState> {
        self.close_dialog();
        let skips_remaining = if let Ok(session_manager) = self.session_manager.0.lock() {
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

    fn handle_tab_key(&self) -> Result<SessionState> {
        // Publish KeyPressed event
        self.event_bus
            .as_event_bus()
            .publish(DomainEvent::KeyPressed {
                key: '\t',
                position: self.typing_core.read().unwrap().current_position_to_type(),
            });

        let result = self.typing_core.write().unwrap().process_tab_input();
        self.handle_input_result(result)
    }

    fn handle_enter_key(&self) -> Result<SessionState> {
        // Publish KeyPressed event
        self.event_bus
            .as_event_bus()
            .publish(DomainEvent::KeyPressed {
                key: '\n',
                position: self.typing_core.read().unwrap().current_position_to_type(),
            });

        let result = self.typing_core.write().unwrap().process_enter_input();
        self.handle_input_result(result)
    }

    fn handle_character_input(&self, ch: char) -> Result<SessionState> {
        // Publish KeyPressed event
        self.event_bus
            .as_event_bus()
            .publish(DomainEvent::KeyPressed {
                key: ch,
                position: self.typing_core.read().unwrap().current_position_to_type(),
            });

        let result = self
            .typing_core
            .write()
            .unwrap()
            .process_character_input(ch);
        self.handle_input_result(result)
    }

    fn handle_input_result(&self, result: InputResult) -> Result<SessionState> {
        match result {
            InputResult::Correct => Ok(SessionState::Continue),
            InputResult::Incorrect => Ok(SessionState::Continue),
            InputResult::Completed => Ok(SessionState::Complete),
            InputResult::NoAction => Ok(SessionState::Continue),
        }
    }

    fn open_dialog(&self) {
        *self.dialog_shown.write().unwrap() = true;

        // Publish StagePaused event
        self.event_bus
            .as_event_bus()
            .publish(DomainEvent::StagePaused);

        self.countdown.write().unwrap().pause();
    }

    fn close_dialog(&self) {
        *self.dialog_shown.write().unwrap() = false;

        // Publish StageResumed event
        self.event_bus
            .as_event_bus()
            .publish(DomainEvent::StageResumed);

        self.countdown.write().unwrap().resume();
    }

    fn handle_countdown_logic(&self) {
        if !self.countdown.read().unwrap().is_active() {
            return;
        }

        if *self.dialog_shown.read().unwrap() {
            return;
        }

        // Update countdown and check if typing should start
        if let Some(typing_start_time) = self.countdown.write().unwrap().update_state() {
            // Publish StageStarted event with start time
            self.event_bus
                .as_event_bus()
                .publish(DomainEvent::StageStarted {
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

pub struct TypingScreenProvider;

impl shaku::Provider<crate::presentation::di::AppModule> for TypingScreenProvider {
    type Interface = TypingScreen;

    fn provide(
        module: &crate::presentation::di::AppModule,
    ) -> std::result::Result<Box<Self::Interface>, Box<dyn std::error::Error>> {
        use shaku::HasComponent;
        let event_bus: std::sync::Arc<dyn crate::domain::events::EventBusInterface> =
            module.resolve();
        let theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface> =
            module.resolve();
        let game_data = crate::presentation::game::GameData::instance();
        let session_manager = crate::presentation::game::SessionManager::instance();
        Ok(Box::new(TypingScreen::new(
            event_bus,
            theme_service,
            game_data,
            session_manager,
        )))
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

    fn init_with_data(&self, _data: Box<dyn std::any::Any>) -> Result<()> {
        Ok(())
    }

    fn handle_key_event(&self, key_event: KeyEvent) -> Result<()> {
        self.handle_countdown_logic();

        let session_state = self.handle_key(key_event)?;

        match session_state {
            SessionState::Complete => {
                // Publish StageFinalized event
                self.event_bus
                    .as_event_bus()
                    .publish(DomainEvent::StageFinalized);
                // Publish NavigateTo event
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::StageSummary));
                Ok(())
            }
            SessionState::Exit => {
                // Publish NavigateTo event
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::PopTo(ScreenType::Title));
                Ok(())
            }
            SessionState::Skip => {
                // Publish StageSkipped event
                self.event_bus
                    .as_event_bus()
                    .publish(DomainEvent::StageSkipped);
                // Publish NavigateTo event
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::StageSummary));
                Ok(())
            }
            SessionState::Failed => {
                // Publish NavigateTo event
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::SessionFailure));
                Ok(())
            }
            SessionState::ShowDialog => Ok(()),
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut ratatui::Frame) -> Result<()> {
        let colors = self.theme_service.get_colors();
        self.handle_countdown_logic();

        let chars: Vec<char> = self
            .typing_core
            .read()
            .unwrap()
            .text_to_display()
            .chars()
            .collect();
        let skips_remaining = if let Ok(session_manager) = self.session_manager.0.lock() {
            session_manager.get_skips_remaining().unwrap_or(0)
        } else {
            0
        };

        self.typing_view.write().unwrap().render(
            frame,
            self.challenge.read().unwrap().as_ref(),
            self.git_repository.read().unwrap().as_ref(),
            &self.typing_core.read().unwrap(),
            &chars,
            &self.code_context.read().unwrap(),
            *self.waiting_to_start.read().unwrap(),
            self.countdown.read().unwrap().get_current_count(),
            skips_remaining,
            *self.dialog_shown.read().unwrap(),
            &self.session_manager.0,
            &colors,
        );

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        if self.countdown.read().unwrap().is_active() {
            UpdateStrategy::Hybrid {
                interval: Duration::from_millis(50),
                input_priority: true,
            }
        } else if *self.waiting_to_start.read().unwrap() {
            UpdateStrategy::InputOnly
        } else {
            UpdateStrategy::Hybrid {
                interval: Duration::from_millis(33), // ~30 FPS for efficient typing display
                input_priority: true,
            }
        }
    }

    fn update(&self) -> Result<bool> {
        Ok(true)
    }

    fn cleanup(&self) -> Result<()> {
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TypingScreenInterface for TypingScreen {}
