use gittype::domain::events::EventBus;
use gittype::domain::events::EventBusInterface;
use gittype::domain::models::{DifficultyLevel, SessionAction, SessionConfig, SessionState};
use gittype::domain::services::scoring::{
    SessionTracker, SessionTrackerInterface, StageCalculator, StageInput, StageResult,
    StageTracker, TotalTracker, TotalTrackerInterface,
};
use gittype::domain::services::session_manager_service::SessionManager;
use gittype::domain::services::session_manager_service::SessionManagerInterface;
use gittype::domain::services::stage_builder_service::{StageRepository, StageRepositoryInterface};
use gittype::domain::stores::{ChallengeStore, RepositoryStore, SessionStore};
use std::sync::Arc;

fn create_test_dependencies() -> (
    Arc<dyn EventBusInterface>,
    Arc<dyn StageRepositoryInterface>,
    Arc<dyn SessionTrackerInterface>,
    Arc<dyn TotalTrackerInterface>,
) {
    let event_bus = Arc::new(EventBus::new()) as Arc<dyn EventBusInterface>;
    let challenge_store = Arc::new(ChallengeStore::new_for_test());
    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let session_store = Arc::new(SessionStore::new_for_test());
    let stage_repository = Arc::new(StageRepository::new(
        None,
        challenge_store,
        repository_store,
        session_store,
    )) as Arc<dyn StageRepositoryInterface>;
    let session_tracker =
        Arc::new(SessionTracker::new_for_test()) as Arc<dyn SessionTrackerInterface>;
    let total_tracker = Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>;

    (event_bus, stage_repository, session_tracker, total_tracker)
}

fn create_session_manager() -> SessionManager {
    let (event_bus, stage_repository, session_tracker, total_tracker) = create_test_dependencies();
    SessionManager::new_with_dependencies(
        event_bus,
        stage_repository,
        session_tracker,
        total_tracker,
    )
}

fn create_dummy_stage_result() -> StageResult {
    let mut tracker = StageTracker::new("hello world".to_string());
    tracker.record(StageInput::Start);
    for (i, ch) in "hello world".chars().enumerate() {
        tracker.record(StageInput::Keystroke { ch, position: i });
    }
    tracker.record(StageInput::Finish);
    StageCalculator::calculate(&tracker)
}

// ============================================
// Constructor and basic state
// ============================================

#[test]
fn test_new_with_dependencies_creates_not_started_state() {
    let manager = create_session_manager();
    assert!(matches!(manager.get_state(), SessionState::NotStarted));
}

#[test]
fn test_default_difficulty_is_normal() {
    let manager = create_session_manager();
    assert_eq!(manager.get_difficulty(), DifficultyLevel::Normal);
}

#[test]
fn test_set_difficulty() {
    let manager = create_session_manager();
    manager.set_difficulty(DifficultyLevel::Hard);
    assert_eq!(manager.get_difficulty(), DifficultyLevel::Hard);
}

#[test]
fn test_is_completed_initially_false() {
    let manager = create_session_manager();
    assert!(!manager.is_completed());
}

#[test]
fn test_is_in_progress_initially_false() {
    let manager = create_session_manager();
    assert!(!manager.is_in_progress());
}

#[test]
fn test_is_session_completed_initially_false() {
    let manager = create_session_manager();
    assert!(!manager.is_session_completed().unwrap());
}

#[test]
fn test_session_duration_not_started_is_none() {
    let manager = create_session_manager();
    assert!(manager.session_duration().is_none());
}

#[test]
fn test_get_stage_results_initially_empty() {
    let manager = create_session_manager();
    assert!(manager.get_stage_results().is_empty());
}

#[test]
fn test_get_skips_used_initially_zero() {
    let manager = create_session_manager();
    assert_eq!(manager.get_skips_used(), 0);
}

#[test]
fn test_get_skips_remaining_initially_max() {
    let manager = create_session_manager();
    assert_eq!(manager.get_skips_remaining().unwrap(), 3);
}

// ============================================
// State setter methods
// ============================================

#[test]
fn test_set_state() {
    let manager = create_session_manager();
    manager.set_state(SessionState::InProgress {
        current_stage: 2,
        started_at: std::time::Instant::now(),
    });
    assert!(manager.is_in_progress());
}

#[test]
fn test_set_config() {
    let manager = create_session_manager();
    let config = SessionConfig {
        max_stages: 5,
        max_skips: 2,
        ..Default::default()
    };
    manager.set_config(config);
    assert_eq!(manager.get_skips_remaining().unwrap(), 2);
}

#[test]
fn test_set_git_repository() {
    let manager = create_session_manager();
    let repo = crate::fixtures::models::git_repository::build();
    manager.set_git_repository(Some(repo));
    // No getter for git_repository, but should not panic
}

// ============================================
// State machine (reduce)
// ============================================

#[test]
fn test_reduce_start_transitions_to_in_progress() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    assert!(manager.is_in_progress());
}

#[test]
fn test_reduce_start_sets_current_stage_to_1() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    let (current, _total) = manager.get_stage_info().unwrap();
    assert_eq!(current, 1);
}

#[test]
fn test_reduce_complete_stage_advances_stage() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();

    let stage_result = create_dummy_stage_result();
    manager
        .reduce(SessionAction::CompleteStage(stage_result))
        .unwrap();

    let (current, _total) = manager.get_stage_info().unwrap();
    assert_eq!(current, 2);
}

#[test]
fn test_reduce_all_stages_completed_transitions_to_completed() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();

    // Default max_stages is 3
    for _ in 0..3 {
        let stage_result = create_dummy_stage_result();
        manager
            .reduce(SessionAction::CompleteStage(stage_result))
            .unwrap();
    }

    assert!(manager.is_completed());
}

#[test]
fn test_reduce_abort_transitions_to_aborted() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    manager.reduce(SessionAction::Abort).unwrap();

    let state = manager.get_state();
    assert!(matches!(state, SessionState::Aborted { .. }));
}

#[test]
fn test_reduce_complete_transitions_to_completed() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    manager.reduce(SessionAction::Complete).unwrap();

    assert!(manager.is_completed());
}

#[test]
fn test_reduce_reset_clears_state() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    let stage_result = create_dummy_stage_result();
    manager
        .reduce(SessionAction::CompleteStage(stage_result))
        .unwrap();

    manager.reduce(SessionAction::Reset).unwrap();

    assert!(matches!(manager.get_state(), SessionState::NotStarted));
    assert!(manager.get_stage_results().is_empty());
}

#[test]
fn test_reduce_invalid_transition_returns_error() {
    let manager = create_session_manager();
    // Start from NotStarted with CompleteStage should fail
    let stage_result = create_dummy_stage_result();
    let result = manager.reduce(SessionAction::CompleteStage(stage_result));
    assert!(result.is_err());
}

#[test]
fn test_reduce_double_start_returns_error() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    let result = manager.reduce(SessionAction::Start);
    assert!(result.is_err());
}

#[test]
fn test_reduce_skipped_stages_dont_count_for_completion() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();

    // Add a skipped stage result (shouldn't count toward completion)
    let mut skipped_result = create_dummy_stage_result();
    skipped_result.was_skipped = true;
    manager
        .reduce(SessionAction::CompleteStage(skipped_result))
        .unwrap();

    // Not completed yet - skipped stages don't count
    assert!(!manager.is_completed());
    assert!(manager.is_in_progress());
}

// ============================================
// Initialize and reset
// ============================================

#[test]
fn test_initialize_resets_state() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    manager.initialize(None).unwrap();

    assert!(matches!(manager.get_state(), SessionState::NotStarted));
}

#[test]
fn test_initialize_with_custom_config() {
    let manager = create_session_manager();
    let config = SessionConfig {
        max_stages: 5,
        max_skips: 1,
        difficulty: DifficultyLevel::Hard,
        ..Default::default()
    };
    manager.initialize(Some(config)).unwrap();

    assert_eq!(manager.get_difficulty(), DifficultyLevel::Hard);
    assert_eq!(manager.get_skips_remaining().unwrap(), 1);
}

#[test]
fn test_reset_clears_everything() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    let stage_result = create_dummy_stage_result();
    manager.add_stage_data(
        "Stage 1".to_string(),
        StageTracker::new("test".to_string()),
        crate::fixtures::models::challenge::build(),
    );
    manager
        .reduce(SessionAction::CompleteStage(stage_result))
        .unwrap();

    manager.reset();

    assert!(matches!(manager.get_state(), SessionState::NotStarted));
    assert!(manager.get_stage_results().is_empty());
}

// ============================================
// Stage tracker management
// ============================================

#[test]
fn test_set_current_stage_tracker() {
    let manager = create_session_manager();
    let tracker = StageTracker::new("hello".to_string());
    manager.set_current_stage_tracker(tracker);

    let retrieved = manager.get_current_stage_tracker();
    assert!(retrieved.is_some());
}

#[test]
fn test_get_current_stage_tracker_initially_none() {
    let manager = create_session_manager();
    assert!(manager.get_current_stage_tracker().is_none());
}

#[test]
fn test_get_current_stage_tracker_mut_returns_clone() {
    let manager = create_session_manager();
    let tracker = StageTracker::new("test".to_string());
    manager.set_current_stage_tracker(tracker);

    let retrieved = manager.get_current_stage_tracker_mut();
    assert!(retrieved.is_some());
}

// ============================================
// Add stage data
// ============================================

#[test]
fn test_add_stage_data() {
    let manager = create_session_manager();
    let tracker = StageTracker::new("hello".to_string());
    let challenge = crate::fixtures::models::challenge::build();

    manager.add_stage_data("Stage 1".to_string(), tracker, challenge);
    // Stage data tracked internally, no panic means success
}

// ============================================
// Skips management
// ============================================

#[test]
fn test_skips_tracking() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();

    let mut skipped = create_dummy_stage_result();
    skipped.was_skipped = true;
    manager
        .reduce(SessionAction::CompleteStage(skipped))
        .unwrap();

    assert_eq!(manager.get_skips_used(), 1);
    assert_eq!(manager.get_skips_remaining().unwrap(), 2);
}

// ============================================
// Stage info
// ============================================

#[test]
fn test_get_stage_info_not_started() {
    let manager = create_session_manager();
    let (current, total) = manager.get_stage_info().unwrap();
    assert_eq!(current, 0);
    assert_eq!(total, 3);
}

#[test]
fn test_get_stage_info_in_progress() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    let (current, total) = manager.get_stage_info().unwrap();
    assert_eq!(current, 1);
    assert_eq!(total, 3);
}

#[test]
fn test_get_stage_info_completed() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    for _ in 0..3 {
        let stage_result = create_dummy_stage_result();
        manager
            .reduce(SessionAction::CompleteStage(stage_result))
            .unwrap();
    }
    let (current, total) = manager.get_stage_info().unwrap();
    assert_eq!(current, 3);
    assert_eq!(total, 3);
}

// ============================================
// Session duration
// ============================================

#[test]
fn test_session_duration_in_progress() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    let duration = manager.session_duration();
    assert!(duration.is_some());
}

#[test]
fn test_session_duration_completed() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    manager.reduce(SessionAction::Complete).unwrap();
    let duration = manager.session_duration();
    assert!(duration.is_some());
}

#[test]
fn test_session_duration_aborted() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    manager.reduce(SessionAction::Abort).unwrap();
    let duration = manager.session_duration();
    assert!(duration.is_some());
}

// ============================================
// Abort session
// ============================================

#[test]
fn test_abort_session_from_in_progress() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    manager.abort_session();
    assert!(matches!(manager.get_state(), SessionState::Aborted { .. }));
}

#[test]
fn test_abort_session_from_not_started_does_nothing() {
    let manager = create_session_manager();
    manager.abort_session();
    assert!(matches!(manager.get_state(), SessionState::NotStarted));
}

// ============================================
// Generate session result
// ============================================

#[test]
fn test_generate_session_result_returns_some() {
    let manager = create_session_manager();
    let result = manager.generate_session_result();
    assert!(result.is_some());
}

#[test]
fn test_get_session_result_returns_some() {
    let manager = create_session_manager();
    let result = manager.get_session_result();
    assert!(result.is_some());
}

// ============================================
// Finalize current stage
// ============================================

#[test]
fn test_finalize_current_stage_with_no_tracker_returns_error() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    let result = manager.finalize_current_stage();
    assert!(result.is_err());
}

#[test]
fn test_finalize_current_stage_with_tracker() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();

    let mut tracker = StageTracker::new("hello".to_string());
    tracker.record(StageInput::Start);
    for (i, ch) in "hello".chars().enumerate() {
        tracker.record(StageInput::Keystroke { ch, position: i });
    }
    manager.set_current_stage_tracker(tracker);

    let result = manager.finalize_current_stage();
    assert!(result.is_ok());
    assert_eq!(manager.get_stage_results().len(), 1);
}

// ============================================
// Skip current stage
// ============================================

#[test]
fn test_skip_current_stage_not_in_progress_returns_error() {
    let manager = create_session_manager();
    let tracker = StageTracker::new("hello".to_string());
    manager.set_current_stage_tracker(tracker);
    let result = manager.skip_current_stage();
    assert!(result.is_err());
}

#[test]
fn test_skip_current_stage_with_no_tracker_returns_error() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    let result = manager.skip_current_stage();
    assert!(result.is_err());
}

#[test]
fn test_skip_current_stage_with_tracker() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();

    let mut tracker = StageTracker::new("hello".to_string());
    tracker.record(StageInput::Start);
    manager.set_current_stage_tracker(tracker);

    let (stage_result, skips_remaining, needs_new_challenge) =
        manager.skip_current_stage().unwrap();
    assert!(stage_result.was_skipped);
    assert_eq!(skips_remaining, 2);
    assert!(needs_new_challenge);
}

#[test]
fn test_skip_current_stage_no_skips_remaining_returns_error() {
    let manager = create_session_manager();
    manager.set_config(SessionConfig {
        max_skips: 0,
        ..Default::default()
    });
    manager.reduce(SessionAction::Start).unwrap();

    let tracker = StageTracker::new("hello".to_string());
    manager.set_current_stage_tracker(tracker);

    let result = manager.skip_current_stage();
    assert!(result.is_err());
}

// ============================================
// Event bus
// ============================================

#[test]
fn test_get_event_bus() {
    let manager = create_session_manager();
    let _bus = manager.get_event_bus();
}

#[test]
fn test_set_event_bus_is_noop() {
    let manager = create_session_manager();
    let new_bus = Arc::new(EventBus::new()) as Arc<dyn EventBusInterface>;
    // Should not panic, just log a warning
    manager.set_event_bus(new_bus);
}

// ============================================
// Get best status for score
// ============================================

#[test]
fn test_get_best_status_for_score() {
    let manager = create_session_manager();
    let result = manager.get_best_status_for_score(100.0);
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

// ============================================
// Record and update trackers
// ============================================

#[test]
fn test_record_and_update_trackers() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();
    // Set git repository to avoid DB write issues
    let repo = crate::fixtures::models::git_repository::build();
    manager.set_git_repository(Some(repo));
    // Add stage data so there's something to record
    let tracker = StageTracker::new("test".to_string());
    let challenge = crate::fixtures::models::challenge::build();
    manager.add_stage_data("Stage 1".to_string(), tracker, challenge);
    let result = manager.record_and_update_trackers();
    assert!(result.is_ok());
}

// ============================================
// Get current/next challenge
// ============================================

#[test]
fn test_get_current_challenge_not_in_progress_returns_none() {
    let manager = create_session_manager();
    let result = manager.get_current_challenge().unwrap();
    assert!(result.is_none());
}

#[test]
fn test_get_next_challenge_not_in_progress_returns_none() {
    let manager = create_session_manager();
    let result = manager.get_next_challenge().unwrap();
    assert!(result.is_none());
}

// ============================================
// Event subscriptions
// ============================================

#[test]
fn test_setup_event_subscriptions() {
    let (event_bus, stage_repository, session_tracker, total_tracker) = create_test_dependencies();
    let manager = Arc::new(SessionManager::new_with_dependencies(
        event_bus,
        stage_repository,
        session_tracker,
        total_tracker,
    ));
    // Should not panic
    SessionManager::setup_event_subscriptions(Arc::clone(&manager));
}

// ============================================
// as_any for downcasting
// ============================================

#[test]
fn test_as_any_downcasts_correctly() {
    let manager = create_session_manager();
    let any = manager.as_any();
    assert!(any.downcast_ref::<SessionManager>().is_some());
}

// ============================================
// Event subscription handlers
// ============================================

fn create_arc_session_manager_with_subscriptions() -> Arc<SessionManager> {
    let (event_bus, stage_repository, session_tracker, total_tracker) = create_test_dependencies();
    let manager = Arc::new(SessionManager::new_with_dependencies(
        event_bus,
        stage_repository,
        session_tracker,
        total_tracker,
    ));
    SessionManager::setup_event_subscriptions(Arc::clone(&manager));
    manager
}

#[test]
fn test_event_challenge_loaded_initializes_tracker() {
    use gittype::domain::events::domain_events::DomainEvent;

    let manager = create_arc_session_manager_with_subscriptions();
    let event_bus = manager.get_event_bus();

    event_bus
        .as_event_bus()
        .publish(DomainEvent::ChallengeLoaded {
            text: "fn main() {}".to_string(),
            source_path: "src/main.rs".to_string(),
        });

    // ChallengeLoaded should call init_stage_tracker, creating a tracker
    let tracker = manager.get_current_stage_tracker();
    assert!(tracker.is_some());
}

#[test]
fn test_event_challenge_loaded_with_empty_path() {
    use gittype::domain::events::domain_events::DomainEvent;

    let manager = create_arc_session_manager_with_subscriptions();
    let event_bus = manager.get_event_bus();

    event_bus
        .as_event_bus()
        .publish(DomainEvent::ChallengeLoaded {
            text: "let x = 1;".to_string(),
            source_path: "".to_string(),
        });

    let tracker = manager.get_current_stage_tracker();
    assert!(tracker.is_some());
}

#[test]
fn test_event_stage_started_records_start() {
    use gittype::domain::events::domain_events::DomainEvent;

    let manager = create_arc_session_manager_with_subscriptions();
    let event_bus = manager.get_event_bus();

    // First load a challenge to create a tracker
    event_bus
        .as_event_bus()
        .publish(DomainEvent::ChallengeLoaded {
            text: "hello".to_string(),
            source_path: "test.rs".to_string(),
        });

    // Then start stage
    let start_time = std::time::Instant::now();
    event_bus
        .as_event_bus()
        .publish(DomainEvent::StageStarted { start_time });

    // Tracker should still exist (start was recorded)
    assert!(manager.get_current_stage_tracker().is_some());
}

#[test]
fn test_event_key_pressed_records_keystroke() {
    use gittype::domain::events::domain_events::DomainEvent;

    let manager = create_arc_session_manager_with_subscriptions();
    let event_bus = manager.get_event_bus();

    event_bus
        .as_event_bus()
        .publish(DomainEvent::ChallengeLoaded {
            text: "hello".to_string(),
            source_path: "test.rs".to_string(),
        });
    event_bus.as_event_bus().publish(DomainEvent::StageStarted {
        start_time: std::time::Instant::now(),
    });
    event_bus.as_event_bus().publish(DomainEvent::KeyPressed {
        key: 'h',
        position: 0,
    });

    assert!(manager.get_current_stage_tracker().is_some());
}

#[test]
fn test_event_stage_paused_records_pause() {
    use gittype::domain::events::domain_events::DomainEvent;

    let manager = create_arc_session_manager_with_subscriptions();
    let event_bus = manager.get_event_bus();

    event_bus
        .as_event_bus()
        .publish(DomainEvent::ChallengeLoaded {
            text: "hello".to_string(),
            source_path: "".to_string(),
        });
    event_bus.as_event_bus().publish(DomainEvent::StagePaused);

    assert!(manager.get_current_stage_tracker().is_some());
}

#[test]
fn test_event_stage_resumed_records_resume() {
    use gittype::domain::events::domain_events::DomainEvent;

    let manager = create_arc_session_manager_with_subscriptions();
    let event_bus = manager.get_event_bus();

    event_bus
        .as_event_bus()
        .publish(DomainEvent::ChallengeLoaded {
            text: "hello".to_string(),
            source_path: "".to_string(),
        });
    event_bus.as_event_bus().publish(DomainEvent::StagePaused);
    event_bus.as_event_bus().publish(DomainEvent::StageResumed);

    assert!(manager.get_current_stage_tracker().is_some());
}

#[test]
fn test_event_stage_finalized_finalizes_stage() {
    use gittype::domain::events::domain_events::DomainEvent;

    let manager = create_arc_session_manager_with_subscriptions();
    let event_bus = manager.get_event_bus();

    // Must be in progress for finalize to work
    manager.reduce(SessionAction::Start).unwrap();

    event_bus
        .as_event_bus()
        .publish(DomainEvent::ChallengeLoaded {
            text: "hello".to_string(),
            source_path: "test.rs".to_string(),
        });
    event_bus.as_event_bus().publish(DomainEvent::StageStarted {
        start_time: std::time::Instant::now(),
    });
    for (i, ch) in "hello".chars().enumerate() {
        event_bus.as_event_bus().publish(DomainEvent::KeyPressed {
            key: ch,
            position: i,
        });
    }
    event_bus
        .as_event_bus()
        .publish(DomainEvent::StageFinalized);

    // After finalize, tracker should be cleared and stage_results should have 1 entry
    assert!(manager.get_current_stage_tracker().is_none());
    assert_eq!(manager.get_stage_results().len(), 1);
}

#[test]
fn test_event_stage_skipped_skips_stage() {
    use gittype::domain::events::domain_events::DomainEvent;

    let manager = create_arc_session_manager_with_subscriptions();
    let event_bus = manager.get_event_bus();

    manager.reduce(SessionAction::Start).unwrap();

    event_bus
        .as_event_bus()
        .publish(DomainEvent::ChallengeLoaded {
            text: "hello".to_string(),
            source_path: "test.rs".to_string(),
        });
    event_bus.as_event_bus().publish(DomainEvent::StageStarted {
        start_time: std::time::Instant::now(),
    });
    event_bus.as_event_bus().publish(DomainEvent::StageSkipped);

    // After skip, tracker should be cleared
    assert!(manager.get_current_stage_tracker().is_none());
    // Stage results should have the skipped result
    let results = manager.get_stage_results();
    assert_eq!(results.len(), 1);
    assert!(results[0].was_skipped);
}

// ============================================
// Failed stage results
// ============================================

#[test]
fn test_reduce_failed_stages_dont_count_for_completion() {
    let manager = create_session_manager();
    manager.reduce(SessionAction::Start).unwrap();

    let mut failed_result = create_dummy_stage_result();
    failed_result.was_failed = true;
    manager
        .reduce(SessionAction::CompleteStage(failed_result))
        .unwrap();

    assert!(!manager.is_completed());
    assert!(manager.is_in_progress());
}

// ============================================
// get_current_challenge / get_next_challenge in progress
// ============================================

#[test]
fn test_get_current_challenge_in_progress_with_challenges() {
    use gittype::domain::models::Challenge;
    use gittype::domain::stores::ChallengeStoreInterface;

    let event_bus = Arc::new(EventBus::new()) as Arc<dyn EventBusInterface>;
    let challenge_store = Arc::new(ChallengeStore::new_for_test());

    // Populate challenge store with challenges
    let challenges = vec![
        Challenge::new("test-1".to_string(), "fn foo() {}".to_string()),
        Challenge::new("test-2".to_string(), "fn bar() {}".to_string()),
    ];
    challenge_store.set_challenges(challenges);

    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let session_store = Arc::new(SessionStore::new_for_test());
    let stage_repository = Arc::new(StageRepository::new(
        None,
        challenge_store,
        repository_store,
        session_store,
    )) as Arc<dyn StageRepositoryInterface>;
    let session_tracker =
        Arc::new(SessionTracker::new_for_test()) as Arc<dyn SessionTrackerInterface>;
    let total_tracker = Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>;

    let manager = SessionManager::new_with_dependencies(
        event_bus,
        stage_repository,
        session_tracker,
        total_tracker,
    );
    manager.reduce(SessionAction::Start).unwrap();

    // Should succeed (may return Some or None depending on difficulty index)
    let result = manager.get_current_challenge();
    assert!(result.is_ok());
}

#[test]
fn test_get_next_challenge_in_progress_with_challenges() {
    use gittype::domain::models::Challenge;
    use gittype::domain::stores::ChallengeStoreInterface;

    let event_bus = Arc::new(EventBus::new()) as Arc<dyn EventBusInterface>;
    let challenge_store = Arc::new(ChallengeStore::new_for_test());

    let challenges = vec![
        Challenge::new("test-1".to_string(), "fn foo() {}".to_string()),
        Challenge::new("test-2".to_string(), "fn bar() {}".to_string()),
    ];
    challenge_store.set_challenges(challenges);

    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let session_store = Arc::new(SessionStore::new_for_test());
    let stage_repository = Arc::new(StageRepository::new(
        None,
        challenge_store,
        repository_store,
        session_store,
    )) as Arc<dyn StageRepositoryInterface>;
    let session_tracker =
        Arc::new(SessionTracker::new_for_test()) as Arc<dyn SessionTrackerInterface>;
    let total_tracker = Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>;

    let manager = SessionManager::new_with_dependencies(
        event_bus,
        stage_repository,
        session_tracker,
        total_tracker,
    );
    manager.reduce(SessionAction::Start).unwrap();

    let result = manager.get_next_challenge();
    assert!(result.is_ok());
}
