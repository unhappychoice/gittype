use crate::Result;
use crossterm::event::KeyEvent;
use std::time::Duration;

/// Trait for screen data providers
pub trait ScreenDataProvider: Send {
    /// Provide data for screen initialization
    fn provide(&self) -> Result<Box<dyn std::any::Any>>;
}

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

// Implement Event trait for ScreenTransition to use as NavigateTo event
impl crate::domain::events::Event for ScreenTransition {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// The Screen trait defines the interface that all screens must implement
pub trait Screen: Send {
    /// Get the type of this screen
    fn get_type(&self) -> ScreenType;

    /// Get the default data provider for this screen type
    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized;

    /// Initialize the screen with data (for screens that need data injection)
    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()>;

    /// Called when this screen is pushed from another screen
    /// Allows the screen to extract data from the source screen
    fn on_pushed_from(&mut self, _source_screen: &dyn Screen) -> Result<()> {
        // Default implementation does nothing
        Ok(())
    }

    /// Handle keyboard input events
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()>;

    /// Render the screen using ratatui
    fn render_ratatui(&mut self, frame: &mut ratatui::Frame) -> Result<()>;

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
