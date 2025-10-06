//! # ScreenManager Architecture
//!
//! The ScreenManager provides a centralized system for managing screen transitions,
//! rendering loops, input handling, and terminal lifecycle in GitType.
//!
//! ## Key Features
//!
//! - **Centralized Rendering Loop**: Single loop manages all screen rendering
//! - **Input Handling**: Centralized input handling with event dispatching
//! - **Screen Management**: Stack-based screen management for dialogs and navigation
//! - **Dual Rendering Support**: Supports both crossterm and ratatui backends
//! - **Flexible Update Strategy**: Screens can define their update frequency needs
//! - **Terminal Lifecycle Management**: Proper terminal setup and cleanup
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use gittype::domain::events::EventBus;
//! use gittype::presentation::game::{ScreenManager, ScreenType};
//! use gittype::presentation::game::screens::title_screen::TitleScreen;
//!
//! fn example() -> gittype::Result<()> {
//!     let event_bus = EventBus::new();
//!     let screen = TitleScreen::new(event_bus.clone());
//!
//!     let mut manager = ScreenManager::new(event_bus);
//!     manager.register_screen(ScreenType::Title, Box::new(screen));
//!     manager.initialize_terminal()?;
//!     manager.run()
//! }
//! ```
//!
use crate::domain::events::EventBus;
use crate::domain::models::{GitRepository, SessionResult, StageResult, TotalResult};
use crate::domain::services::scoring::GLOBAL_TOTAL_TRACKER;
use crate::domain::services::scoring::{TotalCalculator, GLOBAL_SESSION_TRACKER};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::screen_transition_manager::ScreenTransitionManager;
use crate::presentation::game::screens::session_detail_screen::SessionDisplayData;
use crate::presentation::game::screens::{
    AnalyticsScreen, AnimationScreen, HelpScreen, InfoDialogScreen, LoadingScreen, PanicScreen,
    RecordsScreen, SessionDetailScreen, SessionDetailsDialog, SessionFailureScreen,
    SessionSummaryScreen, SessionSummaryShareScreen, SettingsScreen, StageSummaryScreen,
    TitleAction, TitleScreen, TotalSummaryScreen, TotalSummaryShareScreen, VersionCheckScreen,
};
use crate::presentation::game::{GameData, Screen, ScreenTransition, ScreenType, SessionManager, StageRepository, TypingScreen, UpdateStrategy};
use crate::{GitTypeError, Result};
use crossterm::event::{Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::collections::HashMap;
use std::io::{stdout, Stdout, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Central manager for screen transitions, rendering, and input handling
pub struct ScreenManager {
    screens: HashMap<ScreenType, Box<dyn Screen>>,
    screen_stack: Vec<ScreenType>,
    current_screen_type: ScreenType,
    terminal_initialized: bool,
    last_update: Instant,
    render_backend: RenderBackend,
    ratatui_terminal: Option<ratatui::Terminal<ratatui::backend::CrosstermBackend<Stdout>>>,
    exit_requested: bool,

    // Shared data for screens
    pub shared_session_result: Option<SessionResult>,
    pub shared_git_repository: Option<GitRepository>,
    pub shared_total_result: Option<TotalResult>,
    pub shared_stage_result: Option<StageResult>,

    // Pending screen transition - shared across threads
    pending_transition: Arc<Mutex<Option<ScreenTransition>>>,

    // Event bus for UI events
    event_bus: EventBus,
}

/// Rendering backend options
#[derive(Debug, Clone, Copy)]
pub enum RenderBackend {
    /// Use crossterm for rendering (default)
    Crossterm,
    /// Use ratatui for rendering
    Ratatui,
}

impl ScreenManager {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            screens: HashMap::new(),
            screen_stack: Vec::new(),
            current_screen_type: ScreenType::Title,
            terminal_initialized: false,
            last_update: Instant::now(),
            render_backend: RenderBackend::Crossterm,
            ratatui_terminal: None,
            exit_requested: false,
            shared_session_result: None,
            shared_git_repository: None,
            shared_total_result: None,
            shared_stage_result: None,
            pending_transition: Arc::new(Mutex::new(None)),
            event_bus: event_bus.clone(),
        }
    }

    pub fn get_event_bus(&self) -> EventBus {
        self.event_bus.clone()
    }

    /// Set up event subscriptions for navigation events
    /// Takes a weak reference to avoid circular references
    pub fn setup_event_subscriptions(manager_ref: &Arc<Mutex<Self>>) {
        let manager_weak = Arc::downgrade(manager_ref);
        let event_bus = {
            let manager = manager_ref.lock().unwrap();
            manager.event_bus.clone()
        }; // Release lock before subscribing

        let pending_transition = {
            let manager = manager_ref.lock().unwrap();
            manager.pending_transition.clone()
        };

        // Subscribe to NavigateTo events
        {
            let pending_transition_clone = pending_transition.clone();
            event_bus.subscribe(move |event: &NavigateTo| {
                pending_transition_clone.lock().ok().map(|mut pending| {
                    *pending = Some(event.clone());
                });
            });
        }

        // Subscribe to ExitRequested events
        {
            let manager_weak_clone = manager_weak.clone();
            event_bus.subscribe(move |_event: &crate::presentation::game::events::ExitRequested| {
                manager_weak_clone.upgrade().map(|arc| {
                    arc.lock().ok().map(|mut manager| {
                        manager.show_session_summary_on_interrupt();
                        std::process::exit(0);
                    });
                });
            });
        }
    }

    /// Initialize ScreenManager with all screens
    pub fn initialize_all_screens(&mut self) -> Result<()> {
        // Event subscriptions are already set up in new()

        self.register_screen(ScreenType::Title, Box::new(TitleScreen::new(self.event_bus.clone())));
        self.register_screen(ScreenType::Loading, Box::new(LoadingScreen::new(self.event_bus.clone())?));
        self.register_screen(ScreenType::SessionFailure, Box::new(SessionFailureScreen::new(self.event_bus.clone())));
        self.register_screen(
            ScreenType::StageSummary,
            Box::new(StageSummaryScreen::new(self.event_bus.clone())),
        );
        self.register_screen(
            ScreenType::SessionSummary,
            Box::new(SessionSummaryScreen::new(self.event_bus.clone())),
        );
        self.register_screen(
            ScreenType::TotalSummary,
            Box::new(TotalSummaryScreen::new(self.event_bus.clone())),
        );
        if let Ok(typing_screen) = TypingScreen::new(self.event_bus.clone()) {
            self.register_screen(ScreenType::Typing, Box::new(typing_screen));
        }
        self.register_screen(
            ScreenType::SessionDetail,
            Box::new(SessionDetailScreen::new_for_screen_manager(self.event_bus.clone()).unwrap()),
        );
        self.register_screen(ScreenType::Animation, Box::new(AnimationScreen::new(self.event_bus.clone())));
        self.register_screen(
            ScreenType::VersionCheck,
            Box::new(VersionCheckScreen::new(self.event_bus.clone())),
        );
        self.register_screen(
            ScreenType::SessionSharing,
            Box::new(SessionSummaryShareScreen::new(self.event_bus.clone())),
        );
        self.register_screen(
            ScreenType::TotalSummaryShare,
            Box::new(TotalSummaryShareScreen::new(
                TotalResult::new(), // Placeholder - will be updated when transitioning
                self.event_bus.clone(),
            )),
        );
        self.register_screen(ScreenType::InfoDialog, Box::new(InfoDialogScreen::new(self.event_bus.clone())));
        self.register_screen(ScreenType::Help, Box::new(HelpScreen::new(self.event_bus.clone())));
        self.register_screen(
            ScreenType::DetailsDialog,
            Box::new(SessionDetailsDialog::new(self.event_bus.clone())),
        );
        self.register_screen(ScreenType::Panic, Box::new(PanicScreen::new(self.event_bus.clone())));

        if let Ok(records_screen) = RecordsScreen::new_for_screen_manager(self.event_bus.clone()) {
            self.register_screen(ScreenType::Records, Box::new(records_screen));
        }

        if let Ok(analytics_screen) = AnalyticsScreen::new_for_screen_manager(self.event_bus.clone()) {
            self.register_screen(ScreenType::Analytics, Box::new(analytics_screen));
        }

        self.register_screen(ScreenType::Settings, Box::new(SettingsScreen::new(self.event_bus.clone())));

        Ok(())
    }

    /// Register a screen with the manager
    pub fn register_screen(&mut self, screen_type: ScreenType, screen: Box<dyn Screen>) {
        self.screens.insert(screen_type, screen);
    }

    /// Get current total result from GLOBAL_TOTAL_TRACKER
    fn get_current_total_result(&self) -> Option<TotalResult> {
        if let Ok(global_total_tracker) = GLOBAL_TOTAL_TRACKER.lock() {
            (*global_total_tracker)
                .as_ref()
                .map(TotalCalculator::calculate)
        } else {
            None
        }
    }

    /// Set the rendering backend (crossterm or ratatui)
    pub fn set_render_backend(&mut self, backend: RenderBackend) {
        self.render_backend = backend;
    }

    /// Initialize terminal for raw mode and alternate screen
    pub fn initialize_terminal(&mut self) -> Result<()> {
        if !self.terminal_initialized {
            use crossterm::{cursor, execute, terminal};
            use std::io::stdout;

            // Check if we're running in a valid terminal environment using atty
            if !atty::is(atty::Stream::Stdout) {
                return Err(GitTypeError::TerminalError(
                    "Not running in a terminal environment. Please run in a proper terminal."
                        .to_string(),
                ));
            }

            // Enable raw mode with better error handling for WSL
            match terminal::enable_raw_mode() {
                Ok(()) => {}
                Err(e) => {
                    // In WSL, sometimes raw mode fails initially, try after a short delay
                    std::thread::sleep(Duration::from_millis(10));
                    terminal::enable_raw_mode().map_err(|e2| {
                        GitTypeError::TerminalError(format!(
                            "Failed to enable raw mode: {} (retry also failed: {})",
                            e, e2
                        ))
                    })?;
                }
            }

            // Try to enter alternate screen, but continue without it if it fails
            match execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Warning: Could not enter alternate screen mode: {}. Running in normal mode.", e);
                    // Try to at least hide the cursor
                    if let Err(e2) = execute!(stdout(), cursor::Hide) {
                        eprintln!("Warning: Could not hide cursor: {}", e2);
                    }
                }
            }

            self.terminal_initialized = true;
        }
        Ok(())
    }

    pub fn cleanup_terminal(&mut self) -> Result<()> {
        if self.terminal_initialized {
            use crossterm::{cursor, execute, terminal};

            execute!(
                std::io::stdout(),
                terminal::LeaveAlternateScreen,
                cursor::Show
            )
            .map_err(|e| {
                GitTypeError::TerminalError(format!("Failed to restore terminal: {}", e))
            })?;

            terminal::disable_raw_mode().map_err(|e| {
                GitTypeError::TerminalError(format!("Failed to disable raw mode: {}", e))
            })?;

            self.terminal_initialized = false;
        }

        // Clean up ratatui terminal
        if let Some(_terminal) = self.ratatui_terminal.take() {
            // Terminal cleanup is handled automatically when dropped
        }

        Ok(())
    }

    pub fn set_current_screen(&mut self, screen_type: ScreenType) -> Result<()> {
        if !self.screens.contains_key(&screen_type) {
            return Err(GitTypeError::TerminalError(format!(
                "Screen not registered: {:?}",
                screen_type
            )));
        }

        // Flush before cleaning up the current screen
        stdout().flush().map_err(|e| {
            GitTypeError::TerminalError(format!("Failed to flush before screen transition: {}", e))
        })?;

        if let Some(current_screen) = self.screens.get_mut(&self.current_screen_type) {
            current_screen.cleanup()?;
        }

        // Clear screen after cleanup and flush again
        execute!(stdout(), Clear(ClearType::All)).map_err(|e| {
            GitTypeError::TerminalError(format!("Failed to clear screen during transition: {}", e))
        })?;
        stdout().flush().map_err(|e| {
            GitTypeError::TerminalError(format!("Failed to flush after screen clear: {}", e))
        })?;

        // Set appropriate render backend for the screen
        match screen_type {
            ScreenType::Records
            | ScreenType::Analytics
            | ScreenType::DetailsDialog
            | ScreenType::InfoDialog
            | ScreenType::Help
            | ScreenType::Loading
            | ScreenType::SessionDetail
            | ScreenType::Typing
            | ScreenType::Settings
            | ScreenType::Panic => {
                self.render_backend = RenderBackend::Ratatui;
            }
            ScreenType::Animation => {
                // Ensure session result is available for animation
                self.create_session_result_from_trackers()?;
                self.render_backend = RenderBackend::Ratatui;
            }
            ScreenType::StageSummary => {
                // Ensure stage result is available for stage summary
                self.get_latest_stage_result_from_session_tracker();
                self.render_backend = RenderBackend::Crossterm;
            }
            ScreenType::SessionSummary => {
                // Ensure session result is available for session summary
                self.create_session_result_from_trackers()?;
                self.render_backend = RenderBackend::Crossterm;
            }
            _ => {
                self.render_backend = RenderBackend::Crossterm;
            }
        }

        self.current_screen_type = screen_type;

        // Clear the terminal screen before switching to new screen
        self.clear_screen()?;

        if let Some(new_screen) = self.screens.get_mut(&self.current_screen_type) {
            // Pre-inject data BEFORE calling init() to avoid RefCell conflicts
            match self.current_screen_type {
                ScreenType::StageSummary => {
                    if let Some(stage_summary_screen) =
                        new_screen.as_any_mut().downcast_mut::<StageSummaryScreen>()
                    {
                        if let Some(ref stage_result) = self.shared_stage_result {
                            stage_summary_screen.stage_result = Some(stage_result.clone());
                        }
                    }
                }
                ScreenType::Animation => {
                    if let Some(animation_screen) =
                        new_screen.as_any_mut().downcast_mut::<AnimationScreen>()
                    {
                        if let Some(ref session_result) = self.shared_session_result {
                            animation_screen.inject_session_result(session_result.clone());
                        }
                    }
                }
                _ => {}
            }

            log::info!("Initializing screen: {:?}", self.current_screen_type);
            new_screen.init()?;
        }

        // Force immediate render of the new screen
        self.render_current_screen()?;

        // Flush after initializing new screen
        stdout().flush().map_err(|e| {
            GitTypeError::TerminalError(format!("Failed to flush after screen init: {}", e))
        })?;

        Ok(())
    }

    fn clear_screen(&mut self) -> Result<()> {
        match self.render_backend {
            RenderBackend::Crossterm => {
                use crossterm::{execute, terminal};
                use std::io::stdout;

                execute!(stdout(), terminal::Clear(terminal::ClearType::All)).map_err(|e| {
                    GitTypeError::TerminalError(format!("Failed to clear screen: {}", e))
                })?;
            }
            RenderBackend::Ratatui => {
                // For ratatui, we need to clear the terminal buffer
                if let Some(terminal) = &mut self.ratatui_terminal {
                    terminal.clear().map_err(|e| {
                        GitTypeError::TerminalError(format!(
                            "Failed to clear ratatui terminal: {}",
                            e
                        ))
                    })?;
                }
            }
        }
        Ok(())
    }

    pub fn push_screen(&mut self, screen_type: ScreenType) -> Result<()> {
        // Special handling for TotalSummaryShare - update with current total result
        if screen_type == ScreenType::TotalSummaryShare {
            if let Some(total_result) = self.get_current_total_result() {
                let share_screen = TotalSummaryShareScreen::new(total_result, self.event_bus.clone());
                self.screens
                    .insert(ScreenType::TotalSummaryShare, Box::new(share_screen));
            }
        }

        self.screen_stack.push(self.current_screen_type.clone());
        self.set_current_screen(screen_type)
    }

    pub fn pop_screen(&mut self) -> Result<()> {
        if let Some(previous_screen) = self.screen_stack.pop() {
            // Prepare the screen before transitioning, just like in handle_transition
            self.prepare_screen_if_needed(&previous_screen)?;
            self.set_current_screen(previous_screen)
        } else {
            Ok(())
        }
    }

    pub fn pop_to_screen(&mut self, screen_type: ScreenType) -> Result<()> {
        while let Some(stacked_screen) = self.screen_stack.last() {
            if *stacked_screen == screen_type {
                break;
            }
            self.screen_stack.pop();
        }

        self.screen_stack.pop();
        self.set_current_screen(screen_type)
    }

    pub fn handle_transition(&mut self, transition: ScreenTransition) -> Result<()> {
        match transition {
            ScreenTransition::None => Ok(()),
            ScreenTransition::Push(screen_type) => {
                self.prepare_screen_if_needed(&screen_type)?;
                self.push_screen(screen_type)
            }
            ScreenTransition::Pop => {
                self.pop_screen()
            }
            ScreenTransition::Replace(screen_type) => {
                // Use ScreenTransitionManager to handle transition with side effects
                let validated_screen_type =
                    ScreenTransitionManager::reduce(self.current_screen_type.clone(), screen_type)?;

                self.prepare_screen_if_needed(&validated_screen_type)?;
                let result = self.set_current_screen(validated_screen_type);
                result
            }
            ScreenTransition::PopTo(screen_type) => {
                // Use ScreenTransitionManager to handle transition with side effects
                let validated_screen_type =
                    ScreenTransitionManager::reduce(self.current_screen_type.clone(), screen_type)?;

                self.prepare_screen_if_needed(&validated_screen_type)?;
                self.pop_to_screen(validated_screen_type)
            }
            ScreenTransition::Exit => {
                // If we're already on ExitSummary, mark exit requested
                if self.current_screen_type == ScreenType::TotalSummary {
                    self.exit_requested = true;
                } else {
                    // Otherwise, go to ExitSummary screen
                    let _ =
                        self.handle_transition(ScreenTransition::Replace(ScreenType::TotalSummary));
                }
                Ok(())
            }
        }
    }

    fn prepare_screen_if_needed(&mut self, screen_type: &ScreenType) -> Result<()> {
        if *screen_type == ScreenType::Typing {
            // Check if coming from Title screen and apply selected difficulty
            let selected_difficulty =
                if let Some(title_screen) = self.screens.get(&ScreenType::Title) {
                    if let Some(title) = title_screen.as_any().downcast_ref::<TitleScreen>() {
                        if let Some(action) = title.get_action_result() {
                            match action {
                                TitleAction::Start(difficulty) => Some(*difficulty),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

            // Apply difficulty to global repositories if found
            if let Some(difficulty) = selected_difficulty {
                StageRepository::set_global_difficulty(difficulty)?;

                // Also set difficulty in SessionManager
                let session_instance = SessionManager::instance();
                if let Ok(mut session_manager) = session_instance.lock() {
                    session_manager.set_difficulty(difficulty);
                };
            }

            // Session management is handled by ScreenTransitionManager
            // This prepare_screen_if_needed should only handle screen-specific setup

            // Load next challenge in TypingScreen
            if let Some(typing_screen) = self.screens.get_mut(&ScreenType::Typing) {
                if let Some(screen) = typing_screen.as_any_mut().downcast_mut::<TypingScreen>() {
                    if !screen.load_current_challenge()? {
                        // No more challenges available, create session result and go to session summary
                        self.create_session_result_from_trackers()?;

                        // Session completion is handled by ScreenTransitionManager
                        // Use proper transition instead of direct screen setting
                        self.handle_transition(ScreenTransition::Replace(ScreenType::Animation))?;
                        return Ok(());
                    }
                }
            }
        } else if *screen_type == ScreenType::SessionDetail {
            // Configure SessionDetail screen with data from History screen
            self.configure_session_detail_from_history()?;
        } else if *screen_type == ScreenType::TotalSummary {
            // Calculate total result from GLOBAL_TOTAL_TRACKER before showing ExitSummary
            if let Ok(global_total_tracker) = GLOBAL_TOTAL_TRACKER.lock() {
                if let Some(ref tracker) = *global_total_tracker {
                    self.shared_total_result = Some(TotalCalculator::calculate(tracker));
                }
            }
        }

        Ok(())
    }

    fn configure_session_detail_from_history(&mut self) -> Result<()> {
        // Get the selected session data from History screen
        let session_data_to_use =
            if let Some(records_screen) = self.screens.get(&ScreenType::Records) {
                if let Some(records) = records_screen.as_any().downcast_ref::<RecordsScreen>() {
                    records.get_selected_session_for_detail().clone()
                } else {
                    None
                }
            } else {
                None
            };

        // Configure SessionDetail screen with the selected session data
        if let Some(session_data) = session_data_to_use {
            self.configure_session_detail_screen(session_data)?;
        }

        Ok(())
    }

    /// Run the ScreenManager main loop
    pub fn run(&mut self) -> Result<()> {
        // Initialize current screen and force initial render
        self.render_current_screen()?;

        loop {
            // Update and render
            self.update_and_render()?;

            // Handle input
            self.handle_input()?;

            // Check for pending screen transitions
            let pending_transition = {
                self.pending_transition.lock().ok()
                    .and_then(|mut pending| pending.take())
            };

            if let Some(transition) = pending_transition {
                let _ = self.handle_transition(transition);
            }

            // Check if exit was requested
            if self.exit_requested {
                break;
            }
        }

        // Clean up on exit
        self.cleanup_terminal()?;

        Ok(())
    }

    fn update_and_render(&mut self) -> Result<()> {
        if let Some(screen) = self.screens.get_mut(&self.current_screen_type) {
            let strategy = screen.get_update_strategy();
            let now = Instant::now();
            let should_update = match strategy {
                UpdateStrategy::InputOnly => false,
                UpdateStrategy::TimeBased(interval) => {
                    now.duration_since(self.last_update) >= interval
                }
                UpdateStrategy::Hybrid { interval, .. } => {
                    now.duration_since(self.last_update) >= interval
                }
            };

            if should_update {
                let needs_render = screen.update()?;

                // Special handling for LoadingScreen auto-transition
                if self.current_screen_type == ScreenType::Loading && !needs_render {
                    // LoadingScreen completed, transition to Title
                    if GameData::is_loading_completed() {
                        // Update TitleScreen data with challenge counts after loading is complete
                        self.handle_transition(ScreenTransition::Replace(ScreenType::Title))?;

                        // Update title screen with challenge counts from StageRepository
                        let stage_repo_instance = StageRepository::instance();
                        if let Ok(repo) = stage_repo_instance.lock() {
                            let _ = repo.update_title_screen_data(self);
                        }

                        return Ok(());
                    } else if GameData::is_loading_failed() {
                        // Could transition to an error screen or back to title
                        self.handle_transition(ScreenTransition::Replace(ScreenType::Title))?;
                        return Ok(());
                    }
                }

                // Special handling for AnimationScreen auto-transition
                if self.current_screen_type == ScreenType::Animation {
                    if let Some(animation_screen) =
                        screen.as_any_mut().downcast_mut::<AnimationScreen>()
                    {
                        if animation_screen.is_animation_complete() {
                            // Animation is complete, transition to SessionSummary
                            self.handle_transition(ScreenTransition::Replace(
                                ScreenType::SessionSummary,
                            ))?;
                            return Ok(());
                        }
                    }
                }

                if needs_render {
                    self.render_current_screen()?;
                }
                self.last_update = now;
            }
        }
        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        use crossterm::event::{poll, read, KeyCode, KeyModifiers};

        let timeout = if let Some(screen) = self.screens.get(&self.current_screen_type) {
            match screen.get_update_strategy() {
                UpdateStrategy::InputOnly => Duration::from_millis(100),
                UpdateStrategy::TimeBased(interval) => interval.min(Duration::from_millis(50)),
                UpdateStrategy::Hybrid {
                    interval,
                    input_priority,
                } => {
                    if input_priority {
                        Duration::from_millis(50)
                    } else {
                        interval.min(Duration::from_millis(50))
                    }
                }
            }
        } else {
            Duration::from_millis(100)
        };

        if poll(timeout)? {
            if let Event::Key(key_event) = read()? {
                if key_event.kind == KeyEventKind::Press {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL)
                        && key_event.code == KeyCode::Char('c')
                    {
                        // Ctrl+C should either transition to ExitSummary or exit if already there
                        if self.current_screen_type == ScreenType::TotalSummary {
                            self.exit_requested = true;
                        } else {
                            let _ = self.handle_transition(ScreenTransition::Replace(
                                ScreenType::TotalSummary,
                            ));
                        }
                        return Ok(());
                    }

                    if let Some(screen) = self.screens.get_mut(&self.current_screen_type) {
                        screen.handle_key_event(key_event)?;
                    }

                    // For History/Analytics screens using Ratatui, always re-render on key input
                    // as they may have internal state changes (list selection, etc.)
                    if matches!(self.render_backend, RenderBackend::Ratatui) {
                        // Force render for Ratatui screens on any key input
                        self.render_current_screen()?;
                    } else {
                        // For Crossterm screens, check if update is needed
                        let needs_render = if let Some(screen) =
                            self.screens.get_mut(&self.current_screen_type)
                        {
                            screen.update()?
                        } else {
                            false
                        };

                        if needs_render {
                            self.render_current_screen()?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn render_current_screen(&mut self) -> Result<()> {
        use std::io::{stdout, Write};

        match self.render_backend {
            RenderBackend::Crossterm => {
                let mut stdout_handle = stdout();

                // Flush before rendering to clear any pending output
                stdout_handle.flush().map_err(|e| {
                    GitTypeError::TerminalError(format!("Failed to flush before rendering: {}", e))
                })?;

                // Special handling for TypingScreen which needs mutable access
                if self.current_screen_type == ScreenType::StageSummary {
                    // Special handling for StageSummaryScreen to inject stage_result
                    if let Some(screen) = self.screens.get_mut(&self.current_screen_type) {
                        if let Some(stage_summary_screen) =
                            screen.as_any_mut().downcast_mut::<StageSummaryScreen>()
                        {
                            // Inject stage_result if not already set
                            if stage_summary_screen.stage_result.is_none() {
                                if let Some(ref stage_result) = self.shared_stage_result {
                                    stage_summary_screen.stage_result = Some(stage_result.clone());
                                }
                            }
                        }
                        screen.render_crossterm_with_data(
                            &mut stdout_handle,
                            self.shared_session_result.as_ref(),
                            self.shared_total_result.as_ref(),
                        )?;
                    }
                } else if self.current_screen_type == ScreenType::SessionSummary {
                    // Special handling for SessionSummary to check for animation transition
                    if let Some(screen) = self.screens.get_mut(&self.current_screen_type) {
                        screen.render_crossterm_with_data(
                            &mut stdout_handle,
                            self.shared_session_result.as_ref(),
                            self.shared_total_result.as_ref(),
                        )?;
                    }
                } else if let Some(screen) = self.screens.get_mut(&self.current_screen_type) {
                    screen.render_crossterm_with_data(
                        &mut stdout_handle,
                        self.shared_session_result.as_ref(),
                        self.shared_total_result.as_ref(),
                    )?;
                }

                // Flush after rendering to ensure display is updated
                stdout_handle.flush().map_err(|e| {
                    GitTypeError::TerminalError(format!("Failed to flush after rendering: {}", e))
                })?;
            }
            RenderBackend::Ratatui => {
                // Initialize ratatui terminal if not already done
                if self.ratatui_terminal.is_none() {
                    use ratatui::{backend::CrosstermBackend, Terminal};
                    use std::io::stdout;

                    let backend = CrosstermBackend::new(stdout());
                    let terminal = Terminal::new(backend).map_err(|e| {
                        GitTypeError::TerminalError(format!(
                            "Failed to create ratatui terminal: {}",
                            e
                        ))
                    })?;
                    self.ratatui_terminal = Some(terminal);
                }

                // Use the persistent terminal instance
                if let Some(terminal) = &mut self.ratatui_terminal {
                    if let Some(screen) = self.screens.get_mut(&self.current_screen_type) {
                        terminal
                            .draw(|frame| {
                                let _ = screen.render_ratatui(frame);
                            })
                            .map_err(|e| {
                                GitTypeError::TerminalError(format!(
                                    "Failed to draw ratatui frame: {}",
                                    e
                                ))
                            })?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_current_screen_type(&self) -> &ScreenType {
        &self.current_screen_type
    }

    pub fn configure_session_detail_screen(
        &mut self,
        session_data: SessionDisplayData,
    ) -> Result<()> {
        if let Some(screen) = self.screens.get_mut(&ScreenType::SessionDetail) {
            if let Some(session_detail_screen) =
                screen.as_any_mut().downcast_mut::<SessionDetailScreen>()
            {
                session_detail_screen.set_session_data(session_data)?;
            }
        }

        Ok(())
    }

    pub fn get_screen_stack(&self) -> &Vec<ScreenType> {
        &self.screen_stack
    }

    pub fn get_screen(&self, screen_type: &ScreenType) -> Option<&dyn Screen> {
        self.screens.get(screen_type).map(|screen| &**screen)
    }

    pub fn get_screen_mut(&mut self, screen_type: &ScreenType) -> Option<&mut Box<dyn Screen>> {
        self.screens.get_mut(screen_type)
    }

    pub fn is_terminal_initialized(&self) -> bool {
        self.terminal_initialized
    }

    /// Get latest stage result from SessionTracker and store in shared data
    fn get_latest_stage_result_from_session_tracker(&mut self) {
        if let Ok(global_session_tracker) = GLOBAL_SESSION_TRACKER.lock() {
            if let Some(ref session_tracker) = *global_session_tracker {
                let session_data = session_tracker.get_data();
                if let Some(latest_stage_result) = session_data.stage_results.last() {
                    self.shared_stage_result = Some(latest_stage_result.clone());
                }
            }
        }
    }

    /// Create session result from global trackers and store in shared data
    fn create_session_result_from_trackers(&mut self) -> Result<()> {
        // Get session result from SessionManager
        match SessionManager::get_global_session_result() {
            Ok(Some(session_result)) => {
                self.shared_session_result = Some(session_result);
            }
            Ok(None) => {}
            Err(_) => {}
        }

        Ok(())
    }
}

impl Drop for ScreenManager {
    fn drop(&mut self) {
        let _ = self.cleanup_terminal();
    }
}

impl Default for ScreenManager {
    fn default() -> Self {
        Self::new(EventBus::new())
    }
}

impl ScreenManager {
    /// Static cleanup function for use when ScreenManager instance is not available
    pub fn cleanup_terminal_static() {
        use crossterm::{execute, terminal};
        use std::io::{stdout, Write};

        // Disable raw mode first
        if let Err(e) = terminal::disable_raw_mode() {
            eprintln!("Warning: Failed to disable raw mode: {}", e);
        }

        // Exit alternate screen and restore cursor with explicit error handling
        if let Err(e) = execute!(
            stdout(),
            terminal::LeaveAlternateScreen,
            crossterm::cursor::Show,
            crossterm::style::ResetColor,
            terminal::Clear(terminal::ClearType::All)
        ) {
            eprintln!("Warning: Failed to cleanup terminal: {}", e);
        }

        let _ = stdout().flush();
    }

    /// Show session summary on interrupt (Ctrl+C handler)
    fn show_session_summary_on_interrupt(&mut self) {
        // Set the screen to TotalSummary and render it once before cleanup
        let _ = self.set_current_screen(ScreenType::TotalSummary);
        // Force one render cycle to show the summary
        let _ = self.render_current_screen();
        Self::cleanup_terminal_static();
    }
}
