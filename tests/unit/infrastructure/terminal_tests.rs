use gittype::infrastructure::terminal::{TerminalComponent, TerminalInterface};

#[test]
fn terminal_component_get_returns_terminal() {
    if !atty::is(atty::Stream::Stdout) {
        return;
    }

    let terminal = TerminalComponent::default().get();

    assert!(terminal.size().is_ok());
}
