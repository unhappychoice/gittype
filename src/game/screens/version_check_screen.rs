use crate::game::screen_manager::{Screen, ScreenTransition, UpdateStrategy};
use std::io::Stdout;
use crate::ui::colors::Colors;
use crate::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame, Terminal,
};
use std::io::stdout;

pub enum VersionCheckResult {
    Continue,
    Exit,
}

pub struct VersionCheckScreen;

impl VersionCheckScreen {
    pub fn show(current_version: &str, latest_version: &str) -> Result<VersionCheckResult> {
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
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        current_version: &str,
        latest_version: &str,
    ) -> Result<VersionCheckResult> {
        loop {
            terminal.draw(|f| Self::draw_ui(f, current_version, latest_version))?;

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

    fn draw_ui(f: &mut Frame, current_version: &str, latest_version: &str) {
        let size = f.area();

        // Create main layout for content and controls
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Content area
                Constraint::Length(2), // Control instructions area
            ])
            .split(size);

        // Create centered content area (no border)
        let content_area = Self::centered_rect(90, 60, main_chunks[0]);

        // Create layout for content
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Length(2), // Current version
                Constraint::Length(2), // Latest version
                Constraint::Min(1),    // Install instruction
            ])
            .split(content_area);

        // Title
        let title_text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "🎮 GitType Update Available",
                Style::default()
                    .fg(Colors::TITLE)
                    .add_modifier(Modifier::BOLD),
            )]),
        ];
        let title_para = Paragraph::new(title_text);
        f.render_widget(title_para, chunks[0]);

        // Current version
        let current_text = vec![Line::from(vec![
            Span::styled("Current version: ", Style::default().fg(Colors::TEXT)),
            Span::styled(
                format!("v{}", current_version),
                Style::default()
                    .fg(Colors::TEXT)
                    .add_modifier(Modifier::BOLD),
            ),
        ])];
        let current_para = Paragraph::new(current_text);
        f.render_widget(current_para, chunks[1]);

        // Latest version
        let latest_text = vec![Line::from(vec![
            Span::styled("Latest version:  ", Style::default().fg(Colors::TEXT)),
            Span::styled(
                format!("v{}", latest_version),
                Style::default()
                    .fg(Colors::SUCCESS)
                    .add_modifier(Modifier::BOLD),
            ),
        ])];
        let latest_para = Paragraph::new(latest_text);
        f.render_widget(latest_para, chunks[2]);

        // Install instruction with word wrap for narrow terminals
        let install_text = vec![
            Line::from(""),
            Line::from("To update, run:"),
            Line::from(""),
            Line::from("curl -sSL https://raw.githubusercontent.com/unhappychoice/gittype/main/install.sh | bash"),
        ];
        let install_para = Paragraph::new(install_text)
            .style(Style::default().fg(Colors::SECONDARY))
            .wrap(Wrap { trim: true });
        f.render_widget(install_para, chunks[3]);

        // Control instructions with same margins as content
        let control_area = Self::centered_rect(90, 100, main_chunks[1]);
        let control_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "[SPACE] ",
                    Style::default()
                        .fg(Colors::SUCCESS)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("Continue", Style::default().fg(Colors::TEXT)),
                Span::styled("  ", Style::default()),
                Span::styled(
                    "[ESC] ",
                    Style::default()
                        .fg(Colors::ERROR)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("Exit", Style::default().fg(Colors::TEXT)),
            ]),
        ];
        let control_para = Paragraph::new(control_text);
        f.render_widget(control_para, control_area);
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

// Basic Screen trait implementation for ScreenManager compatibility
pub struct ScreenState {
    should_exit: bool,
}

impl ScreenState {
    pub fn new() -> Self {
        Self { should_exit: false }
    }
}

impl Screen for ScreenState {
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> crate::Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Esc => {
                self.should_exit = true;
                Ok(ScreenTransition::None)
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_exit = true;
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm(&self, _stdout: &mut Stdout) -> crate::Result<()> {
        // TODO: Use real version data instead of dummy
        let _ = VersionCheckScreen::show("1.0.0", "1.1.0");
        Ok(())
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> crate::Result<bool> {
        Ok(false)
    }
}
