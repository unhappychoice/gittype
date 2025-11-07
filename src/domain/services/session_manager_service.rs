use crate::domain::events::domain_events::DomainEvent;
use crate::domain::events::{EventBus, EventBusInterface};
use crate::domain::models::GitRepository;
use crate::domain::repositories::session_repository::{BestRecords, BestStatus};
use crate::domain::repositories::SessionRepository;
use crate::domain::services::scoring::{
    SessionCalculator, SessionTracker, StageCalculator, GLOBAL_TOTAL_TRACKER,
};
use crate::domain::services::stage_builder_service::{StageRepository, StageRepositoryInterface};
use crate::{
    domain::models::{Challenge, DifficultyLevel, SessionAction, SessionConfig, SessionResult, SessionState},
    domain::services::scoring::{StageInput, StageResult, StageTracker, GLOBAL_SESSION_TRACKER},
    GitTypeError, Result,
};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};


/// Manages the overall session state and stage progression
#[derive(shaku::Component)]
#[shaku(interface = SessionManagerInterface)]
pub struct SessionManager {
    #[shaku(default)]
    state: Mutex<SessionState>,
    #[shaku(default)]
    config: Mutex<SessionConfig>,
    #[shaku(default)]
    stage_results: Mutex<Vec<StageResult>>,
    #[shaku(default)]
    pub current_stage_tracker: Mutex<Option<StageTracker>>,
    #[shaku(default)]
    stage_trackers: Mutex<Vec<(String, StageTracker)>>,
    #[shaku(default)]
    git_repository: Mutex<Option<GitRepository>>,
    #[shaku(default)]
    session_challenges: Mutex<Vec<Challenge>>,
    #[shaku(default)]
    best_records_at_start: Mutex<Option<BestRecords>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    stage_repository: Arc<dyn StageRepositoryInterface>,
}

pub trait SessionManagerInterface: shaku::Interface {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl SessionManagerInterface for SessionManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl SessionManager {
    // Constructor for manual instantiation (e.g., in tests or screen_runner)
    pub fn new_with_dependencies(
        event_bus: Arc<dyn EventBusInterface>,
        stage_repository: Arc<dyn StageRepositoryInterface>,
    ) -> Self {
        Self {
            state: Mutex::new(SessionState::NotStarted),
            config: Mutex::new(SessionConfig::default()),
            stage_results: Mutex::new(Vec::new()),
            current_stage_tracker: Mutex::new(None),
            stage_trackers: Mutex::new(Vec::new()),
            git_repository: Mutex::new(None),
            session_challenges: Mutex::new(Vec::new()),
            best_records_at_start: Mutex::new(None),
            event_bus,
            stage_repository,
        }
    }

    pub fn set_event_bus(&self, event_bus: Arc<dyn EventBusInterface>) {
        // event_bus is Arc, which is already shared, so we can't replace it in interior mutability pattern
        // This method is deprecated and should not be used with DI
        log::warn!("set_event_bus called but event_bus is injected via DI and cannot be changed");
    }

    pub fn setup_event_subscriptions(instance: Arc<SessionManager>) {
        let instance_weak = Arc::downgrade(&instance);
        let event_bus = instance.event_bus.clone();

        Self::setup_event_subscriptions_internal(event_bus.as_ref(), &instance_weak);
    }

    pub fn get_event_bus(&self) -> Arc<dyn EventBusInterface> {
        self.event_bus.clone()
    }

    pub fn set_state(&self, state: SessionState) {
        *self.state.lock().unwrap() = state;
    }

    pub fn set_current_stage_tracker(&self, tracker: StageTracker) {
        *self.current_stage_tracker.lock().unwrap() = Some(tracker);
    }

    pub fn set_config(&self, config: SessionConfig) {
        *self.config.lock().unwrap() = config;
    }

    fn setup_event_subscriptions_internal(
        bus: &dyn EventBusInterface,
        instance: &std::sync::Weak<SessionManager>,
    ) {
        let instance = instance.clone();

        // Subscribe to unified DomainEvent with pattern matching
        bus.as_event_bus().subscribe(move |event: &DomainEvent| {
            if let Some(manager) = instance.upgrade() {
                match event {
                    DomainEvent::ChallengeLoaded { text, source_path } => {
                        let _ = manager.init_stage_tracker(
                            text.clone(),
                            if source_path.is_empty() {
                                None
                            } else {
                                Some(source_path.clone())
                            },
                        );
                    }
                    DomainEvent::StageStarted { start_time } => {
                        let _ = manager.set_stage_start_time(*start_time);
                        let _ = manager.record_stage_input(StageInput::Start);
                    }
                    DomainEvent::StagePaused => {
                        let _ = manager.record_stage_input(StageInput::Pause);
                    }
                    DomainEvent::StageResumed => {
                        let _ = manager.record_stage_input(StageInput::Resume);
                    }
                    DomainEvent::KeyPressed { key, position } => {
                        let _ = manager.record_stage_input(StageInput::Keystroke {
                            ch: *key,
                            position: *position,
                        });
                    }
                    DomainEvent::StageFinalized => {
                        let _ = manager.finalize_current_stage();
                    }
                    DomainEvent::StageSkipped => {
                        let _ = manager.skip_current_stage();
                    }
                }
            }
        });
    }

    /// Central state machine reducer - handles all state transitions
    pub fn reduce(&self, action: SessionAction) -> Result<()> {
        log::debug!("SessionManager::reduce - {:?}", action);

        let state_guard = self.state.lock().unwrap();
        let new_state = match (&*state_guard, &action) {
            // Start transitions
            (SessionState::NotStarted, SessionAction::Start) => {
                let session_start_time = Instant::now();

                // Capture best records at session start for accurate comparison later
                *self.best_records_at_start.lock().unwrap() =
                    SessionRepository::get_best_records_global().ok().flatten();

                log::debug!(
                    "SessionManager::reduce Start: captured best_records_at_start={:?}",
                    *self.best_records_at_start.lock().unwrap()
                );

                // Initialize global session tracker
                let session_tracker = SessionTracker::new();
                SessionTracker::initialize_global_instance(session_tracker);

                SessionState::InProgress {
                    current_stage: 1,
                    started_at: session_start_time,
                }
            }

            // Complete stage transitions
            (
                SessionState::InProgress {
                    started_at,
                    current_stage,
                },
                SessionAction::CompleteStage(stage_result),
            ) => {
                self.stage_results.lock().unwrap().push(stage_result.clone());

                // Count actually completed stages (not skipped and not failed)
                let completed_stages = self
                    .stage_results
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|sr| !sr.was_skipped && !sr.was_failed)
                    .count();

                if completed_stages >= self.config.lock().unwrap().max_stages {
                    // Session completed - we have enough completed stages
                    self.add_session_to_total_tracker()?;

                    SessionState::Completed {
                        started_at: *started_at,
                        completed_at: Instant::now(),
                    }
                } else {
                    // Move to next stage
                    SessionState::InProgress {
                        current_stage: *current_stage + 1,
                        started_at: *started_at,
                    }
                }
            }

            // Complete session transitions
            (SessionState::InProgress { started_at, .. }, SessionAction::Complete) => {
                self.add_session_to_total_tracker()?;
                SessionState::Completed {
                    started_at: *started_at,
                    completed_at: Instant::now(),
                }
            }

            // Abort transitions
            (SessionState::InProgress { started_at, .. }, SessionAction::Abort) => {
                SessionState::Aborted {
                    started_at: *started_at,
                    aborted_at: Instant::now(),
                }
            }

            // Reset transitions (from any state)
            (_, SessionAction::Reset) => {
                // Clear all state
                self.stage_results.lock().unwrap().clear();
                *self.current_stage_tracker.lock().unwrap() = None;
                self.stage_trackers.lock().unwrap().clear();
                self.session_challenges.lock().unwrap().clear();

                // Clear global session tracker
                let _ = GLOBAL_SESSION_TRACKER.lock().map(|mut tracker| {
                    *tracker = None;
                });

                SessionState::NotStarted
            }

            // Invalid transitions
            (state, action) => {
                log::error!("Invalid state transition: {:?} -> {:?}", state, action);
                return Err(GitTypeError::TerminalError(format!(
                    "Invalid session state transition: {:?} with action {:?}",
                    state, action
                )));
            }
        };

        drop(state_guard);
        log::debug!(
            "SessionManager::reduce - {:?} -> {:?}",
            *self.state.lock().unwrap(),
            new_state
        );
        *self.state.lock().unwrap() = new_state;
        Ok(())
    }

    // ============================================
    // Essential Public API
    // ============================================

    /// Initialize session with configuration (instance method)
    pub fn initialize(&self, config: Option<SessionConfig>) -> Result<()> {
        *self.config.lock().unwrap() = config.unwrap_or_default();
        *self.state.lock().unwrap() = SessionState::NotStarted;
        self.stage_results.lock().unwrap().clear();
        *self.current_stage_tracker.lock().unwrap() = None;
        self.stage_trackers.lock().unwrap().clear();
        *self.git_repository.lock().unwrap() = None;
        self.session_challenges.lock().unwrap().clear();

        // Capture best records at session start for accurate comparison later
        *self.best_records_at_start.lock().unwrap() = SessionRepository::get_best_records_global().ok().flatten();

        log::debug!(
            "SessionManager::initialize: captured best_records_at_start={:?}",
            *self.best_records_at_start.lock().unwrap()
        );

        Ok(())
    }

    /// Set git repository context for the session (instance method)
    pub fn set_git_repository(&self, git_repository: Option<GitRepository>) {
        *self.git_repository.lock().unwrap() = git_repository;
    }

    /// Add stage data (tracker and challenge) for the current stage
    pub fn add_stage_data(
        &self,
        stage_name: String,
        stage_tracker: StageTracker,
        challenge: Challenge,
    ) {
        self.stage_trackers.lock().unwrap().push((stage_name, stage_tracker));
        self.session_challenges.lock().unwrap().push(challenge);
    }

    /// Calculate number of skips used in this session
    pub fn get_skips_used(&self) -> usize {
        self.stage_results
            .lock()
            .unwrap()
            .iter()
            .filter(|result| result.was_skipped)
            .count()
    }

    /// Calculate remaining skips for this session
    pub fn get_skips_remaining(&self) -> Result<usize> {
        let used = self.get_skips_used();
        Ok(self.config.lock().unwrap().max_skips.saturating_sub(used))
    }

    /// Get stage info (current_stage, total_stages)
    pub fn get_stage_info(&self) -> Result<(usize, usize)> {
        let state = self.state.lock().unwrap();
        let current = match &*state {
            SessionState::InProgress { current_stage, .. } => *current_stage,
            SessionState::Completed { .. } => {
                let completed = self
                    .stage_results
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|sr| !sr.was_skipped && !sr.was_failed)
                    .count();
                completed.max(1).min(self.config.lock().unwrap().max_stages)
            }
            _ => 0,
        };
        Ok((current, self.config.lock().unwrap().max_stages))
    }

    /// Check if session is completed
    pub fn is_session_completed(&self) -> Result<bool> {
        Ok(matches!(*self.state.lock().unwrap(), SessionState::Completed { .. }))
    }

    /// Get current challenge for the session
    pub fn get_current_challenge(&self) -> Result<Option<Challenge>> {
        if matches!(*self.state.lock().unwrap(), SessionState::InProgress { .. }) {
            let stage_repo = self.stage_repository.as_any()
                .downcast_ref::<StageRepository>()
                .ok_or_else(|| GitTypeError::TerminalError("Failed to downcast StageRepository".to_string()))?;
            Ok(stage_repo.get_challenge_for_difficulty(self.config.lock().unwrap().difficulty))
        } else {
            Ok(None)
        }
    }

    /// Get best status for a given score
    pub fn get_best_status_for_score(&self, score: f64) -> Result<Option<BestStatus>> {
        let best_records_at_start = self.best_records_at_start.lock().unwrap();
        log::debug!(
            "SessionManager::get_best_status_for_score: score={}, best_records_at_start={:?}",
            score,
            *best_records_at_start
        );

        let best_status = SessionRepository::determine_best_status_with_start_records(
            score,
            best_records_at_start.as_ref(),
        );

        Ok(Some(best_status))
    }

    /// Start the session
    fn start_session(&self) -> Result<()> {
        let state = self.state.lock().unwrap();
        match *state {
            SessionState::NotStarted => {
                drop(state);
                let session_start_time = Instant::now();
                *self.state.lock().unwrap() = SessionState::InProgress {
                    current_stage: 1,
                    started_at: session_start_time,
                };

                // Initialize global session tracker
                let session_tracker = SessionTracker::new();
                SessionTracker::initialize_global_instance(session_tracker);

                Ok(())
            }
            _ => Err(GitTypeError::TerminalError(
                "Session is already started or completed".to_string(),
            )),
        }
    }

    /// Abort the current session
    pub fn abort_session(&self) {
        let state = self.state.lock().unwrap();
        if let SessionState::InProgress { started_at, .. } = *state {
            drop(state);
            *self.state.lock().unwrap() = SessionState::Aborted {
                started_at,
                aborted_at: Instant::now(),
            };
        }
    }

    /// Check if session is completed
    pub fn is_completed(&self) -> bool {
        matches!(*self.state.lock().unwrap(), SessionState::Completed { .. })
    }

    /// Check if session is in progress
    pub fn is_in_progress(&self) -> bool {
        matches!(*self.state.lock().unwrap(), SessionState::InProgress { .. })
    }

    /// Get session duration so far
    pub fn session_duration(&self) -> Option<Duration> {
        match *self.state.lock().unwrap() {
            SessionState::InProgress { started_at, .. } => Some(started_at.elapsed()),
            SessionState::Completed {
                started_at,
                completed_at,
                ..
            } => Some(completed_at.duration_since(started_at)),
            SessionState::Aborted {
                started_at,
                aborted_at,
                ..
            } => Some(aborted_at.duration_since(started_at)),
            _ => None,
        }
    }

    /// Get stage results
    pub fn get_stage_results(&self) -> Vec<StageResult> {
        self.stage_results.lock().unwrap().clone()
    }

    /// Generate SessionResult using proper flow: SessionTracker -> SessionCalculator
    pub fn generate_session_result(&self) -> Option<SessionResult> {
        // Use GLOBAL_SESSION_TRACKER and SessionCalculator for proper flow implementation
        if let Ok(global_session_tracker) = GLOBAL_SESSION_TRACKER.lock() {
            if let Some(ref session_tracker) = *global_session_tracker {
                let result = SessionCalculator::calculate(session_tracker);
                return Some(result);
            }
        }

        None
    }

    // Removed generate_total_result - not used

    /// Record session to database and update total tracker
    pub fn record_and_update_trackers(&self) -> Result<()> {
        if let Some(session_result) = self.generate_session_result() {
            // Record session to database
            self.record_session_to_database(&session_result)?;

            // Record session result in GLOBAL_TOTAL_TRACKER
            if let Ok(mut global_total_tracker) = GLOBAL_TOTAL_TRACKER.lock() {
                if let Some(ref mut tracker) = global_total_tracker.as_mut() {
                    tracker.record(session_result);
                }
            }
        }
        Ok(())
    }

    /// Record session to database
    fn record_session_to_database(&self, session_result: &SessionResult) -> Result<()> {
        // Get game mode and difficulty from global repositories or session config
        let game_mode = format!("{:?}", self.config.lock().unwrap().difficulty);

        let difficulty_level = Some(format!("{:?}", self.config.lock().unwrap().difficulty));

        // Use git repository from session context
        let git_repository = self.git_repository.lock().unwrap();

        // Clone stage_trackers and session_challenges to avoid holding lock
        let stage_trackers = self.stage_trackers.lock().unwrap().clone();
        let session_challenges = self.session_challenges.lock().unwrap().clone();

        // Call SessionRepository to save to database
        SessionRepository::record_session_global(
            session_result,
            git_repository.as_ref(),
            &game_mode,
            difficulty_level.as_deref(),
            &stage_trackers,
            &session_challenges,
        )?;

        Ok(())
    }

    /// Add completed session to TotalTracker
    fn add_session_to_total_tracker(&self) -> Result<()> {
        if let Some(session_result) = self.generate_session_result() {
            // Record session result in GLOBAL_TOTAL_TRACKER
            if let Ok(mut global_total_tracker) = GLOBAL_TOTAL_TRACKER.lock() {
                if let Some(ref mut tracker) = global_total_tracker.as_mut() {
                    tracker.record(session_result);
                }
            }
        }
        Ok(())
    }

    pub fn reset(&self) {
        *self.state.lock().unwrap() = SessionState::NotStarted;
        self.stage_results.lock().unwrap().clear();
        *self.current_stage_tracker.lock().unwrap() = None;
        self.stage_trackers.lock().unwrap().clear();
        self.session_challenges.lock().unwrap().clear();
        *self.best_records_at_start.lock().unwrap() = None;

        // Clear global session tracker
        let _ = GLOBAL_SESSION_TRACKER.lock().map(|mut tracker| {
            *tracker = None;
        });
    }

    // ============================================
    // Challenge Management (delegated to StageRepository)
    // ============================================

    /// Get the next challenge for the current stage using StageRepository
    pub fn get_next_challenge(&self) -> Result<Option<Challenge>> {
        if matches!(*self.state.lock().unwrap(), SessionState::InProgress { .. }) {
            // Get challenge from StageRepository based on current difficulty setting
            let stage_repo = self.stage_repository.as_any()
                .downcast_ref::<StageRepository>()
                .ok_or_else(|| GitTypeError::TerminalError("Failed to downcast StageRepository".to_string()))?;
            Ok(stage_repo.get_challenge_for_difficulty(self.config.lock().unwrap().difficulty))
        } else {
            Ok(None)
        }
    }

    // ============================================
    // StageTracker Management Methods
    // ============================================

    /// Get current stage number (used by global API)
    fn current_stage(&self) -> usize {
        match *self.state.lock().unwrap() {
            SessionState::InProgress { current_stage, .. } => current_stage,
            SessionState::Completed { .. } => {
                // For completed sessions, return the number of successfully completed stages
                let completed = self
                    .stage_results
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|sr| !sr.was_skipped && !sr.was_failed)
                    .count();
                completed.max(1).min(self.config.lock().unwrap().max_stages)
            }
            _ => 0,
        }
    }

    /// Get total stages (used by global API)
    fn total_stages(&self) -> usize {
        self.config.lock().unwrap().max_stages
    }

    /// Initialize stage tracker (used by global API)
    fn init_stage_tracker(
        &self,
        target_text: String,
        challenge_path: Option<String>,
    ) -> Result<()> {
        *self.current_stage_tracker.lock().unwrap() = Some(match challenge_path {
            Some(path) => StageTracker::new_with_path(target_text, path),
            None => StageTracker::new(target_text),
        });
        Ok(())
    }

    /// Record stage input (used by global API)
    fn record_stage_input(&self, input: StageInput) -> Result<()> {
        if let Some(ref mut tracker) = *self.current_stage_tracker.lock().unwrap() {
            tracker.record(input);
        }
        Ok(())
    }

    /// Set stage start time (used by global API)
    fn set_stage_start_time(&self, start_time: Instant) -> Result<()> {
        if let Some(ref mut tracker) = *self.current_stage_tracker.lock().unwrap() {
            tracker.set_start_time(start_time);
        }
        Ok(())
    }

    /// Get a copy of the current stage tracker
    pub fn get_current_stage_tracker(&self) -> Option<StageTracker> {
        self.current_stage_tracker.lock().unwrap().clone()
    }

    /// Get a copy of the current stage tracker (deprecated, use get_current_stage_tracker instead)
    pub fn get_current_stage_tracker_mut(&self) -> Option<StageTracker> {
        self.current_stage_tracker.lock().unwrap().clone()
    }

    /// Complete the current stage and calculate results
    /// Flow: StageTracker -> StageCalculator -> SessionTracker -> SessionCalculator
    pub fn skip_current_stage(&self) -> Result<(StageResult, usize, bool)> {
        if self.get_skips_remaining()? == 0 {
            return Err(GitTypeError::TerminalError(
                "No skips remaining".to_string(),
            ));
        }

        match *self.state.lock().unwrap() {
            SessionState::InProgress { .. } => {
                // Record skip event and finalize current stage tracker
                let mut tracker_guard = self.current_stage_tracker.lock().unwrap();
                if let Some(ref mut tracker) = *tracker_guard {
                    tracker.record(StageInput::Skip);
                    let mut stage_result = StageCalculator::calculate(tracker);
                    stage_result.was_skipped = true;

                    // Record in global session tracker
                    if let Ok(mut global_session_tracker) = GLOBAL_SESSION_TRACKER.lock() {
                        if let Some(ref mut session_tracker) = *global_session_tracker {
                            session_tracker.record(stage_result.clone());
                        }
                    }

                    // Collect data before borrowing conflicts - move tracker out
                    let tracker_clone = tracker_guard.clone();
                    drop(tracker_guard);
                    let current_challenge = self.get_current_challenge().ok().flatten();
                    let stage_name = format!("Stage {}", self.current_stage());

                    // Clear current stage tracker for new challenge
                    *self.current_stage_tracker.lock().unwrap() = None;

                    // Add stage data to session before updating results
                    if let Some(tracker) = tracker_clone {
                        if let Some(challenge) = current_challenge {
                            self.stage_trackers.lock().unwrap().push((stage_name.clone(), tracker.clone()));
                            self.session_challenges.lock().unwrap().push(challenge);
                        } else {
                            self.stage_trackers.lock().unwrap().push((stage_name, tracker));
                        }
                    }

                    // Add skipped stage to results (don't advance stage)
                    self.stage_results.lock().unwrap().push(stage_result.clone());

                    // Return true to indicate new challenge should be generated
                    let skips_remaining = self.get_skips_remaining()?;
                    Ok((stage_result, skips_remaining, true))
                } else {
                    Err(GitTypeError::TerminalError(
                        "No active stage tracker to skip".to_string(),
                    ))
                }
            }
            _ => Err(GitTypeError::TerminalError(
                "Cannot skip stage: Session is not in progress".to_string(),
            )),
        }
    }

    pub fn finalize_current_stage(&self) -> Result<StageResult> {
        let mut tracker_guard = self.current_stage_tracker.lock().unwrap();
        if let Some(ref mut tracker) = *tracker_guard {
            // 1. StageTracker: Record finish event
            tracker.record(StageInput::Finish);

            // 2. StageCalculator: Calculate stage result from StageTracker
            let stage_result = StageCalculator::calculate(tracker);

            // 3. SessionTracker: Record stage result in global session tracker
            if let Ok(mut global_session_tracker) = GLOBAL_SESSION_TRACKER.lock() {
                if let Some(ref mut session_tracker) = *global_session_tracker {
                    session_tracker.record(stage_result.clone());
                }
            }

            // 4. Collect data before borrowing conflicts - clone tracker
            let tracker_clone = Some(tracker.clone());
            drop(tracker_guard);
            let current_challenge = self.get_current_challenge().ok().flatten();
            let stage_name = format!("Stage {}", self.current_stage());

            // Clear current stage tracker to avoid borrow issues
            *self.current_stage_tracker.lock().unwrap() = None;

            // 5. Add stage data to session
            if let Some(tracker) = tracker_clone {
                if let Some(challenge) = current_challenge {
                    self.stage_trackers.lock().unwrap().push((stage_name.clone(), tracker.clone()));
                    self.session_challenges.lock().unwrap().push(challenge);
                } else {
                    self.stage_trackers.lock().unwrap().push((stage_name, tracker));
                }
            }

            // Update SessionManager state using reducer pattern
            self.reduce(SessionAction::CompleteStage(stage_result.clone()))?;

            Ok(stage_result)
        } else {
            Err(GitTypeError::TerminalError(
                "No active stage tracker to finalize".to_string(),
            ))
        }
    }

    // ============================================
    // SessionTracker Management Methods
    // ============================================

    /// Get session state for debugging
    pub fn get_state(&self) -> SessionState {
        self.state.lock().unwrap().clone()
    }

    /// Get session result (instance method)
    pub fn get_session_result(&self) -> Option<SessionResult> {
        self.generate_session_result()
    }

    /// Set difficulty level for the session
    pub fn set_difficulty(&self, difficulty: DifficultyLevel) {
        self.config.lock().unwrap().difficulty = difficulty;
    }

    /// Get current difficulty level
    pub fn get_difficulty(&self) -> DifficultyLevel {
        self.config.lock().unwrap().difficulty
    }

    // Global methods removed - use instance methods instead through DI

    // Global methods removed - all functionality available through instance methods
    // Use dependency injection to get SessionManager instance
}
