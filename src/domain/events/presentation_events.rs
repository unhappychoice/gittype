use std::any::Any;

/// Event emitted when user requests to exit the application (Ctrl+C)
#[derive(Clone, Debug)]
pub struct ExitRequested;

impl crate::domain::events::Event for ExitRequested {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Re-export ScreenTransition as NavigateTo event
pub use crate::presentation::tui::ScreenTransition as NavigateTo;
