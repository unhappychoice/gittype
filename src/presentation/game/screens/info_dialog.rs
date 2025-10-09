use crate::domain::events::EventBus;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub enum InfoAction {
    OpenGithub,
    OpenX,
    Close,
}

pub enum InfoDialogState {
    Menu { selected_option: usize },
    Fallback { title: String, url: String },
}

pub struct InfoDialogScreen {
    state: InfoDialogState,
    event_bus: EventBus,
}

impl InfoDialogScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            state: InfoDialogState::Menu { selected_option: 0 },
            event_bus,
        }
    }

    pub fn new_fallback(title: String, url: String, event_bus: EventBus) -> Self {
        Self {
            state: InfoDialogState::Fallback { title, url },
            event_bus,
        }
    }

    fn get_options() -> [(&'static str, InfoAction); 3] {
        [
            ("GitHub Repository", InfoAction::OpenGithub),
            ("X #gittype", InfoAction::OpenX),
            ("Close", InfoAction::Close),
        ]
    }

    fn handle_option_select(&mut self) -> Result<()> {
        if let InfoDialogState::Menu { selected_option } = &self.state {
            match selected_option {
                0 => {
                    if Self::try_open_github()? {
                        self.event_bus.publish(NavigateTo::Pop);
                        Ok(())
                    } else {
                        self.state = InfoDialogState::Fallback {
                            title: "GitHub Repository".to_string(),
                            url: "https://github.com/unhappychoice/gittype".to_string(),
                        };
                        Ok(())
                    }
                }
                1 => {
                    if Self::try_open_x()? {
                        self.event_bus.publish(NavigateTo::Pop);
                        Ok(())
                    } else {
                        self.state = InfoDialogState::Fallback {
                            title: "X Search".to_string(),
                            url: "https://x.com/search?q=%23gittype".to_string(),
                        };
                        Ok(())
                    }
                }
                _ => {
                    self.event_bus.publish(NavigateTo::Pop);
                    Ok(())
                }
            }
        } else {
            self.event_bus.publish(NavigateTo::Pop);
            Ok(())
        }
    }

    fn try_open_github() -> Result<bool> {
        let url = "https://github.com/unhappychoice/gittype";
        Ok(open::that(url).is_ok())
    }

    fn try_open_x() -> Result<bool> {
        let url = "https://x.com/search?q=%23gittype";
        Ok(open::that(url).is_ok())
    }

    fn render_menu_ratatui(&self, frame: &mut Frame, selected_option: usize) {
        let area = Self::centered_rect(50, 10, frame.area());

        frame.render_widget(Clear, area);

        let block = Block::default()
            .title("Information & Links")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Colors::text()));

        frame.render_widget(block, area);

        let inner = area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        });

        let options = Self::get_options();
        let items: Vec<ListItem> = options
            .iter()
            .enumerate()
            .map(|(i, (label, _))| {
                let style = if i == selected_option {
                    Style::default().fg(Colors::warning()).bold()
                } else {
                    Style::default().fg(Colors::text_secondary())
                };

                let content = if i == selected_option {
                    format!("> {}", label)
                } else {
                    format!("  {}", label)
                };

                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items).style(Style::default().fg(Colors::text()));

        let list_area = Rect {
            x: inner.x,
            y: inner.y + 1,
            width: inner.width,
            height: 3,
        };

        frame.render_widget(list, list_area);

        let instructions = vec![
            Span::styled("[↑↓/JK]", Style::default().fg(Colors::info())),
            Span::styled(" Navigate ", Style::default().fg(Colors::text())),
            Span::styled("[SPACE]", Style::default().fg(Colors::key_action())),
            Span::styled(" Select ", Style::default().fg(Colors::text())),
            Span::styled("[ESC]", Style::default().fg(Colors::error())),
            Span::styled(" Close", Style::default().fg(Colors::text())),
        ];

        let instructions_para =
            Paragraph::new(Line::from(instructions)).alignment(Alignment::Center);

        let instructions_area = Rect {
            x: inner.x,
            y: inner.y + 5,
            width: inner.width,
            height: 1,
        };

        frame.render_widget(instructions_para, instructions_area);
    }

    fn render_fallback_ratatui(&self, frame: &mut Frame, title: &str, url: &str) {
        let width = std::cmp::max(60, url.len() + 4) as u16;
        let area = Self::centered_rect(width, 8, frame.area());

        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(format!("Cannot open {}", title))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Colors::error()));

        frame.render_widget(block, area);

        let inner = area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        });

        let message = Paragraph::new("Please copy and paste the URL below:")
            .style(Style::default().fg(Colors::warning()))
            .alignment(Alignment::Center);

        let message_area = Rect {
            x: inner.x,
            y: inner.y + 1,
            width: inner.width,
            height: 1,
        };

        frame.render_widget(message, message_area);

        let url_para = Paragraph::new(url)
            .style(Style::default().fg(Colors::info()).bold())
            .alignment(Alignment::Center);

        let url_area = Rect {
            x: inner.x,
            y: inner.y + 2,
            width: inner.width,
            height: 1,
        };

        frame.render_widget(url_para, url_area);

        let back_instructions = vec![
            Span::styled("[ESC]", Style::default().fg(Colors::key_action())),
            Span::styled(" Back", Style::default().fg(Colors::text())),
        ];

        let back_para = Paragraph::new(Line::from(back_instructions)).alignment(Alignment::Center);

        let back_area = Rect {
            x: inner.x,
            y: inner.y + 4,
            width: inner.width,
            height: 1,
        };

        frame.render_widget(back_para, back_area);
    }

    fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length((r.height.saturating_sub(height)) / 2),
                Constraint::Length(height),
                Constraint::Length((r.height.saturating_sub(height)) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((r.width.saturating_sub(width)) / 2),
                Constraint::Length(width),
                Constraint::Length((r.width.saturating_sub(width)) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

pub struct InfoDialogScreenDataProvider;

impl ScreenDataProvider for InfoDialogScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

impl Screen for InfoDialogScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::InfoDialog
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(InfoDialogScreenDataProvider)
    }

    fn init_with_data(&mut self, _data: Box<dyn std::any::Any>) -> Result<()> {
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> crate::Result<()> {
        match &mut self.state {
            InfoDialogState::Menu { selected_option } => {
                let options = Self::get_options();
                match key_event.code {
                    KeyCode::Char(' ') => self.handle_option_select(),
                    KeyCode::Up | KeyCode::Char('k') => {
                        *selected_option = if *selected_option == 0 {
                            options.len() - 1
                        } else {
                            *selected_option - 1
                        };
                        Ok(())
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        *selected_option = (*selected_option + 1) % options.len();
                        Ok(())
                    }
                    KeyCode::Esc => {
                        self.event_bus.publish(NavigateTo::Pop);
                        Ok(())
                    }
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.event_bus.publish(NavigateTo::Exit);
                        Ok(())
                    }
                    _ => Ok(()),
                }
            }
            InfoDialogState::Fallback { .. } => match key_event.code {
                KeyCode::Esc => {
                    self.state = InfoDialogState::Menu { selected_option: 0 };
                    Ok(())
                }
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.event_bus.publish(NavigateTo::Exit);
                    Ok(())
                }
                _ => Ok(()),
            },
        }
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
        match &self.state {
            InfoDialogState::Menu { selected_option } => {
                self.render_menu_ratatui(frame, *selected_option);
            }
            InfoDialogState::Fallback { title, url } => {
                self.render_fallback_ratatui(frame, title, url);
            }
        }
        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> crate::Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
