use std::any::Any;

// Re-export ScreenTransition as NavigateTo event
pub use super::models::screen::ScreenTransition as NavigateTo;

/// Event emitted when user requests to exit the application (Ctrl+C)
#[derive(Clone, Debug)]
pub struct ExitRequested;

impl crate::domain::events::Event for ExitRequested {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
