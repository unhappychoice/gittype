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
//! - **Ratatui Rendering**: Uses ratatui for all terminal UI rendering
//! - **Flexible Update Strategy**: Screens can define their update frequency needs
//! - **Terminal Lifecycle Management**: Proper terminal setup and cleanup
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use gittype::domain::events::{EventBus, EventBusInterface};
//! use gittype::presentation::tui::screen_manager::ScreenManagerImpl;
//! use gittype::presentation::tui::screens::TitleScreen;
//! use gittype::presentation::game::GameData;
//! use ratatui::backend::CrosstermBackend;
//! use ratatui::Terminal;
//! use std::io::stdout;
//! use std::sync::Arc;
//!
//! fn example() -> gittype::Result<()> {
//!     let event_bus = Arc::new(EventBus::new()) as Arc<dyn EventBusInterface>;
//!     let screen = TitleScreen::new(event_bus.clone());
//!     let game_data = GameData::instance();
//!     let backend = CrosstermBackend::new(stdout());
//!     let terminal = Terminal::new(backend).unwrap();
//!
//!     let mut manager = ScreenManagerImpl::default();
//!     // Manager usage example
//!     Ok(())
//! }
//! ```
//!
use crate::domain::events::{EventBus, EventBusInterface};
use crate::infrastructure::terminal::TerminalInterface;
use crate::presentation::game::events::{ExitRequested, NavigateTo};
use crate::presentation::game::{GameData, SessionManager, StageRepository};
use crate::presentation::tui::screen_transition_manager::ScreenTransitionManager;
use crate::presentation::tui::screens::{
    AnalyticsScreen, AnalyticsScreenInterface, AnimationScreen, AnimationScreenInterface,
    HelpScreen, HelpScreenInterface, InfoDialogScreen, InfoDialogScreenInterface, LoadingScreen,
    LoadingScreenInterface, PanicScreen, PanicScreenInterface, RecordsScreen,
    RecordsScreenInterface, RepoListScreen, RepoPlayScreen, SessionDetailScreen,
    SessionDetailScreenInterface, SessionDetailsDialog, SessionDetailsDialogInterface,
    SessionFailureScreen, SessionFailureScreenInterface, SessionSummaryScreen,
    SessionSummaryScreenInterface, SessionSummaryShareScreen, SessionSummaryShareScreenInterface,
    SettingsScreen, SettingsScreenInterface, StageSummaryScreen, StageSummaryScreenInterface,
    TitleAction, TitleScreen, TitleScreenInterface, TotalSummaryScreen,
    TotalSummaryScreenInterface, TotalSummaryShareScreen, TotalSummaryShareScreenInterface,
    TrendingLanguageSelectionScreen, TrendingRepositorySelectionScreen, TypingScreen,
    TypingScreenInterface, VersionCheckScreen, VersionCheckScreenInterface,
};
use crate::presentation::tui::{
    Screen, ScreenDataProvider, ScreenTransition, ScreenType, UpdateStrategy,
};
use crate::{GitTypeError, Result};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::style::ResetColor;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use shaku::{Component, Interface};
use std::collections::HashMap;
use std::io::{stdout, Stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Wrapper to make Arc<T: Screen> implement Screen
struct ArcScreenWrapper<T: Screen + ?Sized>(Arc<T>);

impl<T: Screen + Send + Sync + ?Sized> Screen for ArcScreenWrapper<T> {
    fn get_type(&self) -> ScreenType {
        self.0.get_type()
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        // This method is only callable when T is Sized due to trait constraints
        panic!("default_provider should not be called on ArcScreenWrapper with unsized types")
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        self.0.init_with_data(data)
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        self.0.handle_key_event(key_event)
    }

    fn render_ratatui(&self, frame: &mut ratatui::Frame) -> Result<()> {
        self.0.render_ratatui(frame)
    }

    fn cleanup(&self) -> Result<()> {
        self.0.cleanup()
    }

    fn on_pushed_from(&self, source_screen: &dyn Screen) -> Result<()> {
        self.0.on_pushed_from(source_screen)
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        self.0.get_update_strategy()
    }

    fn update(&self) -> Result<bool> {
        self.0.update()
    }

    fn is_exitable(&self) -> bool {
        self.0.is_exitable()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self.0.as_any()
    }
}

/// Factory for creating ScreenManager instances
pub trait ScreenManagerFactory: Interface {
    fn create(
        &self,
        game_data: Arc<Mutex<GameData>>,
        module: &crate::presentation::di::AppModule,
    ) -> ScreenManagerImpl<CrosstermBackend<Stdout>>;
}

/// Central manager for screen transitions, rendering, and input handling
pub struct ScreenManagerImpl<
    B: ratatui::backend::Backend + Send + 'static = CrosstermBackend<Stdout>,
> {
    screens: HashMap<ScreenType, Box<dyn Screen>>,
    screen_stack: Vec<ScreenType>,
    current_screen_type: ScreenType,
    terminal_initialized: bool,
    last_update: Instant,
    ratatui_terminal: Terminal<B>,
    exit_requested: bool,

    // Pending screen transition - shared across threads
    pending_transition: Arc<Mutex<Option<ScreenTransition>>>,

    // Event bus for UI events
    event_bus: Arc<dyn EventBusInterface>,

    // Game data
    game_data: Arc<Mutex<GameData>>,
}

impl<B: ratatui::backend::Backend + Send + 'static> ScreenManagerImpl<B> {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        game_data: Arc<Mutex<GameData>>,
        terminal: Terminal<B>,
    ) -> Self {
        Self {
            screens: HashMap::new(),
            screen_stack: Vec::new(),
            current_screen_type: ScreenType::Title,
            terminal_initialized: false,
            last_update: Instant::now(),
            ratatui_terminal: terminal,
            exit_requested: false,
            pending_transition: Arc::new(Mutex::new(None)),
            event_bus: event_bus.clone(),
            game_data,
        }
    }

    fn get_game_data(&self) -> Arc<Mutex<GameData>> {
        self.game_data.clone()
    }

    pub fn get_event_bus(&self) -> Arc<dyn EventBusInterface> {
        Arc::clone(&self.event_bus)
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

        log::info!(
            "ScreenManager: EventBus subscribers address: {:p}",
            event_bus.as_event_bus().get_subscribers_ptr()
        );

        // Subscribe to NavigateTo events
        {
            let pending_transition_clone = pending_transition.clone();
            event_bus
                .as_event_bus()
                .subscribe(move |event: &NavigateTo| {
                    log::info!("ScreenManager: Received NavigateTo event: {:?}", event);
                    if let Ok(mut pending) = pending_transition_clone.lock() {
                        *pending = Some(event.clone());
                        log::info!("ScreenManager: Set pending transition to {:?}", event);
                    } else {
                        log::error!("ScreenManager: Failed to lock pending_transition");
                    }
                });
        }

        // Subscribe to ExitRequested events
        {
            let manager_weak_clone = manager_weak.clone();
            event_bus
                .as_event_bus()
                .subscribe(move |_event: &ExitRequested| {
                    if let Some(arc) = manager_weak_clone.upgrade() {
                        if let Ok(mut manager) = arc.lock() {
                            manager.show_session_summary_on_interrupt();
                            std::process::exit(0);
                        }
                    }
                });
        }
    }

    /// Register a screen with the manager
    pub fn register_screen(&mut self, screen: impl Screen + 'static) {
        let screen_type = screen.get_type();
        self.screens.insert(screen_type, Box::new(screen));
    }

    /// Register a screen from Arc
    pub fn register_screen_arc<T: Screen + Send + Sync + 'static>(&mut self, screen: Arc<T>) {
        let screen_type = screen.get_type();
        // Clone the Arc and convert to Box by implementing Screen on Arc<T>
        self.screens
            .insert(screen_type, Box::new(ArcScreenWrapper(screen)));
    }

    /// Register a screen from Arc implementing Screen trait
    pub fn register_screen_interface(&mut self, screen: Arc<dyn Screen>) {
        let screen_type = screen.get_type();
        self.screens
            .insert(screen_type, Box::new(ArcScreenWrapper(screen)));
    }

    /// Initialize terminal for raw mode and alternate screen
    pub fn initialize_terminal(&mut self) -> Result<()> {
        if !self.terminal_initialized {
            // Check if we're running in a valid terminal environment using atty
            if !atty::is(atty::Stream::Stdout) {
                return Err(GitTypeError::TerminalError(
                    "Not running in a terminal environment. Please run in a proper terminal."
                        .to_string(),
                ));
            }

            // Enable raw mode with better error handling for WSL
            match enable_raw_mode() {
                Ok(()) => {}
                Err(e) => {
                    // In WSL, sometimes raw mode fails initially, try after a short delay
                    sleep(Duration::from_millis(10));
                    enable_raw_mode().map_err(|e2| {
                        GitTypeError::TerminalError(format!(
                            "Failed to enable raw mode: {} (retry also failed: {})",
                            e, e2
                        ))
                    })?;
                }
            }

            // Try to enter alternate screen, but continue without it if it fails
            match execute!(stdout(), EnterAlternateScreen, Hide) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Warning: Could not enter alternate screen mode: {}. Running in normal mode.", e);
                    // Try to at least hide the cursor
                    if let Err(e2) = execute!(stdout(), Hide) {
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
            execute!(stdout(), LeaveAlternateScreen, Show).map_err(|e| {
                GitTypeError::TerminalError(format!("Failed to restore terminal: {}", e))
            })?;

            disable_raw_mode().map_err(|e| {
                GitTypeError::TerminalError(format!("Failed to disable raw mode: {}", e))
            })?;

            self.terminal_initialized = false;
        }

        // Ratatui terminal cleanup is handled automatically when dropped

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

        self.current_screen_type = screen_type;

        // Clear the terminal screen before switching to new screen
        self.clear_screen()?;

        if let Some(new_screen) = self.screens.get_mut(&self.current_screen_type) {
            log::info!("Initializing screen: {:?}", self.current_screen_type);

            // Get data using provider and call init_with_data
            // If get_screen_data fails (e.g., for test screens), use empty data
            let data = Self::get_screen_data(self.current_screen_type.clone())
                .unwrap_or_else(|_| Box::new(()));
            new_screen.init_with_data(data).map_err(|e| {
                GitTypeError::ScreenInitializationError(format!(
                    "Failed to initialize screen {:?}: {}",
                    self.current_screen_type, e
                ))
            })?;
        }

        // Flush after initializing new screen
        stdout().flush().map_err(|e| {
            GitTypeError::TerminalError(format!("Failed to flush after screen init: {}", e))
        })?;

        Ok(())
    }

    fn get_screen_data(screen_type: ScreenType) -> Result<Box<dyn std::any::Any>> {
        let provider: Box<dyn ScreenDataProvider> = match screen_type {
            ScreenType::Title => TitleScreen::default_provider(),
            ScreenType::Loading => LoadingScreen::default_provider(),
            ScreenType::Typing => TypingScreen::default_provider(),
            ScreenType::StageSummary => StageSummaryScreen::default_provider(),
            ScreenType::SessionSummary => SessionSummaryScreen::default_provider(),
            ScreenType::TotalSummary => TotalSummaryScreen::default_provider(),
            ScreenType::TotalSummaryShare => TotalSummaryShareScreen::default_provider(),
            ScreenType::SessionFailure => SessionFailureScreen::default_provider(),
            ScreenType::Records => RecordsScreen::default_provider(),
            ScreenType::Analytics => AnalyticsScreen::default_provider(),
            ScreenType::SessionDetail => SessionDetailScreen::default_provider(),
            ScreenType::SessionSharing => SessionSummaryShareScreen::default_provider(),
            ScreenType::Animation => AnimationScreen::default_provider(),
            ScreenType::VersionCheck => VersionCheckScreen::default_provider(),
            ScreenType::InfoDialog => InfoDialogScreen::default_provider(),
            ScreenType::Help => HelpScreen::default_provider(),
            ScreenType::DetailsDialog => SessionDetailsDialog::default_provider(),
            ScreenType::Settings => SettingsScreen::default_provider(),
            ScreenType::Panic => PanicScreen::default_provider(),
            // CLI screens
            ScreenType::RepoPlay => RepoPlayScreen::default_provider(),
            ScreenType::RepoList => RepoListScreen::default_provider(),
            ScreenType::TrendingLanguageSelection => {
                TrendingLanguageSelectionScreen::default_provider()
            }
            ScreenType::TrendingRepositorySelection => {
                TrendingRepositorySelectionScreen::default_provider()
            }
        };

        provider.provide()
    }

    fn clear_screen(&mut self) -> Result<()> {
        // Clear the ratatui terminal buffer
        self.ratatui_terminal.clear().map_err(|e| {
            GitTypeError::TerminalError(format!("Failed to clear ratatui terminal: {}", e))
        })?;
        Ok(())
    }

    pub fn push_screen(&mut self, screen_type: ScreenType) -> Result<()> {
        let source_screen_type = self.current_screen_type.clone();
        self.screen_stack.push(source_screen_type.clone());
        self.set_current_screen(screen_type)?;

        // Call on_pushed_from to allow the new screen to extract data from source screen
        // We need to split borrow screens HashMap to access both source and destination
        let source_ptr = self
            .screens
            .get(&source_screen_type)
            .map(|s| s.as_ref() as *const dyn Screen);

        if let Some(source_ptr) = source_ptr {
            if let Some(new_screen) = self.screens.get_mut(&self.current_screen_type) {
                // Safe because we're accessing different keys in the HashMap
                let source_screen = unsafe { &*source_ptr };
                new_screen.on_pushed_from(source_screen)?;
            }
        }

        Ok(())
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
                self.push_screen(screen_type)?;
                self.render_current_screen()
            }
            ScreenTransition::Pop => {
                self.pop_screen()?;
                self.render_current_screen()
            }
            ScreenTransition::Replace(screen_type) => {
                let validated_screen_type =
                    ScreenTransitionManager::reduce(self.current_screen_type.clone(), screen_type)?;

                self.prepare_screen_if_needed(&validated_screen_type)?;
                self.set_current_screen(validated_screen_type)?;
                self.render_current_screen()
            }
            ScreenTransition::PopTo(screen_type) => {
                let validated_screen_type =
                    ScreenTransitionManager::reduce(self.current_screen_type.clone(), screen_type)?;

                self.prepare_screen_if_needed(&validated_screen_type)?;
                self.pop_to_screen(validated_screen_type)?;
                self.render_current_screen()
            }
            ScreenTransition::Exit => {
                // Check if current screen can exit directly
                let can_exit_directly = self.current_screen_type == ScreenType::TotalSummary
                    || self
                        .screens
                        .get(&self.current_screen_type)
                        .map(|screen| screen.is_exitable())
                        .unwrap_or(false);

                if can_exit_directly {
                    self.exit_requested = true;
                } else {
                    let _ =
                        self.handle_transition(ScreenTransition::Replace(ScreenType::TotalSummary));
                }
                Ok(())
            }
        }
    }

    fn prepare_screen_if_needed(&mut self, screen_type: &ScreenType) -> Result<()> {
        match screen_type {
            ScreenType::Typing => {
                // Check if coming from Title screen and apply selected difficulty
                let selected_difficulty = self
                    .screens
                    .get(&ScreenType::Title)
                    .and_then(|title_screen| title_screen.as_any().downcast_ref::<TitleScreen>())
                    .and_then(|title| title.get_action_result())
                    .and_then(|action| match action {
                        TitleAction::Start(difficulty) => Some(difficulty),
                        _ => None,
                    });

                // Apply difficulty to global repositories if found
                selected_difficulty
                    .map(|difficulty| {
                        StageRepository::set_global_difficulty(difficulty)?;

                        // Also set difficulty in SessionManager
                        if let Ok(mut session_manager) = SessionManager::instance().lock() {
                            session_manager.set_difficulty(difficulty);
                        }

                        Ok::<_, GitTypeError>(())
                    })
                    .transpose()?;

                // Load next challenge in TypingScreen
                let load_result = self
                    .screens
                    .get_mut(&ScreenType::Typing)
                    .and_then(|screen| screen.as_any().downcast_ref::<TypingScreen>())
                    .map(|screen| screen.load_current_challenge())
                    .transpose()?
                    .map(|loaded| {
                        if loaded {
                            Ok(())
                        } else {
                            Err(GitTypeError::TerminalError(
                                "No challenges available. ".to_string(),
                            ))
                        }
                    })
                    .transpose();

                if let Err(e) = load_result {
                    log::error!(
                        "ScreenManager: Failed to load challenge in TypingScreen: {}",
                        e
                    );
                    return Err(e);
                }

                Ok(())
            }
            _ => Ok(()),
        }
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
                self.pending_transition
                    .lock()
                    .ok()
                    .and_then(|mut pending| pending.take())
            };

            if let Some(transition) = pending_transition {
                log::info!(
                    "ScreenManager: Processing pending transition: {:?}",
                    transition
                );
                if let Err(e) = self.handle_transition(transition) {
                    log::error!("ScreenManager: Failed to handle transition: {}", e);
                }
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
        // Get game_data before mutable borrow to avoid borrow checker error
        let game_data = self.get_game_data();

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
                    let data = game_data.lock().unwrap();
                    let loading_completed = data.loading_completed;
                    let loading_failed = data.loading_failed;
                    drop(data);

                    // LoadingScreen completed, transition to Title
                    if loading_completed {
                        // Update TitleScreen data with challenge counts after loading is complete
                        self.handle_transition(ScreenTransition::Replace(ScreenType::Title))?;

                        // Update title screen with challenge counts from StageRepository
                        let stage_repo_instance = StageRepository::instance();
                        if let Ok(repo) = stage_repo_instance.lock() {
                            let _ = repo.update_title_screen_data(self);
                        }

                        return Ok(());
                    } else if loading_failed {
                        // Could transition to an error screen or back to title
                        self.handle_transition(ScreenTransition::Replace(ScreenType::Title))?;
                        return Ok(());
                    }
                }

                // Special handling for AnimationScreen auto-transition
                if self.current_screen_type == ScreenType::Animation {
                    if let Some(animation_screen) =
                        screen.as_any().downcast_ref::<AnimationScreen>()
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

                    // Always re-render on key input for ratatui screens
                    // as they may have internal state changes (list selection, etc.)
                    self.render_current_screen()?;
                }
            }
        }

        Ok(())
    }

    pub fn render_current_screen(&mut self) -> Result<()> {
        if let Some(screen) = self.screens.get_mut(&self.current_screen_type) {
            self.ratatui_terminal
                .draw(|frame| {
                    let _ = screen.render_ratatui(frame);
                })
                .map_err(|e| {
                    GitTypeError::TerminalError(format!("Failed to draw ratatui frame: {}", e))
                })?;
        }

        Ok(())
    }

    pub fn get_current_screen_type(&self) -> &ScreenType {
        &self.current_screen_type
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

    /// Static cleanup function for use when ScreenManager instance is not available
    pub fn cleanup_terminal_static() {
        // Disable raw mode first
        if let Err(e) = disable_raw_mode() {
            eprintln!("Warning: Failed to disable raw mode: {}", e);
        }

        // Exit alternate screen and restore cursor with explicit error handling
        if let Err(e) = execute!(
            stdout(),
            LeaveAlternateScreen,
            Show,
            ResetColor,
            Clear(ClearType::All)
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

impl<B: ratatui::backend::Backend + Send + 'static> Drop for ScreenManagerImpl<B> {
    fn drop(&mut self) {
        let _ = self.cleanup_terminal();
    }
}

impl Default for ScreenManagerImpl<CrosstermBackend<Stdout>> {
    fn default() -> Self {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend).expect("Failed to create terminal");
        Self::new(Arc::new(EventBus::new()), GameData::instance(), terminal)
    }
}

#[derive(Component)]
#[shaku(interface = ScreenManagerFactory)]
pub struct ScreenManagerFactoryImpl {
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    terminal: Arc<dyn TerminalInterface>,
    #[shaku(inject)]
    title_screen: Arc<dyn TitleScreenInterface>,
    #[shaku(inject)]
    typing_screen: Arc<dyn TypingScreenInterface>,
    #[shaku(inject)]
    animation_screen: Arc<dyn AnimationScreenInterface>,
    #[shaku(inject)]
    help_screen: Arc<dyn HelpScreenInterface>,
    #[shaku(inject)]
    loading_screen: Arc<dyn LoadingScreenInterface>,
    #[shaku(inject)]
    panic_screen: Arc<dyn PanicScreenInterface>,
    #[shaku(inject)]
    session_failure_screen: Arc<dyn SessionFailureScreenInterface>,
    #[shaku(inject)]
    info_dialog_screen: Arc<dyn InfoDialogScreenInterface>,
    #[shaku(inject)]
    session_details_dialog: Arc<dyn SessionDetailsDialogInterface>,
    #[shaku(inject)]
    stage_summary_screen: Arc<dyn StageSummaryScreenInterface>,
    #[shaku(inject)]
    analytics_screen: Arc<dyn AnalyticsScreenInterface>,
    #[shaku(inject)]
    records_screen: Arc<dyn RecordsScreenInterface>,
    #[shaku(inject)]
    session_detail_screen: Arc<dyn SessionDetailScreenInterface>,
    #[shaku(inject)]
    session_summary_screen: Arc<dyn SessionSummaryScreenInterface>,
    #[shaku(inject)]
    session_summary_share_screen: Arc<dyn SessionSummaryShareScreenInterface>,
    #[shaku(inject)]
    settings_screen: Arc<dyn SettingsScreenInterface>,
    #[shaku(inject)]
    total_summary_screen: Arc<dyn TotalSummaryScreenInterface>,
    #[shaku(inject)]
    total_summary_share_screen: Arc<dyn TotalSummaryShareScreenInterface>,
    #[shaku(inject)]
    version_check_screen: Arc<dyn VersionCheckScreenInterface>,
}

impl ScreenManagerFactory for ScreenManagerFactoryImpl {
    fn create(
        &self,
        game_data: Arc<Mutex<GameData>>,
        _module: &crate::presentation::di::AppModule,
    ) -> ScreenManagerImpl<CrosstermBackend<Stdout>> {
        // Use the Arc<dyn EventBusInterface> directly from DI - ensures same instance everywhere
        let event_bus = self.event_bus.clone();
        let terminal = self.terminal.get();
        let mut manager = ScreenManagerImpl::new(event_bus.clone(), game_data.clone(), terminal);

        // Register screens from DI (Components)
        // Explicit type coercion from Arc<dyn Interface> to Arc<dyn Screen>
        let title_screen: Arc<dyn Screen> = self.title_screen.clone();
        manager.register_screen_interface(title_screen);
        let typing_screen: Arc<dyn Screen> = self.typing_screen.clone();
        manager.register_screen_interface(typing_screen);
        let animation_screen: Arc<dyn Screen> = self.animation_screen.clone();
        manager.register_screen_interface(animation_screen);
        let help_screen: Arc<dyn Screen> = self.help_screen.clone();
        manager.register_screen_interface(help_screen);
        let loading_screen: Arc<dyn Screen> = self.loading_screen.clone();
        manager.register_screen_interface(loading_screen);
        let panic_screen: Arc<dyn Screen> = self.panic_screen.clone();
        manager.register_screen_interface(panic_screen);
        let session_failure_screen: Arc<dyn Screen> = self.session_failure_screen.clone();
        manager.register_screen_interface(session_failure_screen);
        let info_dialog_screen: Arc<dyn Screen> = self.info_dialog_screen.clone();
        manager.register_screen_interface(info_dialog_screen);
        let session_details_dialog: Arc<dyn Screen> = self.session_details_dialog.clone();
        manager.register_screen_interface(session_details_dialog);
        let stage_summary_screen: Arc<dyn Screen> = self.stage_summary_screen.clone();
        manager.register_screen_interface(stage_summary_screen);
        let analytics_screen: Arc<dyn Screen> = self.analytics_screen.clone();
        manager.register_screen_interface(analytics_screen);
        let records_screen: Arc<dyn Screen> = self.records_screen.clone();
        manager.register_screen_interface(records_screen);
        let session_detail_screen: Arc<dyn Screen> = self.session_detail_screen.clone();
        manager.register_screen_interface(session_detail_screen);
        let session_summary_screen: Arc<dyn Screen> = self.session_summary_screen.clone();
        manager.register_screen_interface(session_summary_screen);
        let session_summary_share_screen: Arc<dyn Screen> =
            self.session_summary_share_screen.clone();
        manager.register_screen_interface(session_summary_share_screen);
        let settings_screen: Arc<dyn Screen> = self.settings_screen.clone();
        manager.register_screen_interface(settings_screen);
        let total_summary_screen: Arc<dyn Screen> = self.total_summary_screen.clone();
        manager.register_screen_interface(total_summary_screen);
        let total_summary_share_screen: Arc<dyn Screen> = self.total_summary_share_screen.clone();
        manager.register_screen_interface(total_summary_share_screen);
        let version_check_screen: Arc<dyn Screen> = self.version_check_screen.clone();
        manager.register_screen_interface(version_check_screen);

        manager
    }
}
