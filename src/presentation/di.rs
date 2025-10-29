use crate::domain::events::EventBus;
use crate::domain::repositories::challenge_repository::ChallengeRepository;
use crate::domain::repositories::session_repository::SessionRepository;
use crate::domain::repositories::stage_repository::StageRepository as DomainStageRepository;
use crate::domain::repositories::trending_repository::TrendingRepository;
use crate::domain::services::session_service::SessionService;
use crate::infrastructure::http::oss_insight_client::OssInsightClient;
use crate::infrastructure::storage::compressed_file_storage::CompressedFileStorage;
use crate::infrastructure::storage::file_storage::FileStorage;
use crate::infrastructure::terminal::TerminalComponent;
use crate::presentation::tui::screens::{
    AnalyticsScreen, AnimationScreen, HelpScreen, InfoDialogScreen, LoadingScreen, PanicScreen,
    RecordsScreen, SessionDetailScreen, SessionDetailsDialog, SessionFailureScreen,
    SessionSummaryScreen, SessionSummaryShareScreen, SettingsScreen, StageSummaryScreen,
    TitleScreen, TotalSummaryScreen, TotalSummaryShareScreen, TypingScreen, VersionCheckScreen,
};
use crate::presentation::tui::ScreenManagerFactoryImpl;

shaku::module! {
    pub AppModule {
        components = [
            FileStorage,
            CompressedFileStorage,
            OssInsightClient,
            EventBus,
            TerminalComponent,
            SessionRepository,
            DomainStageRepository,
            ChallengeRepository,
            TrendingRepository,
            SessionService,
            ScreenManagerFactoryImpl,
            TitleScreen,
            TypingScreen,
            AnimationScreen,
            HelpScreen,
            LoadingScreen,
            PanicScreen,
            SessionFailureScreen,
            InfoDialogScreen,
            SessionDetailsDialog,
            StageSummaryScreen,
            AnalyticsScreen,
            RecordsScreen,
            SessionDetailScreen,
            SessionSummaryScreen,
            SessionSummaryShareScreen,
            SettingsScreen,
            TotalSummaryScreen,
            TotalSummaryShareScreen,
            VersionCheckScreen
        ],
        providers = []
    }
}
