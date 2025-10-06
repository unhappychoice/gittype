use crate::domain::events::EventBus;
use crate::domain::models::{DifficultyLevel, GitRepository, SessionResult, TotalResult};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::views::title::{DifficultySelectionView, StaticElementsView};
use crate::presentation::game::{
    GameData, Screen, ScreenType, StageRepository, UpdateStrategy,
};
use crate::Result;
use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    terminal::{self},
};
use std::io::Stdout;

const DIFFICULTIES: [(&str, DifficultyLevel); 5] = [
    ("Easy", DifficultyLevel::Easy),
    ("Normal", DifficultyLevel::Normal),
    ("Hard", DifficultyLevel::Hard),
    ("Wild", DifficultyLevel::Wild),
    ("Zen", DifficultyLevel::Zen),
];

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
    fn init(&mut self) -> Result<()> {
        self.action_result = None;
        self.needs_render = true;

        // Update challenge counts from StageRepository when initializing
        if let Ok(stage_repo) = StageRepository::instance().lock() {
            let challenge_counts = stage_repo.count_challenges_by_difficulty();
            self.challenge_counts = challenge_counts;
        }

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
                    self.event_bus.publish(NavigateTo::Replace(ScreenType::Typing));
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
                self.event_bus.publish(NavigateTo::Replace(ScreenType::Records));
                Ok(())
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.action_result = Some(TitleAction::Analytics);
                self.event_bus.publish(NavigateTo::Replace(ScreenType::Analytics));
                Ok(())
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.action_result = Some(TitleAction::Settings);
                self.event_bus.publish(NavigateTo::Push(ScreenType::Settings));
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        stdout: &mut Stdout,
        _session_result: Option<&SessionResult>,
        _total_result: Option<&TotalResult>,
    ) -> Result<()> {
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Get git repository from global GameData or use local one as fallback
        let binding = GameData::get_git_repository();
        let git_repo_to_use = binding.as_ref().or(self.git_repository.as_ref());
        let difficulties_array = &DIFFICULTIES;

        StaticElementsView::draw(stdout, center_row, center_col, git_repo_to_use)?;

        DifficultySelectionView::draw(
            stdout,
            center_row,
            center_col,
            difficulties_array,
            self.selected_difficulty,
            &self.challenge_counts,
            self.error_message.as_ref(),
        )?;

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
