use crate::domain::models::TotalResult;
use crate::presentation::sharing::SharingPlatform;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct SharingView;

impl SharingView {
    pub fn render_menu(frame: &mut Frame, total_summary: &TotalResult, colors: &Colors) {
        let area = frame.area();
        let platforms = SharingPlatform::all();

        // Calculate total height
        let title_height = 1;
        let title_spacing = 2;
        let preview_height = 1;
        let preview_spacing = 2;
        let platforms_height = platforms.len() as u16;
        let platforms_spacing = 2;
        let back_height = 1;

        let total_content_height = title_height
            + title_spacing
            + preview_height
            + preview_spacing
            + platforms_height
            + platforms_spacing
            + back_height;

        let top_padding = (area.height.saturating_sub(total_content_height)) / 2;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(top_padding),
                Constraint::Length(title_height),
                Constraint::Length(title_spacing),
                Constraint::Length(preview_height),
                Constraint::Length(preview_spacing),
                Constraint::Length(platforms_height),
                Constraint::Length(platforms_spacing),
                Constraint::Length(back_height),
                Constraint::Min(0),
            ])
            .split(area);

        // Title
        let title = Paragraph::new(Line::from(vec![Span::styled(
            "Share Your Total Results",
            Style::default()
                .fg(colors.info())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(title, chunks[1]);

        // Preview line with colors
        let preview = Line::from(vec![
            Span::styled("Score: ", Style::default().fg(colors.score())),
            Span::styled(
                format!("{:.0}", total_summary.total_score),
                Style::default().fg(colors.text()),
            ),
            Span::styled(", ", Style::default().fg(colors.text())),
            Span::styled("CPM: ", Style::default().fg(colors.cpm_wpm())),
            Span::styled(
                format!("{:.0}", total_summary.overall_cpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(", ", Style::default().fg(colors.text())),
            Span::styled("Keystrokes: ", Style::default().fg(colors.stage_info())),
            Span::styled(
                format!("{}", total_summary.total_keystrokes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(", ", Style::default().fg(colors.text())),
            Span::styled("Sessions: ", Style::default().fg(colors.info())),
            Span::styled(
                format!(
                    "{}/{}",
                    total_summary.total_sessions_completed, total_summary.total_sessions_attempted
                ),
                Style::default().fg(colors.text()),
            ),
            Span::styled(", ", Style::default().fg(colors.text())),
            Span::styled("Time: ", Style::default().fg(colors.duration())),
            Span::styled(
                format!(
                    "{:.1}min",
                    total_summary.total_duration.as_secs_f64() / 60.0
                ),
                Style::default().fg(colors.text()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(preview).alignment(Alignment::Center),
            chunks[3],
        );

        // Platform options
        let platform_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); platforms.len()])
            .split(chunks[5]);

        for (i, platform) in platforms.iter().enumerate() {
            let option = Line::from(vec![
                Span::styled(
                    format!("[{}]", i + 1),
                    Style::default().fg(colors.success()),
                ),
                Span::styled(
                    format!(" {}", platform.name()),
                    Style::default().fg(colors.text()),
                ),
            ]);
            frame.render_widget(
                Paragraph::new(option).alignment(Alignment::Center),
                platform_chunks[i],
            );
        }

        // Back option
        let back = Line::from(vec![
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(" Back to Exit Screen", Style::default().fg(colors.text())),
        ]);
        frame.render_widget(Paragraph::new(back).alignment(Alignment::Center), chunks[7]);
    }

    pub fn render_fallback_url(
        frame: &mut Frame,
        url: &str,
        platform: &SharingPlatform,
        colors: &Colors,
    ) {
        let area = frame.area();

        // Calculate total height
        let title_height = 1;
        let title_spacing = 2;
        let instruction_height = 1;
        let instruction_spacing = 3;
        let url_height = 1;
        let url_spacing = 5;
        let exit_height = 1;

        let total_content_height = title_height
            + title_spacing
            + instruction_height
            + instruction_spacing
            + url_height
            + url_spacing
            + exit_height;

        let top_padding = (area.height.saturating_sub(total_content_height)) / 2;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(top_padding),
                Constraint::Length(title_height),
                Constraint::Length(title_spacing),
                Constraint::Length(instruction_height),
                Constraint::Length(instruction_spacing),
                Constraint::Length(url_height),
                Constraint::Length(url_spacing),
                Constraint::Length(exit_height),
                Constraint::Min(0),
            ])
            .split(area);

        // Title
        let title = Paragraph::new(Line::from(vec![Span::styled(
            format!("Could not open {} automatically", platform.name()),
            Style::default()
                .fg(colors.warning())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(title, chunks[1]);

        // Instructions
        let instruction = Paragraph::new(Line::from(vec![Span::styled(
            "Please copy the URL below and open it in your browser:",
            Style::default().fg(colors.text()),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(instruction, chunks[3]);

        // URL display
        let url_widget = Paragraph::new(Line::from(vec![Span::styled(
            url,
            Style::default()
                .fg(colors.info())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(url_widget, chunks[5]);

        // Exit option
        let exit = Line::from(vec![
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(" Exit", Style::default().fg(colors.text())),
        ]);
        frame.render_widget(Paragraph::new(exit).alignment(Alignment::Center), chunks[7]);
    }

    pub fn render_exit_options(frame: &mut Frame, area: ratatui::layout::Rect, colors: &Colors) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Thanks message
                Constraint::Length(1), // GitHub link
                Constraint::Length(1), // Spacing
                Constraint::Length(1), // Share option
                Constraint::Length(1), // Exit option
            ])
            .split(area);

        // Thanks message
        let thanks = Paragraph::new(Line::from(vec![Span::styled(
            "Thanks for playing GitType!",
            Style::default()
                .fg(colors.success())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(thanks, chunks[0]);

        // GitHub link
        let github = Paragraph::new(Line::from(vec![Span::styled(
            "✨ Star us on GitHub: https://github.com/unhappychoice/gittype",
            Style::default().fg(colors.warning()),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(github, chunks[1]);

        // Share option
        let share = Line::from(vec![
            Span::styled("[S]", Style::default().fg(colors.success())),
            Span::styled(" Share Result", Style::default().fg(colors.text())),
        ]);
        frame.render_widget(
            Paragraph::new(share).alignment(Alignment::Center),
            chunks[3],
        );

        // Exit option
        let exit = Line::from(vec![
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(" Exit", Style::default().fg(colors.text())),
        ]);
        frame.render_widget(Paragraph::new(exit).alignment(Alignment::Center), chunks[4]);
    }
}
