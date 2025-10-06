use crate::domain::events::EventBus;
use crate::domain::models::{SessionResult, TotalResult};
use crate::domain::services::scoring::{TotalCalculator, GLOBAL_TOTAL_TRACKER};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::views::{AsciiScoreView, SharingView, StatisticsView};
use crate::presentation::game::{Screen, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::Result;
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

#[derive(Debug)]
pub enum ExitAction {
    Exit,
    Share,
}

pub struct TotalSummaryScreen {
    displayed: bool,
    event_bus: EventBus,
}

impl TotalSummaryScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            displayed: false,
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

    fn get_total_result_from_tracker() -> Option<TotalResult> {
        if let Ok(global_total_tracker) = GLOBAL_TOTAL_TRACKER.lock() {
            (*global_total_tracker)
                .as_ref()
                .map(TotalCalculator::calculate)
        } else {
            None
        }
    }
}

impl Screen for TotalSummaryScreen {
    fn init(&mut self) -> Result<()> {
        self.displayed = false;
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
        _session_result: Option<&SessionResult>,
        _total_result: Option<&TotalResult>,
    ) -> Result<()> {
        if !self.displayed {
            let mut total_result = Self::get_total_result_from_tracker().unwrap_or_default();
            total_result.finalize(); // Ensure MAX values are converted to 0.0

            let _ = TotalSummaryScreen::show(&total_result);
            self.displayed = true;
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
