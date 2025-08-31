pub mod title_screen;
pub mod result_screen;
pub mod countdown_screen;
pub mod typing_screen;
pub mod loading_screen;
pub mod animation_screen;
pub mod exit_summary_screen;
pub mod info_dialog;

pub use title_screen::{TitleScreen, TitleAction};
pub use result_screen::{ResultScreen, ResultAction};
pub use countdown_screen::CountdownScreen;
pub use typing_screen::TypingScreen;
pub use loading_screen::LoadingScreen;
pub use animation_screen::AnimationScreen;
pub use exit_summary_screen::{ExitSummaryScreen, ExitAction};
pub use info_dialog::{InfoDialog, InfoAction};