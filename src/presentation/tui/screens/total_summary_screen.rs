use crate::domain::events::EventBusInterface;
use crate::domain::models::TotalResult;
use crate::domain::services::scoring::{TotalCalculator, TotalTracker, GLOBAL_TOTAL_TRACKER};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::tui::views::{AsciiScoreView, SharingView, StatisticsView};
use crate::presentation::tui::ScreenDataProvider;
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
use std::sync::RwLock;
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

pub trait TotalSummaryScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = TotalSummaryScreenInterface)]
pub struct TotalSummaryScreen {
    #[shaku(default)]
    displayed: RwLock<bool>,
    #[shaku(default)]
    total_result: RwLock<Option<TotalResult>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
}

impl TotalSummaryScreen {
    pub fn new(event_bus: Arc<dyn EventBusInterface>) -> Self {
        Self {
            displayed: RwLock::new(false),
            total_result: RwLock::new(None),
            event_bus,
        }
    }
}

pub struct TotalSummaryScreenProvider;

impl shaku::Provider<crate::presentation::di::AppModule> for TotalSummaryScreenProvider {
    type Interface = TotalSummaryScreen;

    fn provide(
        module: &crate::presentation::di::AppModule,
    ) -> std::result::Result<Box<Self::Interface>, Box<dyn std::error::Error>> {
        use shaku::HasComponent;
        let event_bus: std::sync::Arc<dyn crate::domain::events::EventBusInterface> =
            module.resolve();
        Ok(Box::new(TotalSummaryScreen::new(event_bus)))
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

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        let screen_data = data.downcast::<TotalSummaryScreenData>()?;
        *self.total_result.write().unwrap() = Some(screen_data.total_result);
        *self.displayed.write().unwrap() = false; // Reset displayed flag to allow re-rendering
        Ok(())
    }

    fn handle_key_event(&self, key_event: event::KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Push(ScreenType::TotalSummaryShare));
                Ok(())
            }
            KeyCode::Esc => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut Frame) -> Result<()> {
        let total_result = self.total_result.read().unwrap();
        if let Some(ref total_result) = *total_result {
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

    fn update(&self) -> Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TotalSummaryScreenInterface for TotalSummaryScreen {}
