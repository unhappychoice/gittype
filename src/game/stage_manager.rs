use super::{
    screens::{
        exit_summary_screen::ExitAction, session_summary_screen::ResultAction,
        typing_screen::SessionState, CancelScreen, CountdownScreen, ExitSummaryScreen,
        FailureScreen, SessionSummaryScreen, SharingScreen, TitleAction, TitleScreen, TypingScreen,
    },
    session_tracker::SessionTracker,
    stage_builder::{DifficultyLevel, GameMode, StageBuilder},
    total_tracker::TotalTracker,
};
use crate::models::Challenge;
use crate::models::{GitRepository, SessionResult};
use crate::scoring::{ScoringEngine, StageResult};
use crate::storage::SessionRepository;
use crate::Result;
use crossterm::{
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute, terminal,
};
use once_cell::sync::Lazy;
use std::io::stdout;
use std::sync::{Arc, Mutex};

// Raw mode cleanup guard to ensure raw mode is disabled on drop
struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        cleanup_terminal();
    }
}

// Global session tracker for Ctrl+C handler
static GLOBAL_SESSION_TRACKER: Lazy<Arc<Mutex<Option<SessionTracker>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

// Global total tracker for game-wide statistics
static GLOBAL_TOTAL_TRACKER: Lazy<Arc<Mutex<Option<TotalTracker>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub struct StageManager {
    available_challenges: Vec<Challenge>,
    current_challenges: Vec<Challenge>,
    current_stage: usize,
    stage_engines: Vec<(String, ScoringEngine)>,
    current_game_mode: Option<GameMode>,
    session_tracker: SessionTracker,
    total_tracker: TotalTracker,
    git_repository: Option<GitRepository>,
    skips_remaining: usize,
}

impl StageManager {
    pub fn new(challenges: Vec<Challenge>) -> Self {
        Self {
            available_challenges: challenges,
            current_challenges: Vec::new(),
            current_stage: 0,
            stage_engines: Vec::new(),
            current_game_mode: None,
            session_tracker: SessionTracker::new(),
            total_tracker: TotalTracker::new(),
            git_repository: None,
            skips_remaining: 3,
        }
    }

    pub fn set_git_repository(&mut self, git_repository: Option<GitRepository>) {
        self.git_repository = git_repository;
    }

    pub fn run_session(&mut self) -> Result<()> {
        // Set global trackers for Ctrl+C handler
        {
            let mut global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
            *global_tracker = Some(self.session_tracker.clone());
        }
        {
            let mut global_total_tracker = GLOBAL_TOTAL_TRACKER.lock().unwrap();
            *global_total_tracker = Some(self.total_tracker.clone());
        }

        // Enable raw mode for entire application session
        match terminal::enable_raw_mode() {
            Ok(_) => {}
            Err(e) => {
                return Err(crate::error::GitTypeError::TerminalError(format!(
                    "Failed to enable raw mode: {}",
                    e
                )));
            }
        }

        // Create guard to ensure raw mode is disabled on function exit
        let _raw_mode_guard = RawModeGuard;

        // Enable keyboard enhancement flags to better detect modifier combinations
        let mut stdout_handle = stdout();
        execute!(
            stdout_handle,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
            )
        )
        .ok(); // Ignore errors in case terminal doesn't support it

        loop {
            // Count challenges by difficulty level
            let challenge_counts = self.count_challenges_by_difficulty();

            match TitleScreen::show_with_challenge_counts_and_git_repository(
                &challenge_counts,
                self.git_repository.as_ref(),
            )? {
                TitleAction::Start(difficulty) => {
                    // Build stages based on selected difficulty using pre-generated challenges
                    let game_mode = GameMode::Custom {
                        max_stages: Some(3),
                        time_limit: None,
                        difficulty: difficulty.clone(),
                    };

                    self.current_game_mode = Some(game_mode.clone());

                    loop {
                        let stage_builder = StageBuilder::with_mode(game_mode.clone());
                        self.current_challenges =
                            stage_builder.build_stages(self.available_challenges.clone());

                        if self.current_challenges.is_empty() {
                            break; // Go back to title screen
                        }

                        // Reset session state for new session
                        self.reset_session_state();

                        match self.run_stages() {
                            Ok(session_complete) => {
                                if !session_complete {
                                    // Session was incomplete (user quit or back to title)
                                    // The session should have been recorded already in handle_*_result_navigation
                                    // or other failure points, so just break
                                    break; // User chose to quit or back to title
                                }
                                // If session_complete is true, retry with same settings
                            }
                            Err(e) => {
                                cleanup_terminal();
                                return Err(e);
                            }
                        }
                    }
                }
                TitleAction::Quit => {
                    // Show total summary before exiting
                    let total_summary = self.total_tracker.clone().finalize_and_get_total();
                    let _ = ExitSummaryScreen::show_total(&total_summary)?;
                    cleanup_terminal();
                    std::process::exit(0);
                }
            }
        }

        #[allow(unreachable_code)]
        {
            // Clear global session tracker
            let mut global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
            *global_tracker = None;
        }

        // Disable keyboard enhancement flags
        let mut stdout_handle = stdout();
        execute!(stdout_handle, PopKeyboardEnhancementFlags).ok();

        cleanup_terminal();
        Ok(())
    }

    fn run_stages(&mut self) -> Result<bool> {
        while self.current_stage < self.current_challenges.len() {
            let challenge = &self.current_challenges[self.current_stage];

            // Show countdown before each stage
            if self.current_stage == 0 {
                // First stage - show initial countdown with challenge info
                CountdownScreen::show_with_challenge_and_repo(
                    Some(challenge),
                    &self.git_repository,
                )?;
            } else {
                // Subsequent stages - show stage transition countdown with challenge info
                CountdownScreen::show_stage_transition_with_challenge_and_repo(
                    self.current_stage + 1,
                    self.current_challenges.len(),
                    Some(challenge),
                    &self.git_repository,
                )?;
            }

            let mut screen =
                TypingScreen::new_with_challenge(challenge, self.git_repository.clone())?;
            screen.set_skips_remaining(self.skips_remaining);
            let (stage_result, final_state) = screen.show_with_state()?;
            self.skips_remaining = screen.get_skips_remaining();

            // Handle different exit states
            match final_state {
                SessionState::Complete => {
                    // Normal completion - advance to next stage
                    let stage_name = challenge.get_display_title();
                    let engine = screen.get_scoring_engine().clone();

                    self.stage_engines
                        .push((stage_name.clone(), engine.clone()));

                    // Track in session tracker
                    self.session_tracker.record_stage_completion(
                        stage_name,
                        stage_result.clone(),
                        &engine,
                    );

                    // Update global session tracker
                    {
                        let mut global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
                        if let Some(ref mut tracker) = *global_tracker {
                            *tracker = self.session_tracker.clone();
                        }
                    }

                    // Show brief result and auto-advance
                    if let Some(ResultAction::Quit) = self.show_stage_completion(&stage_result)? {
                        // Treat as failed - show fail result screen and handle navigation
                        return self.handle_fail_result_navigation();
                    }

                    // Move to next stage
                    self.current_stage += 1;
                }
                SessionState::Skip => {
                    // Skipped - record skip and partial effort
                    let engine = screen.get_scoring_engine();
                    self.session_tracker.record_skip();
                    self.session_tracker
                        .record_partial_effort(engine, &stage_result);

                    // Update global session tracker
                    {
                        let mut global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
                        if let Some(ref mut tracker) = *global_tracker {
                            *tracker = self.session_tracker.clone();
                        }
                    }

                    if let Some(ResultAction::Quit) = self.show_stage_completion(&stage_result)? {
                        // Treat as failed - show fail result screen and handle navigation
                        return self.handle_fail_result_navigation();
                    }

                    // Generate a new challenge for the current stage
                    if let Some(ref game_mode) = self.current_game_mode {
                        let stage_builder = StageBuilder::with_mode(game_mode.clone());
                        let new_challenges =
                            stage_builder.build_stages(self.available_challenges.clone());

                        if !new_challenges.is_empty() && self.current_stage < new_challenges.len() {
                            // Replace current challenge with a new one
                            self.current_challenges[self.current_stage] =
                                new_challenges[self.current_stage].clone();
                        }
                    }
                    // Don't increment current_stage - retry same stage with new challenge
                }
                SessionState::Failed => {
                    // Failed - show fail result screen with navigation options
                    let stage_name = challenge.get_display_title();
                    let engine = screen.get_scoring_engine().clone();

                    self.stage_engines
                        .push((stage_name.clone(), engine.clone()));

                    // Track in session tracker
                    self.session_tracker.record_stage_completion(
                        stage_name,
                        stage_result.clone(),
                        &engine,
                    );

                    // Show fail result screen and handle navigation
                    let should_retry = self.handle_fail_result_navigation()?;
                    if should_retry {
                        // Retry the same stage - don't increment current_stage
                        continue;
                    } else {
                        // Exit to title or quit
                        return Ok(false);
                    }
                }
                SessionState::Exit => {
                    // User wants to exit - record partial effort only
                    let engine = screen.get_scoring_engine();
                    self.session_tracker
                        .record_partial_effort(engine, &stage_result);

                    // Update global session tracker with current state
                    {
                        let mut global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
                        if let Some(ref mut tracker) = *global_tracker {
                            *tracker = self.session_tracker.clone();
                        }
                    }

                    // Show cancel result screen and handle navigation
                    let should_retry = self.handle_cancel_result_navigation()?;
                    if should_retry {
                        // Retry the same stage - don't increment current_stage
                        continue;
                    } else {
                        // Exit to title or quit
                        return Ok(false);
                    }
                }
                SessionState::Continue | SessionState::ShowDialog => {
                    // This shouldn't happen in final state
                    unreachable!("Continue/ShowDialog state should not be final");
                }
            }
        }

        // All stages completed - show final results (raw mode still enabled)
        let mut first_show = true;
        loop {
            let action = if first_show {
                first_show = false;
                self.show_session_summary()?
            } else {
                self.show_session_summary_no_animation()?
            };

            match action {
                ResultAction::Retry => {
                    // Record session attempt in total tracker before retry
                    let session_summary = self.session_tracker.clone().finalize_and_get_summary();

                    // Record session to database
                    self.record_session_to_database(&session_summary);

                    self.total_tracker.record_session_attempt(&session_summary);

                    // Update global total tracker
                    {
                        let mut global_total_tracker = GLOBAL_TOTAL_TRACKER.lock().unwrap();
                        if let Some(ref mut tracker) = *global_total_tracker {
                            *tracker = self.total_tracker.clone();
                        }
                    }

                    // Reset session state for session retry
                    self.reset_session_state();
                    return Ok(true); // Return true to indicate retry requested
                }
                ResultAction::Share => {
                    // Show sharing menu with combined engine metrics (same as result screen)
                    if !self.stage_engines.is_empty() {
                        let combined_engine = self
                            .stage_engines
                            .iter()
                            .map(|(_, engine)| engine.clone())
                            .reduce(|acc, engine| acc + engine)
                            .unwrap();

                        if let Ok(session_metrics) = combined_engine.calculate_result() {
                            let _ = SharingScreen::show_sharing_menu(
                                &session_metrics,
                                &self.git_repository,
                            );
                        }
                    }
                    // Continue showing the summary screen after sharing (without animation)
                    continue;
                }
                ResultAction::Quit => {
                    // Show session summary before exiting
                    let session_summary = self.session_tracker.clone().finalize_and_get_summary();

                    // Record session to database
                    self.record_session_to_database(&session_summary);

                    // Record completed session in total tracker
                    self.total_tracker
                        .record_session_completion(&session_summary);

                    // Update global total tracker
                    {
                        let mut global_total_tracker = GLOBAL_TOTAL_TRACKER.lock().unwrap();
                        if let Some(ref mut tracker) = *global_total_tracker {
                            *tracker = self.total_tracker.clone();
                        }
                    }

                    loop {
                        let total_summary = self.total_tracker.clone().finalize_and_get_total();
                        let exit_action = ExitSummaryScreen::show(&total_summary)?;

                        match exit_action {
                            ExitAction::Exit => {
                                cleanup_terminal();
                                std::process::exit(0);
                            }
                            ExitAction::Share => {
                                // Sharing menu disabled for now
                                // Continue showing exit screen after sharing
                                continue;
                            }
                        }
                    }
                }
                _ => return Ok(false), // Return false for back to title
            }
        }
    }

    fn show_stage_completion(&self, stage_result: &StageResult) -> Result<Option<ResultAction>> {
        // Get keystrokes from the latest scoring engine
        let keystrokes = if let Some((_, engine)) = self.stage_engines.last() {
            engine.total_chars()
        } else {
            0
        };

        SessionSummaryScreen::show_stage_completion(
            stage_result,
            self.current_stage + 1,
            self.current_challenges.len(),
            self.current_stage < self.current_challenges.len() - 1, // has_next_stage
            keystrokes,
        )
    }

    fn show_session_summary(&self) -> Result<ResultAction> {
        self.show_session_summary_internal(true)
    }

    fn show_session_summary_no_animation(&self) -> Result<ResultAction> {
        self.show_session_summary_internal(false)
    }

    fn show_session_summary_internal(&self, show_animation: bool) -> Result<ResultAction> {
        if show_animation {
            SessionSummaryScreen::show_session_summary_with_input(
                self.current_challenges.len(),
                self.stage_engines.len(),
                &self.stage_engines,
                &self.git_repository,
            )
        } else {
            SessionSummaryScreen::show_session_summary_with_input_no_animation(
                self.current_challenges.len(),
                self.stage_engines.len(),
                &self.stage_engines,
                &self.git_repository,
            )
        }
    }

    pub fn get_current_stage(&self) -> usize {
        self.current_stage
    }

    pub fn get_total_stages(&self) -> usize {
        self.current_challenges.len()
    }

    /// Reset session state for session retry (keeps available challenges and total stats intact)
    fn reset_session_state(&mut self) {
        self.current_stage = 0;
        self.stage_engines.clear();
        self.skips_remaining = 3;
        self.session_tracker = SessionTracker::new(); // Reset session tracking data only
                                                      // Note: total_tracker is NOT reset, as retry doesn't affect total game statistics
    }

    fn handle_fail_result_navigation(&mut self) -> Result<bool> {
        use crate::game::screens::ResultAction;

        // Show fail result screen and get user action
        let action = FailureScreen::show_session_summary_fail_mode(
            self.current_challenges.len(),
            self.stage_engines.len(),
            &self.stage_engines,
            &self.git_repository,
        )?;

        match action {
            ResultAction::Retry => {
                // Record session attempt in total tracker before retry
                let session_summary = self.session_tracker.clone().finalize_and_get_summary();

                // Record session to database
                self.record_session_to_database(&session_summary);

                self.total_tracker.record_session_attempt(&session_summary);

                // Reset session state when retrying
                self.reset_session_state();
                // Restart the same challenge
                Ok(true) // Return true to indicate retry
            }
            ResultAction::BackToTitle => {
                // Record attempted session before going back to title
                let session_summary = self.session_tracker.clone().finalize_and_get_summary();

                // Record session to database
                self.record_session_to_database(&session_summary);

                self.total_tracker.record_session_attempt(&session_summary);

                // Back to title screen
                Ok(false)
            }
            ResultAction::Quit => {
                // Show session summary and exit
                let session_summary = self.session_tracker.clone().finalize_and_get_summary();

                // Record session to database
                self.record_session_to_database(&session_summary);

                // Record attempted session in total tracker (failed session)
                self.total_tracker.record_session_attempt(&session_summary);

                let total_summary = self.total_tracker.clone().finalize_and_get_total();
                let _ = ExitSummaryScreen::show(&total_summary)?;
                cleanup_terminal();
                std::process::exit(0);
            }
            _ => {
                // Record attempted session for unknown actions, default to back to title
                let session_summary = self.session_tracker.clone().finalize_and_get_summary();

                // Record session to database
                self.record_session_to_database(&session_summary);

                self.total_tracker.record_session_attempt(&session_summary);

                // For other actions, default to back to title
                Ok(false)
            }
        }
    }

    fn handle_cancel_result_navigation(&mut self) -> Result<bool> {
        use crate::game::screens::ResultAction;

        // Show cancel result screen and get user action
        let action = CancelScreen::show_session_summary_cancel_mode(
            self.current_challenges.len(),
            self.stage_engines.len(),
            &self.stage_engines,
            &self.git_repository,
        )?;

        match action {
            ResultAction::Retry => {
                // Record session attempt in total tracker before retry
                let session_summary = self.session_tracker.clone().finalize_and_get_summary();

                // Record session to database
                self.record_session_to_database(&session_summary);

                self.total_tracker.record_session_attempt(&session_summary);

                // Reset session state when retrying
                self.reset_session_state();
                // Restart the same challenge
                Ok(true) // Return true to indicate retry
            }
            ResultAction::BackToTitle => {
                // Record attempted session before going back to title
                let session_summary = self.session_tracker.clone().finalize_and_get_summary();

                // Record session to database
                self.record_session_to_database(&session_summary);

                self.total_tracker.record_session_attempt(&session_summary);

                // Back to title screen
                Ok(false)
            }
            ResultAction::Quit => {
                // Show session summary and exit
                let session_summary = self.session_tracker.clone().finalize_and_get_summary();

                // Record session to database
                self.record_session_to_database(&session_summary);

                // Record attempted session in total tracker (failed session)
                self.total_tracker.record_session_attempt(&session_summary);

                let total_summary = self.total_tracker.clone().finalize_and_get_total();
                let _ = ExitSummaryScreen::show(&total_summary)?;
                cleanup_terminal();
                std::process::exit(0);
            }
            _ => {
                // Record attempted session for unknown actions, default to back to title
                let session_summary = self.session_tracker.clone().finalize_and_get_summary();

                // Record session to database
                self.record_session_to_database(&session_summary);

                self.total_tracker.record_session_attempt(&session_summary);

                // For other actions, default to back to title
                Ok(false)
            }
        }
    }

    fn count_challenges_by_difficulty(&self) -> [usize; 5] {
        let mut counts = [0usize; 5];

        for challenge in &self.available_challenges {
            if let Some(ref difficulty) = challenge.difficulty_level {
                match difficulty {
                    DifficultyLevel::Easy => counts[0] += 1,
                    DifficultyLevel::Normal => counts[1] += 1,
                    DifficultyLevel::Hard => counts[2] += 1,
                    DifficultyLevel::Wild => counts[3] += 1,
                    DifficultyLevel::Zen => counts[4] += 1,
                }
            }
        }

        counts
    }

    /// Record a session to the database
    fn record_session_to_database(&self, session_result: &SessionResult) {
        log::debug!(
            "Recording session to database with {} stages completed",
            session_result.stages_completed
        );

        if let Err(e) = self.try_record_session_to_database(session_result) {
            log::warn!("Failed to record session to database: {}", e);
            // Continue execution - database errors should not interrupt the game
        } else {
            log::debug!("Successfully recorded session to database");
        }
    }

    /// Try to record a session to the database, returning any errors
    fn try_record_session_to_database(&self, session_result: &SessionResult) -> Result<()> {
        let game_mode = self
            .current_game_mode
            .as_ref()
            .map(|mode| format!("{:?}", mode))
            .unwrap_or_else(|| "Unknown".to_string());

        let difficulty_level = self.current_game_mode.as_ref().and_then(|mode| match mode {
            GameMode::Custom { difficulty, .. } => Some(format!("{:?}", difficulty)),
            _ => None,
        });

        SessionRepository::record_session_global(
            session_result,
            self.git_repository.as_ref(),
            &game_mode,
            difficulty_level.as_deref(),
            &self.stage_engines,
            &self.current_challenges,
        )?;

        Ok(())
    }
}

// Comprehensive terminal cleanup function
pub fn cleanup_terminal() {
    use crossterm::{event::PopKeyboardEnhancementFlags, execute, terminal};
    use std::io::{stdout, Write};

    // Disable raw mode first
    if let Err(e) = terminal::disable_raw_mode() {
        eprintln!("Warning: Failed to disable raw mode: {}", e);
    }

    // Pop keyboard enhancement flags to avoid leaving terminal in special key-report mode
    // This is especially important for iTerm2 which keeps modes until explicitly reverted
    let _ = execute!(stdout(), PopKeyboardEnhancementFlags);

    // Exit alternate screen and restore cursor with explicit error handling
    if let Err(e) = execute!(
        stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show,
        crossterm::style::ResetColor,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    ) {
        eprintln!("Warning: Failed to cleanup terminal: {}", e);
    }

    // Ensure output is flushed
    let _ = stdout().flush();
}

// Public function for Ctrl+C handler
pub fn show_session_summary_on_interrupt() {
    // Keep raw mode enabled since ExitSummaryScreen needs it for input handling

    let global_tracker = GLOBAL_SESSION_TRACKER.lock().unwrap();
    if let Some(ref _tracker) = *global_tracker {
        // Show total summary with raw mode enabled
        // Try to get total summary from global tracker
        let global_total_tracker = GLOBAL_TOTAL_TRACKER.lock().unwrap();
        if let Some(ref tracker) = *global_total_tracker {
            let total_summary = tracker.clone().finalize_and_get_total();
            let _ = ExitSummaryScreen::show(&total_summary);
        } else {
            // Fallback to show_total method for single session
            let _ = ExitSummaryScreen::show_total(&Default::default());
        }
        // Complete terminal cleanup after ExitSummaryScreen completes
        cleanup_terminal();
    } else {
        // Show simple interruption message
        if let Err(e) = terminal::disable_raw_mode() {
            eprintln!("Warning: Failed to disable raw mode: {}", e);
        }
        use crossterm::{
            cursor::MoveTo,
            execute,
            style::{Color, Print, ResetColor, SetForegroundColor},
        };
        use std::io::stdout;

        let mut stdout = stdout();
        let _ = execute!(
            stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        );
        let _ = execute!(stdout, MoveTo(10, 5));
        let _ = execute!(stdout, SetForegroundColor(Color::Yellow));
        let _ = execute!(
            stdout,
            Print("Interrupted by user - no session data available")
        );
        let _ = execute!(stdout, ResetColor);
        let _ = execute!(stdout, MoveTo(10, 7));
        let _ = execute!(stdout, Print("Thanks for playing GitType!"));
        let _ = execute!(stdout, MoveTo(10, 9));
        let _ = execute!(stdout, SetForegroundColor(Color::Grey));
        let _ = execute!(stdout, Print("[ESC] Exit"));
        let _ = execute!(stdout, ResetColor);

        // Enable raw mode temporarily for input
        let _ = terminal::enable_raw_mode();
        use crossterm::event;
        loop {
            if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
                if let Ok(event::Event::Key(key_event)) = event::read() {
                    if key_event.code == event::KeyCode::Esc {
                        break;
                    }
                }
            }
        }
        cleanup_terminal();
    }
}
