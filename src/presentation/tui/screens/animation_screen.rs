use crate::domain::events::EventBusInterface;
use crate::domain::models::{RankTier, SessionResult};
use crate::domain::services::scoring::Rank;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::SessionManager;
use crate::presentation::tui::views::typing::typing_animation_view::AnimationPhase;
use crate::presentation::tui::views::TypingAnimationView;
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::{GitTypeError, Result};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};
use std::sync::RwLock;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AnimationData {
    pub session_result: SessionResult,
}

pub struct AnimationDataProvider {
    session_manager: Arc<Mutex<SessionManager>>,
}

impl ScreenDataProvider for AnimationDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_result = self
            .session_manager
            .lock()
            .map_err(|_| GitTypeError::TerminalError("Failed to lock SessionManager".to_string()))?
            .get_session_result()
            .ok_or_else(|| {
                GitTypeError::TerminalError("No session result available".to_string())
            })?;

        Ok(Box::new(AnimationData { session_result }))
    }
}

pub trait AnimationScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = AnimationScreenInterface)]
pub struct AnimationScreen {
    animation: RwLock<Option<TypingAnimationView>>,

    session_result: RwLock<Option<SessionResult>>,

    animation_initialized: RwLock<bool>,

    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
}

impl AnimationScreen {
    pub fn new(event_bus: Arc<dyn EventBusInterface>) -> Self {
        Self {
            animation: RwLock::new(None),
            session_result: RwLock::new(None),
            animation_initialized: RwLock::new(false),
            event_bus,
        }
    }

    /// Pre-inject session result from ScreenManager (avoids RefCell conflicts)
    pub fn set_session_result(&self, session_result: SessionResult) {
        *self.session_result.write().unwrap() = Some(session_result.clone());

        if let Some(ref session_result) = *self.session_result.read().unwrap() {
            let best_rank = Rank::for_score(session_result.session_score);
            let tier = Self::get_tier_from_rank_name(best_rank.name());
            let mut typing_animation = TypingAnimationView::new(tier, 80, 24);
            typing_animation.set_rank_messages(best_rank.name());
            *self.animation.write().unwrap() = Some(typing_animation);
            *self.animation_initialized.write().unwrap() = true;
        }
    }

    /// Check if the animation is complete (for ScreenManager auto-transition)
    pub fn is_animation_complete(&self) -> bool {
        if let Some(ref animation) = *self.animation.read().unwrap() {
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

    fn get_tier_from_rank_name(rank_name: &str) -> RankTier {
        Rank::all_ranks()
            .iter()
            .find(|rank| rank.name() == rank_name)
            .map(|rank| rank.tier().clone())
            .unwrap_or(RankTier::Beginner)
    }
}

impl Screen for AnimationScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::Animation
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(AnimationDataProvider {
            session_manager: SessionManager::instance(),
        })
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        // Initialize state
        if self.animation.read().unwrap().is_none() {
            *self.animation_initialized.write().unwrap() = false;
            *self.session_result.write().unwrap() = None;
        }

        let data = data.downcast::<AnimationData>()?;

        self.set_session_result(data.session_result);
        Ok(())
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::SessionSummary));
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut ratatui::Frame) -> Result<()> {
        let animation_guard = self.animation.read().unwrap();
        if let Some(ref animation) = *animation_guard {
            let session_result_guard = self.session_result.read().unwrap();
            if let Some(ref session_result) = *session_result_guard {
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

    fn update(&self) -> Result<bool> {
        let mut animation_guard = self.animation.write().unwrap();
        if let Some(ref mut animation) = *animation_guard {
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
}

impl AnimationScreenInterface for AnimationScreen {}
