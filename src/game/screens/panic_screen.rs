use crate::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::logging::get_current_log_file_path;
use crate::presentation::ui::colors::Colors;
use crate::Result;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};
use std::io::Stdout;

pub struct PanicScreen {
    error_message: String,
    timestamp: String,
}

impl Default for PanicScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl PanicScreen {
    pub fn new() -> Self {
        Self {
            error_message: "An unexpected error occurred".to_string(),
            timestamp: Self::get_current_timestamp(),
        }
    }

    pub fn with_error_message(error_message: String) -> Self {
        Self {
            error_message,
            timestamp: Self::get_current_timestamp(),
        }
    }

    fn get_current_timestamp() -> String {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| {
                let secs = d.as_secs();
                let dt = std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs);
                format!("{:?}", dt)
            })
            .unwrap_or_else(|_| "Unknown time".to_string())
    }

    fn render_panic_content(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Apology message
                Constraint::Length(9), // Help info
                Constraint::Min(6),    // Error details (flexible)
                Constraint::Length(1), // ESC key (no border)
            ])
            .split(frame.area());

        // Render header (no border)
        let header_lines = vec![Line::from(""), Line::from("💥 APPLICATION PANIC 💥")];
        let header = Paragraph::new(header_lines)
            .style(Style::default().fg(Colors::error()))
            .alignment(Alignment::Center);
        frame.render_widget(header, chunks[0]);

        // Render apology message (no border)
        let apology_lines = vec![
            Line::from(Span::styled(
                "We sincerely apologize for this unexpected error.",
                Style::default().fg(Colors::error()),
            )),
            Line::from(Span::styled(
                "Progress is saved per session. Current session data may be lost.",
                Style::default().fg(Colors::warning()),
            )),
        ];
        let apology_widget = Paragraph::new(apology_lines).alignment(Alignment::Center);
        frame.render_widget(apology_widget, chunks[1]);

        // Render help information
        let log_path = get_current_log_file_path()
            .unwrap_or_else(|| "~/.gittype/logs/gittype_YYYYMMDD_HHMMSS.log".to_string());

        let help_lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "📋 Logs saved to:",
                Style::default().fg(Colors::success()),
            )),
            Line::from(Span::raw(log_path)),
            Line::from(""),
            Line::from(Span::styled(
                "🐛 Report this issue:",
                Style::default().fg(Colors::stage_info()),
            )),
            Line::from(Span::raw("https://github.com/unhappychoice/gittype/issues")),
            Line::from(""),
        ];

        let help_widget = Paragraph::new(help_lines)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Help"));
        frame.render_widget(help_widget, chunks[2]);

        // Render detailed error info
        let mut error_lines = vec![
            Line::from(Span::styled("Time:", Style::default().fg(Colors::info()))),
            Line::from(Span::raw(&self.timestamp)),
            Line::from(""),
            Line::from(Span::styled(
                "Panic Message:",
                Style::default().fg(Colors::error()),
            )),
        ];

        // Add error message lines (split for better readability)
        for line in self.error_message.lines() {
            // Check if this line contains Location info and format it properly
            if line.trim().starts_with("Location:") {
                error_lines.push(Line::from(""));
                error_lines.push(Line::from(Span::styled(
                    "Location:",
                    Style::default().fg(Colors::info()),
                )));
                let location_info = line.trim().strip_prefix("Location:").unwrap_or("").trim();
                error_lines.push(Line::from(location_info));
            } else {
                error_lines.push(Line::from(line));
            }
        }

        let error_info = Paragraph::new(error_lines)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Error Details")
                    .padding(Padding::uniform(1)),
            );
        frame.render_widget(error_info, chunks[3]);

        // Render ESC key at the bottom (no border, consistent with other screens)
        let esc_instruction = Paragraph::new(Line::from(vec![
            Span::styled("[ESC]", Style::default().fg(Colors::key_back())),
            Span::raw(" Exit"),
        ]))
        .alignment(Alignment::Center);
        frame.render_widget(esc_instruction, chunks[4]);
    }
}

impl Screen for PanicScreen {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Result<ScreenTransition> {
        use crossterm::event::KeyCode;
        match key_event.code {
            KeyCode::Esc => Ok(ScreenTransition::Exit),
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut Stdout,
        _session_result: Option<&crate::domain::models::SessionResult>,
        _total_result: Option<&crate::domain::services::scoring::TotalResult>,
    ) -> Result<()> {
        // This is a fallback - panic screen should use ratatui
        Ok(())
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
        self.render_panic_content(frame);
        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
