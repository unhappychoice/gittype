use crate::domain::events::EventBus;
use crate::domain::models::TotalResult;
use crate::domain::services::scoring::{TotalCalculator, TotalTracker, GLOBAL_TOTAL_TRACKER};
use crate::infrastructure::browser;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::views::SharingView;
use crate::presentation::game::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::presentation::sharing::SharingPlatform;
use crate::{GitTypeError, Result};
use crossterm::event::{self};
use ratatui::Frame;
use std::sync::{Arc, Mutex};

pub struct TotalSummaryShareData {
    pub total_result: TotalResult,
}

pub struct TotalSummaryShareDataProvider {
    total_tracker: Arc<Mutex<Option<TotalTracker>>>,
}

impl ScreenDataProvider for TotalSummaryShareDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let total_result = self
            .total_tracker
            .lock()
            .map_err(|_| GitTypeError::TerminalError("Failed to lock TotalTracker".to_string()))?
            .as_ref()
            .ok_or_else(|| GitTypeError::TerminalError("No total tracker available".to_string()))
            .map(TotalCalculator::calculate)?;

        Ok(Box::new(TotalSummaryShareData { total_result }))
    }
}

#[derive(Debug)]
pub enum ShareAction {
    Back,
    Exit,
}

pub struct TotalSummaryShareScreen {
    total_result: TotalResult,
    fallback_url: Option<(String, SharingPlatform)>,
    last_fallback_state: bool,
    event_bus: EventBus,
}

impl TotalSummaryShareScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            total_result: TotalResult::new(),
            fallback_url: None,
            last_fallback_state: false,
            event_bus,
        }
    }

    pub fn set_total_result(&mut self, total_result: TotalResult) {
        self.total_result = total_result;
    }

    fn handle_share_platform(&mut self, platform: SharingPlatform) -> Result<()> {
        let text = self.total_result.create_share_text();
        let url = self.generate_share_url(&text, &platform);

        match self.open_browser(&url) {
            Ok(()) => {
                self.event_bus.publish(NavigateTo::Pop);
                Ok(())
            }
            Err(_) => {
                log::warn!(
                    "Failed to open browser for {}. URL: {}",
                    platform.name(),
                    url
                );
                self.fallback_url = Some((url.clone(), platform));
                Ok(())
            }
        }
    }

    fn generate_share_url(&self, text: &str, platform: &SharingPlatform) -> String {
        match platform {
            SharingPlatform::X => {
                format!(
                    "https://x.com/intent/tweet?text={}",
                    urlencoding::encode(text)
                )
            }
            SharingPlatform::Reddit => {
                let title = format!(
                    "Just demolished {} keystrokes total in gittype! Score: {:.0}, CPM: {:.0}",
                    self.total_result.total_keystrokes,
                    self.total_result.total_score,
                    self.total_result.overall_cpm
                );
                format!(
                    "https://www.reddit.com/submit?title={}&selftext=true&text={}",
                    urlencoding::encode(&title),
                    urlencoding::encode(text)
                )
            }
            SharingPlatform::LinkedIn => {
                format!(
                    "https://www.linkedin.com/feed/?shareActive=true&mini=true&text={}",
                    urlencoding::encode(text)
                )
            }
            SharingPlatform::Facebook => {
                format!(
                    "https://www.facebook.com/sharer/sharer.php?u={}&quote={}",
                    urlencoding::encode("https://github.com/unhappychoice/gittype"),
                    urlencoding::encode(text)
                )
            }
        }
    }

    fn open_browser(&self, url: &str) -> crate::Result<()> {
        browser::open_url(url)
            .map_err(|e| GitTypeError::TerminalError(format!("Failed to open browser: {}", e)))
    }
}

impl Screen for TotalSummaryShareScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::TotalSummaryShare
    }

    fn default_provider() -> Box<dyn crate::presentation::game::ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(TotalSummaryShareDataProvider {
            total_tracker: GLOBAL_TOTAL_TRACKER.clone(),
        })
    }

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        self.last_fallback_state = false;

        let data = data.downcast::<TotalSummaryShareData>()?;
        self.total_result = data.total_result;

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Char('1') => self.handle_share_platform(SharingPlatform::X),
            KeyCode::Char('2') => self.handle_share_platform(SharingPlatform::Reddit),
            KeyCode::Char('3') => self.handle_share_platform(SharingPlatform::LinkedIn),
            KeyCode::Char('4') => self.handle_share_platform(SharingPlatform::Facebook),
            KeyCode::Esc => {
                if self.fallback_url.is_some() {
                    self.fallback_url = None;
                    Ok(())
                } else {
                    self.event_bus.publish(NavigateTo::Pop);
                    Ok(())
                }
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
        if let Some((url, platform)) = &self.fallback_url {
            SharingView::render_fallback_url(frame, url, platform);
        } else {
            SharingView::render_menu(frame, &self.total_result);
        }
        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::TimeBased(std::time::Duration::from_millis(200))
    }

    fn update(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
