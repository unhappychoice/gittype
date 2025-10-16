pub mod ascii_digits;
pub mod ascii_rank_titles;
pub mod context_loader;
pub mod events;
pub mod game_data;
pub mod models;
pub mod rank_colors;
pub mod rank_messages;
pub mod session_manager;
pub mod stage_repository;
pub mod text_processor;
pub mod typing_core;

pub use game_data::GameData;
pub use models::ScreenDataProvider;
pub use session_manager::{SessionConfig, SessionManager, SessionState};
pub use stage_repository::{GameMode, StageConfig, StageRepository};

// Re-export from tui module for backward compatibility
pub use crate::presentation::tui::{
    Screen, ScreenManager, ScreenTransition, ScreenTransitionManager, ScreenType, UpdateStrategy,
};
pub use crate::presentation::tui::screens::TypingScreen;
pub use crate::presentation::tui::views;
