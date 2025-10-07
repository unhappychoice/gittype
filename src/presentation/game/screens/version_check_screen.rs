use crate::domain::events::EventBus;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::views::VersionCheckView;
use crate::presentation::game::{
    RenderBackend, Screen, ScreenDataProvider, ScreenType, UpdateStrategy,
};
use crate::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::stdout;
use std::io::Stdout;

pub enum VersionCheckResult {
    Continue,
    Exit,
}

pub struct VersionCheckScreen {
    event_bus: EventBus,
}

impl VersionCheckScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }

    pub fn show_legacy(current_version: &str, latest_version: &str) -> Result<VersionCheckResult> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = Self::run_notification_loop(&mut terminal, current_version, latest_version);

        disable_raw_mode()?;
        terminal.backend_mut().execute(LeaveAlternateScreen)?;

        result
    }

    fn run_notification_loop(
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        current_version: &str,
        latest_version: &str,
    ) -> Result<VersionCheckResult> {
        loop {
            terminal.draw(|f| VersionCheckView::draw_ui(f, current_version, latest_version))?;

            if let Ok(true) = event::poll(std::time::Duration::from_millis(50)) {
                if let Ok(Event::Key(key_event)) = event::read() {
                    match key_event.code {
                        KeyCode::Char(' ') => {
                            return Ok(VersionCheckResult::Continue);
                        }
                        KeyCode::Esc => {
                            return Ok(VersionCheckResult::Exit);
                        }
                        _ => {
                            // Ignore other keys
                        }
                    }
                }
            }
        }
    }
}

pub struct VersionCheckScreenDataProvider;

impl ScreenDataProvider for VersionCheckScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

impl Screen for VersionCheckScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::VersionCheck
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(VersionCheckScreenDataProvider)
    }

    fn get_render_backend(&self) -> RenderBackend {
        RenderBackend::Ratatui
    }

    fn init_with_data(&mut self, _data: Box<dyn std::any::Any>) -> Result<()> {
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Esc => {
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_crossterm_with_data(&mut self, _stdout: &mut Stdout) -> crate::Result<()> {
        // Version check is now handled by ScreenManager
        // let current_version = env!("CARGO_PKG_VERSION");
        // let _ = VersionCheckScreen::show(current_version, "1.1.0");
        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> crate::Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
