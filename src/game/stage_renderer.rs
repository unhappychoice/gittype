use crate::game::context_loader::CodeContext;
use crate::models::Challenge;
use crate::{models::GitRepository, Result};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};
use std::io;

struct ContentRenderParams<'a> {
    current_display_position: usize,
    current_line_number: usize,
    current_mistake_position: Option<usize>,
    terminal_width: u16,
    challenge: Option<&'a Challenge>,
    display_comment_ranges: &'a [(usize, usize)],
    code_context: &'a CodeContext,
}

pub struct StageRenderer {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    chars: Vec<char>,
    last_position: usize,
}

impl StageRenderer {
    pub fn new(challenge_text: &str) -> Result<Self> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        Ok(Self {
            terminal,
            chars: challenge_text.chars().collect(),
            last_position: 0,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn display_challenge_with_info(
        &mut self,
        challenge_text: &str,
        current_display_position: usize,
        current_line: usize,
        mistakes: usize,
        challenge: Option<&Challenge>,
        current_mistake_position: Option<usize>,
        skips_remaining: usize,
        dialog_shown: bool,
        scoring_engine: &crate::scoring::engine::ScoringEngine,
        repo_info: &Option<GitRepository>,
        display_comment_ranges: &[(usize, usize)],
        code_context: &CodeContext,
    ) -> Result<()> {
        // Update character cache if needed
        if self.chars.len() != challenge_text.chars().count() {
            self.chars = challenge_text.chars().collect();
        }

        // Pre-calculate progress percentage for progress bar
        let progress_percent = if !self.chars.is_empty() {
            (current_display_position as f32 / self.chars.len() as f32 * 100.0) as u8
        } else {
            0
        };

        let header_text = if let Some(challenge) = challenge {
            let difficulty_text = match &challenge.difficulty_level {
                Some(difficulty) => format!("{:?}", difficulty),
                None => "Unknown".to_string(),
            };
            format!(
                "[{}] [{}]",
                challenge.get_display_title_with_repo(repo_info),
                difficulty_text
            )
        } else {
            "[Challenge]".to_string()
        };

        let terminal_size = self.terminal.size()?;
        let render_params = ContentRenderParams {
            current_display_position,
            current_line_number: current_line,
            current_mistake_position,
            terminal_width: terminal_size.width,
            challenge,
            display_comment_ranges,
            code_context,
        };
        let content_spans = self.create_content_spans(render_params);

        let elapsed_time = scoring_engine.get_elapsed_time();
        let metrics = crate::scoring::engine::ScoringEngine::calculate_real_time_result(
            current_display_position,
            mistakes,
            elapsed_time,
        );
        let current_line = 0; // Simplified - no line tracking needed
        let elapsed_secs = elapsed_time.as_secs();

        let streak = scoring_engine.get_current_streak();
        let first_line = format!(
            "WPM: {:.0} | CPM: {:.0} | Accuracy: {:.0}% | Mistakes: {} | Streak: {} | Time: {}s | Skips: {}",
            metrics.wpm, metrics.cpm, metrics.accuracy, metrics.mistakes, streak, elapsed_secs, skips_remaining
        );

        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1) // Add margin for better spacing
                .constraints(
                    [
                        Constraint::Length(3), // Header (more compact - only challenge info)
                        Constraint::Min(3),    // Content (minimum height)
                        Constraint::Length(3), // Metrics section (compact)
                        Constraint::Length(3), // Progress bar (compact)
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // Header with basic info
            let header = Paragraph::new(vec![Line::from(header_text.clone())]).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title("Challenge")
                    .title_style(Style::default().fg(Color::Cyan))
                    .padding(ratatui::widgets::Padding::horizontal(1)),
            ); // Only horizontal padding
            f.render_widget(header, chunks[0]);

            // Content with syntax highlighting and cursor with padding
            let scroll_offset = if current_line > chunks[1].height.saturating_sub(2) as usize / 2 {
                (current_line - chunks[1].height.saturating_sub(2) as usize / 2) as u16
            } else {
                0
            };

            let content = Paragraph::new(Text::from(content_spans.clone()))
                .scroll((scroll_offset, 0))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Blue))
                        .title("Code")
                        .title_style(Style::default().fg(Color::LightBlue))
                        .padding(ratatui::widgets::Padding::uniform(1)),
                );
            f.render_widget(content, chunks[1]);

            // Metrics section below the code - less prominent
            let metrics_widget = Paragraph::new(vec![Line::from(vec![Span::styled(
                first_line.clone(),
                Style::default().fg(Color::DarkGray),
            )])])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title("Metrics")
                    .title_style(Style::default().fg(Color::DarkGray))
                    .padding(ratatui::widgets::Padding::horizontal(1)),
            ); // Only horizontal padding
            f.render_widget(metrics_widget, chunks[2]);

            // Progress bar using ratatui's Gauge widget
            let progress_widget = Gauge::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray))
                        .title("Progress")
                        .title_style(Style::default().fg(Color::DarkGray)),
                )
                .gauge_style(Style::default().fg(Color::Cyan))
                .percent(progress_percent as u16)
                .label(format!("{}%", progress_percent));
            f.render_widget(progress_widget, chunks[3]);

            // Render [ESC] Options in bottom left without border
            let esc_area = ratatui::layout::Rect {
                x: 1,                                 // Left margin
                y: f.size().height.saturating_sub(1), // Bottom of screen
                width: 15,                            // Width for "[ESC] Options"
                height: 1,
            };
            let esc_text = Paragraph::new(vec![Line::from(vec![
                Span::styled("[ESC]", Style::default().fg(Color::LightBlue)),
                Span::styled(" Options", Style::default().fg(Color::White)),
            ])]);
            f.render_widget(esc_text, esc_area);

            // Render dialog if shown
            if dialog_shown {
                Self::render_dialog(f, skips_remaining);
            }
        })?;

        self.last_position = current_display_position;
        Ok(())
    }

    fn create_content_spans(&self, params: ContentRenderParams) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let mut current_line_spans = Vec::new();
        let mut current_line_width = 0u16;
        let mut line_number = 0;
        let mut line_start = true;

        // Reserve space for line numbers
        let line_number_width = 6u16;
        let max_width = params.terminal_width.saturating_sub(line_number_width + 1);

        // Get the starting line number from challenge
        let start_line_number = params.challenge.and_then(|c| c.start_line).unwrap_or(1);

        // Add pre-context lines (read-only, dimmed)
        for (ctx_idx, pre_line) in params.code_context.pre_context.iter().enumerate() {
            let ctx_line_number =
                start_line_number.saturating_sub(params.code_context.pre_context.len() - ctx_idx);
            let line_num_text = format!("{:>4} │ ", ctx_line_number);
            let mut line_spans = vec![Span::styled(
                line_num_text,
                Style::default().fg(Color::DarkGray),
            )];

            // Add the context line content with dimmed style
            line_spans.push(Span::styled(
                pre_line.clone(),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            ));

            lines.push(Line::from(line_spans));
        }

        for (i, &ch) in self.chars.iter().enumerate() {
            // Add line number at the start of each line
            if line_start {
                let actual_line_number = start_line_number + line_number;
                let line_num_text = format!("{:>4} │ ", actual_line_number);
                let line_num_style = if line_number == params.current_line_number {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(ratatui::style::Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                current_line_spans.push(Span::styled(line_num_text, line_num_style));
                current_line_width += line_number_width;
                line_start = false;
            }

            // Handle newlines
            if ch == '\n' {
                lines.push(Line::from(current_line_spans));
                current_line_spans = Vec::new();
                current_line_width = 0;
                line_number += 1;
                line_start = true;
                continue;
            }

            // Check if this character is in a comment
            let is_in_comment = params.display_comment_ranges
                .iter()
                .any(|&(start, end)| i >= start && i < end);

            // Determine character style
            let style = if is_in_comment {
                // Comments use same color as context lines
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM)
            } else if i < params.current_display_position {
                // Already typed - light blue dimmed for non-comments
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::DIM)
            } else if i == params.current_display_position {
                // Current cursor position - highlighted
                if let Some(mistake_pos) = params.current_mistake_position {
                    if i == mistake_pos {
                        Style::default().fg(Color::White).bg(Color::Red)
                    } else {
                        Style::default().fg(Color::White).bg(Color::DarkGray)
                    }
                } else {
                    Style::default().fg(Color::White).bg(Color::DarkGray)
                }
            } else {
                // Not yet typed - dim white for non-comments
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::DIM)
            };

            let (display_char, char_width) = match ch {
                '\t' => ("    ".to_string(), 4),
                c if c.is_control() => ("?".to_string(), 1),
                c => (c.to_string(), 1),
            };

            // Check if we need to wrap
            if current_line_width + char_width > max_width {
                lines.push(Line::from(current_line_spans));
                current_line_spans = Vec::new();
                current_line_width = 0;
            }

            current_line_spans.push(Span::styled(display_char, style));
            current_line_width += char_width;
        }

        if !current_line_spans.is_empty() {
            lines.push(Line::from(current_line_spans));
        }

        // Add post-context lines (read-only, dimmed)
        let end_line_number = params.challenge
            .and_then(|c| c.end_line)
            .unwrap_or(start_line_number);
        for (ctx_idx, post_line) in params.code_context.post_context.iter().enumerate() {
            let ctx_line_number = end_line_number + ctx_idx + 1;
            let line_num_text = format!("{:>4} │ ", ctx_line_number);
            let mut line_spans = vec![Span::styled(
                line_num_text,
                Style::default().fg(Color::DarkGray),
            )];

            // Add the context line content with dimmed style
            line_spans.push(Span::styled(
                post_line.clone(),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            ));

            lines.push(Line::from(line_spans));
        }

        if lines.is_empty() {
            let line_num_text = format!("{:>4} │ ", start_line_number);
            let line_num_style = Style::default().fg(Color::DarkGray);
            lines.push(Line::from(vec![Span::styled(
                line_num_text,
                line_num_style,
            )]));
        }

        lines
    }

    pub fn cleanup(&mut self) -> Result<()> {
        use crossterm::{cursor::MoveTo, execute, terminal::ClearType};
        use std::io::{stdout, Write};

        self.terminal.clear()?;

        // Additional cleanup for proper state reset
        let mut stdout = stdout();
        execute!(stdout, crossterm::terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, crossterm::style::ResetColor)?;

        // Ensure alternate screen is exited and cursor is restored
        execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;
        execute!(stdout, crossterm::cursor::Show)?;

        stdout.flush()?;

        Ok(())
    }

    fn render_dialog(f: &mut ratatui::Frame, skips_remaining: usize) {
        use ratatui::widgets::Clear;

        // Calculate dialog size and position
        let area = f.size();
        let dialog_width = 50.min(area.width - 4);
        let dialog_height = 9; // Increased to accommodate all options

        let dialog_area = ratatui::layout::Rect {
            x: (area.width - dialog_width) / 2,
            y: (area.height - dialog_height) / 2,
            width: dialog_width,
            height: dialog_height,
        };

        // Clear the area behind the dialog
        f.render_widget(Clear, dialog_area);

        // Create dialog content
        let dialog_lines = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "Choose an option:",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                if skips_remaining > 0 {
                    Span::styled(
                        "[S] ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::styled(
                        "[S] ",
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::DIM),
                    )
                },
                if skips_remaining > 0 {
                    Span::styled(
                        format!("Skip challenge ({})", skips_remaining),
                        Style::default().fg(Color::White),
                    )
                } else {
                    Span::styled(
                        "No skips remaining",
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::DIM),
                    )
                },
            ]),
            Line::from(vec![
                Span::styled(
                    "[Q] ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled("Quit (fail)", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled(
                    "[ESC] ",
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("Back to game", Style::default().fg(Color::White)),
            ]),
            Line::from(""),
        ];

        let dialog = Paragraph::new(dialog_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Game Options")
                    .title_style(
                        Style::default()
                            .fg(Color::LightBlue)
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(Style::default().fg(Color::Blue)),
            )
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(dialog, dialog_area);
    }
}
