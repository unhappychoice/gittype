use crate::presentation::ui::Colors;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding},
    Frame,
};

const SUPPORTED_LANGUAGES: &[(&str, &str)] = &[
    ("C", "C"),
    ("C#", "C#"),
    ("C++", "C++"),
    ("Dart", "Dart"),
    ("Elixir", "Elixir"),
    ("Erlang", "Erlang"),
    ("Go", "Go"),
    ("Haskell", "Haskell"),
    ("Java", "Java"),
    ("JavaScript", "JavaScript"),
    ("Kotlin", "Kotlin"),
    ("PHP", "PHP"),
    ("Python", "Python"),
    ("Ruby", "Ruby"),
    ("Rust", "Rust"),
    ("Scala", "Scala"),
    ("Swift", "Swift"),
    ("TypeScript", "TypeScript"),
];

pub struct LanguageListView;

impl LanguageListView {
    pub fn render(frame: &mut Frame, area: Rect, list_state: &mut ListState, colors: &Colors) {
        let items: Vec<ListItem> = SUPPORTED_LANGUAGES
            .iter()
            .enumerate()
            .map(|(i, (display_name, _))| {
                let line_spans = vec![
                    Span::styled(
                        format!("{:2}. ", i + 1),
                        Style::default().fg(colors.text_secondary()),
                    ),
                    Span::styled(
                        format!("{:<20}", display_name),
                        Style::default()
                            .fg(colors.text())
                            .add_modifier(Modifier::BOLD),
                    ),
                ];

                ListItem::new(Line::from(line_spans))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border()))
                    .title("Programming Languages")
                    .title_style(
                        Style::default()
                            .fg(colors.text())
                            .add_modifier(Modifier::BOLD),
                    )
                    .padding(Padding::uniform(1)),
            )
            .style(Style::default().fg(colors.text()))
            .highlight_style(
                Style::default()
                    .bg(colors.background_secondary())
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_stateful_widget(list, area, list_state);
    }

    pub fn get_language_code(index: usize) -> Option<&'static str> {
        SUPPORTED_LANGUAGES.get(index).map(|(_, code)| *code)
    }

    pub fn languages_count() -> usize {
        SUPPORTED_LANGUAGES.len()
    }
}
