use crate::domain::events::EventBus;
use crate::domain::repositories::challenge_repository::ChallengeRepository;
use crate::domain::repositories::git_repository_repository::GitRepositoryRepository;
use crate::domain::repositories::session_repository::SessionRepository;
use crate::domain::repositories::stage_repository::StageRepository as DomainStageRepository;
use crate::domain::repositories::trending_repository::TrendingRepository;
use crate::domain::repositories::version_repository::VersionRepository;
use crate::domain::services::analytics_service::AnalyticsService;
use crate::domain::services::config_service::ConfigService;
use crate::domain::services::repository_service::RepositoryService;
use crate::domain::services::session_service::SessionService;
use crate::domain::services::version_service::VersionService;
use crate::infrastructure::database::daos::{ChallengeDao, RepositoryDao, SessionDao, StageDao};
use crate::infrastructure::database::database::Database;
use crate::infrastructure::http::github_api_client::GitHubApiClientFactoryImpl;
use crate::infrastructure::http::oss_insight_client::OssInsightClient;
use crate::infrastructure::storage::compressed_file_storage::CompressedFileStorage;
use crate::infrastructure::storage::file_storage::FileStorage;
use crate::infrastructure::terminal::TerminalComponent;
use crate::presentation::tui::screens::{
    AnalyticsScreen, AnimationScreen, HelpScreen, InfoDialogScreen, LoadingScreen, PanicScreen,
    RecordsScreen, RepoListScreen, RepoPlayScreen, SessionDetailScreen, SessionDetailsDialog,
    SessionFailureScreen, SessionSummaryScreen, SessionSummaryShareScreen, SettingsScreen,
    StageSummaryScreen, TitleScreen, TotalSummaryScreen, TotalSummaryShareScreen,
    TrendingLanguageSelectionScreen, TrendingRepositorySelectionScreen, TypingScreen,
    VersionCheckScreen,
};
use crate::presentation::tui::ScreenManagerFactoryImpl;

shaku::module! {
    pub AppModule {
        components = [
            FileStorage,
            CompressedFileStorage,
            OssInsightClient,
            GitHubApiClientFactoryImpl,
            Database,
            ChallengeDao,
            RepositoryDao,
            SessionDao,
            StageDao,
            EventBus,
            TerminalComponent,
            GitRepositoryRepository,
            SessionRepository,
            DomainStageRepository,
            ChallengeRepository,
            TrendingRepository,
            VersionRepository,
            SessionService,
            AnalyticsService,
            RepositoryService,
            VersionService,
            ConfigService,
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
            RepoListScreen,
            RepoPlayScreen,
            SessionDetailScreen,
            SessionSummaryScreen,
            SessionSummaryShareScreen,
            SettingsScreen,
            TotalSummaryScreen,
            TotalSummaryShareScreen,
            TrendingLanguageSelectionScreen,
            TrendingRepositorySelectionScreen,
            VersionCheckScreen
        ],
        providers = []
    }
}
