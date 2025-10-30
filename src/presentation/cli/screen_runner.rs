use crate::domain::events::{EventBus, EventBusInterface};
use crate::presentation::game::GameData;
use crate::presentation::tui::{Screen, ScreenManagerImpl, ScreenType};
use crate::Result;
use std::sync::{Arc, Mutex};

/// Runs a single screen with optional data initialization and result extraction
pub fn run_screen<S, D, R, F>(
    screen_type: ScreenType,
    screen_factory: impl FnOnce(Arc<dyn EventBusInterface>) -> S,
    data: Option<D>,
    extract_result: Option<F>,
) -> Result<Option<R>>
where
    S: Screen + 'static,
    D: 'static,
    F: FnOnce(&S) -> Option<R>,
{
    // Create EventBus and ScreenManager
    let event_bus: Arc<dyn EventBusInterface> = Arc::new(EventBus::new());
    let backend = ratatui::backend::CrosstermBackend::new(std::io::stdout());
    let terminal = ratatui::Terminal::new(backend).map_err(|e| {
        crate::GitTypeError::TerminalError(format!("Failed to create terminal: {}", e))
    })?;
    let mut screen_manager =
        ScreenManagerImpl::new(Arc::clone(&event_bus), GameData::instance(), terminal);

    // Create and register screen
    let screen = screen_factory(Arc::clone(&event_bus));
    screen_manager.register_screen(screen);

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
