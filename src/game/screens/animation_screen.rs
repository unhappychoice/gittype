use crate::game::typing_animation::{AnimationPhase, TypingAnimation};
use crate::scoring::{RankingTitle, ScoringEngine};
use crate::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame, Terminal,
};
use std::io;

pub struct AnimationScreen;

impl AnimationScreen {
    // Helper function to convert crossterm::Color to ratatui::Color
    fn convert_crossterm_color(color: crossterm::style::Color) -> ratatui::style::Color {
        match color {
            crossterm::style::Color::Black => ratatui::style::Color::Black,
            crossterm::style::Color::DarkGrey => ratatui::style::Color::DarkGray,
            crossterm::style::Color::Red => ratatui::style::Color::Red,
            crossterm::style::Color::DarkRed => ratatui::style::Color::DarkGray,
            crossterm::style::Color::Green => ratatui::style::Color::Green,
            crossterm::style::Color::DarkGreen => ratatui::style::Color::DarkGray,
            crossterm::style::Color::Yellow => ratatui::style::Color::Yellow,
            crossterm::style::Color::DarkYellow => ratatui::style::Color::DarkGray,
            crossterm::style::Color::Blue => ratatui::style::Color::Blue,
            crossterm::style::Color::DarkBlue => ratatui::style::Color::DarkGray,
            crossterm::style::Color::Magenta => ratatui::style::Color::Magenta,
            crossterm::style::Color::DarkMagenta => ratatui::style::Color::DarkGray,
            crossterm::style::Color::Cyan => ratatui::style::Color::Cyan,
            crossterm::style::Color::DarkCyan => ratatui::style::Color::DarkGray,
            crossterm::style::Color::White => ratatui::style::Color::White,
            crossterm::style::Color::Grey => ratatui::style::Color::Gray,
            _ => ratatui::style::Color::White, // Default fallback
        }
    }

    // Helper function to render typing animation with ratatui
    fn render_typing_animation_ratatui(
        frame: &mut Frame,
        animation: &TypingAnimation,
        _ranking_title: &str,
    ) {
        let area = frame.size();

        // Create vertical layout for centering
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40), // Top padding
                Constraint::Min(4),         // Animation area
                Constraint::Percentage(40), // Bottom padding
            ])
            .split(area);

        match animation.get_current_phase() {
            AnimationPhase::ConcentrationLines => {
                let mut lines = Vec::new();

                for (i, line) in animation.get_hacking_lines().iter().enumerate() {
                    let text = &line.text[..line.typed_length];
                    let line_color = Self::convert_crossterm_color(line.color);

                    if i == animation.get_current_line()
                        && line.typed_length < line.text.len()
                        && !line.completed
                    {
                        // Show cursor on current line
                        lines.push(Line::from(vec![
                            Span::styled(text, Style::default().fg(line_color)),
                            Span::styled("█", Style::default().fg(ratatui::style::Color::White)),
                        ]));
                    } else if !text.is_empty() {
                        // Regular completed or typing line
                        lines.push(Line::from(Span::styled(
                            text,
                            Style::default().fg(line_color),
                        )));
                    } else {
                        // Empty placeholder line
                        lines.push(Line::from(""));
                    }
                }

                let paragraph = Paragraph::new(Text::from(lines)).alignment(Alignment::Center);

                frame.render_widget(paragraph, chunks[1]);

                // Render skip hint in bottom right
                Self::render_skip_hint(frame, area);
            }
            AnimationPhase::Pause => {
                // Show all completed lines plus dots
                let mut lines = Vec::new();

                for line in animation.get_hacking_lines().iter() {
                    let line_color = Self::convert_crossterm_color(line.color);
                    lines.push(Line::from(Span::styled(
                        &line.text,
                        Style::default().fg(line_color),
                    )));
                }

                // Add dots line
                let dots = ".".repeat(animation.get_pause_dots());
                lines.push(Line::from(Span::styled(
                    dots,
                    Style::default().fg(ratatui::style::Color::Gray),
                )));

                let paragraph = Paragraph::new(Text::from(lines)).alignment(Alignment::Center);

                frame.render_widget(paragraph, chunks[1]);

                // Render skip hint in bottom right
                Self::render_skip_hint(frame, area);
            }
            AnimationPhase::Complete => {
                // Animation is complete, ready to transition to result
            }
        }
    }

    // Helper function to render skip hint in bottom right corner
    fn render_skip_hint(frame: &mut Frame, area: ratatui::layout::Rect) {
        let skip_text = "[S] Skip";
        let skip_width = skip_text.len() as u16;
        let skip_height = 1;

        // Position in bottom right corner with small margin
        let skip_x = area.width.saturating_sub(skip_width + 1);
        let skip_y = area.height.saturating_sub(skip_height + 1);

        let skip_area = ratatui::layout::Rect {
            x: skip_x,
            y: skip_y,
            width: skip_width,
            height: skip_height,
        };

        let skip_paragraph =
            Paragraph::new(skip_text).style(Style::default().fg(ratatui::style::Color::Gray));

        frame.render_widget(skip_paragraph, skip_area);
    }

    // Helper function to get tier from ranking title name
    fn get_tier_from_title(title_name: &str) -> crate::models::Rank {
        RankingTitle::all_titles()
            .iter()
            .find(|title| title.name() == title_name)
            .map(|title| title.tier().clone())
            .unwrap_or(crate::models::Rank::Beginner)
    }

    pub fn show_session_animation(
        _total_stages: usize,
        _completed_stages: usize,
        stage_engines: &[(String, ScoringEngine)],
    ) -> Result<()> {
        // Calculate aggregated session metrics by combining ScoringEngines with + operator
        if stage_engines.is_empty() {
            return Ok(());
        }

        let combined_engine = stage_engines
            .iter()
            .map(|(_, engine)| engine.clone())
            .reduce(|acc, engine| acc + engine)
            .unwrap(); // Safe because we checked is_empty() above

        let session_metrics = match combined_engine.calculate_result() {
            Ok(metrics) => metrics,
            Err(_) => {
                // Fallback if calculation fails
                return Ok(());
            }
        };

        // Set up ratatui terminal
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        // Create typing animation for session complete
        let tier = Self::get_tier_from_title(&session_metrics.ranking_title);
        let mut typing_animation =
            TypingAnimation::new(tier, terminal.size()?.width, terminal.size()?.height);
        typing_animation.set_rank_messages(&session_metrics.ranking_title);

        // Show typing reveal animation with ratatui
        while !typing_animation.is_complete() {
            let updated = typing_animation.update();

            if updated {
                let ranking_title = session_metrics.ranking_title.clone();
                terminal.draw(|frame| {
                    Self::render_typing_animation_ratatui(frame, &typing_animation, &ranking_title);
                })?;
            }

            // Check for S key to skip animation
            if event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            break;
                        }
                        _ => {
                            // Ignore other keys to prevent accidental skipping
                        }
                    }
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(16)); // ~60fps
        }

        Ok(())
    }
}
