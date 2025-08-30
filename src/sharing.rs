use crate::scoring::TypingMetrics;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum SharingPlatform {
    Twitter,
    Reddit,
    LinkedIn,
    Facebook,
}

impl SharingPlatform {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Twitter => "Twitter",
            Self::Reddit => "Reddit",
            Self::LinkedIn => "LinkedIn", 
            Self::Facebook => "Facebook",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Twitter, Self::Reddit, Self::LinkedIn, Self::Facebook]
    }
}

pub struct SharingService;

impl SharingService {
    pub fn share_result(metrics: &TypingMetrics, platform: SharingPlatform) -> Result<()> {
        let url = Self::generate_share_url(metrics, &platform);
        
        match Self::open_browser(&url) {
            Ok(()) => {
                // Browser opened successfully
                Ok(())
            }
            Err(_) => {
                // Fallback: display URL to user
                Self::display_url_fallback(&url, &platform)
            }
        }
    }

    fn generate_share_url(metrics: &TypingMetrics, platform: &SharingPlatform) -> String {
        let text = Self::create_share_text(metrics);
        
        match platform {
            SharingPlatform::Twitter => {
                format!("https://twitter.com/intent/tweet?text={}", urlencoding::encode(&text))
            },
            SharingPlatform::Reddit => {
                let title = format!("Achieved {} rank with {:.0} points in gittype!", metrics.ranking_title, metrics.challenge_score);
                format!("https://www.reddit.com/submit?title={}&selftext=true&text={}", 
                    urlencoding::encode(&title), urlencoding::encode(&text))
            },
            SharingPlatform::LinkedIn => {
                format!("https://www.linkedin.com/feed/?shareActive=true&mini=true&text={}",
                    urlencoding::encode(&text))
            },
            SharingPlatform::Facebook => {
                // Facebook's quote parameter may not work reliably, but it's still the best option
                format!("https://www.facebook.com/sharer/sharer.php?u={}&quote={}",
                    urlencoding::encode("https://github.com/unhappychoice/gittype"),
                    urlencoding::encode(&text))
            },
        }
    }

    fn create_share_text(metrics: &TypingMetrics) -> String {
        format!(
            "I achieved the rank \"{}\" with a score of {:.0} points! CPM: {:.0}, Mistakes: {} in gittype! 🚀\n\nType your own code! https://github.com/unhappychoice/gittype\n\n#gittype #typing #coding",
            metrics.ranking_title,
            metrics.challenge_score,
            metrics.cpm,
            metrics.mistakes
        )
    }

    fn open_browser(url: &str) -> Result<()> {
        open::that(url).map_err(|e| anyhow::anyhow!("Failed to open browser: {}", e))
    }

    fn display_url_fallback(url: &str, platform: &SharingPlatform) -> Result<()> {
        use crossterm::{
            cursor::MoveTo,
            event::{self, Event},
            execute,
            style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
            terminal::{self, ClearType},
        };
        use std::io::{stdout, Write};

        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Title
        let title = format!("⚠️  Could not open {} automatically", platform.name());
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(6)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Yellow))?;
        execute!(stdout, Print(&title))?;
        execute!(stdout, ResetColor)?;

        // Instructions
        let instruction = "Please copy the URL below and open it in your browser:";
        let instruction_col = center_col.saturating_sub(instruction.len() as u16 / 2);
        execute!(stdout, MoveTo(instruction_col, center_row.saturating_sub(4)))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(instruction))?;
        execute!(stdout, ResetColor)?;

        // URL display box
        let url_display = format!("📋 {}", url);
        let url_col = center_col.saturating_sub(url_display.len() as u16 / 2);
        execute!(stdout, MoveTo(url_col, center_row.saturating_sub(1)))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(&url_display))?;
        execute!(stdout, ResetColor)?;

        // Additional info
        let info = "💡 Tip: Select and copy the URL with your mouse or keyboard";
        let info_col = center_col.saturating_sub(info.len() as u16 / 2);
        execute!(stdout, MoveTo(info_col, center_row + 2))?;
        execute!(stdout, SetForegroundColor(Color::Grey))?;
        execute!(stdout, Print(info))?;
        execute!(stdout, ResetColor)?;

        // Continue prompt
        let continue_text = "Press any key to continue...";
        let continue_col = center_col.saturating_sub(continue_text.len() as u16 / 2);
        execute!(stdout, MoveTo(continue_col, center_row + 4))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(continue_text))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;

        // Wait for user input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(_) = event::read()? {
                    break;
                }
            }
        }

        Ok(())
    }
}