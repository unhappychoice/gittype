use crate::domain::events::EventBus;
use crate::domain::models::{Rank, SessionResult, TotalResult};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::views::{
    OptionsView, RankView, ScoreView, SessionSummaryHeaderView, SummaryView,
};
use crate::presentation::game::{
    GameData, Screen, ScreenType, SessionManager, UpdateStrategy,
};
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
    event_bus: EventBus,
}

impl SessionSummaryScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            action_result: None,
            event_bus,
        }
    }

    pub fn get_action_result(&self) -> Option<ResultAction> {
        self.action_result.clone()
    }

    fn show_session_summary(
        &mut self,
        session_result: &SessionResult,
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

        let best_rank = Rank::for_score(session_result.session_score);

        // Get best status using session start records from SessionManager
        let best_status = SessionManager::get_best_status_for_score(session_result.session_score)
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
    ) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key_event.code {
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.event_bus.publish(NavigateTo::Push(ScreenType::DetailsDialog));
                Ok(())
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.action_result = Some(ResultAction::Retry);
                self.event_bus.publish(NavigateTo::Replace(ScreenType::Typing));
                Ok(())
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.action_result = Some(ResultAction::Share);
                self.event_bus.publish(NavigateTo::Push(ScreenType::SessionSharing));
                Ok(())
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.action_result = Some(ResultAction::BackToTitle);
                self.event_bus.publish(NavigateTo::PopTo(ScreenType::Title));
                Ok(())
            }
            KeyCode::Esc => {
                self.action_result = Some(ResultAction::Quit);
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.action_result = Some(ResultAction::Quit);
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut std::io::Stdout,
        session_result: Option<&SessionResult>,
        _total_result: Option<&TotalResult>,
    ) -> Result<()> {
        if let Some(session_result) = session_result {
            // Get git repository from global GameData
            let git_repository = GameData::get_git_repository();
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
