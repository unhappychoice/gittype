use gittype::infrastructure::terminal::{TerminalComponent, TerminalInterface};

#[test]
fn terminal_component_get_creates_terminal_backend() {
    if !atty::is(atty::Stream::Stdout) {
        return;
    }

    let mut terminal = TerminalComponent::default().get();
    let area = terminal.current_buffer_mut().area;

    assert_eq!(area.x, 0);
    assert_eq!(area.y, 0);
}

#[test]
fn terminal_component_get_returns_terminal() {
    if !atty::is(atty::Stream::Stdout) {
        return;
    }

    let terminal = TerminalComponent::default().get();

    assert!(terminal.size().is_ok());
}
