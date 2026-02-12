#[cfg(test)]
mod tests {
    use gittype::domain::events::EventBus;
    use gittype::domain::services::scoring::{
        SessionTracker, SessionTrackerInterface, TotalTracker, TotalTrackerInterface,
    };
    use gittype::domain::services::session_manager_service::SessionManagerInterface;
    use gittype::domain::services::stage_builder_service::StageRepositoryInterface;
    use gittype::domain::services::{stage_builder_service::StageRepository, SessionManager};
    use gittype::domain::stores::{
        ChallengeStore, ChallengeStoreInterface, RepositoryStore, RepositoryStoreInterface,
        SessionStore, SessionStoreInterface,
    };
    use gittype::presentation::tui::ScreenTransitionManager;
    use gittype::presentation::tui::ScreenType;
    use std::sync::Arc;

    fn create_session_manager() -> Arc<dyn SessionManagerInterface> {
        let event_bus = Arc::new(EventBus::new());
        let challenge_store =
            Arc::new(ChallengeStore::new_for_test()) as Arc<dyn ChallengeStoreInterface>;
        let repository_store =
            Arc::new(RepositoryStore::new_for_test()) as Arc<dyn RepositoryStoreInterface>;
        let session_store =
            Arc::new(SessionStore::new_for_test()) as Arc<dyn SessionStoreInterface>;

        let stage_repository = Arc::new(StageRepository::new(
            None,
            challenge_store,
            repository_store,
            session_store,
        )) as Arc<dyn StageRepositoryInterface>;

        let session_tracker: Arc<dyn SessionTrackerInterface> = Arc::new(SessionTracker::default());
        let total_tracker: Arc<dyn TotalTrackerInterface> = Arc::new(TotalTracker::default());

        Arc::new(SessionManager::new_with_dependencies(
            event_bus,
            stage_repository,
            session_tracker,
            total_tracker,
        ))
    }

    // === Title transitions ===

    #[test]
    fn test_title_to_records() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Title, ScreenType::Records, &sm).unwrap();
        assert_eq!(result, ScreenType::Records);
    }

    #[test]
    fn test_title_to_analytics() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Title, ScreenType::Analytics, &sm).unwrap();
        assert_eq!(result, ScreenType::Analytics);
    }

    #[test]
    fn test_title_to_total_summary() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Title, ScreenType::TotalSummary, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    #[test]
    fn test_title_to_version_check() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Title, ScreenType::VersionCheck, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::VersionCheck);
    }

    // === Typing transitions ===

    #[test]
    fn test_typing_to_stage_summary() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Typing, ScreenType::StageSummary, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::StageSummary);
    }

    #[test]
    fn test_typing_to_total_summary() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Typing, ScreenType::TotalSummary, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === StageSummary transitions ===

    #[test]
    fn test_stage_summary_to_typing() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::StageSummary, ScreenType::Typing, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::Typing);
    }

    #[test]
    fn test_stage_summary_to_total_summary() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::StageSummary,
            ScreenType::TotalSummary,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === Animation transitions ===

    #[test]
    fn test_animation_to_session_summary() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Animation, ScreenType::SessionSummary, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::SessionSummary);
    }

    #[test]
    fn test_animation_to_total_summary() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Animation, ScreenType::TotalSummary, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === SessionSummary transitions ===

    #[test]
    fn test_session_summary_to_title() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::SessionSummary, ScreenType::Title, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::Title);
    }

    #[test]
    fn test_session_summary_to_records() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::SessionSummary, ScreenType::Records, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::Records);
    }

    #[test]
    fn test_session_summary_to_analytics() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::SessionSummary, ScreenType::Analytics, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::Analytics);
    }

    #[test]
    fn test_session_summary_to_session_sharing() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::SessionSummary,
            ScreenType::SessionSharing,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::SessionSharing);
    }

    #[test]
    fn test_session_summary_to_total_summary() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::SessionSummary,
            ScreenType::TotalSummary,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === DetailsDialog transitions ===

    #[test]
    fn test_details_dialog_to_session_summary() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::DetailsDialog,
            ScreenType::SessionSummary,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::SessionSummary);
    }

    #[test]
    fn test_details_dialog_to_total_summary() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::DetailsDialog,
            ScreenType::TotalSummary,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === SessionFailure transitions ===

    #[test]
    fn test_session_failure_to_title() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::SessionFailure, ScreenType::Title, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::Title);
    }

    #[test]
    fn test_session_failure_to_total_summary() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::SessionFailure,
            ScreenType::TotalSummary,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === Records transitions ===

    #[test]
    fn test_records_to_title() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Records, ScreenType::Title, &sm).unwrap();
        assert_eq!(result, ScreenType::Title);
    }

    #[test]
    fn test_records_to_session_detail() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Records, ScreenType::SessionDetail, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::SessionDetail);
    }

    #[test]
    fn test_records_to_total_summary() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Records, ScreenType::TotalSummary, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === Analytics transitions ===

    #[test]
    fn test_analytics_to_title() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Analytics, ScreenType::Title, &sm).unwrap();
        assert_eq!(result, ScreenType::Title);
    }

    #[test]
    fn test_analytics_to_total_summary() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Analytics, ScreenType::TotalSummary, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === SessionDetail transitions ===

    #[test]
    fn test_session_detail_to_records() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::SessionDetail, ScreenType::Records, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::Records);
    }

    #[test]
    fn test_session_detail_to_total_summary() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::SessionDetail,
            ScreenType::TotalSummary,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === Sharing transitions ===

    #[test]
    fn test_session_sharing_to_session_summary() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::SessionSharing,
            ScreenType::SessionSummary,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::SessionSummary);
    }

    #[test]
    fn test_session_sharing_to_total_summary() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::SessionSharing,
            ScreenType::TotalSummary,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === TotalSummary transitions ===

    #[test]
    fn test_total_summary_to_total_summary_share() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::TotalSummary,
            ScreenType::TotalSummaryShare,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::TotalSummaryShare);
    }

    #[test]
    fn test_total_summary_share_to_total_summary() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::TotalSummaryShare,
            ScreenType::TotalSummary,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === VersionCheck transitions ===

    #[test]
    fn test_version_check_to_title() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::VersionCheck, ScreenType::Title, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::Title);
    }

    #[test]
    fn test_version_check_to_total_summary() {
        let sm = create_session_manager();
        let result = ScreenTransitionManager::reduce(
            ScreenType::VersionCheck,
            ScreenType::TotalSummary,
            &sm,
        )
        .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === Settings transitions ===

    #[test]
    fn test_settings_to_title() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Settings, ScreenType::Title, &sm).unwrap();
        assert_eq!(result, ScreenType::Title);
    }

    #[test]
    fn test_settings_to_total_summary() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Settings, ScreenType::TotalSummary, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === Help transitions ===

    #[test]
    fn test_help_to_title() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Help, ScreenType::Title, &sm).unwrap();
        assert_eq!(result, ScreenType::Title);
    }

    #[test]
    fn test_help_to_total_summary() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Help, ScreenType::TotalSummary, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::TotalSummary);
    }

    // === Loading transitions ===

    #[test]
    fn test_loading_to_typing() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Loading, ScreenType::Typing, &sm).unwrap();
        assert_eq!(result, ScreenType::Typing);
    }

    #[test]
    fn test_title_to_loading() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Title, ScreenType::Loading, &sm).unwrap();
        assert_eq!(result, ScreenType::Loading);
    }

    // === Same screen transition ===

    #[test]
    fn test_same_screen_noop() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Title, ScreenType::Title, &sm).unwrap();
        assert_eq!(result, ScreenType::Title);
    }

    // === Invalid transition panics ===

    #[test]
    #[should_panic(expected = "Invalid screen transition")]
    fn test_invalid_transition_panics() {
        let sm = create_session_manager();
        let _ =
            ScreenTransitionManager::reduce(ScreenType::Records, ScreenType::Typing, &sm).unwrap();
    }

    // === Transitions with side effects ===

    #[test]
    fn test_title_to_typing_starts_game() {
        let sm = create_session_manager();
        let result =
            ScreenTransitionManager::reduce(ScreenType::Title, ScreenType::Typing, &sm).unwrap();
        assert_eq!(result, ScreenType::Typing);
    }

    #[test]
    fn test_session_failure_to_typing_retries() {
        let sm = create_session_manager();
        // Need to start a session first so retry makes sense
        let _ = ScreenTransitionManager::reduce(ScreenType::Title, ScreenType::Typing, &sm);
        let result =
            ScreenTransitionManager::reduce(ScreenType::SessionFailure, ScreenType::Typing, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::Typing);
    }

    #[test]
    fn test_session_summary_to_typing_retry() {
        let sm = create_session_manager();
        let _ = ScreenTransitionManager::reduce(ScreenType::Title, ScreenType::Typing, &sm);
        let result =
            ScreenTransitionManager::reduce(ScreenType::SessionSummary, ScreenType::Typing, &sm)
                .unwrap();
        assert_eq!(result, ScreenType::Typing);
    }
}
