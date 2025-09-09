use gittype::game::{BasicScreen, ScreenManager, ScreenType, UpdateStrategy};

fn main() -> gittype::Result<()> {
    let mut screen_manager = ScreenManager::new();
    
    let title_screen = BasicScreen::new(
        "ScreenManager Demo".to_string(),
        vec![
            "Welcome to the new ScreenManager architecture!".to_string(),
            "".to_string(),
            "Features implemented:".to_string(),
            "✓ Centralized rendering loop".to_string(),
            "✓ Input handling with event dispatching".to_string(),
            "✓ Screen management with stack support".to_string(),
            "✓ Dual rendering support (crossterm/ratatui)".to_string(),
            "✓ Flexible update strategies".to_string(),
            "✓ Terminal lifecycle management".to_string(),
            "".to_string(),
            "Update Strategies:".to_string(),
            "• InputOnly - Updates only on user input".to_string(),
            "• TimeBased - Updates at regular intervals".to_string(),
            "• Hybrid - Combines both strategies".to_string(),
            "".to_string(),
            "This screen uses InputOnly strategy.".to_string(),
        ],
        UpdateStrategy::InputOnly,
    );
    
    screen_manager.register_screen(ScreenType::Title, Box::new(title_screen));
    
    screen_manager.run()?;
    
    Ok(())
}