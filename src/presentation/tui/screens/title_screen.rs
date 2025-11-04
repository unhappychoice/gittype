use crate::domain::events::EventBusInterface;
use crate::domain::models::{DifficultyLevel, GitRepository};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::{GameData, StageRepository};
use crate::presentation::tui::views::title::{DifficultySelectionView, StaticElementsView};
use crate::presentation::tui::ScreenDataProvider;
use crate::presentation::tui::{Screen, ScreenType, UpdateStrategy};
use crate::{GitTypeError, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::sync::{Arc, Mutex, RwLock};

const DIFFICULTIES: [(&str, DifficultyLevel); 5] = [
    ("Easy", DifficultyLevel::Easy),
    ("Normal", DifficultyLevel::Normal),
    ("Hard", DifficultyLevel::Hard),
    ("Wild", DifficultyLevel::Wild),
    ("Zen", DifficultyLevel::Zen),
];

pub struct TitleScreenData {
    pub challenge_counts: [usize; 5],
    pub git_repository: Option<GitRepository>,
}

pub struct TitleScreenDataProvider {
    stage_repository: Arc<Mutex<StageRepository>>,
    game_data: Arc<Mutex<GameData>>,
}

impl ScreenDataProvider for TitleScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let challenge_counts = self
            .stage_repository
            .lock()
            .map_err(|e| {
                GitTypeError::TerminalError(format!("Failed to lock StageRepository: {}", e))
            })?
            .count_challenges_by_difficulty();

        let git_repository = self
            .game_data
            .lock()
            .map_err(|e| GitTypeError::TerminalError(format!("Failed to lock GameData: {}", e)))?
            .repository();

        Ok(Box::new(TitleScreenData {
            challenge_counts,
            git_repository,
        }))
    }
}

#[derive(Clone, Debug)]
pub enum TitleAction {
    Start(DifficultyLevel),
    Records,
    Analytics,
    Settings,
    Quit,
}

pub trait TitleScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = TitleScreenInterface)]
pub struct TitleScreen {
    #[shaku(default)]
    selected_difficulty: RwLock<usize>,
    #[shaku(default)]
    challenge_counts: RwLock<[usize; 5]>,
    #[shaku(default)]
    git_repository: RwLock<Option<GitRepository>>,
    #[shaku(default)]
    action_result: RwLock<Option<TitleAction>>,
    #[shaku(default)]
    needs_render: RwLock<bool>,
    #[shaku(default)]
    error_message: RwLock<Option<String>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
}

impl TitleScreen {
    pub fn new(event_bus: Arc<dyn EventBusInterface>) -> Self {
        Self {
            selected_difficulty: RwLock::new(1),
            challenge_counts: RwLock::new([0, 0, 0, 0, 0]),
            git_repository: RwLock::new(None),
            action_result: RwLock::new(None),
            needs_render: RwLock::new(true),
            error_message: RwLock::new(None),
            event_bus,
        }
    }

    pub fn with_challenge_counts(self, counts: [usize; 5]) -> Self {
        *self.challenge_counts.write().unwrap() = counts;
        self
    }

    pub fn with_git_repository(self, repo: Option<GitRepository>) -> Self {
        *self.git_repository.write().unwrap() = repo;
        self
    }

    pub fn get_action_result(&self) -> Option<TitleAction> {
        self.action_result.read().unwrap().clone()
    }

    pub fn get_selected_difficulty(&self) -> DifficultyLevel {
        DIFFICULTIES[*self.selected_difficulty.read().unwrap()].1
    }

    pub fn set_challenge_counts(&self, counts: [usize; 5]) {
        *self.challenge_counts.write().unwrap() = counts;
    }

    pub fn set_git_repository(&self, repo: Option<GitRepository>) {
        *self.git_repository.write().unwrap() = repo;
    }

    pub fn get_error_message(&self) -> Option<String> {
        self.error_message.read().unwrap().clone()
    }
}

impl Screen for TitleScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::Title
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(TitleScreenDataProvider {
            stage_repository: StageRepository::instance(),
            game_data: GameData::instance(),
        })
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        *self.action_result.write().unwrap() = None;
        *self.needs_render.write().unwrap() = true;

        let screen_data = data.downcast::<TitleScreenData>()?;
        *self.challenge_counts.write().unwrap() = screen_data.challenge_counts;
        *self.git_repository.write().unwrap() = screen_data.git_repository;

        Ok(())
    }

    fn handle_key_event(&self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char(' ') => {
                // Check if challenges are available for the selected difficulty
                let selected_difficulty = *self.selected_difficulty.read().unwrap();
                if self.challenge_counts.read().unwrap()[selected_difficulty] == 0 {
                    *self.error_message.write().unwrap() = Some(
                        "No challenges available for this difficulty. Please try a different difficulty or repository.".to_string()
                    );
                    *self.needs_render.write().unwrap() = true;
                    Ok(())
                } else {
                    *self.error_message.write().unwrap() = None;
                    *self.action_result.write().unwrap() =
                        Some(TitleAction::Start(DIFFICULTIES[selected_difficulty].1));
                    let event_bus = self.event_bus.as_event_bus();
                    log::info!(
                        "TitleScreen: EventBus subscribers address: {:p}",
                        event_bus.get_subscribers_ptr()
                    );
                    log::info!(
                        "TitleScreen: Publishing NavigateTo::Replace(ScreenType::Typing) event"
                    );
                    event_bus.publish(NavigateTo::Replace(ScreenType::Typing));
                    log::info!("TitleScreen: NavigateTo event published");
                    Ok(())
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                let current = *self.selected_difficulty.read().unwrap();
                *self.selected_difficulty.write().unwrap() = if current == 0 {
                    DIFFICULTIES.len() - 1
                } else {
                    current - 1
                };
                *self.error_message.write().unwrap() = None;
                *self.needs_render.write().unwrap() = true;
                Ok(())
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let current = *self.selected_difficulty.read().unwrap();
                *self.selected_difficulty.write().unwrap() = (current + 1) % DIFFICULTIES.len();
                *self.error_message.write().unwrap() = None;
                *self.needs_render.write().unwrap() = true;
                Ok(())
            }
            KeyCode::Esc => {
                *self.action_result.write().unwrap() = Some(TitleAction::Quit);
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                *self.action_result.write().unwrap() = Some(TitleAction::Quit);
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('i') | KeyCode::Char('?') => {
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Push(ScreenType::Help));
                Ok(())
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                *self.action_result.write().unwrap() = Some(TitleAction::Records);
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::Records));
                Ok(())
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                *self.action_result.write().unwrap() = Some(TitleAction::Analytics);
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::Analytics));
                Ok(())
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                *self.action_result.write().unwrap() = Some(TitleAction::Settings);
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Push(ScreenType::Settings));
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut Frame) -> Result<()> {
        let area = frame.area();

        // Calculate content layout
        let logo_height = 6;
        let subtitle_height = 1;
        let instructions_height = 3;
        let difficulty_height = 4;
        let spacing = 1;
        let git_info_height = 1;

        let total_content_height = logo_height
            + spacing
            + subtitle_height
            + spacing
            + difficulty_height
            + spacing
            + instructions_height
            + spacing
            + git_info_height;

        let top_padding = (area.height.saturating_sub(total_content_height as u16)) / 2;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(top_padding),
                Constraint::Length(logo_height as u16), // Logo
                Constraint::Length(spacing as u16),     // Spacing
                Constraint::Length(subtitle_height as u16), // Subtitle
                Constraint::Length(spacing as u16),     // Spacing
                Constraint::Length(difficulty_height as u16), // Difficulty selection
                Constraint::Length(spacing as u16),     // Spacing
                Constraint::Length(instructions_height as u16), // Instructions
                Constraint::Min(0),                     // Bottom (includes git info)
            ])
            .split(area);

        // Render static elements (logo, subtitle, instructions, git info)
        StaticElementsView::render(
            frame,
            chunks[1], // logo
            chunks[3], // subtitle
            chunks[7], // instructions
            self.git_repository.read().unwrap().as_ref(),
        );

        // Render difficulty selection
        DifficultySelectionView::render(
            frame,
            chunks[5],
            &DIFFICULTIES,
            *self.selected_difficulty.read().unwrap(),
            &self.challenge_counts.read().unwrap(),
            self.error_message.read().unwrap().as_ref(),
        );

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&self) -> Result<bool> {
        let should_render = *self.needs_render.read().unwrap();
        if should_render {
            *self.needs_render.write().unwrap() = false;
        }
        Ok(should_render)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TitleScreenInterface for TitleScreen {}
