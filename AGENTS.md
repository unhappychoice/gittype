# Agent Guidelines

## Tests

- **Never write tests inside `src/`.** Do not add inline `#[cfg(test)] mod tests { ... }` blocks in source files. All tests live under the top-level `tests/` directory, mirroring the `src/` layout (e.g. tests for `src/foo/bar.rs` go in `tests/unit/foo/bar_tests.rs`).
- When a test needs access to a type's private internals, expose a `pub fn new_for_test(...)` constructor (or similar test helper) gated by `#[cfg(feature = "test-mocks")]` on the `src/` side, then construct it from `tests/`. The dev-dependency `gittype = { ..., features = ["test-mocks"] }` makes this code visible to the test crate.
- CI runs on Linux GitHub Actions runners with **no TTY**. Tests that construct a real `ratatui` terminal — directly via `CrosstermBackend::new(stdout())` or transitively through `TerminalComponent::get()` / `ScreenManagerFactory::create()` — panic with `Os { code: 11, kind: WouldBlock }`. Either guard such tests with `if !atty::is(atty::Stream::Stdout) { return; }` or use `ratatui::backend::TestBackend`.
