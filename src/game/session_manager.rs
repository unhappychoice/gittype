use crate::scoring::{SessionTracker, SessionTrackerData, StageCalculator, GLOBAL_TOTAL_TRACKER};
use crate::storage::session_repository::BestStatus;
use crate::storage::SessionRepository;
use crate::{
    game::{stage_repository::StageRepository, DifficultyLevel},
    models::{Challenge, SessionResult},
    scoring::{StageInput, StageResult, StageTracker, GLOBAL_SESSION_TRACKER},
    Result,
};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum SessionState {
    NotStarted,
    InProgress {
        current_stage: usize,
        started_at: Instant,
    },
    Completed {
        started_at: Instant,
        completed_at: Instant,
    },
    Aborted {
        started_at: Instant,
        aborted_at: Instant,
    },
}

#[derive(Debug, Clone)]
pub enum SessionAction {
    Start,
    CompleteStage(StageResult),
    Complete,
    Abort,
    Reset,
}

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub max_stages: usize,
    pub session_timeout: Option<Duration>,
    pub difficulty: DifficultyLevel,
    pub max_skips: usize,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_stages: 3,
            session_timeout: None,
            difficulty: DifficultyLevel::Normal,
            max_skips: 3,
        }
    }
}

/// Manages the overall session state and stage progression
/// Singleton pattern for global state management
pub struct SessionManager {
    state: SessionState,
    config: SessionConfig,
    stage_results: Vec<StageResult>,
    // Tracker management
    current_stage_tracker: Option<StageTracker>,
    stage_trackers: Vec<(String, crate::scoring::StageTracker)>,
    // Git repository context
    git_repository: Option<crate::models::GitRepository>,
    // Challenge management - tracks all challenges used in this session (completed, failed, skipped)
    session_challenges: Vec<crate::models::Challenge>,
    // Best records at session start (for accurate comparison)
    best_records_at_start: Option<crate::storage::repositories::session_repository::BestRecords>,
}

static GLOBAL_SESSION_MANAGER: Lazy<Arc<Mutex<SessionManager>>> =
    Lazy::new(|| Arc::new(Mutex::new(SessionManager::new())));

impl SessionManager {
    pub fn new() -> Self {
        Self {
            state: SessionState::NotStarted,
            config: SessionConfig::default(),
            stage_results: Vec::new(),
            current_stage_tracker: None,
            stage_trackers: Vec::new(),
            git_repository: None,
            session_challenges: Vec::new(),
            best_records_at_start: None,
        }
    }

    /// Central state machine reducer - handles all state transitions
    fn reduce(&mut self, action: SessionAction) -> Result<()> {
        log::debug!("SessionManager::reduce - {:?}", action);

        let new_state = match (&self.state, &action) {
            // Start transitions
            (SessionState::NotStarted, SessionAction::Start) => {
                let session_start_time = Instant::now();

                // Capture best records at session start for accurate comparison later
                self.best_records_at_start =
                    SessionRepository::get_best_records_global().ok().flatten();

                log::debug!(
                    "SessionManager::reduce Start: captured best_records_at_start={:?}",
                    self.best_records_at_start
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
                self.stage_results.push(stage_result.clone());

                // Count actually completed stages (not skipped and not failed)
                let completed_stages = self
                    .stage_results
                    .iter()
                    .filter(|sr| !sr.was_skipped && !sr.was_failed)
                    .count();

                if completed_stages >= self.config.max_stages {
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
                self.stage_results.clear();
                self.current_stage_tracker = None;
                self.stage_trackers.clear();
                self.session_challenges.clear();

                // Clear global session tracker
                let _ = GLOBAL_SESSION_TRACKER.lock().map(|mut tracker| {
                    *tracker = None;
                });

                SessionState::NotStarted
            }

            // Invalid transitions
            (state, action) => {
                log::error!("Invalid state transition: {:?} -> {:?}", state, action);
                return Err(crate::GitTypeError::TerminalError(format!(
                    "Invalid session state transition: {:?} with action {:?}",
                    state, action
                )));
            }
        };

        log::debug!(
            "SessionManager::reduce - {:?} -> {:?}",
            self.state,
            new_state
        );
        self.state = new_state;
        Ok(())
    }

    /// Get the global SessionManager instance
    pub fn instance() -> Arc<Mutex<SessionManager>> {
        GLOBAL_SESSION_MANAGER.clone()
    }

    // ============================================
    // Essential Public API (keep only what's actually used)
    // ============================================

    /// Initialize a new session with configuration
    pub fn initialize_session(config: Option<SessionConfig>) -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.config = config.unwrap_or_default();
        manager.state = SessionState::NotStarted;
        manager.stage_results.clear();
        manager.current_stage_tracker = None;
        manager.stage_trackers.clear();
        manager.git_repository = None;
        manager.session_challenges.clear();

        // Capture best records at session start for accurate comparison later
        manager.best_records_at_start = SessionRepository::get_best_records_global().ok().flatten();

        log::debug!(
            "SessionManager::initialize_session: captured best_records_at_start={:?}",
            manager.best_records_at_start
        );

        Ok(())
    }

    /// Set git repository context for the session
    pub fn set_git_repository(git_repository: Option<crate::models::GitRepository>) -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.git_repository = git_repository;
        Ok(())
    }

    /// Add stage data (tracker and challenge) for the current stage
    /// StageResult will be calculated by StageCalculator when needed
    pub fn add_stage_data(
        stage_name: String,
        stage_tracker: StageTracker,
        challenge: Challenge,
    ) -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.stage_trackers.push((stage_name, stage_tracker));
        manager.session_challenges.push(challenge);

        Ok(())
    }

    /// Calculate number of skips used in this session
    pub fn get_skips_used(&self) -> usize {
        self.stage_results
            .iter()
            .filter(|result| result.was_skipped)
            .count()
    }

    /// Calculate remaining skips for this session
    pub fn get_skips_remaining(&self) -> usize {
        let used = self.get_skips_used();
        self.config.max_skips.saturating_sub(used)
    }

    /// Start the session
    fn start_session(&mut self) -> Result<()> {
        match self.state {
            SessionState::NotStarted => {
                let session_start_time = Instant::now();
                self.state = SessionState::InProgress {
                    current_stage: 1,
                    started_at: session_start_time,
                };

                // Initialize global session tracker
                let session_tracker = SessionTracker::new();
                SessionTracker::initialize_global_instance(session_tracker);

                Ok(())
            }
            _ => Err(crate::GitTypeError::TerminalError(
                "Session is already started or completed".to_string(),
            )),
        }
    }

    /// Abort the current session
    pub fn abort_session(&mut self) {
        if let SessionState::InProgress { started_at, .. } = self.state {
            self.state = SessionState::Aborted {
                started_at,
                aborted_at: Instant::now(),
            };
        }
    }

    /// Check if session is completed
    pub fn is_completed(&self) -> bool {
        matches!(self.state, SessionState::Completed { .. })
    }

    /// Check if session is in progress
    pub fn is_in_progress(&self) -> bool {
        matches!(self.state, SessionState::InProgress { .. })
    }

    /// Get session duration so far
    pub fn session_duration(&self) -> Option<Duration> {
        match self.state {
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
    pub fn get_stage_results(&self) -> &[StageResult] {
        &self.stage_results
    }

    /// Generate SessionResult using proper flow: SessionTracker -> SessionCalculator
    pub fn generate_session_result(&self) -> Option<SessionResult> {
        // Use GLOBAL_SESSION_TRACKER and SessionCalculator for proper flow implementation
        if let Ok(global_session_tracker) = GLOBAL_SESSION_TRACKER.lock() {
            if let Some(ref session_tracker) = *global_session_tracker {
                let result =
                    crate::scoring::calculator::SessionCalculator::calculate(session_tracker);
                return Some(result);
            }
        }

        None
    }

    // Removed generate_total_result - not used

    /// Record session to database and update total tracker
    fn record_and_update_trackers(&self) -> Result<()> {
        if let Some(session_result) = self.generate_session_result() {
            // Record session to database
            self.record_session_to_database(&session_result)?;

            // Record session result in GLOBAL_TOTAL_TRACKER
            use crate::scoring::GLOBAL_TOTAL_TRACKER;
            if let Ok(mut global_total_tracker) = GLOBAL_TOTAL_TRACKER.lock() {
                if let Some(ref mut tracker) = global_total_tracker.as_mut() {
                    tracker.record(session_result);
                }
            }
        }
        Ok(())
    }

    /// Record session to database
    fn record_session_to_database(
        &self,
        session_result: &crate::models::SessionResult,
    ) -> Result<()> {
        // Get game mode and difficulty from global repositories or session config
        let game_mode = format!("{:?}", self.config.difficulty);

        let difficulty_level = Some(format!("{:?}", self.config.difficulty));

        // Use git repository from session context
        let git_repository = &self.git_repository;

        // Call SessionRepository to save to database
        SessionRepository::record_session_global(
            session_result,
            git_repository.as_ref(),
            &game_mode,
            difficulty_level.as_deref(),
            &self.stage_trackers,
            &self.session_challenges,
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

    pub fn reset(&mut self) {
        self.state = SessionState::NotStarted;
        self.stage_results.clear();
        self.current_stage_tracker = None;
        self.stage_trackers.clear();
        self.session_challenges.clear();
        self.best_records_at_start = None;

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
        if matches!(self.state, SessionState::InProgress { .. }) {
            // Get challenge from StageRepository based on current difficulty setting
            StageRepository::get_global_challenge_for_difficulty(self.config.difficulty)
        } else {
            Ok(None)
        }
    }

    // ============================================
    // StageTracker Management Methods
    // ============================================

    /// Get current challenge (used by global API)
    fn get_current_challenge(&self) -> Option<Challenge> {
        self.get_next_challenge().unwrap_or_default()
    }

    /// Get current stage number (used by global API)
    fn current_stage(&self) -> usize {
        match self.state {
            SessionState::InProgress { current_stage, .. } => current_stage,
            SessionState::Completed { .. } => {
                // For completed sessions, return the number of successfully completed stages
                let completed = self
                    .stage_results
                    .iter()
                    .filter(|sr| !sr.was_skipped && !sr.was_failed)
                    .count();
                completed.max(1).min(self.config.max_stages)
            }
            _ => 0,
        }
    }

    /// Get total stages (used by global API)
    fn total_stages(&self) -> usize {
        self.config.max_stages
    }

    /// Initialize stage tracker (used by global API)
    fn init_stage_tracker(
        &mut self,
        target_text: String,
        challenge_path: Option<String>,
    ) -> Result<()> {
        use crate::scoring::tracker::StageTracker;

        self.current_stage_tracker = Some(match challenge_path {
            Some(path) => StageTracker::new_with_path(target_text, path),
            None => StageTracker::new(target_text),
        });
        Ok(())
    }

    /// Record stage input (used by global API)
    fn record_stage_input(&mut self, input: StageInput) -> Result<()> {
        if let Some(ref mut tracker) = self.current_stage_tracker {
            tracker.record(input);
        }
        Ok(())
    }

    /// Set stage start time (used by global API)
    fn set_stage_start_time(&mut self, start_time: Instant) -> Result<()> {
        if let Some(ref mut tracker) = self.current_stage_tracker {
            tracker.set_start_time(start_time);
        }
        Ok(())
    }

    /// Get a reference to the current stage tracker
    pub fn get_current_stage_tracker(&self) -> Option<&StageTracker> {
        self.current_stage_tracker.as_ref()
    }

    /// Get a mutable reference to the current stage tracker
    pub fn get_current_stage_tracker_mut(&mut self) -> Option<&mut StageTracker> {
        self.current_stage_tracker.as_mut()
    }

    /// Complete the current stage and calculate results
    /// Flow: StageTracker -> StageCalculator -> SessionTracker -> SessionCalculator
    pub fn finalize_current_stage(&mut self) -> Result<StageResult> {
        if let Some(ref mut tracker) = self.current_stage_tracker {
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
        }

        // 4. Collect data before borrowing conflicts - move tracker out to avoid borrowing issues
        let tracker_clone = self.current_stage_tracker.clone();
        let current_challenge = self.get_current_challenge();
        let stage_name = format!("Stage {}", self.current_stage());

        // Clear current stage tracker to avoid borrow issues
        let stage_result = if let Some(tracker) = self.current_stage_tracker.take() {
            StageCalculator::calculate(&tracker)
        } else {
            return Err(crate::GitTypeError::TerminalError(
                "No active stage tracker to finalize".to_string(),
            ));
        };

        // 5. Add stage data to session
        if let Some(tracker) = tracker_clone {
            if let Some(challenge) = current_challenge {
                self.stage_trackers.push((stage_name, tracker));
                self.session_challenges.push(challenge);
            } else {
                self.stage_trackers.push((stage_name, tracker));
            }
        }

        // Update SessionManager state using reducer pattern
        self.reduce(SessionAction::CompleteStage(stage_result.clone()))?;

        Ok(stage_result)
    }

    // ============================================
    // SessionTracker Management Methods
    // ============================================

    /// Get session tracker data from global tracker
    pub fn get_global_session_tracker_data() -> Result<Option<SessionTrackerData>> {
        if let Ok(global_session_tracker) = GLOBAL_SESSION_TRACKER.lock() {
            if let Some(ref session_tracker) = *global_session_tracker {
                Ok(Some(session_tracker.get_data()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Get session state for debugging
    pub fn get_state(&self) -> &SessionState {
        &self.state
    }

    /// Set difficulty level for the session
    pub fn set_difficulty(&mut self, difficulty: DifficultyLevel) {
        self.config.difficulty = difficulty;
    }

    /// Get current difficulty level
    pub fn get_difficulty(&self) -> &DifficultyLevel {
        &self.config.difficulty
    }

    /// Static convenience methods for global instance
    pub fn start_global_session() -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.start_session()
    }

    pub fn get_global_current_challenge() -> Result<Option<Challenge>> {
        let instance = Self::instance();
        let manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        Ok(manager.get_current_challenge())
    }

    pub fn complete_global_stage(stage_result: StageResult) -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.reduce(SessionAction::CompleteStage(stage_result))?;
        Ok(())
    }

    pub fn get_global_stage_info() -> Result<(usize, usize)> {
        let instance = Self::instance();
        let manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        Ok((manager.current_stage(), manager.total_stages()))
    }

    pub fn is_global_session_completed() -> Result<bool> {
        let instance = Self::instance();
        let manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        Ok(manager.is_completed())
    }

    pub fn is_global_session_in_progress() -> Result<bool> {
        let instance = Self::instance();
        let manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        Ok(manager.is_in_progress())
    }

    pub fn get_global_session_result() -> Result<Option<SessionResult>> {
        let instance = Self::instance();
        let manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        Ok(manager.generate_session_result())
    }

    /// Get best status using session start records
    pub fn get_best_status_for_score(session_score: f64) -> Result<Option<BestStatus>> {
        let instance = Self::instance();
        let manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        log::debug!("SessionManager::get_best_status_for_score: session_score={}, best_records_at_start={:?}", 
                   session_score, manager.best_records_at_start);

        let best_status = SessionRepository::determine_best_status_with_start_records(
            session_score,
            manager.best_records_at_start.as_ref(),
        );

        Ok(Some(best_status))
    }

    // ============================================
    // Global StageTracker Management Methods
    // ============================================

    /// Initialize global stage tracker
    pub fn init_global_stage_tracker(
        target_text: String,
        challenge_path: Option<String>,
    ) -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.init_stage_tracker(target_text, challenge_path)
    }

    /// Record global stage input
    pub fn record_global_stage_input(input: StageInput) -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.record_stage_input(input)
    }

    /// Set global stage start time
    pub fn set_global_stage_start_time(start_time: Instant) -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.set_stage_start_time(start_time)
    }

    /// Finalize current global stage and return result
    pub fn finalize_global_stage() -> Result<StageResult> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.finalize_current_stage()
    }

    /// Get current skips remaining
    pub fn get_global_skips_remaining() -> Result<usize> {
        let instance = Self::instance();
        let manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        Ok(manager.get_skips_remaining())
    }

    /// Skip current global stage - returns (stage_result, skips_remaining, should_generate_new_challenge)
    pub fn skip_global_stage() -> Result<(StageResult, usize, bool)> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        if manager.get_skips_remaining() == 0 {
            return Err(crate::GitTypeError::TerminalError(
                "No skips remaining".to_string(),
            ));
        }

        match manager.state {
            SessionState::InProgress { .. } => {
                // Record skip event and finalize current stage tracker
                if let Some(ref mut tracker) = manager.current_stage_tracker {
                    tracker.record(StageInput::Skip);
                    let mut stage_result = crate::scoring::StageCalculator::calculate(tracker);
                    stage_result.was_skipped = true;

                    // Record in global session tracker
                    if let Ok(mut global_session_tracker) = GLOBAL_SESSION_TRACKER.lock() {
                        if let Some(ref mut session_tracker) = *global_session_tracker {
                            session_tracker.record(stage_result.clone());
                        }
                    }

                    // Collect data before borrowing conflicts - move tracker out
                    let tracker_clone = manager.current_stage_tracker.clone();
                    let current_challenge = manager.get_current_challenge();
                    let stage_name = format!("Stage {}", manager.current_stage());

                    // Clear current stage tracker for new challenge
                    manager.current_stage_tracker = None;

                    // Add stage data to session before updating results
                    if let Some(tracker) = tracker_clone {
                        if let Some(challenge) = current_challenge {
                            manager.stage_trackers.push((stage_name, tracker));
                            manager.session_challenges.push(challenge);
                        } else {
                            manager.stage_trackers.push((stage_name, tracker));
                        }
                    }

                    // Add skipped stage to results (don't advance stage)
                    manager.stage_results.push(stage_result.clone());

                    // Return true to indicate new challenge should be generated
                    let skips_remaining = manager.get_skips_remaining();
                    Ok((stage_result, skips_remaining, true))
                } else {
                    Err(crate::GitTypeError::TerminalError(
                        "No active stage tracker to skip".to_string(),
                    ))
                }
            }
            _ => Err(crate::GitTypeError::TerminalError(
                "Cannot skip stage: Session is not in progress".to_string(),
            )),
        }
    }

    /// Reset global SessionManager instance
    pub fn reset_global_session() -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.reduce(SessionAction::Reset)
    }

    // ============================================
    // Public Event-Based API (3 main events)
    // ============================================

    /// Handle session start event
    pub fn on_session_start() -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.reduce(SessionAction::Start)
    }

    /// Handle stage completion event
    pub fn on_stage_complete() -> Result<()> {
        Ok(())
    }

    /// Handle session completion event (normal completion)
    pub fn on_session_complete() -> Result<()> {
        let instance = Self::instance();
        let manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.record_and_update_trackers()
    }

    /// Handle session retry event (completion + reset)
    pub fn on_session_retry() -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        manager.reset();
        Ok(())
    }

    /// Handle session failure event (record failed stage then abort session)
    pub fn on_session_failure() -> Result<()> {
        let instance = Self::instance();
        let mut manager = instance.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        if let Some(ref mut tracker) = manager.current_stage_tracker {
            tracker.record(StageInput::Fail);
            let stage_result = crate::scoring::StageCalculator::calculate(tracker);
            manager.reduce(SessionAction::CompleteStage(stage_result))?;
        }

        manager.reduce(SessionAction::Abort)
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
