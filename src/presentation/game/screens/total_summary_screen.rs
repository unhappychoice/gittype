use crate::domain::events::EventBus;
use crate::domain::models::TotalResult;
use crate::domain::services::scoring::{TotalCalculator, TotalTracker, GLOBAL_TOTAL_TRACKER};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::models::screen::ScreenDataProvider;
use crate::presentation::game::views::{AsciiScoreView, SharingView, StatisticsView};
use crate::presentation::game::{RenderBackend, Screen, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::{GitTypeError, Result};
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::{
    cursor::MoveTo,
    event::{self},
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self},
};
use std::io::Stdout;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};

pub struct TotalSummaryScreenData {
    pub total_result: TotalResult,
}

pub struct TotalSummaryScreenDataProvider {
    total_tracker: Arc<Mutex<Option<TotalTracker>>>,
}

impl ScreenDataProvider for TotalSummaryScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let total_result = self.total_tracker
            .lock()
            .map_err(|e| GitTypeError::TerminalError(format!("Failed to lock total tracker: {}", e)))?
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

    fn show(total_summary: &TotalResult) -> Result<()> {
        let mut stdout = stdout();

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        let title = "=== TOTAL SUMMARY ===";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(8)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Colors::to_crossterm(Colors::info()))
        )?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;

        AsciiScoreView::render(
            &mut stdout,
            total_summary.total_score,
            center_col,
            center_row,
        )?;

        let stats_start_row = center_row.saturating_sub(1);
        let options_start = stats_start_row + 8;

        StatisticsView::render(&mut stdout, total_summary, center_col, stats_start_row)?;
        SharingView::render_exit_options(&mut stdout, center_col, options_start)?;

        stdout.flush()?;
        Ok(())
    }

}

impl Screen for TotalSummaryScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::TotalSummary
    }

    fn default_provider() -> Box<dyn crate::presentation::game::ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(TotalSummaryScreenDataProvider {
            total_tracker: GLOBAL_TOTAL_TRACKER.clone(),
        })
    }

    fn get_render_backend(&self) -> RenderBackend {
        RenderBackend::Crossterm
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
                self.event_bus.publish(NavigateTo::Push(ScreenType::TotalSummaryShare));
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

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut Stdout,
    ) -> Result<()> {
        if !self.displayed {
            if let Some(total_result) = &self.total_result {
                let _ = TotalSummaryScreen::show(total_result);
                self.displayed = true;
            }
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
