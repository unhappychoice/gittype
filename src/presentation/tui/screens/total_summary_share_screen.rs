use crate::domain::events::EventBusInterface;
use crate::domain::models::TotalResult;
use crate::domain::services::scoring::{TotalCalculator, TotalTracker, GLOBAL_TOTAL_TRACKER};
use crate::infrastructure::browser;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::sharing::SharingPlatform;
use crate::presentation::tui::views::SharingView;
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::{GitTypeError, Result};
use crossterm::event::{self};
use ratatui::Frame;
use std::sync::RwLock;
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

pub trait TotalSummaryShareScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = TotalSummaryShareScreenInterface)]
pub struct TotalSummaryShareScreen {
    #[shaku(default)]
    total_result: RwLock<TotalResult>,
    #[shaku(default)]
    fallback_url: RwLock<Option<(String, SharingPlatform)>>,
    #[shaku(default)]
    last_fallback_state: RwLock<bool>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
}

impl TotalSummaryShareScreen {
    pub fn new(event_bus: Arc<dyn EventBusInterface>) -> Self {
        Self {
            total_result: RwLock::new(TotalResult::new()),
            fallback_url: RwLock::new(None),
            last_fallback_state: RwLock::new(false),
            event_bus,
        }
    }

    pub fn set_total_result(&self, total_result: TotalResult) {
        *self.total_result.write().unwrap() = total_result;
    }

    fn handle_share_platform(&self, platform: SharingPlatform) -> Result<()> {
        let total_result = self.total_result.read().unwrap();
        let text = total_result.create_share_text();
        let url = self.generate_share_url(&text, &platform);

        match self.open_browser(&url) {
            Ok(()) => {
                self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                Ok(())
            }
            Err(_) => {
                log::warn!(
                    "Failed to open browser for {}. URL: {}",
                    platform.name(),
                    url
                );
                *self.fallback_url.write().unwrap() = Some((url.clone(), platform));
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
                let total_result = self.total_result.read().unwrap();
                let title = format!(
                    "Just demolished {} keystrokes total in gittype! Score: {:.0}, CPM: {:.0}",
                    total_result.total_keystrokes,
                    total_result.total_score,
                    total_result.overall_cpm
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

pub struct TotalSummaryShareScreenProvider;

impl shaku::Provider<crate::presentation::di::AppModule> for TotalSummaryShareScreenProvider {
    type Interface = TotalSummaryShareScreen;

    fn provide(
        module: &crate::presentation::di::AppModule,
    ) -> std::result::Result<Box<Self::Interface>, Box<dyn std::error::Error>> {
        use shaku::HasComponent;
        let event_bus: std::sync::Arc<dyn crate::domain::events::EventBusInterface> =
            module.resolve();
        Ok(Box::new(TotalSummaryShareScreen::new(event_bus)))
    }
}

impl Screen for TotalSummaryShareScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::TotalSummaryShare
    }

    fn default_provider() -> Box<dyn crate::presentation::tui::ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(TotalSummaryShareDataProvider {
            total_tracker: GLOBAL_TOTAL_TRACKER.clone(),
        })
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        *self.last_fallback_state.write().unwrap() = false;

        let data = data.downcast::<TotalSummaryShareData>()?;
        *self.total_result.write().unwrap() = data.total_result;

        Ok(())
    }

    fn handle_key_event(&self, key_event: event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Char('1') => self.handle_share_platform(SharingPlatform::X),
            KeyCode::Char('2') => self.handle_share_platform(SharingPlatform::Reddit),
            KeyCode::Char('3') => self.handle_share_platform(SharingPlatform::LinkedIn),
            KeyCode::Char('4') => self.handle_share_platform(SharingPlatform::Facebook),
            KeyCode::Esc => {
                let fallback_url = self.fallback_url.read().unwrap();
                if fallback_url.is_some() {
                    drop(fallback_url);
                    *self.fallback_url.write().unwrap() = None;
                    Ok(())
                } else {
                    self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                    Ok(())
                }
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut Frame) -> Result<()> {
        let fallback_url = self.fallback_url.read().unwrap();
        if let Some((url, platform)) = &*fallback_url {
            SharingView::render_fallback_url(frame, url, platform);
        } else {
            let total_result = self.total_result.read().unwrap();
            SharingView::render_menu(frame, &total_result);
        }
        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::TimeBased(std::time::Duration::from_millis(200))
    }

    fn update(&self) -> Result<bool> {
        Ok(true)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TotalSummaryShareScreenInterface for TotalSummaryShareScreen {}
