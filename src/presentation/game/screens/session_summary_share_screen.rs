use crate::presentation::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::presentation::game::views::{
    ShareBackOptionView, SharePlatformOptionsView, SharePreviewView, ShareTitleView,
};
use crate::presentation::sharing::{SharingPlatform, SharingService};
use crate::{domain::models::GitRepository, Result};
use crossterm::terminal::{self};
use std::io::Stdout;

pub struct SessionSummaryShareScreen {
    session_result: Option<crate::domain::models::SessionResult>,
    git_repository: Option<crate::domain::models::GitRepository>,
}

impl Default for SessionSummaryShareScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionSummaryShareScreen {
    pub fn new() -> Self {
        Self {
            session_result: None,
            git_repository: None,
        }
    }

    fn render(
        metrics: &crate::domain::models::SessionResult,
        repo_info: &Option<GitRepository>,
    ) -> Result<()> {
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        let platforms = SharingPlatform::all();
        let start_row = center_row.saturating_sub(2);

        ShareTitleView::render(center_col, center_row)?;
        SharePreviewView::render(metrics, repo_info, center_col, center_row)?;
        SharePlatformOptionsView::render(center_col, start_row)?;
        ShareBackOptionView::render(center_col, start_row + platforms.len() as u16 + 2)?;

        Ok(())
    }
}

impl Screen for SessionSummaryShareScreen {
    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Char('1') => {
                if let Some(ref session_result) = self.session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::X,
                        &self.git_repository,
                    );
                }
                Ok(ScreenTransition::Pop)
            }
            KeyCode::Char('2') => {
                if let Some(ref session_result) = self.session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::Reddit,
                        &self.git_repository,
                    );
                }
                Ok(ScreenTransition::Pop)
            }
            KeyCode::Char('3') => {
                if let Some(ref session_result) = self.session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::LinkedIn,
                        &self.git_repository,
                    );
                }
                Ok(ScreenTransition::Pop)
            }
            KeyCode::Char('4') => {
                if let Some(ref session_result) = self.session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::Facebook,
                        &self.git_repository,
                    );
                }
                Ok(ScreenTransition::Pop)
            }
            KeyCode::Esc => Ok(ScreenTransition::Pop),
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut Stdout,
        session_result: Option<&crate::domain::models::SessionResult>,
        _total_result: Option<&crate::domain::services::scoring::TotalResult>,
    ) -> Result<()> {
        self.session_result = session_result.cloned();
        // Get git repository from global GameData
        let git_repository = crate::presentation::game::game_data::GameData::get_git_repository();
        self.git_repository = git_repository.clone();

        if let Some(session_result) = session_result {
            Self::render(session_result, &git_repository)?;
        }
        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
