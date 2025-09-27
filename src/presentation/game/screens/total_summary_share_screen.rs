use crate::presentation::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::presentation::game::views::SharingView;
use crate::domain::models::TotalResult;
use crate::presentation::sharing::SharingPlatform;
use crate::Result;
use crossterm::{
    event::{self},
    execute,
    terminal::{self, ClearType},
};
use std::io::Stdout;

#[derive(Debug)]
pub enum ShareAction {
    Back,
    Exit,
}

pub struct TotalSummaryShareScreen {
    total_result: TotalResult,
    fallback_url: Option<(String, SharingPlatform)>,
    last_fallback_state: bool,
}

impl TotalSummaryShareScreen {
    pub fn new(total_result: TotalResult) -> Self {
        Self {
            total_result,
            fallback_url: None,
            last_fallback_state: false,
        }
    }

    fn handle_share_platform(&mut self, platform: SharingPlatform) -> Result<ScreenTransition> {
        let text = self.total_result.create_share_text();
        let url = self.generate_share_url(&text, &platform);

        match self.open_browser(&url) {
            Ok(()) => Ok(ScreenTransition::Pop),
            Err(_) => {
                log::warn!(
                    "Failed to open browser for {}. URL: {}",
                    platform.name(),
                    url
                );
                self.fallback_url = Some((url.clone(), platform));
                Ok(ScreenTransition::None)
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
        open::that(url).map_err(|e| {
            crate::domain::error::GitTypeError::TerminalError(format!("Failed to open browser: {}", e))
        })
    }
}

impl Screen for TotalSummaryShareScreen {
    fn init(&mut self) -> crate::Result<()> {
        self.last_fallback_state = false;
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Char('1') => self.handle_share_platform(SharingPlatform::X),
            KeyCode::Char('2') => self.handle_share_platform(SharingPlatform::Reddit),
            KeyCode::Char('3') => self.handle_share_platform(SharingPlatform::LinkedIn),
            KeyCode::Char('4') => self.handle_share_platform(SharingPlatform::Facebook),
            KeyCode::Esc => {
                if self.fallback_url.is_some() {
                    self.fallback_url = None;
                    Ok(ScreenTransition::None)
                } else {
                    Ok(ScreenTransition::Pop)
                }
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        stdout: &mut Stdout,
        _session_result: Option<&crate::domain::models::SessionResult>,
        _total_result: Option<&TotalResult>,
    ) -> Result<()> {
        let current_fallback_state = self.fallback_url.is_some();

        if current_fallback_state != self.last_fallback_state {
            execute!(stdout, terminal::Clear(ClearType::All))?;
            self.last_fallback_state = current_fallback_state;
        }

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        if let Some((url, platform)) = &self.fallback_url {
            SharingView::render_fallback_url(stdout, url, platform, center_col, center_row)?;
        } else {
            SharingView::render_menu(stdout, &self.total_result, center_col, center_row)?;
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
