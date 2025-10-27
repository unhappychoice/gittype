use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use shaku::{Component, Interface};
use std::io::{stdout, Stdout};

pub trait TerminalInterface: Interface {
    fn get(&self) -> Terminal<CrosstermBackend<Stdout>>;
}

#[derive(Component)]
#[shaku(interface = TerminalInterface)]
#[derive(Default)]
pub struct TerminalComponent {
    #[shaku(default)]
    _marker: (),
}

impl TerminalInterface for TerminalComponent {
    fn get(&self) -> Terminal<CrosstermBackend<Stdout>> {
        let backend = CrosstermBackend::new(stdout());
        Terminal::new(backend).expect("Failed to create terminal")
    }
}
