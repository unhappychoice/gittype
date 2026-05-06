use gittype::infrastructure::terminal::{TerminalComponent, TerminalInterface};

#[test]
fn terminal_component_get_returns_terminal() {
    let terminal = TerminalComponent::default().get();

    assert!(terminal.size().is_ok());
}
