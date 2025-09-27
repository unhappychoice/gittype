use crate::Result;
use crossterm::event::KeyEvent;
use std::io::Stdout;
use std::time::Duration;

/// Screen type identifiers for different application screens
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScreenType {
    Title,
    Loading,
    Typing,
    StageSummary,
    SessionSummary,
    TotalSummary,
    TotalSummaryShare,
    SessionFailure,
    Records,
    Analytics,
    SessionDetail,
    SessionSharing,
    Animation,
    VersionCheck,
    InfoDialog,
    Help,
    DetailsDialog,
    Settings,
    Panic,
}

/// Update strategy defines how and when a screen should be updated and re-rendered
#[derive(Debug, Clone)]
pub enum UpdateStrategy {
    /// Screen only updates when user provides input
    InputOnly,
    /// Screen updates at regular time intervals
    TimeBased(Duration),
    /// Screen combines both input and time-based updates
    Hybrid {
        /// Time interval for automatic updates
        interval: Duration,
        /// Whether input events should trigger immediate updates
        input_priority: bool,
    },
}

/// Screen transition actions that can be returned from input handling
#[derive(Debug, Clone)]
pub enum ScreenTransition {
    /// No transition - stay on current screen
    None,
    /// Push new screen onto the stack
    Push(ScreenType),
    /// Pop current screen from stack
    Pop,
    /// Replace current screen with new screen
    Replace(ScreenType),
    /// Pop screens until reaching the specified screen type
    PopTo(ScreenType),
    /// Exit the application
    Exit,
}

/// The Screen trait defines the interface that all screens must implement
pub trait Screen: Send {
    /// Initialize the screen - called when screen becomes active
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    /// Handle keyboard input events and return appropriate screen transition
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<ScreenTransition>;

    /// Render the screen with access to shared data
    fn render_crossterm_with_data(
        &mut self,
        stdout: &mut Stdout,
        session_result: Option<&crate::domain::models::SessionResult>,
        total_result: Option<&crate::domain::services::scoring::TotalResult>,
    ) -> Result<()>;

    /// Render the screen using ratatui backend (optional)
    fn render_ratatui(&mut self, _frame: &mut ratatui::Frame) -> Result<()> {
        // Default implementation for backward compatibility
        // Individual screens can override this when ratatui support is needed
        Ok(())
    }

    /// Clean up screen resources - called when screen becomes inactive
    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Get the update strategy for this screen
    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    /// Update screen state and return whether a re-render is needed
    fn update(&mut self) -> Result<bool> {
        Ok(false)
    }

    /// Downcast to Any for type-specific access (read-only)
    fn as_any(&self) -> &dyn std::any::Any;

    /// Downcast to Any for type-specific access (mutable)
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
