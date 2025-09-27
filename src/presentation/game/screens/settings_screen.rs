use crate::domain::models::color_mode::ColorMode;
use crate::domain::models::theme::Theme;
use crate::domain::models::{SessionResult, TotalResult};
use crate::domain::services::theme_manager::THEME_MANAGER;
use crate::infrastructure::config::ConfigManager;
use crate::presentation::game::{Screen, ScreenTransition};
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph, Tabs, Wrap},
    Frame,
};
use std::io::Stdout;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SettingsSection {
    ColorMode,
    Theme,
}

impl SettingsSection {
    fn all() -> &'static [SettingsSection] {
        &[SettingsSection::ColorMode, SettingsSection::Theme]
    }

    fn title(&self) -> &'static str {
        match self {
            SettingsSection::ColorMode => "Color Mode",
            SettingsSection::Theme => "Theme",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            SettingsSection::ColorMode => "Choose between dark and light modes",
            SettingsSection::Theme => "Select theme - preview changes instantly",
        }
    }
}

pub struct SettingsScreen {
    current_section: SettingsSection,
    color_mode_state: ListState,
    theme_state: ListState,
    color_modes: Vec<ColorMode>,
    themes: Vec<Theme>,
    original_theme: Theme,
    original_color_mode: ColorMode,
    is_preview_mode: bool,
}

impl Default for SettingsScreen {
    fn default() -> Self {
        let mut color_mode_state = ListState::default();
        let mut theme_state = ListState::default();

        let theme_manager = THEME_MANAGER.read().unwrap();
        let current_color_mode = theme_manager.current_color_mode.clone();
        let current_theme = theme_manager.current_theme.clone();

        let color_modes = vec![ColorMode::Dark, ColorMode::Light];
        let themes = theme_manager.get_available_themes();
        drop(theme_manager); // Release the lock early

        // Set initial selections
        if let Some(pos) = color_modes.iter().position(|m| m == &current_color_mode) {
            color_mode_state.select(Some(pos));
        }
        if let Some(pos) = themes.iter().position(|t| t.id == current_theme.id) {
            theme_state.select(Some(pos));
        }

        Self {
            current_section: SettingsSection::ColorMode,
            color_mode_state,
            theme_state,
            color_modes,
            themes,
            original_theme: current_theme,
            original_color_mode: current_color_mode,
            is_preview_mode: false,
        }
    }
}

impl SettingsScreen {
    fn apply_current_selection(&mut self) {
        self.is_preview_mode = true;

        let selected_color_mode = self.get_selected_color_mode();
        let selected_theme = self.get_selected_theme();

        if let (Some(color_mode), Some(theme)) = (selected_color_mode, selected_theme) {
            let mut theme_manager = THEME_MANAGER.write().unwrap();
            theme_manager.current_color_mode = color_mode.clone();
            theme_manager.current_theme = theme.clone();
        }
    }

    fn revert_to_original(&mut self) {
        if self.is_preview_mode {
            let mut theme_manager = THEME_MANAGER.write().unwrap();
            theme_manager.current_color_mode = self.original_color_mode.clone();
            theme_manager.current_theme = self.original_theme.clone();
            self.is_preview_mode = false;
        }
    }

    fn save_settings(&mut self) {
        self.is_preview_mode = false;

        // Save theme and color mode to config file
        if let Ok(mut config_manager) = ConfigManager::new() {
            let selected_color_mode = self.get_selected_color_mode();
            let selected_theme = self.get_selected_theme();

            if let (Some(color_mode), Some(theme)) = (selected_color_mode, selected_theme) {
                config_manager.get_config_mut().theme.current_color_mode = color_mode.clone();
                config_manager.get_config_mut().theme.current_theme_id = theme.id.clone();
                let _ = config_manager.save();
            }
        }
    }

    fn get_selected_color_mode(&self) -> Option<&ColorMode> {
        self.color_mode_state
            .selected()
            .and_then(|i| self.color_modes.get(i))
    }

    fn get_selected_theme(&self) -> Option<&Theme> {
        self.theme_state.selected().and_then(|i| self.themes.get(i))
    }

    fn render_color_mode_section(&self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .color_modes
            .iter()
            .map(|mode| {
                let text = match mode {
                    ColorMode::Dark => "Dark",
                    ColorMode::Light => "Light",
                };
                ListItem::new(text)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Color Mode")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .padding(Padding::horizontal(2)),
            )
            .highlight_style(Style::default().bg(Colors::text()).fg(Colors::background()));

        f.render_stateful_widget(list, area, &mut self.color_mode_state.clone());
    }

    fn render_theme_section(&self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .themes
            .iter()
            .map(|theme| ListItem::new(theme.name.as_str()))
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Theme")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .padding(Padding::horizontal(2)),
            )
            .highlight_style(Style::default().bg(Colors::text()).fg(Colors::background()));

        f.render_stateful_widget(list, area, &mut self.theme_state.clone());
    }

    fn render_description(&self, f: &mut Frame, area: Rect) {
        let content = match self.current_section {
            SettingsSection::ColorMode => {
                vec![Line::from(self.current_section.description())]
            }
            SettingsSection::Theme => {
                let mut lines = vec![Line::from(self.current_section.description())];

                if let Some(theme) = self.get_selected_theme() {
                    lines.push(Line::from(""));
                    lines.push(Line::from(theme.description.as_str()));
                    lines.push(Line::from(""));
                    lines.push(Line::from("Color Preview:"));

                    // Add color preview lines with actual colors
                    let color_examples = vec![
                        ("Border", Colors::border()),
                        ("Title", Colors::title()),
                        ("Text", Colors::text()),
                        ("Text Secondary", Colors::text_secondary()),
                        ("Success", Colors::success()),
                        ("Error", Colors::error()),
                        ("Warning", Colors::warning()),
                        ("Info", Colors::info()),
                        ("Key Action", Colors::key_action()),
                        ("Key Navigation", Colors::key_navigation()),
                        ("Key Back", Colors::key_back()),
                        ("Typed Text", Colors::typed_text()),
                        ("Cursor", Colors::current_cursor()),
                        ("Mistake", Colors::mistake_bg()),
                        ("Untyped Text", Colors::untyped_text()),
                    ];

                    for (name, color) in color_examples {
                        lines.push(Line::from(vec![
                            Span::styled("● ", Style::default().fg(color)),
                            Span::styled(
                                format!("This is {} color", name),
                                Style::default().fg(color),
                            ),
                        ]));
                    }
                }

                lines
            }
        };

        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(Colors::text()))
            .block(
                Block::default()
                    .title("Description")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .padding(Padding::horizontal(2)),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        f.render_widget(paragraph, area);
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let sections = SettingsSection::all();
        let titles: Vec<Line> = sections
            .iter()
            .map(|section| Line::from(section.title()))
            .collect();

        let selected_index = sections
            .iter()
            .position(|&s| s == self.current_section)
            .unwrap_or(0);

        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Settings"),
            )
            .highlight_style(Style::default().fg(Colors::text()).bold())
            .select(selected_index);

        f.render_widget(tabs, area);
    }

    fn render_content(&mut self, f: &mut Frame, area: Rect) {
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        match self.current_section {
            SettingsSection::ColorMode => {
                self.render_color_mode_section(f, content_chunks[0]);
                self.render_description(f, content_chunks[1]);
            }
            SettingsSection::Theme => {
                self.render_theme_section(f, content_chunks[0]);
                self.render_description(f, content_chunks[1]);
            }
        }
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        // Instructions (matching help screen format)
        let instructions = vec![
            Span::styled("[←→/HL]", Style::default().fg(Colors::info())),
            Span::styled(" Switch tabs ", Style::default().fg(Colors::text())),
            Span::styled("[↑↓/JK]", Style::default().fg(Colors::info())),
            Span::styled(" Navigate ", Style::default().fg(Colors::text())),
            Span::styled("[SPACE]", Style::default().fg(Colors::key_action())),
            Span::styled(" Save ", Style::default().fg(Colors::text())),
            Span::styled("[ESC]", Style::default().fg(Colors::error())),
            Span::styled(" Cancel", Style::default().fg(Colors::text())),
        ];
        let instructions_para =
            Paragraph::new(Line::from(instructions)).alignment(Alignment::Center);
        f.render_widget(instructions_para, area);
    }
}

impl Screen for SettingsScreen {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<ScreenTransition> {
        match key_event.code {
            KeyCode::Left | KeyCode::Char('h') => {
                let sections = SettingsSection::all();
                let current_index = sections
                    .iter()
                    .position(|&s| s == self.current_section)
                    .unwrap_or(0);
                let new_index = if current_index == 0 {
                    sections.len() - 1
                } else {
                    current_index - 1
                };
                self.current_section = sections[new_index];
                Ok(ScreenTransition::None)
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let sections = SettingsSection::all();
                let current_index = sections
                    .iter()
                    .position(|&s| s == self.current_section)
                    .unwrap_or(0);
                let new_index = (current_index + 1) % sections.len();
                self.current_section = sections[new_index];
                Ok(ScreenTransition::None)
            }
            KeyCode::Up | KeyCode::Char('k') => {
                match self.current_section {
                    SettingsSection::ColorMode => {
                        let selected = self.color_mode_state.selected().unwrap_or(0);
                        if selected > 0 {
                            self.color_mode_state.select(Some(selected - 1));
                            self.apply_current_selection();
                        }
                    }
                    SettingsSection::Theme => {
                        let selected = self.theme_state.selected().unwrap_or(0);
                        if selected > 0 {
                            self.theme_state.select(Some(selected - 1));
                            self.apply_current_selection();
                        }
                    }
                }
                Ok(ScreenTransition::None)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                match self.current_section {
                    SettingsSection::ColorMode => {
                        let selected = self.color_mode_state.selected().unwrap_or(0);
                        if selected < self.color_modes.len() - 1 {
                            self.color_mode_state.select(Some(selected + 1));
                            self.apply_current_selection();
                        }
                    }
                    SettingsSection::Theme => {
                        let selected = self.theme_state.selected().unwrap_or(0);
                        if selected < self.themes.len() - 1 {
                            self.theme_state.select(Some(selected + 1));
                            self.apply_current_selection();
                        }
                    }
                }
                Ok(ScreenTransition::None)
            }
            KeyCode::Char(' ') => {
                self.save_settings();
                Ok(ScreenTransition::Pop)
            }
            KeyCode::Esc => {
                self.revert_to_original();
                Ok(ScreenTransition::Pop)
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut Stdout,
        _session_result: Option<&SessionResult>,
        _total_result: Option<&TotalResult>,
    ) -> Result<()> {
        // This should not be used for ratatui screens
        Ok(())
    }

    fn render_ratatui(&mut self, f: &mut Frame) -> Result<()> {
        let area = f.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        self.render_tabs(f, chunks[0]);
        self.render_content(f, chunks[1]);
        self.render_footer(f, chunks[2]);

        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
