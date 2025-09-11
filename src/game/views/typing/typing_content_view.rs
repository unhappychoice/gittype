use crate::{
    game::{context_loader::CodeContext, typing_core::TypingCore},
    models::Challenge,
    ui::Colors,
};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct TypingContentView;

impl TypingContentView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        show_code: bool,
        challenge: Option<&Challenge>,
        typing_core: &TypingCore,
        chars: &[char],
        code_context: &CodeContext,
    ) {
        if show_code {
            let content_spans =
                Self::create_content_spans(area.width, challenge, typing_core, chars, code_context);

            // Keep the current line roughly centered within the viewport
            let view_height = area.height.saturating_sub(2); // account for borders/padding
            let total_lines = content_spans.len() as u16;
            let pre_context_lines = code_context.pre_context.len() as u16;
            let current_line = typing_core.current_line_to_display();
            let absolute_line_index = pre_context_lines.saturating_add(current_line as u16);

            let desired_center = view_height / 2;
            let mut scroll_offset = if total_lines > view_height {
                absolute_line_index.saturating_sub(desired_center)
            } else {
                0
            };
            // Clamp so we don't scroll past the end
            let max_scroll = total_lines.saturating_sub(view_height);
            if scroll_offset > max_scroll {
                scroll_offset = max_scroll;
            }

            let content = Paragraph::new(Text::from(content_spans))
                .scroll((scroll_offset, 0))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Colors::BORDER))
                        .title("Code")
                        .title_style(Style::default().fg(Colors::ACTION_KEY))
                        .padding(ratatui::widgets::Padding::uniform(1)),
                );
            frame.render_widget(content, area);
        } else {
            // Hide code during waiting and countdown, show empty block
            let empty_content = Paragraph::new(Text::from(vec![])).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Code")
                    .title_style(Style::default().fg(Colors::ACTION_KEY))
                    .padding(ratatui::widgets::Padding::uniform(1)),
            );
            frame.render_widget(empty_content, area);
        }
    }

    fn create_content_spans(
        terminal_width: u16,
        challenge: Option<&Challenge>,
        typing_core: &TypingCore,
        chars: &[char],
        code_context: &CodeContext,
    ) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let mut current_line_spans = Vec::new();
        let mut current_line_width = 0u16;
        let mut line_number = 0;
        let mut line_start = true;

        // Reserve space for line numbers
        let line_number_width = 6u16;
        let max_width = terminal_width.saturating_sub(line_number_width + 1);

        // Get the starting line number from challenge
        let start_line_number = challenge.and_then(|c| c.start_line).unwrap_or(1);

        // Add pre-context lines (read-only, dimmed)
        for (ctx_idx, pre_line) in code_context.pre_context.iter().enumerate() {
            let ctx_line_number =
                start_line_number.saturating_sub(code_context.pre_context.len() - ctx_idx);
            let line_num_text = format!("{:>4} │ ", ctx_line_number);
            let mut line_spans = vec![Span::styled(
                line_num_text,
                Style::default().fg(Colors::COMMENT_TEXT),
            )];

            // Add the context line content with dimmed style
            line_spans.push(Span::styled(
                pre_line.clone(),
                Style::default()
                    .fg(Colors::COMMENT_TEXT)
                    .add_modifier(Modifier::DIM),
            ));

            lines.push(Line::from(line_spans));
        }

        let mut byte_position = 0; // Track byte position as we iterate
        let current_display_position = typing_core.current_position_to_display();
        let current_mistake_position = typing_core.current_mistake_position();
        let current_line_number = typing_core.current_line_to_display();
        let display_comment_ranges = typing_core.display_comment_ranges();

        for (i, &ch) in chars.iter().enumerate() {
            // Add line number at the start of each line
            if line_start {
                let actual_line_number = start_line_number + line_number;
                let line_num_text = format!("{:>4} │ ", actual_line_number);
                let line_num_style = if line_number == current_line_number {
                    Style::default()
                        .fg(Colors::WARNING)
                        .add_modifier(ratatui::style::Modifier::BOLD)
                } else {
                    Style::default().fg(Colors::SECONDARY)
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
                byte_position += ch.len_utf8(); // Update byte position
                continue;
            }

            // Check if this character is in a comment using byte position
            let is_in_comment = display_comment_ranges
                .iter()
                .any(|&(start, end)| byte_position >= start && byte_position < end);

            // Determine character style
            let style = if is_in_comment {
                // Comments use same color as context lines
                Style::default()
                    .fg(Colors::COMMENT_TEXT)
                    .add_modifier(Modifier::DIM)
            } else if i < current_display_position {
                // Already typed - light blue dimmed for non-comments
                Style::default()
                    .fg(Colors::TYPED_TEXT)
                    .add_modifier(Modifier::DIM)
            } else if i == current_display_position {
                // Current cursor position - highlighted
                if let Some(mistake_pos) = current_mistake_position {
                    if i == mistake_pos {
                        Style::default()
                            .fg(Colors::CURRENT_CURSOR)
                            .bg(Colors::MISTAKE_BG)
                    } else {
                        Style::default()
                            .fg(Colors::CURRENT_CURSOR)
                            .bg(Colors::CURSOR_BG)
                    }
                } else {
                    Style::default().fg(Colors::TEXT).bg(Colors::MUTED)
                }
            } else {
                // Not yet typed - dim white for non-comments
                Style::default()
                    .fg(Colors::UNTYPED_TEXT)
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

            // Update byte position for next iteration
            byte_position += ch.len_utf8();
        }

        if !current_line_spans.is_empty() {
            lines.push(Line::from(current_line_spans));
        }

        // Add post-context lines (read-only, dimmed)
        let end_line_number = challenge
            .and_then(|c| c.end_line)
            .unwrap_or(start_line_number);
        for (ctx_idx, post_line) in code_context.post_context.iter().enumerate() {
            let ctx_line_number = end_line_number + ctx_idx + 1;
            let line_num_text = format!("{:>4} │ ", ctx_line_number);
            let mut line_spans = vec![Span::styled(
                line_num_text,
                Style::default().fg(Colors::COMMENT_TEXT),
            )];

            // Add the context line content with dimmed style
            line_spans.push(Span::styled(
                post_line.clone(),
                Style::default()
                    .fg(Colors::COMMENT_TEXT)
                    .add_modifier(Modifier::DIM),
            ));

            lines.push(Line::from(line_spans));
        }

        if lines.is_empty() {
            let line_num_text = format!("{:>4} │ ", start_line_number);
            let line_num_style = Style::default().fg(Colors::SECONDARY);
            lines.push(Line::from(vec![Span::styled(
                line_num_text,
                line_num_style,
            )]));
        }

        lines
    }
}
