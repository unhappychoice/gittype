use crossterm::event::KeyEvent;
use gittype::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use ratatui::Frame;
use std::any::Any;

struct UnitProvider;

impl ScreenDataProvider for UnitProvider {
    fn provide(&self) -> gittype::Result<Box<dyn Any>> {
        Ok(Box::new(()))
    }
}

struct MinimalScreen;

impl Screen for MinimalScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::Help
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(UnitProvider)
    }

    fn init_with_data(&self, _data: Box<dyn Any>) -> gittype::Result<()> {
        Ok(())
    }

    fn handle_key_event(&self, _key_event: KeyEvent) -> gittype::Result<()> {
        Ok(())
    }

    fn render_ratatui(&self, _frame: &mut Frame) -> gittype::Result<()> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn default_screen_methods_are_noop_and_input_only() {
    let screen = MinimalScreen;
    let source_screen = MinimalScreen;

    assert!(screen.on_pushed_from(&source_screen).is_ok());
    assert!(screen.cleanup().is_ok());
    assert!(screen.update().is_ok_and(|should_render| !should_render));
    assert!(!screen.is_exitable());
    assert!(matches!(
        screen.get_update_strategy(),
        UpdateStrategy::InputOnly
    ));
}
