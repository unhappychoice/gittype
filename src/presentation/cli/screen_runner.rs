use ratatui::backend::CrosstermBackend;
use shaku::HasComponent;

use std::io::Stdout;
use std::sync::{Arc, Mutex};

use crate::presentation::di::AppModule;
use crate::presentation::tui::{Screen, ScreenManagerImpl, ScreenType};
use crate::Result;

/// Runs a single screen with optional data initialization and result extraction
pub fn run_screen<S, D, R, F>(
    screen_type: ScreenType,
    data: Option<D>,
    extract_result: Option<F>,
) -> Result<Option<R>>
where
    S: Screen + 'static,
    D: 'static,
    F: FnOnce(&S) -> Option<R>,
{
    // Create DI container
    let container = AppModule::builder().build();

    // Get ScreenManagerFactory from DI container
    let factory: &dyn crate::presentation::tui::ScreenManagerFactory = container.resolve_ref();

    // Create ScreenManager using DI container factory (all screens already registered)
    let screen_manager = factory.create(&container);

    // Setup event subscriptions
    let manager_ref = Arc::new(Mutex::new(screen_manager));
    ScreenManagerImpl::setup_event_subscriptions(&manager_ref);

    // Initialize terminal and set current screen
    {
        let mut manager = manager_ref.lock().unwrap();
        manager.initialize_terminal()?;
        manager.set_current_screen(screen_type.clone())?;

        // Initialize screen with data
        if let Some(screen_box) = manager.get_screen_mut(&screen_type) {
            let init_data = if let Some(user_data) = data {
                // Use provided data
                Box::new(user_data) as Box<dyn std::any::Any>
            } else {
                // Use default provider to fetch data
                S::default_provider().provide()?
            };
            screen_box.init_with_data(init_data)?;
        }
    }

    // Run the screen manager loop
    {
        let mut manager = manager_ref.lock().unwrap();
        manager.run()?;
    }

    // Extract result if extractor function provided
    let result = if let Some(extractor) = extract_result {
        let manager = manager_ref.lock().unwrap();
        manager
            .get_screen(&screen_type)
            .and_then(|screen| screen.as_any().downcast_ref::<S>())
            .and_then(extractor)
    } else {
        None
    };

    Ok(result)
}

/// Context that holds shared terminal state for running multiple screens sequentially
pub struct ScreenRunnerContext {
    container: AppModule,
    terminal_active: bool,
}

impl ScreenRunnerContext {
    /// Create a new context with initialized terminal
    pub fn new() -> Result<Self> {
        let container = AppModule::builder().build();
        let factory: &dyn crate::presentation::tui::ScreenManagerFactory = container.resolve_ref();
        let screen_manager = factory.create(&container);

        let manager_ref = Arc::new(Mutex::new(screen_manager));
        ScreenManagerImpl::setup_event_subscriptions(&manager_ref);

        {
            let mut manager = manager_ref.lock().unwrap();
            manager.initialize_terminal()?;
            // Prevent Drop from cleaning up terminal - we'll do it manually
            manager.skip_cleanup_on_drop();
        }

        Ok(Self {
            container,
            terminal_active: true,
        })
    }

    /// Run a screen within this context, keeping terminal state between screens
    pub fn run_screen<S, D, R, F>(
        &self,
        screen_type: ScreenType,
        data: Option<D>,
        extract_result: Option<F>,
    ) -> Result<Option<R>>
    where
        S: Screen + 'static,
        D: 'static,
        F: FnOnce(&S) -> Option<R>,
    {
        // Re-create screen manager to get fresh screen state
        let factory: &dyn crate::presentation::tui::ScreenManagerFactory =
            self.container.resolve_ref();
        let screen_manager = factory.create(&self.container);

        // Transfer to new manager ref but keep subscriptions working
        let manager_ref = Arc::new(Mutex::new(screen_manager));
        ScreenManagerImpl::setup_event_subscriptions(&manager_ref);

        // Set current screen and initialize
        {
            let mut manager = manager_ref.lock().unwrap();
            // Terminal is already initialized, just mark it as such
            manager.mark_terminal_initialized();
            manager.set_current_screen(screen_type.clone())?;

            if let Some(screen_box) = manager.get_screen_mut(&screen_type) {
                let init_data = if let Some(user_data) = data {
                    Box::new(user_data) as Box<dyn std::any::Any>
                } else {
                    S::default_provider().provide()?
                };
                screen_box.init_with_data(init_data)?;
            }
        }

        // Run the screen manager loop (without cleanup at end)
        {
            let mut manager = manager_ref.lock().unwrap();
            manager.run_without_cleanup()?;
        }

        // Extract result
        let result = if let Some(extractor) = extract_result {
            let manager = manager_ref.lock().unwrap();
            manager
                .get_screen(&screen_type)
                .and_then(|screen| screen.as_any().downcast_ref::<S>())
                .and_then(extractor)
        } else {
            None
        };

        Ok(result)
    }

    /// Cleanup terminal when done with all screens
    pub fn cleanup(mut self) -> Result<()> {
        if self.terminal_active {
            self.terminal_active = false;
            ScreenManagerImpl::<CrosstermBackend<Stdout>>::cleanup_terminal_static();
        }
        Ok(())
    }
}

impl Drop for ScreenRunnerContext {
    fn drop(&mut self) {
        if self.terminal_active {
            ScreenManagerImpl::<CrosstermBackend<Stdout>>::cleanup_terminal_static();
        }
    }
}
