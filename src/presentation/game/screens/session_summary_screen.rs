use crate::presentation::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::presentation::game::views::{OptionsView, RankView, ScoreView, SessionSummaryHeaderView, SummaryView};
use crate::presentation::game::ScreenType;
use crate::{domain::models::GitRepository, Result};
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::ResetColor,
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

#[derive(Debug, Clone)]
pub enum ResultAction {
    Restart,
    BackToTitle,
    Quit,
    Retry,
    Share,
}

pub struct SessionSummaryScreen {
    action_result: Option<ResultAction>,
}

impl Default for SessionSummaryScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionSummaryScreen {
    pub fn new() -> Self {
        Self {
            action_result: None,
        }
    }

    pub fn get_action_result(&self) -> Option<ResultAction> {
        self.action_result.clone()
    }

    fn show_session_summary(
        &mut self,
        session_result: &crate::domain::models::SessionResult,
        _repo_info: &Option<GitRepository>,
    ) -> Result<()> {
        let mut stdout = stdout();

        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, Hide)?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(10));

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        let best_rank = crate::domain::services::scoring::Rank::for_score(session_result.session_score);

        // Get best status using session start records from SessionManager
        let best_status = crate::presentation::game::session_manager::SessionManager::get_best_status_for_score(
            session_result.session_score,
        )
        .ok()
        .flatten();

        let total_content_height = 4 + 5 + 1 + 3 + 1 + 4 + 2 + 2;
        let rank_start_row = if total_content_height < terminal_height {
            center_row.saturating_sub(total_content_height / 2)
        } else {
            3
        };

        SessionSummaryHeaderView::render(center_col, rank_start_row)?;

        let rank_height = RankView::render(
            best_rank.clone(),
            session_result.session_score,
            center_col,
            rank_start_row,
        )?;

        let score_label_row = rank_start_row + rank_height + 4;
        let summary_start_row = ScoreView::render(
            session_result,
            best_rank,
            center_col,
            score_label_row,
            best_status.as_ref(),
        )?;

        SummaryView::render(session_result, center_col, summary_start_row)?;

        let options_start = summary_start_row + 2 + 2;
        OptionsView::render(center_col, options_start)?;

        Ok(())
    }
}

impl Screen for SessionSummaryScreen {
    fn init(&mut self) -> Result<()> {
        self.action_result = None;
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key_event.code {
            KeyCode::Char('d') | KeyCode::Char('D') => {
                Ok(ScreenTransition::Push(ScreenType::DetailsDialog))
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.action_result = Some(ResultAction::Retry);
                Ok(ScreenTransition::Replace(ScreenType::Typing))
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                // Show sharing screen
                self.action_result = Some(ResultAction::Share);
                Ok(ScreenTransition::Push(ScreenType::SessionSharing))
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.action_result = Some(ResultAction::BackToTitle);
                Ok(ScreenTransition::PopTo(ScreenType::Title))
            }
            KeyCode::Esc => {
                self.action_result = Some(ResultAction::Quit);
                Ok(ScreenTransition::Exit)
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.action_result = Some(ResultAction::Quit);
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut std::io::Stdout,
        session_result: Option<&crate::domain::models::SessionResult>,
        _total_result: Option<&crate::domain::services::scoring::TotalResult>,
    ) -> Result<()> {
        if let Some(session_result) = session_result {
            // Get git repository from global GameData
            let git_repository = crate::presentation::game::game_data::GameData::get_git_repository();
            let _ = self.show_session_summary(session_result, &git_repository);
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
