// Game screens
pub mod analytics_screen;
pub mod animation_screen;
pub mod help_screen;
pub mod info_dialog;
pub mod loading_screen;
pub mod panic_screen;
pub mod records_screen;
pub mod session_detail_screen;
pub mod session_details_dialog;
pub mod session_failure_screen;
pub mod session_summary_screen;
pub mod session_summary_share_screen;
pub mod settings_screen;
pub mod stage_summary_screen;
pub mod title_screen;
pub mod total_summary_screen;
pub mod total_summary_share_screen;
pub mod typing_screen;
pub mod version_check_screen;

// CLI screens
pub mod repo_list_screen;
pub mod repo_play_screen;
pub mod trending_language_selection_screen;
pub mod trending_repository_selection_screen;

// Re-exports
pub use analytics_screen::AnalyticsScreen;
pub use animation_screen::AnimationScreen;
pub use help_screen::HelpScreen;
pub use info_dialog::{InfoDialogScreen, InfoDialogScreenDataProvider};
pub use loading_screen::LoadingScreen;
pub use panic_screen::PanicScreen;
pub use records_screen::RecordsScreen;
pub use repo_list_screen::{RepoListScreen, RepoListScreenDataProvider};
pub use repo_play_screen::{RepoPlayScreen, RepoPlayScreenDataProvider};
pub use session_detail_screen::SessionDetailScreen;
pub use session_details_dialog::SessionDetailsDialog;
pub use session_failure_screen::SessionFailureScreen;
pub use session_summary_screen::{ResultAction, SessionSummaryScreen};
pub use session_summary_share_screen::SessionSummaryShareScreen;
pub use settings_screen::SettingsScreen;
pub use stage_summary_screen::StageSummaryScreen;
pub use title_screen::{TitleAction, TitleScreen};
pub use total_summary_screen::TotalSummaryScreen;
pub use total_summary_share_screen::TotalSummaryShareScreen;
pub use trending_language_selection_screen::{
    TrendingLanguageSelectionScreen, TrendingLanguageSelectionScreenDataProvider,
};
pub use trending_repository_selection_screen::{
    TrendingRepositorySelectionScreen, TrendingRepositorySelectionScreenDataProvider,
};
pub use typing_screen::TypingScreen;
pub use version_check_screen::{VersionCheckResult, VersionCheckScreen};
