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
//! use gittype::game::{BasicScreen, ScreenManager, ScreenType, UpdateStrategy};
//!
//! fn main() -> gittype::Result<()> {
//!     let mut screen_manager = ScreenManager::new();
//!     
//!     let screen = BasicScreen::new(
//!         "Demo".to_string(),
//!         vec!["Hello World!".to_string()],
//!         UpdateStrategy::InputOnly,
//!     );
//!     
//!     screen_manager.register_screen(ScreenType::Title, Box::new(screen));
//!     screen_manager.run()?;
//!     
//!     Ok(())
//! }
//! ```

use crate::Result;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use std::collections::HashMap;
use std::io::Stdout;
use std::time::{Duration, Instant};

/// Screen type identifiers for different application screens
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScreenType {
    Title,
    Loading,
    Typing,
    StageSummary,
    SessionSummary,
    ExitSummary,
    Cancel,
    Failure,
    History,
    Analytics,
    SessionDetail,
    Sharing,
    Animation,
    VersionCheck,
    InfoDialog,
    DetailsDialog,
}

/// Update strategy defines how and when a screen should be updated and re-rendered
#[derive(Debug, Clone)]
pub enum UpdateStrategy {
    /// Screen only updates when user provides input
    InputOnly,
    /// Screen updates at regular time intervals
    TimeBased(Duration),
    /// Screen combines both input and time-based updates
    Hybrid {
        /// Time interval for automatic updates
        interval: Duration,
        /// Whether input events should trigger immediate updates
        input_priority: bool,
    },
}

/// The Screen trait defines the interface that all screens must implement
pub trait Screen: Send {
    /// Initialize the screen - called when screen becomes active
    fn init(&mut self) -> Result<()> {
        Ok(())
    }
    
    /// Handle keyboard input events and return appropriate screen transition
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<ScreenTransition>;
    
    /// Render the screen using crossterm backend
    fn render_crossterm(&self, stdout: &mut Stdout) -> Result<()>;
    
    /// Render the screen using ratatui backend (optional)
    fn render_ratatui(&self, _frame: &mut ratatui::Frame) -> Result<()> {
        // Default implementation for backward compatibility
        // Individual screens can override this when ratatui support is needed
        Ok(())
    }
    
    /// Clean up screen resources - called when screen becomes inactive
    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
    
    /// Whether this screen should cause the application to exit
    fn should_exit(&self) -> bool {
        false
    }

    /// Get the update strategy for this screen
    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    /// Update screen state and return whether a re-render is needed
    fn update(&mut self) -> Result<bool> {
        Ok(false)
    }
}

/// Screen transition actions that can be returned from input handling
#[derive(Debug, Clone)]
pub enum ScreenTransition {
    /// No transition - stay on current screen
    None,
    /// Push new screen onto the stack
    Push(ScreenType),
    /// Pop current screen from stack
    Pop,
    /// Replace current screen with new screen
    Replace(ScreenType),
    /// Pop screens until reaching the specified screen type
    PopTo(ScreenType),
    /// Exit the application
    Exit,
}

/// Central manager for screen transitions, rendering, and input handling
pub struct ScreenManager {
    screens: HashMap<ScreenType, Box<dyn Screen>>,
    screen_stack: Vec<ScreenType>,
    current_screen_type: ScreenType,
    should_exit: bool,
    terminal_initialized: bool,
    last_update: Instant,
    render_backend: RenderBackend,
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
    /// Create a new ScreenManager with default settings
    pub fn new() -> Self {
        Self {
            screens: HashMap::new(),
            screen_stack: Vec::new(),
            current_screen_type: ScreenType::Title,
            should_exit: false,
            terminal_initialized: false,
            last_update: Instant::now(),
            render_backend: RenderBackend::Crossterm,
        }
    }
    
    /// Register a screen with the manager
    pub fn register_screen(&mut self, screen_type: ScreenType, screen: Box<dyn Screen>) {
        self.screens.insert(screen_type, screen);
    }

    /// Set the rendering backend (crossterm or ratatui)
    pub fn set_render_backend(&mut self, backend: RenderBackend) {
        self.render_backend = backend;
    }

    /// Initialize terminal for raw mode and alternate screen
    pub fn initialize_terminal(&mut self) -> Result<()> {
        if !self.terminal_initialized {
            use crossterm::{execute, terminal, cursor};
            
            terminal::enable_raw_mode()
                .map_err(|e| crate::error::GitTypeError::TerminalError(format!("Failed to enable raw mode: {}", e)))?;
            
            execute!(
                std::io::stdout(),
                terminal::EnterAlternateScreen,
                cursor::Hide
            ).map_err(|e| crate::error::GitTypeError::TerminalError(format!("Failed to initialize terminal: {}", e)))?;
            
            self.terminal_initialized = true;
        }
        Ok(())
    }

    pub fn cleanup_terminal(&mut self) -> Result<()> {
        if self.terminal_initialized {
            use crossterm::{execute, terminal, cursor};
            
            execute!(
                std::io::stdout(),
                terminal::LeaveAlternateScreen,
                cursor::Show
            ).map_err(|e| crate::error::GitTypeError::TerminalError(format!("Failed to restore terminal: {}", e)))?;
            
            terminal::disable_raw_mode()
                .map_err(|e| crate::error::GitTypeError::TerminalError(format!("Failed to disable raw mode: {}", e)))?;
            
            self.terminal_initialized = false;
        }
        Ok(())
    }
    
    pub fn set_current_screen(&mut self, screen_type: ScreenType) -> Result<()> {
        if !self.screens.contains_key(&screen_type) {
            return Err(crate::GitTypeError::TerminalError(format!(
                "Screen not registered: {:?}",
                screen_type
            )));
        }
        
        if let Some(current_screen) = self.screens.get_mut(&self.current_screen_type) {
            current_screen.cleanup()?;
        }
        
        self.current_screen_type = screen_type;
        
        if let Some(new_screen) = self.screens.get_mut(&self.current_screen_type) {
            new_screen.init()?;
        }
        
        Ok(())
    }
    
    pub fn push_screen(&mut self, screen_type: ScreenType) -> Result<()> {
        self.screen_stack.push(self.current_screen_type.clone());
        self.set_current_screen(screen_type)
    }
    
    pub fn pop_screen(&mut self) -> Result<()> {
        if let Some(previous_screen) = self.screen_stack.pop() {
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
        
        if let Some(_) = self.screen_stack.pop() {
            self.set_current_screen(screen_type)
        } else {
            self.set_current_screen(screen_type)
        }
    }
    
    pub fn handle_transition(&mut self, transition: ScreenTransition) -> Result<()> {
        match transition {
            ScreenTransition::None => Ok(()),
            ScreenTransition::Push(screen_type) => self.push_screen(screen_type),
            ScreenTransition::Pop => self.pop_screen(),
            ScreenTransition::Replace(screen_type) => self.set_current_screen(screen_type),
            ScreenTransition::PopTo(screen_type) => self.pop_to_screen(screen_type),
            ScreenTransition::Exit => {
                self.should_exit = true;
                Ok(())
            }
        }
    }
    
    pub fn run(&mut self) -> Result<()> {
        self.initialize_terminal()?;
        
        if let Some(current_screen) = self.screens.get_mut(&self.current_screen_type) {
            current_screen.init()?;
        }
        
        while !self.should_exit {
            self.update_and_render()?;
            self.handle_input()?;
            
            if let Some(screen) = self.screens.get(&self.current_screen_type) {
                if screen.should_exit() {
                    self.should_exit = true;
                }
            }
        }
        
        if let Some(current_screen) = self.screens.get_mut(&self.current_screen_type) {
            current_screen.cleanup()?;
        }
        
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
                UpdateStrategy::Hybrid { interval, input_priority } => {
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
            match read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.code == KeyCode::Char('c') {
                            self.should_exit = true;
                            return Ok(());
                        }

                        let transition = if let Some(screen) = self.screens.get_mut(&self.current_screen_type) {
                            screen.handle_key_event(key_event)?
                        } else {
                            ScreenTransition::None
                        };
                        
                        self.handle_transition(transition)?;
                        
                        let needs_render = if let Some(screen) = self.screens.get_mut(&self.current_screen_type) {
                            screen.update()?
                        } else {
                            false
                        };
                        
                        if needs_render {
                            self.render_current_screen()?;
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn render_current_screen(&mut self) -> Result<()> {
        use std::io::{stdout, Write};
        
        match self.render_backend {
            RenderBackend::Crossterm => {
                let mut stdout = stdout();
                if let Some(screen) = self.screens.get(&self.current_screen_type) {
                    screen.render_crossterm(&mut stdout)?;
                }
                stdout.flush()?;
            }
            RenderBackend::Ratatui => {
                
            }
        }
        Ok(())
    }
    
    pub fn get_current_screen_type(&self) -> &ScreenType {
        &self.current_screen_type
    }
    
    pub fn get_screen_stack(&self) -> &Vec<ScreenType> {
        &self.screen_stack
    }

    pub fn is_terminal_initialized(&self) -> bool {
        self.terminal_initialized
    }
}

impl Drop for ScreenManager {
    fn drop(&mut self) {
        let _ = self.cleanup_terminal();
    }
}

impl Default for ScreenManager {
    fn default() -> Self {
        Self::new()
    }
}

/// A basic screen implementation that displays text content
pub struct BasicScreen {
    title: String,
    content: Vec<String>,
    should_exit: bool,
    update_strategy: UpdateStrategy,
}

impl BasicScreen {
    /// Create a new BasicScreen with specified title, content, and update strategy
    pub fn new(title: String, content: Vec<String>, update_strategy: UpdateStrategy) -> Self {
        Self {
            title,
            content,
            should_exit: false,
            update_strategy,
        }
    }
}

impl Screen for BasicScreen {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<ScreenTransition> {
        use crossterm::event::KeyCode;
        
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.should_exit = true;
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm(&self, stdout: &mut Stdout) -> Result<()> {
        use crossterm::{
            cursor,
            execute,
            style::{Color, Print, ResetColor, SetForegroundColor},
            terminal::{Clear, ClearType},
        };

        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        
        execute!(
            stdout,
            SetForegroundColor(Color::White),
            cursor::MoveTo(2, 1),
            Print(&self.title),
            ResetColor
        )?;
        
        for (i, line) in self.content.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(2, 3 + i as u16), Print(line))?;
        }
        
        execute!(
            stdout,
            cursor::MoveTo(2, (self.content.len() + 5) as u16),
            SetForegroundColor(Color::DarkGrey),
            Print("[ESC/q] Exit"),
            ResetColor
        )?;
        
        Ok(())
    }

    fn render_ratatui(&self, _frame: &mut ratatui::Frame) -> Result<()> {
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        self.update_strategy.clone()
    }

    fn update(&mut self) -> Result<bool> {
        Ok(false)
    }
}