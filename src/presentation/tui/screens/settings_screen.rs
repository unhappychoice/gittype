use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::events::EventBusInterface;
use crate::domain::models::color_mode::ColorMode;
use crate::domain::models::theme::Theme;
use crate::domain::services::config_service::{ConfigService, ConfigServiceInterface};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType};
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph, Tabs, Wrap},
    Frame,
};
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SettingsSection {
    #[default]
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

pub struct SettingsScreenData {
    pub color_modes: Vec<ColorMode>,
    pub themes: Vec<Theme>,
    pub current_theme: Theme,
    pub current_color_mode: ColorMode,
}

pub trait SettingsScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = SettingsScreenInterface)]
pub struct SettingsScreen {
    #[shaku(default)]
    current_section: RwLock<SettingsSection>,
    #[shaku(default)]
    color_mode_state: RwLock<ListState>,
    #[shaku(default)]
    theme_state: RwLock<ListState>,
    #[shaku(default)]
    color_modes: RwLock<Vec<ColorMode>>,
    #[shaku(default)]
    themes: RwLock<Vec<Theme>>,
    #[shaku(default)]
    original_theme: RwLock<Theme>,
    #[shaku(default)]
    original_color_mode: RwLock<ColorMode>,
    #[shaku(default)]
    is_preview_mode: RwLock<bool>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
    #[shaku(inject)]
    config_service: Arc<dyn ConfigServiceInterface>,
}

impl SettingsScreen {
    #[cfg(feature = "test-mocks")]
    pub fn new(event_bus: Arc<dyn EventBusInterface>) -> Self {
        use crate::domain::services::theme_service::ThemeService;
        use crate::infrastructure::storage::file_storage::FileStorage;
        let file_storage = Arc::new(FileStorage::new());
        let config_service = Arc::new(ConfigService::new(file_storage.clone()).unwrap());
        let theme_service = Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        ));
        Self {
            current_section: RwLock::new(SettingsSection::default()),
            color_mode_state: RwLock::new(ListState::default()),
            theme_state: RwLock::new(ListState::default()),
            color_modes: RwLock::new(Vec::new()),
            themes: RwLock::new(Vec::new()),
            original_theme: RwLock::new(Theme::default()),
            original_color_mode: RwLock::new(ColorMode::default()),
            is_preview_mode: RwLock::new(false),
            event_bus,
            config_service,
            theme_service,
        }
    }

    fn apply_current_selection(&self) {
        *self.is_preview_mode.write().unwrap() = true;

        let selected_color_mode = self.get_selected_color_mode();
        let selected_theme = self.get_selected_theme();

        if let (Some(color_mode), Some(theme)) = (selected_color_mode, selected_theme) {
            self.theme_service
                .set_current_color_mode(color_mode.clone());
            self.theme_service.set_current_theme(theme.clone());
        }
    }

    fn revert_to_original(&self) {
        let is_preview_mode = *self.is_preview_mode.read().unwrap();
        if is_preview_mode {
            let original_color_mode = self.original_color_mode.read().unwrap();
            let original_theme = self.original_theme.read().unwrap();
            self.theme_service
                .set_current_color_mode(original_color_mode.clone());
            self.theme_service.set_current_theme(original_theme.clone());

            // Color scheme is already updated in theme_service
            *self.is_preview_mode.write().unwrap() = false;
        }
    }

    fn save_settings(&self) {
        *self.is_preview_mode.write().unwrap() = false;

        // Save theme and color mode to config file
        let selected_color_mode = self.get_selected_color_mode();
        let selected_theme = self.get_selected_theme();

        if let (Some(color_mode), Some(theme)) = (selected_color_mode, selected_theme) {
            // Downcast to concrete type to access update_config method
            if let Some(config_service) =
                (self.config_service.as_ref() as &dyn std::any::Any).downcast_ref::<ConfigService>()
            {
                let _ = config_service.update_config(|config| {
                    config.theme.current_color_mode = color_mode.clone();
                    config.theme.current_theme_id = theme.id.clone();
                });
                let _ = self.config_service.save();
            }
        }
    }

    fn get_selected_color_mode(&self) -> Option<ColorMode> {
        let color_mode_state = self.color_mode_state.read().unwrap();
        let color_modes = self.color_modes.read().unwrap();
        color_mode_state
            .selected()
            .and_then(|i| color_modes.get(i).cloned())
    }

    fn get_selected_theme(&self) -> Option<Theme> {
        let theme_state = self.theme_state.read().unwrap();
        let themes = self.themes.read().unwrap();
        theme_state.selected().and_then(|i| themes.get(i).cloned())
    }

    fn render_color_mode_section(&self, f: &mut Frame, area: Rect, colors: &Colors) {
        let color_modes = self.color_modes.read().unwrap();
        let items: Vec<ListItem> = color_modes
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
                    .border_style(Style::default().fg(colors.border()))
                    .padding(Padding::horizontal(2)),
            )
            .highlight_style(Style::default().bg(colors.text()).fg(colors.background()));

        let mut color_mode_state = self.color_mode_state.write().unwrap();
        f.render_stateful_widget(list, area, &mut *color_mode_state);
    }

    fn render_theme_section(&self, f: &mut Frame, area: Rect, colors: &Colors) {
        let themes = self.themes.read().unwrap();
        let items: Vec<ListItem> = themes
            .iter()
            .map(|theme| ListItem::new(theme.name.as_str()))
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Theme")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border()))
                    .padding(Padding::horizontal(2)),
            )
            .highlight_style(Style::default().bg(colors.text()).fg(colors.background()));

        let mut theme_state = self.theme_state.write().unwrap();
        f.render_stateful_widget(list, area, &mut *theme_state);
    }

    fn render_description(&self, f: &mut Frame, area: Rect, colors: &Colors) {
        let current_section = *self.current_section.read().unwrap();
        let content = match current_section {
            SettingsSection::ColorMode => {
                vec![Line::from(current_section.description())]
            }
            SettingsSection::Theme => {
                let mut lines = vec![Line::from(current_section.description())];

                if let Some(theme) = self.get_selected_theme() {
                    lines.push(Line::from(""));
                    lines.push(Line::from(theme.description.clone()));
                    lines.push(Line::from(""));
                    lines.push(Line::from("Color Preview:"));

                    // Add color preview lines with actual colors
                    let color_examples = vec![
                        ("Border", colors.border()),
                        ("Title", colors.title()),
                        ("Text", colors.text()),
                        ("Text Secondary", colors.text_secondary()),
                        ("Success", colors.success()),
                        ("Error", colors.error()),
                        ("Warning", colors.warning()),
                        ("Info", colors.info()),
                        ("Key Action", colors.key_action()),
                        ("Key Navigation", colors.key_navigation()),
                        ("Key Back", colors.key_back()),
                        ("Typed Text", colors.typed_text()),
                        ("Cursor", colors.current_cursor()),
                        ("Mistake", colors.mistake_bg()),
                        ("Untyped Text", colors.untyped_text()),
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
            .style(Style::default().fg(colors.text()))
            .block(
                Block::default()
                    .title("Description")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border()))
                    .padding(Padding::horizontal(2)),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        f.render_widget(paragraph, area);
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect, colors: &Colors) {
        let sections = SettingsSection::all();
        let titles: Vec<Line> = sections
            .iter()
            .map(|section| Line::from(section.title()))
            .collect();

        let current_section = *self.current_section.read().unwrap();
        let selected_index = sections
            .iter()
            .position(|&s| s == current_section)
            .unwrap_or(0);

        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border()))
                    .title("Settings"),
            )
            .highlight_style(Style::default().fg(colors.text()).bold())
            .select(selected_index);

        f.render_widget(tabs, area);
    }

    fn render_content(&self, f: &mut Frame, area: Rect, colors: &Colors) {
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let current_section = *self.current_section.read().unwrap();
        match current_section {
            SettingsSection::ColorMode => {
                self.render_color_mode_section(f, content_chunks[0], colors);
                self.render_description(f, content_chunks[1], colors);
            }
            SettingsSection::Theme => {
                self.render_theme_section(f, content_chunks[0], colors);
                self.render_description(f, content_chunks[1], colors);
            }
        }
    }

    fn render_footer(&self, f: &mut Frame, area: Rect, colors: &Colors) {
        // Instructions (matching help screen format)
        let instructions = vec![
            Span::styled("[←→/HL]", Style::default().fg(colors.info())),
            Span::styled(" Switch tabs ", Style::default().fg(colors.text())),
            Span::styled("[↑↓/JK]", Style::default().fg(colors.info())),
            Span::styled(" Navigate ", Style::default().fg(colors.text())),
            Span::styled("[SPACE]", Style::default().fg(colors.key_action())),
            Span::styled(" Save ", Style::default().fg(colors.text())),
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(" Cancel", Style::default().fg(colors.text())),
        ];
        let instructions_para =
            Paragraph::new(Line::from(instructions)).alignment(Alignment::Center);
        f.render_widget(instructions_para, area);
    }
}

pub struct SettingsScreenDataProvider;

impl ScreenDataProvider for SettingsScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        // Return empty data - SettingsScreen will fetch data from its injected theme_service
        let data = SettingsScreenData {
            color_modes: vec![],
            themes: vec![],
            current_theme: Theme::default(),
            current_color_mode: ColorMode::default(),
        };
        Ok(Box::new(data))
    }
}

impl Screen for SettingsScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::Settings
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(SettingsScreenDataProvider)
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        let _data = data.downcast::<SettingsScreenData>()?;

        // Fetch data from theme_service since provider returns empty data
        let color_modes = vec![ColorMode::Dark, ColorMode::Light];
        let themes = self.theme_service.get_available_themes();
        let current_theme = self.theme_service.get_current_theme();
        let current_color_mode = self.theme_service.get_current_color_mode();

        *self.color_modes.write().unwrap() = color_modes.clone();
        *self.themes.write().unwrap() = themes.clone();
        *self.original_theme.write().unwrap() = current_theme.clone();
        *self.original_color_mode.write().unwrap() = current_color_mode.clone();

        // Set initial selections
        if let Some(pos) = color_modes.iter().position(|m| m == &current_color_mode) {
            self.color_mode_state.write().unwrap().select(Some(pos));
        }

        if let Some(pos) = themes.iter().position(|t| t.id == current_theme.id) {
            self.theme_state.write().unwrap().select(Some(pos));
        }

        Ok(())
    }

    fn handle_key_event(&self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Left | KeyCode::Char('h') => {
                let sections = SettingsSection::all();
                let current_section = *self.current_section.read().unwrap();
                let current_index = sections
                    .iter()
                    .position(|&s| s == current_section)
                    .unwrap_or(0);
                let new_index = if current_index == 0 {
                    sections.len() - 1
                } else {
                    current_index - 1
                };
                *self.current_section.write().unwrap() = sections[new_index];
                Ok(())
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let sections = SettingsSection::all();
                let current_section = *self.current_section.read().unwrap();
                let current_index = sections
                    .iter()
                    .position(|&s| s == current_section)
                    .unwrap_or(0);
                let new_index = (current_index + 1) % sections.len();
                *self.current_section.write().unwrap() = sections[new_index];
                Ok(())
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let current_section = *self.current_section.read().unwrap();
                match current_section {
                    SettingsSection::ColorMode => {
                        let mut color_mode_state = self.color_mode_state.write().unwrap();
                        let selected = color_mode_state.selected().unwrap_or(0);
                        if selected > 0 {
                            color_mode_state.select(Some(selected - 1));
                            drop(color_mode_state);
                            self.apply_current_selection();
                        }
                    }
                    SettingsSection::Theme => {
                        let mut theme_state = self.theme_state.write().unwrap();
                        let selected = theme_state.selected().unwrap_or(0);
                        if selected > 0 {
                            theme_state.select(Some(selected - 1));
                            drop(theme_state);
                            self.apply_current_selection();
                        }
                    }
                }
                Ok(())
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let current_section = *self.current_section.read().unwrap();
                match current_section {
                    SettingsSection::ColorMode => {
                        let mut color_mode_state = self.color_mode_state.write().unwrap();
                        let selected = color_mode_state.selected().unwrap_or(0);
                        let color_modes_len = self.color_modes.read().unwrap().len();
                        if selected < color_modes_len - 1 {
                            color_mode_state.select(Some(selected + 1));
                            drop(color_mode_state);
                            self.apply_current_selection();
                        }
                    }
                    SettingsSection::Theme => {
                        let mut theme_state = self.theme_state.write().unwrap();
                        let selected = theme_state.selected().unwrap_or(0);
                        let themes_len = self.themes.read().unwrap().len();
                        if selected < themes_len - 1 {
                            theme_state.select(Some(selected + 1));
                            drop(theme_state);
                            self.apply_current_selection();
                        }
                    }
                }
                Ok(())
            }
            KeyCode::Char(' ') => {
                self.save_settings();
                self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Esc => {
                self.revert_to_original();
                self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, f: &mut Frame) -> Result<()> {
        let colors = self.theme_service.get_colors();
        let area = f.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        self.render_tabs(f, chunks[0], &colors);
        self.render_content(f, chunks[1], &colors);
        self.render_footer(f, chunks[2], &colors);

        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl SettingsScreenInterface for SettingsScreen {}
