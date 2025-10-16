use crate::domain::events::EventBus;
use crate::domain::models::TotalResult;
use crate::domain::services::scoring::{TotalCalculator, TotalTracker, GLOBAL_TOTAL_TRACKER};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::tui::ScreenDataProvider;
use crate::presentation::tui::views::{AsciiScoreView, SharingView, StatisticsView};
use crate::presentation::tui::{Screen, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::{GitTypeError, Result};
use crossterm::event::{self, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use std::sync::{Arc, Mutex};

pub struct TotalSummaryScreenData {
    pub total_result: TotalResult,
}

pub struct TotalSummaryScreenDataProvider {
    total_tracker: Arc<Mutex<Option<TotalTracker>>>,
}

impl ScreenDataProvider for TotalSummaryScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let total_result = self
            .total_tracker
            .lock()
            .map_err(|e| {
                GitTypeError::TerminalError(format!("Failed to lock total tracker: {}", e))
            })?
            .as_ref()
            .map(|tracker| {
                let mut result = TotalCalculator::calculate(tracker);
                result.finalize();
                result
            })
            .ok_or_else(|| GitTypeError::TerminalError("No total tracker available".to_string()))?;

        Ok(Box::new(TotalSummaryScreenData { total_result }))
    }
}

#[derive(Debug)]
pub enum ExitAction {
    Exit,
    Share,
}

pub struct TotalSummaryScreen {
    displayed: bool,
    total_result: Option<TotalResult>,
    event_bus: EventBus,
}

impl TotalSummaryScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            displayed: false,
            total_result: None,
            event_bus,
        }
    }
}

impl Screen for TotalSummaryScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::TotalSummary
    }

    fn default_provider() -> Box<dyn crate::presentation::tui::ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(TotalSummaryScreenDataProvider {
            total_tracker: GLOBAL_TOTAL_TRACKER.clone(),
        })
    }

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        let screen_data = data.downcast::<TotalSummaryScreenData>()?;
        self.total_result = Some(screen_data.total_result);
        self.displayed = false; // Reset displayed flag to allow re-rendering
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.event_bus
                    .publish(NavigateTo::Push(ScreenType::TotalSummaryShare));
                Ok(())
            }
            KeyCode::Esc => {
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
        if let Some(ref total_result) = self.total_result {
            let area = frame.area();

            // Calculate content heights
            let title_height = 1;
            let score_height = 4; // ASCII digits height
            let stats_height = 4; // 4 lines of statistics
            let options_height = 5; // Thanks, GitHub, spacing, Share, Exit
            let spacing = 2; // Spacing between sections

            let total_content_height = title_height
                + spacing
                + score_height
                + spacing
                + stats_height
                + spacing
                + options_height;

            let top_spacing = (area.height.saturating_sub(total_content_height as u16)) / 2;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(top_spacing),
                    Constraint::Length(1), // Title
                    Constraint::Length(2), // Spacing
                    Constraint::Length(4), // Score
                    Constraint::Length(2), // Spacing
                    Constraint::Length(4), // Statistics
                    Constraint::Length(2), // Spacing
                    Constraint::Length(5), // Options
                    Constraint::Min(0),
                ])
                .split(area);

            // Title
            let title = Paragraph::new(Line::from(vec![Span::styled(
                "=== TOTAL SUMMARY ===",
                Style::default()
                    .fg(Colors::info())
                    .add_modifier(Modifier::BOLD),
            )]))
            .alignment(Alignment::Center);
            frame.render_widget(title, chunks[1]);

            // Score
            AsciiScoreView::render(frame, chunks[3], total_result.total_score);

            // Statistics
            StatisticsView::render(frame, chunks[5], total_result);

            // Options
            SharingView::render_exit_options(frame, chunks[7]);
        }
        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
