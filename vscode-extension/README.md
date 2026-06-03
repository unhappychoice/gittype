# GitType Practice

GitType Practice is a VS Code extension MVP for practicing selected code with a GitType-style typing flow.

## MVP Commands

- `GitType: Practice Selection`: opens the current editor selection as a `gittype-practice:` virtual document.
- `GitType: Restart Practice`: restarts the active practice session.
- `GitType: Toggle Practice Mode`: switches between strict and flow modes.

Strict mode blocks progress on incorrect input. Flow mode records mistakes and allows the session to continue.

## Repository Snippets

Repository-wide extraction is planned for a later phase through a CLI-backed snippet provider. The extension already uses a `SnippetProvider` shape so a future provider can call `gittype snippets --format json` and open the selected result through the same practice document surface.
