use crate::presentation::game::ascii_digits::get_digit_patterns;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct AsciiScoreView;

impl AsciiScoreView {
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, score: f64) {
        let score_value = format!("{:.0}", score);
        let ascii_numbers = Self::create_ascii_numbers(&score_value);

        let mut constraints = vec![];
        for _ in &ascii_numbers {
            constraints.push(Constraint::Length(1));
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        for (i, line) in ascii_numbers.iter().enumerate() {
            let widget = Paragraph::new(Line::from(vec![Span::styled(
                line.as_str(),
                Style::default()
                    .fg(Colors::score())
                    .add_modifier(Modifier::BOLD),
            )]))
            .alignment(Alignment::Center);
            frame.render_widget(widget, chunks[i]);
        }
    }

    fn create_ascii_numbers(score: &str) -> Vec<String> {
        let digit_patterns = get_digit_patterns();
        let max_height = 4;
        let mut result = vec![String::new(); max_height];

        for ch in score.chars() {
            if let Some(digit) = ch.to_digit(10) {
                let pattern = &digit_patterns[digit as usize];
                for (i, line) in pattern.iter().enumerate() {
                    result[i].push_str(line);
                    result[i].push(' ');
                }
            }
        }

        result
    }
}
