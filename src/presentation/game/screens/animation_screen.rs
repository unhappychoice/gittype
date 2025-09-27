use crate::domain::services::scoring::Rank;
use crate::presentation::game::views::typing::typing_animation_view::AnimationPhase;
use crate::presentation::game::views::TypingAnimationView;
use crate::presentation::game::{Screen, ScreenTransition, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};
use std::io::Stdout;
use std::time::Duration;

pub struct AnimationScreen {
    animation: Option<TypingAnimationView>,
    session_result: Option<crate::domain::models::SessionResult>,
    animation_initialized: bool,
}

impl Default for AnimationScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationScreen {
    pub fn new() -> Self {
        Self {
            animation: None,
            session_result: None,
            animation_initialized: false,
        }
    }

    /// Pre-inject session result from ScreenManager (avoids RefCell conflicts)
    pub fn inject_session_result(&mut self, session_result: crate::domain::models::SessionResult) {
        self.session_result = Some(session_result);

        if let Some(ref session_result) = self.session_result {
            let best_rank = Rank::for_score(session_result.session_score);
            let tier = Self::get_tier_from_rank_name(best_rank.name());
            let mut typing_animation = TypingAnimationView::new(tier, 80, 24);
            typing_animation.set_rank_messages(best_rank.name());
            self.animation = Some(typing_animation);
            self.animation_initialized = true;
        }
    }

    /// Check if the animation is complete (for ScreenManager auto-transition)
    pub fn is_animation_complete(&self) -> bool {
        if let Some(ref animation) = self.animation {
            animation.is_complete()
        } else {
            false
        }
    }

    fn render_typing_animation_ratatui(
        &self,
        frame: &mut Frame,
        animation: &TypingAnimationView,
        _rank_name: &str,
    ) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Min(4),
                Constraint::Percentage(40),
            ])
            .split(area);

        match animation.get_current_phase() {
            AnimationPhase::ConcentrationLines => {
                let mut lines = Vec::new();

                for (i, line) in animation.get_hacking_lines().iter().enumerate() {
                    let text = &line.text[..line.typed_length];
                    let line_color = line.color;

                    if i == animation.get_current_line()
                        && line.typed_length < line.text.len()
                        && !line.completed
                    {
                        lines.push(Line::from(vec![
                            Span::styled(text, Style::default().fg(line_color)),
                            Span::styled("█", Style::default().fg(Colors::text())),
                        ]));
                    } else if !text.is_empty() {
                        lines.push(Line::from(Span::styled(
                            text,
                            Style::default().fg(line_color),
                        )));
                    } else {
                        lines.push(Line::from(""));
                    }
                }

                let paragraph = Paragraph::new(Text::from(lines)).alignment(Alignment::Center);

                frame.render_widget(paragraph, chunks[1]);

                self.render_skip_hint(frame, area);
            }
            AnimationPhase::Pause => {
                let mut lines = Vec::new();

                for line in animation.get_hacking_lines().iter() {
                    let line_color = line.color;
                    lines.push(Line::from(Span::styled(
                        &line.text,
                        Style::default().fg(line_color),
                    )));
                }

                let dots = ".".repeat(animation.get_pause_dots());
                lines.push(Line::from(Span::styled(
                    dots,
                    Style::default().fg(Colors::text_secondary()),
                )));

                let paragraph = Paragraph::new(Text::from(lines)).alignment(Alignment::Center);

                frame.render_widget(paragraph, chunks[1]);

                self.render_skip_hint(frame, area);
            }
            AnimationPhase::Complete => {
                // Animation is complete, ready to transition to result
            }
        }
    }

    fn render_skip_hint(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let skip_text = "[S] Skip";
        let skip_width = skip_text.len() as u16;
        let skip_height = 1;

        let skip_x = area.width.saturating_sub(skip_width + 1);
        let skip_y = area.height.saturating_sub(skip_height + 1);

        let skip_area = ratatui::layout::Rect {
            x: skip_x,
            y: skip_y,
            width: skip_width,
            height: skip_height,
        };

        let skip_paragraph =
            Paragraph::new(skip_text).style(Style::default().fg(Colors::text_secondary()));

        frame.render_widget(skip_paragraph, skip_area);
    }

    fn get_tier_from_rank_name(rank_name: &str) -> crate::domain::models::RankTier {
        Rank::all_ranks()
            .iter()
            .find(|rank| rank.name() == rank_name)
            .map(|rank| rank.tier().clone())
            .unwrap_or(crate::domain::models::RankTier::Beginner)
    }
}

impl Screen for AnimationScreen {
    fn init(&mut self) -> Result<()> {
        if self.animation.is_none() {
            self.animation_initialized = false;
            self.session_result = None;
        }

        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Result<ScreenTransition> {
        match key_event.code {
            KeyCode::Char('s') | KeyCode::Char('S') => Ok(ScreenTransition::Replace(
                ScreenType::SessionSummary,
            )),
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut Stdout,
        _session_result: Option<&crate::domain::models::SessionResult>,
        _total_result: Option<&crate::domain::services::scoring::TotalResult>,
    ) -> Result<()> {
        Ok(())
    }

    fn render_ratatui(&mut self, frame: &mut ratatui::Frame) -> Result<()> {
        if let Some(ref animation) = self.animation {
            if let Some(ref session_result) = self.session_result {
                let best_rank = Rank::for_score(session_result.session_score);
                let rank_name = best_rank.name();

                self.render_typing_animation_ratatui(frame, animation, rank_name);
            }
        }
        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::TimeBased(Duration::from_millis(16)) // ~60 FPS for smooth animation
    }

    fn update(&mut self) -> Result<bool> {
        if let Some(ref mut animation) = self.animation {
            let updated = animation.update();

            if animation.is_complete() {
                return Ok(true);
            }

            return Ok(updated);
        }
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
