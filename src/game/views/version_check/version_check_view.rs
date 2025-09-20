use crate::ui::colors::Colors;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

pub struct VersionCheckView;

impl VersionCheckView {
    pub fn draw_ui(f: &mut Frame, current_version: &str, latest_version: &str) {
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
                    .fg(Colors::title())
                    .add_modifier(Modifier::BOLD),
            )]),
        ];
        let title_para = Paragraph::new(title_text);
        f.render_widget(title_para, chunks[0]);

        // Current version
        let current_text = vec![Line::from(vec![
            Span::styled("Current version: ", Style::default().fg(Colors::text())),
            Span::styled(
                format!("v{}", current_version),
                Style::default()
                    .fg(Colors::text())
                    .add_modifier(Modifier::BOLD),
            ),
        ])];
        let current_para = Paragraph::new(current_text);
        f.render_widget(current_para, chunks[1]);

        // Latest version
        let latest_text = vec![Line::from(vec![
            Span::styled("Latest version:  ", Style::default().fg(Colors::text())),
            Span::styled(
                format!("v{}", latest_version),
                Style::default()
                    .fg(Colors::success())
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
            .style(Style::default().fg(Colors::secondary()))
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
                        .fg(Colors::success())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("Continue", Style::default().fg(Colors::text())),
                Span::styled("  ", Style::default()),
                Span::styled(
                    "[ESC] ",
                    Style::default()
                        .fg(Colors::error())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("Exit", Style::default().fg(Colors::text())),
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
