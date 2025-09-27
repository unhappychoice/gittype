use crate::presentation::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::presentation::game::views::version_check::VersionCheckView;
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

pub struct VersionCheckScreen;

impl VersionCheckScreen {
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

pub struct ScreenState {}

impl Default for ScreenState {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenState {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for ScreenState {
    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Esc => Ok(ScreenTransition::Exit),
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut Stdout,
        _session_result: Option<&crate::domain::models::SessionResult>,
        _total_result: Option<&crate::domain::services::scoring::TotalResult>,
    ) -> crate::Result<()> {
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
