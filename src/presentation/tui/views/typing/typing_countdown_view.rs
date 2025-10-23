use crate::presentation::game::ascii_digits::get_digit_patterns;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

pub struct TypingCountdownView;

impl TypingCountdownView {
    pub fn render(frame: &mut Frame, count: u8) {
        let area = frame.area();
        let center_x = area.width / 2;
        let center_y = area.height / 2;

        let color = match count {
            3 => Colors::countdown_3(),
            2 => Colors::countdown_2(),
            1 => Colors::countdown_1(),
            0 => Colors::countdown_go(),
            _ => Colors::text(),
        };

        if count > 0 && count <= 3 {
            // Use ASCII art for numbers 1, 2, 3
            let digit_patterns = get_digit_patterns();
            let pattern = &digit_patterns[count as usize];

            let ascii_start_y = center_y.saturating_sub(2); // Center the 4-line ASCII art

            let mut lines = Vec::new();
            for line in pattern {
                lines.push(Line::from(vec![Span::styled(
                    *line,
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                )]));
            }

            let max_width = pattern.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
            let countdown_area = Rect {
                x: center_x.saturating_sub(max_width / 2),
                y: ascii_start_y,
                width: max_width,
                height: pattern.len() as u16,
            };

            let countdown_text = Paragraph::new(Text::from(lines)).alignment(Alignment::Center);
            frame.render_widget(countdown_text, countdown_area);
        } else if count == 0 {
            // Use ASCII art for "GO!"
            let go_art = [
                "   ____  ___  ",
                "  / ___|/ _ \\ ",
                " | |  _| | | |",
                " | |_| | |_| |",
                "  \\____|\\___/ ",
            ];

            let ascii_start_y = center_y.saturating_sub(2); // Center the 5-line ASCII art

            let mut lines = Vec::new();
            for line in &go_art {
                lines.push(Line::from(vec![Span::styled(
                    *line,
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                )]));
            }

            let max_width = go_art.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
            let countdown_area = Rect {
                x: center_x.saturating_sub(max_width / 2),
                y: ascii_start_y,
                width: max_width,
                height: go_art.len() as u16,
            };

            let countdown_text = Paragraph::new(Text::from(lines)).alignment(Alignment::Center);
            frame.render_widget(countdown_text, countdown_area);
        }
    }
}
