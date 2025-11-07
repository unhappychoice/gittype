use std::sync::Arc;

use crate::domain::models::SessionAction;
use crate::domain::services::scoring::{StageCalculator, StageInput};
use crate::domain::services::session_manager_service::SessionManagerInterface;
use crate::domain::services::SessionManager;
use crate::presentation::tui::ScreenType;
use crate::Result;

pub struct ScreenTransitionManager;

impl ScreenTransitionManager {
    /// Central reducer for screen transitions based on from->to screen types
    pub fn reduce(
        current_screen: ScreenType,
        to_screen: ScreenType,
        session_manager: &Arc<dyn SessionManagerInterface>,
    ) -> Result<ScreenType> {
        let from_screen = current_screen;

        // Handle transition-specific side effects based on from->to pattern
        // All valid transitions must be explicitly listed
        match (&from_screen, &to_screen) {
            // From Title
            (ScreenType::Title, ScreenType::Typing) => {
                Self::handle_start_game_transition(session_manager)?;
            }
            (ScreenType::Title, ScreenType::Records) => {}
            (ScreenType::Title, ScreenType::Analytics) => {}
            (ScreenType::Title, ScreenType::TotalSummary) => {}
            (ScreenType::Title, ScreenType::VersionCheck) => {}

            // From Typing
            (ScreenType::Typing, ScreenType::StageSummary) => {}
            (ScreenType::Typing, ScreenType::Animation) => {
                // Session completed - handle completion
                Self::handle_session_completion(session_manager)?;
            }
            (ScreenType::Typing, ScreenType::SessionFailure) => {
                Self::handle_game_failure(session_manager)?;
            }
            (ScreenType::Typing, ScreenType::TotalSummary) => {}

            // From StageSummary
            (ScreenType::StageSummary, ScreenType::Typing) => {}
            (ScreenType::StageSummary, ScreenType::Animation) => {
                // Session completed from stage summary
                Self::handle_session_completion(session_manager)?;
            }
            (ScreenType::StageSummary, ScreenType::SessionFailure) => {
                Self::handle_game_failure(session_manager)?;
            }
            (ScreenType::StageSummary, ScreenType::TotalSummary) => {}

            // From Animation
            (ScreenType::Animation, ScreenType::SessionSummary) => {}
            (ScreenType::Animation, ScreenType::TotalSummary) => {}

            // From SessionSummary
            (ScreenType::SessionSummary, ScreenType::Title) => {
                // Reset session when going back to title
                Self::handle_session_reset(session_manager)?;
            }
            (ScreenType::SessionSummary, ScreenType::Records) => {}
            (ScreenType::SessionSummary, ScreenType::Analytics) => {}
            (ScreenType::SessionSummary, ScreenType::SessionSharing) => {}
            (ScreenType::SessionSummary, ScreenType::TotalSummary) => {}
            (ScreenType::SessionSummary, ScreenType::Typing) => {
                // Session retry - reset and start new session
                Self::handle_session_retry(session_manager)?;
            }

            (ScreenType::DetailsDialog, ScreenType::SessionSummary) => {}
            (ScreenType::DetailsDialog, ScreenType::TotalSummary) => {}

            // From Failure
            (ScreenType::SessionFailure, ScreenType::Typing) => {
                Self::handle_retry_transition(session_manager)?;
            }
            (ScreenType::SessionFailure, ScreenType::Title) => {
                // Reset session when going back to title from failure
                Self::handle_session_reset(session_manager)?;
            }
            (ScreenType::SessionFailure, ScreenType::TotalSummary) => {}

            // From Records
            (ScreenType::Records, ScreenType::Title) => {}
            (ScreenType::Records, ScreenType::SessionDetail) => {}
            (ScreenType::Records, ScreenType::TotalSummary) => {}

            // From Analytics
            (ScreenType::Analytics, ScreenType::Title) => {}
            (ScreenType::Analytics, ScreenType::TotalSummary) => {}

            // From SessionDetail
            (ScreenType::SessionDetail, ScreenType::Records) => {}
            (ScreenType::SessionDetail, ScreenType::TotalSummary) => {}

            // From Sharing
            (ScreenType::SessionSharing, ScreenType::SessionSummary) => {}
            (ScreenType::SessionSharing, ScreenType::TotalSummary) => {}

            // From ExitSummary
            (ScreenType::TotalSummary, ScreenType::TotalSummaryShare) => {}

            // From TotalSummaryShare
            (ScreenType::TotalSummaryShare, ScreenType::TotalSummary) => {}

            // From VersionCheck
            (ScreenType::VersionCheck, ScreenType::Title) => {}
            (ScreenType::VersionCheck, ScreenType::TotalSummary) => {}

            // From Settings (Help/Settings use Push/Pop, but in case of direct transitions)
            (ScreenType::Settings, ScreenType::Title) => {}
            (ScreenType::Settings, ScreenType::TotalSummary) => {}

            // From Help (Help/Settings use Push/Pop, but in case of direct transitions)
            (ScreenType::Help, ScreenType::Title) => {}
            (ScreenType::Help, ScreenType::TotalSummary) => {}

            // Loading is handled specially by ScreenManager
            (ScreenType::Loading, _) | (_, ScreenType::Loading) => {}

            // Same screen transition (no-op)
            (from, to) if from == to => {}

            // All other transitions are invalid
            (from, to) => {
                panic!("Invalid screen transition: {:?} -> {:?}", from, to);
            }
        }

        Ok(to_screen)
    }

    fn handle_start_game_transition(
        session_manager: &Arc<dyn SessionManagerInterface>,
    ) -> Result<()> {
        // Start session if not already in progress
        if let Some(sm) = session_manager.as_any().downcast_ref::<SessionManager>() {
            if !sm.is_in_progress() {
                sm.reduce(SessionAction::Start)?;
            }
        }
        Ok(())
    }

    fn handle_retry_transition(session_manager: &Arc<dyn SessionManagerInterface>) -> Result<()> {
        // Reset session state then start new session
        if let Some(sm) = session_manager.as_any().downcast_ref::<SessionManager>() {
            sm.reduce(SessionAction::Reset)?;
            sm.reduce(SessionAction::Start)?;
        }
        Ok(())
    }

    fn handle_game_failure(session_manager: &Arc<dyn SessionManagerInterface>) -> Result<()> {
        if let Some(sm) = session_manager.as_any().downcast_ref::<SessionManager>() {
            let mut tracker_guard = sm.current_stage_tracker.lock().unwrap();
            if let Some(ref mut tracker) = *tracker_guard {
                tracker.record(StageInput::Fail);
                let stage_result = StageCalculator::calculate(tracker);
                drop(tracker_guard);
                sm.reduce(SessionAction::CompleteStage(stage_result))?;
            } else {
                drop(tracker_guard);
            }
            sm.reduce(SessionAction::Abort)?;
        }
        Ok(())
    }

    fn handle_session_completion(session_manager: &Arc<dyn SessionManagerInterface>) -> Result<()> {
        if let Some(sm) = session_manager.as_any().downcast_ref::<SessionManager>() {
            sm.record_and_update_trackers()?;
        }
        Ok(())
    }

    fn handle_session_retry(session_manager: &Arc<dyn SessionManagerInterface>) -> Result<()> {
        // Record completed session, reset state, then start new session
        if let Some(sm) = session_manager.as_any().downcast_ref::<SessionManager>() {
            sm.reset();
            sm.reduce(SessionAction::Start)?;
        }
        Ok(())
    }

    fn handle_session_reset(session_manager: &Arc<dyn SessionManagerInterface>) -> Result<()> {
        if let Some(sm) = session_manager.as_any().downcast_ref::<SessionManager>() {
            sm.reduce(SessionAction::Reset)?;
        }
        Ok(())
    }
}
