use crate::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, collections::HashMap};
use super::{text_processor::TextProcessor, challenge::Challenge};

pub struct GameDisplayRatatui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    chars: Vec<char>,
    last_position: usize,
}

impl GameDisplayRatatui {
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

    pub fn display_challenge_with_info(
        &mut self,
        challenge_text: &str,
        current_position: usize,
        mistakes: usize,
        start_time: &std::time::Instant,
        line_starts: &[usize],
        comment_ranges: &[(usize, usize)],
        challenge: Option<&Challenge>,
        current_mistake_position: Option<usize>,
    ) -> Result<()> {
        // Update character cache if needed
        if self.chars.len() != challenge_text.chars().count() {
            self.chars = challenge_text.chars().collect();
        }

        // Pre-calculate all data before drawing
        let progress = if !self.chars.is_empty() {
            (current_position as f32 / self.chars.len() as f32 * 100.0) as u32
        } else {
            0
        };
        
        let header_text = if let Some(challenge) = challenge {
            let difficulty_text = match &challenge.difficulty_level {
                Some(difficulty) => format!("{:?}", difficulty),
                None => "Unknown".to_string(),
            };
            format!("[{}] [{}] - Progress: {}%", challenge.get_display_title(), difficulty_text, progress)
        } else {
            format!("[Challenge] - Progress: {}%", progress)
        };

        let terminal_size = self.terminal.size()?;
        let content_spans = self.create_content_spans(
            current_position,
            line_starts,
            comment_ranges,
            current_mistake_position,
            terminal_size.width,
        );

        let metrics = crate::scoring::engine::ScoringEngine::calculate_real_time_metrics(current_position, mistakes, start_time);
        let current_line = self.find_line_for_position(current_position, line_starts);
        let elapsed_secs = start_time.elapsed().as_secs();
        let total_chars = self.chars.len();
        let progress_percent = if total_chars > 0 {
            (current_position as f32 / total_chars as f32 * 100.0) as u8
        } else {
            0
        };
        
        let metrics_text = format!(
            "CPM: {:.0} | WPM: {:.0} | Accuracy: {:.0}% | Mistakes: {} | Progress: {}/{}({:.0}%) | Time: {}s | Title: {} | [ESC to quit]",
            metrics.cpm, metrics.wpm, metrics.accuracy, metrics.mistakes, 
            current_position, total_chars, progress_percent, elapsed_secs,
            metrics.ranking_title
        );

        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Min(1),    // Content
                    Constraint::Length(1), // Metrics
                ].as_ref())
                .split(f.size());

            // Header
            let header = Paragraph::new(vec![
                Line::from(header_text.clone()),
                Line::from("Press ESC to quit"),
            ])
            .block(Block::default().borders(Borders::BOTTOM));
            f.render_widget(header, chunks[0]);

            // Content with syntax highlighting and cursor
            let scroll_offset = if current_line > chunks[1].height as usize / 2 {
                (current_line - chunks[1].height as usize / 2) as u16
            } else {
                0
            };
            
            let content = Paragraph::new(Text::from(content_spans.clone()))
                .scroll((scroll_offset, 0));
            f.render_widget(content, chunks[1]);

            // Metrics
            let metrics_widget = Paragraph::new(metrics_text.clone())
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(metrics_widget, chunks[2]);
        })?;

        self.last_position = current_position;
        Ok(())
    }

    fn create_content_spans(
        &self,
        current_position: usize,
        line_starts: &[usize],
        comment_ranges: &[(usize, usize)],
        current_mistake_position: Option<usize>,
        terminal_width: u16,
    ) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let mut current_line_spans = Vec::new();
        let mut current_line_width = 0u16;
        let max_width = terminal_width.saturating_sub(1);
        
        // Pre-calculate all character properties to avoid O(n²) complexity
        let skip_cache = self.create_skip_cache(line_starts, comment_ranges);
        let comment_cache = self.create_comment_cache(comment_ranges);
        
        for (i, &ch) in self.chars.iter().enumerate() {
            // Handle explicit newlines
            if ch == '\n' {
                lines.push(Line::from(current_line_spans));
                current_line_spans = Vec::new();
                current_line_width = 0;
                continue;
            }

            // Use cached properties - O(1) lookup instead of O(n) calculation
            let should_skip = skip_cache.get(&i).copied().unwrap_or(false);
            let is_comment = comment_cache.get(&i).copied().unwrap_or(false);
            
            let style = self.get_char_style(i, current_position, should_skip, is_comment, current_mistake_position);

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

        if lines.is_empty() {
            lines.push(Line::from(""));
        }

        lines
    }

    fn create_skip_cache(&self, line_starts: &[usize], comment_ranges: &[(usize, usize)]) -> HashMap<usize, bool> {
        let mut cache = HashMap::new();
        let text_str = self.chars.iter().collect::<String>();
        
        // Calculate line starts lookup for O(1) access
        let mut line_lookup = HashMap::new();
        for (line_num, &start) in line_starts.iter().enumerate() {
            line_lookup.insert(start, line_num);
        }
        
        // Pre-calculate first non-whitespace for each line
        let mut first_non_ws_cache = HashMap::new();
        for &line_start in line_starts {
            let first_non_ws = TextProcessor::find_first_non_whitespace(&text_str, line_start);
            first_non_ws_cache.insert(line_start, first_non_ws);
        }
        
        for i in 0..self.chars.len() {
            cache.insert(i, self.calculate_should_skip(i, line_starts, comment_ranges, &first_non_ws_cache));
        }
        
        cache
    }

    fn create_comment_cache(&self, comment_ranges: &[(usize, usize)]) -> HashMap<usize, bool> {
        let mut cache = HashMap::new();
        
        for i in 0..self.chars.len() {
            cache.insert(i, Self::is_position_in_comment(i, comment_ranges));
        }
        
        cache
    }

    fn calculate_should_skip(
        &self,
        position: usize,
        line_starts: &[usize],
        comment_ranges: &[(usize, usize)],
        first_non_ws_cache: &HashMap<usize, usize>,
    ) -> bool {
        if position >= self.chars.len() {
            return false;
        }
        
        let ch = self.chars[position];
        
        // Handle newlines separately
        if ch == '\n' {
            return false; // Simplified for now
        }
        
        // Check if this position is before the first non-whitespace character of a line
        // Find which line this position belongs to - O(log n) instead of O(n)
        if let Some(line_start) = line_starts.iter()
            .rev()
            .find(|&&start| position >= start) {
            if let Some(&first_non_ws) = first_non_ws_cache.get(line_start) {
                if position < first_non_ws {
                    return true;
                }
            }
        }
        
        // Check if this position is within a comment
        Self::is_position_in_comment(position, comment_ranges)
    }

    fn get_char_style(
        &self,
        i: usize,
        current_position: usize,
        should_skip: bool,
        is_comment: bool,
        current_mistake_position: Option<usize>,
    ) -> Style {
        if i < current_position || should_skip {
            if is_comment {
                Style::default().fg(Color::Green).add_modifier(Modifier::DIM)
            } else if should_skip {
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM)
            } else {
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            }
        } else if i == current_position {
            if let Some(mistake_pos) = current_mistake_position {
                if i == mistake_pos {
                    Style::default().fg(Color::White).bg(Color::Red)
                } else {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                }
            } else {
                Style::default().fg(Color::Black).bg(Color::Yellow)
            }
        } else {
            if is_comment {
                Style::default().fg(Color::Green).add_modifier(Modifier::DIM)
            } else {
                Style::default().fg(Color::White).add_modifier(Modifier::DIM)
            }
        }
    }


    fn find_line_for_position(&self, position: usize, line_starts: &[usize]) -> usize {
        for (line_num, &line_start) in line_starts.iter().enumerate() {
            if position < line_start {
                return line_num.saturating_sub(1);
            }
        }
        line_starts.len().saturating_sub(1)
    }


    pub fn cleanup(&mut self) -> Result<()> {
        self.terminal.clear()?;
        Ok(())
    }

    fn is_position_in_comment(position: usize, comment_ranges: &[(usize, usize)]) -> bool {
        comment_ranges.iter().any(|&(start, end)| position >= start && position < end)
    }
}