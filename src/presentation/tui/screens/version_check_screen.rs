use crate::domain::events::EventBusInterface;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::tui::views::VersionCheckView;
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Stdout};
use std::sync::Arc;
use std::sync::RwLock;

pub enum VersionCheckResult {
    Continue,
    Exit,
}

pub trait VersionCheckScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = VersionCheckScreenInterface)]
pub struct VersionCheckScreen {
    current_version: RwLock<String>,
    latest_version: RwLock<String>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
}

impl VersionCheckScreen {
    pub fn new(event_bus: Arc<dyn EventBusInterface>) -> Self {
        Self {
            event_bus,
            current_version: RwLock::new(String::new()),
            latest_version: RwLock::new(String::new()),
        }
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

    fn init_with_data(&self, _data: Box<dyn std::any::Any>) -> Result<()> {
        Ok(())
    }

    fn handle_key_event(&self, key_event: event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Esc => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&self) -> crate::Result<bool> {
        Ok(false)
    }

    fn render_ratatui(&self, frame: &mut ratatui::Frame) -> Result<()> {
        let current = self.current_version.read().unwrap();
        let latest = self.latest_version.read().unwrap();
        VersionCheckView::draw_ui(frame, &current, &latest);
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl VersionCheckScreenInterface for VersionCheckScreen {}
