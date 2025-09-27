use crate::{
    game::{context_loader::CodeContext, typing_core::TypingCore},
    domain::models::Challenge,
    presentation::ui::Colors,
};
use ratatui::{
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct TypingContentView {
    // Individual caches with different update frequencies
    pre_context_cache: Option<(u64, Vec<Line<'static>>)>,
    post_context_cache: Option<(u64, Vec<Line<'static>>)>,
    main_content_cache: Option<(u64, Vec<Line<'static>>)>,
}

impl Default for TypingContentView {
    fn default() -> Self {
        Self::new()
    }
}
impl TypingContentView {
    pub fn new() -> Self {
        Self {
            pre_context_cache: None,
            post_context_cache: None,
            main_content_cache: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        show_code: bool,
        challenge: Option<&Challenge>,
        typing_core: &TypingCore,
        chars: &[char],
        code_context: &CodeContext,
    ) {
        if show_code {
            let view_height = area.height.saturating_sub(2);
            let content_spans = self.create_content_spans(
                area.width,
                challenge,
                typing_core,
                chars,
                code_context,
                view_height,
            );
            let total_lines = content_spans.len() as u16;

            let current_display_line_index =
                self.find_current_display_line_index(&content_spans, typing_core);
            let effective_line_index = if current_display_line_index == 0 {
                let pre_context_lines = code_context.pre_context.len() as u16;
                let current_line = typing_core.current_line_to_display() as u16;
                pre_context_lines + current_line
            } else {
                current_display_line_index
            };

            let scroll_offset =
                Self::calculate_scroll_offset(view_height, total_lines, effective_line_index);

            let content = Paragraph::new(Text::from(content_spans))
                .scroll((scroll_offset, 0))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Colors::border()))
                        .title("Code")
                        .title_style(Style::default().fg(Colors::key_action()))
                        .padding(ratatui::widgets::Padding::uniform(1)),
                );
            frame.render_widget(content, area);
        } else {
            let empty_content = Paragraph::new(Text::from(vec![])).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Code")
                    .title_style(Style::default().fg(Colors::key_action()))
                    .padding(ratatui::widgets::Padding::uniform(1)),
            );
            frame.render_widget(empty_content, area);
        }
    }

    fn create_content_spans(
        &mut self,
        terminal_width: u16,
        challenge: Option<&Challenge>,
        typing_core: &TypingCore,
        chars: &[char],
        code_context: &CodeContext,
        view_height: u16,
    ) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let start_line_number = challenge.and_then(|c| c.start_line).unwrap_or(1);

        let pre_context_lines = self.get_cached_pre_context_lines(code_context, start_line_number);
        lines.extend(pre_context_lines);

        let main_content_lines = self.get_cached_main_content_lines(
            terminal_width,
            challenge,
            typing_core,
            chars,
            start_line_number,
            view_height,
        );
        lines.extend(main_content_lines);

        let post_context_lines =
            self.get_cached_post_context_lines(code_context, challenge, start_line_number);
        lines.extend(post_context_lines);

        if lines.is_empty() {
            let line_num_span = self.create_line_number_span(start_line_number, false);
            lines.push(Line::from(vec![line_num_span]));
        }

        let padding_lines = view_height as usize / 2;
        for _ in 0..padding_lines {
            lines.push(Line::from(vec![Span::raw("")]));
        }

        lines
    }

    fn add_pre_context_lines(
        &self,
        lines: &mut Vec<Line<'static>>,
        code_context: &CodeContext,
        start_line_number: usize,
    ) {
        for (ctx_idx, pre_line) in code_context.pre_context.iter().enumerate() {
            let ctx_line_number =
                start_line_number.saturating_sub(code_context.pre_context.len() - ctx_idx);
            let line_num_span = Span::styled(
                format!("{:>4} │ ", ctx_line_number),
                Style::default().fg(Colors::text_secondary()),
            );
            let content_span = Span::styled(
                pre_line.clone(),
                Style::default().fg(Colors::text_secondary()),
            );
            lines.push(Line::from(vec![line_num_span, content_span]));
        }
    }

    fn add_post_context_lines(
        &self,
        lines: &mut Vec<Line<'static>>,
        code_context: &CodeContext,
        challenge: Option<&Challenge>,
        start_line_number: usize,
    ) {
        let end_line_number = challenge
            .and_then(|c| c.end_line)
            .unwrap_or(start_line_number);

        for (ctx_idx, post_line) in code_context.post_context.iter().enumerate() {
            let ctx_line_number = end_line_number + ctx_idx + 1;
            let line_num_span = Span::styled(
                format!("{:>4} │ ", ctx_line_number),
                Style::default().fg(Colors::text_secondary()),
            );
            let content_span = Span::styled(
                post_line.clone(),
                Style::default().fg(Colors::text_secondary()),
            );
            lines.push(Line::from(vec![line_num_span, content_span]));
        }
    }

    fn get_cached_pre_context_lines(
        &mut self,
        code_context: &CodeContext,
        start_line_number: usize,
    ) -> Vec<Line<'static>> {
        let cache_key = self.calculate_pre_context_cache_key(code_context, start_line_number);

        // Check if we can use cached result
        if let Some((cached_key, ref cached_lines)) = self.pre_context_cache {
            if cached_key == cache_key {
                return cached_lines.clone();
            }
        }

        // Generate new lines
        let mut lines = Vec::new();
        self.add_pre_context_lines(&mut lines, code_context, start_line_number);

        // Cache the result
        self.pre_context_cache = Some((cache_key, lines.clone()));
        lines
    }

    fn get_cached_post_context_lines(
        &mut self,
        code_context: &CodeContext,
        challenge: Option<&Challenge>,
        start_line_number: usize,
    ) -> Vec<Line<'static>> {
        let cache_key =
            self.calculate_post_context_cache_key(code_context, challenge, start_line_number);

        // Check if we can use cached result
        if let Some((cached_key, ref cached_lines)) = self.post_context_cache {
            if cached_key == cache_key {
                return cached_lines.clone();
            }
        }

        // Generate new lines
        let mut lines = Vec::new();
        self.add_post_context_lines(&mut lines, code_context, challenge, start_line_number);

        // Cache the result
        self.post_context_cache = Some((cache_key, lines.clone()));
        lines
    }

    fn get_cached_main_content_lines(
        &mut self,
        terminal_width: u16,
        challenge: Option<&Challenge>,
        typing_core: &TypingCore,
        chars: &[char],
        start_line_number: usize,
        view_height: u16,
    ) -> Vec<Line<'static>> {
        let cache_key = self.calculate_main_content_cache_key(
            terminal_width,
            challenge,
            typing_core,
            chars,
            start_line_number,
            view_height,
        );

        // Check if we can use cached result
        if let Some((cached_key, ref cached_lines)) = self.main_content_cache {
            if cached_key == cache_key {
                return cached_lines.clone();
            }
        }

        // Generate new lines
        let mut lines = Vec::new();
        self.process_main_content(
            &mut lines,
            terminal_width,
            typing_core,
            chars,
            start_line_number,
        );

        // Cache the result
        self.main_content_cache = Some((cache_key, lines.clone()));
        lines
    }

    fn process_main_content(
        &self,
        lines: &mut Vec<Line<'static>>,
        terminal_width: u16,
        typing_core: &TypingCore,
        chars: &[char],
        start_line_number: usize,
    ) {
        let line_number_width = 6u16;
        let max_width = terminal_width.saturating_sub(line_number_width + 1);

        let mut current_line_spans = Vec::new();
        let mut current_line_width = 0u16;
        let mut line_number = 0;
        let mut line_start = true;
        let mut byte_position = 0;

        let current_display_position = typing_core.current_position_to_display();
        let current_mistake_position = typing_core.current_mistake_position();
        let current_line_number = typing_core.current_line_to_display();
        let display_comment_ranges = typing_core.display_comment_ranges();

        for (i, &ch) in chars.iter().enumerate() {
            // Add line number at the start of each line
            if line_start {
                let line_num_span = self.create_line_number_span(
                    start_line_number + line_number,
                    line_number == current_line_number,
                );
                current_line_spans.push(line_num_span);
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
                byte_position += ch.len_utf8();
                continue;
            }

            let is_in_comment = self.is_in_comment_range(byte_position, &display_comment_ranges);
            let style = self.determine_character_style(
                i,
                is_in_comment,
                current_display_position,
                current_mistake_position,
            );

            let (display_char, char_width) = self.format_character(ch);

            // Check if we need to wrap
            if current_line_width + char_width > max_width {
                lines.push(Line::from(current_line_spans));
                current_line_spans = Vec::new();
                current_line_width = 0;
            }

            current_line_spans.push(Span::styled(display_char, style));
            current_line_width += char_width;
            byte_position += ch.len_utf8();
        }

        if !current_line_spans.is_empty() {
            lines.push(Line::from(current_line_spans));
        }
    }

    fn create_line_number_span(&self, line_number: usize, is_current: bool) -> Span<'static> {
        let line_num_text = format!("{:>4} │ ", line_number);
        let style = if is_current {
            Style::default()
                .fg(Colors::warning())
                .add_modifier(ratatui::style::Modifier::BOLD)
        } else {
            Style::default().fg(Colors::text_secondary())
        };
        Span::styled(line_num_text, style)
    }

    fn is_in_comment_range(&self, byte_position: usize, comment_ranges: &[(usize, usize)]) -> bool {
        comment_ranges
            .iter()
            .any(|(start, end)| byte_position >= *start && byte_position < *end)
    }

    fn determine_character_style(
        &self,
        char_index: usize,
        is_in_comment: bool,
        current_display_position: usize,
        current_mistake_position: Option<usize>,
    ) -> Style {
        if is_in_comment {
            Style::default().fg(Colors::text_secondary())
        } else if char_index < current_display_position {
            Style::default().fg(Colors::typed_text())
        } else if char_index == current_display_position {
            if let Some(mistake_pos) = current_mistake_position {
                if char_index == mistake_pos {
                    Style::default()
                        .fg(Colors::current_cursor())
                        .bg(Colors::mistake_bg())
                } else {
                    Style::default()
                        .fg(Colors::current_cursor())
                        .bg(Colors::cursor_bg())
                }
            } else {
                Style::default()
                    .fg(Colors::current_cursor())
                    .bg(Colors::cursor_bg())
            }
        } else {
            Style::default().fg(Colors::untyped_text())
        }
    }

    fn format_character(&self, ch: char) -> (String, u16) {
        match ch {
            '\t' => ("    ".to_string(), 4),
            c if c.is_control() => ("?".to_string(), 1),
            c => (c.to_string(), 1),
        }
    }

    fn calculate_pre_context_cache_key(
        &self,
        code_context: &CodeContext,
        start_line_number: usize,
    ) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        // Pre-context rarely changes
        start_line_number.hash(&mut hasher);
        code_context.pre_context.hash(&mut hasher);

        hasher.finish()
    }

    fn calculate_post_context_cache_key(
        &self,
        code_context: &CodeContext,
        challenge: Option<&Challenge>,
        start_line_number: usize,
    ) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        // Post-context rarely changes
        if let Some(challenge) = challenge {
            challenge.end_line.hash(&mut hasher);
        } else {
            start_line_number.hash(&mut hasher);
        }
        code_context.post_context.hash(&mut hasher);

        hasher.finish()
    }

    fn calculate_scroll_offset(view_height: u16, total_lines: u16, current_line_index: u16) -> u16 {
        let desired_center = view_height / 2;

        if current_line_index > desired_center {
            let centered_offset = current_line_index.saturating_sub(desired_center);

            if total_lines > view_height {
                let max_scroll = total_lines.saturating_sub(view_height);
                centered_offset.min(max_scroll)
            } else {
                centered_offset
            }
        } else {
            0
        }
    }

    fn find_current_display_line_index(
        &self,
        content_spans: &[Line<'static>],
        _typing_core: &TypingCore,
    ) -> u16 {
        for (display_line_index, line) in content_spans.iter().enumerate() {
            for span in &line.spans {
                if span
                    .style
                    .add_modifier
                    .intersects(ratatui::style::Modifier::BOLD)
                {
                    return display_line_index as u16;
                }
            }
        }
        0
    }

    fn calculate_main_content_cache_key(
        &self,
        terminal_width: u16,
        challenge: Option<&Challenge>,
        typing_core: &TypingCore,
        chars: &[char],
        start_line_number: usize,
        view_height: u16,
    ) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        terminal_width.hash(&mut hasher);
        start_line_number.hash(&mut hasher);
        view_height.hash(&mut hasher);

        if let Some(challenge) = challenge {
            challenge.start_line.hash(&mut hasher);
            challenge.end_line.hash(&mut hasher);
        }

        typing_core.current_position_to_display().hash(&mut hasher);
        typing_core.current_mistake_position().hash(&mut hasher);
        typing_core.current_line_to_display().hash(&mut hasher);

        for (start, end) in typing_core.display_comment_ranges() {
            start.hash(&mut hasher);
            end.hash(&mut hasher);
        }

        chars.len().hash(&mut hasher);
        for &ch in chars {
            ch.hash(&mut hasher);
        }

        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::TypingContentView;

    #[test]
    fn test_calculate_scroll_offset() {
        assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 10), 0);
        assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 15), 5);
        assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 30), 20);
        assert_eq!(TypingContentView::calculate_scroll_offset(20, 25, 50), 5);
        assert_eq!(TypingContentView::calculate_scroll_offset(20, 30, 15), 5);
        assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 5), 0);
    }
}
