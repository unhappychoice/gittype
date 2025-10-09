use crate::domain::events::EventBus;
use crate::domain::models::{DifficultyLevel, GitRepository};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::models::ScreenDataProvider;
use crate::presentation::game::views::title::{DifficultySelectionView, StaticElementsView};
use crate::presentation::game::{GameData, Screen, ScreenType, StageRepository, UpdateStrategy};
use crate::{GitTypeError, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::sync::{Arc, Mutex};

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

pub struct TitleScreen {
    selected_difficulty: usize,
    challenge_counts: [usize; 5],
    git_repository: Option<GitRepository>,
    action_result: Option<TitleAction>,
    needs_render: bool,
    error_message: Option<String>,
    event_bus: EventBus,
}

impl TitleScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            selected_difficulty: 1,
            challenge_counts: [0, 0, 0, 0, 0],
            git_repository: None,
            action_result: None,
            needs_render: true,
            error_message: None,
            event_bus,
        }
    }

    pub fn with_challenge_counts(mut self, counts: [usize; 5]) -> Self {
        self.challenge_counts = counts;
        self
    }

    pub fn with_git_repository(mut self, repo: Option<GitRepository>) -> Self {
        self.git_repository = repo;
        self
    }

    pub fn get_action_result(&self) -> Option<&TitleAction> {
        self.action_result.as_ref()
    }

    pub fn get_selected_difficulty(&self) -> DifficultyLevel {
        DIFFICULTIES[self.selected_difficulty].1
    }

    pub fn set_challenge_counts(&mut self, counts: [usize; 5]) {
        self.challenge_counts = counts;
    }

    pub fn set_git_repository(&mut self, repo: Option<GitRepository>) {
        self.git_repository = repo;
    }

    pub fn get_error_message(&self) -> Option<&String> {
        self.error_message.as_ref()
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

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        self.action_result = None;
        self.needs_render = true;

        let screen_data = data.downcast::<TitleScreenData>()?;
        self.challenge_counts = screen_data.challenge_counts;
        self.git_repository = screen_data.git_repository;

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char(' ') => {
                // Check if challenges are available for the selected difficulty
                if self.challenge_counts[self.selected_difficulty] == 0 {
                    self.error_message = Some(
                        "No challenges available for this difficulty. Please try a different difficulty or repository.".to_string()
                    );
                    self.needs_render = true;
                    Ok(())
                } else {
                    self.error_message = None;
                    self.action_result =
                        Some(TitleAction::Start(DIFFICULTIES[self.selected_difficulty].1));
                    self.event_bus
                        .publish(NavigateTo::Replace(ScreenType::Typing));
                    Ok(())
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.selected_difficulty = if self.selected_difficulty == 0 {
                    DIFFICULTIES.len() - 1
                } else {
                    self.selected_difficulty - 1
                };
                self.error_message = None;
                self.needs_render = true;
                Ok(())
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.selected_difficulty = (self.selected_difficulty + 1) % DIFFICULTIES.len();
                self.error_message = None;
                self.needs_render = true;
                Ok(())
            }
            KeyCode::Esc => {
                self.action_result = Some(TitleAction::Quit);
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.action_result = Some(TitleAction::Quit);
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('i') | KeyCode::Char('?') => {
                self.event_bus.publish(NavigateTo::Push(ScreenType::Help));
                Ok(())
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.action_result = Some(TitleAction::Records);
                self.event_bus
                    .publish(NavigateTo::Replace(ScreenType::Records));
                Ok(())
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.action_result = Some(TitleAction::Analytics);
                self.event_bus
                    .publish(NavigateTo::Replace(ScreenType::Analytics));
                Ok(())
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.action_result = Some(TitleAction::Settings);
                self.event_bus
                    .publish(NavigateTo::Push(ScreenType::Settings));
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
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
            self.git_repository.as_ref(),
        );

        // Render difficulty selection
        DifficultySelectionView::render(
            frame,
            chunks[5],
            &DIFFICULTIES,
            self.selected_difficulty,
            &self.challenge_counts,
            self.error_message.as_ref(),
        );

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> Result<bool> {
        let should_render = self.needs_render;
        if self.needs_render {
            self.needs_render = false;
        }
        Ok(should_render)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
