# GitType Architecture

This document describes the overall architecture and design decisions of GitType, a CLI typing game that uses source code as practice material.

## Table of Contents

- [Overview](#overview)
- [Architecture Patterns](#architecture-patterns)
- [Core Modules](#core-modules)
- [External Dependencies](#external-dependencies)
- [Design Decisions](#design-decisions)

---

## Overview

GitType follows a modular architecture with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────────────┐
│                             CLI                                 │
├─────────────────────────────────────────────────────────────────┤
│                         Game Engine                             │
├─────────────────────────────────────────────────────────────────┤
│ Models │ Extractor │ Scoring │ Storage │ Sharing │
├─────────────────────────────────────────────────────────────────┤
│              External Dependencies (tree-sitter, etc.)         │
└─────────────────────────────────────────────────────────────────┘
```

### Design Principles

- **Modularity**: Each component has a single responsibility
- **Testability**: Components are easily unit tested in isolation
- **Performance**: Async processing and parallel parsing where beneficial
- **Extensibility**: Easy to add new languages and features
- **User Experience**: Responsive UI with real-time feedback

---

## Architecture Patterns

### Module Pattern
Each major feature area is organized as a separate module with clear public APIs:
- `cli`: Command-line interface and configuration.
- `models`: Core data structures for the application (Challenge, Session, etc.).
- `extractor`: Code extraction and parsing from repositories.
- `game`: Game mechanics, state management, and UI rendering.
- `scoring`: Performance metrics calculation and tracking.
- `storage`: Data persistence, session history, and database management.
- `sharing`: Exporting and sharing session data.

### Repository Pattern
The storage layer uses a repository pattern to abstract database access, making it easier to manage data entities and test business logic.

### Strategy Pattern
Language-specific extraction is handled through a strategy pattern, allowing easy extension for new programming languages using tree-sitter.

---

## Core Modules

### 1. CLI Module (`src/cli/`)
**Purpose**: Handles command-line argument parsing, configuration, and dispatching commands. It serves as the main entry point for user interaction.

### 2. Models Module (`src/models/`)
**Purpose**: Defines the core data structures used throughout the application, such as `Challenge`, `Chunk`, `Session`, and `Stage`. This module ensures a consistent data model across different parts of the system.

### 3. Extractor Module (`src/extractor/`)
**Purpose**: Responsible for finding and parsing source code files from a given repository. It uses `tree-sitter` to analyze the code and extract meaningful chunks (like functions and classes) that can be converted into typing challenges.

### 4. Game Module (`src/game/`)
**Purpose**: Manages the entire game lifecycle, including the title screen, loading, countdown, the typing challenge itself, and results screens. It handles user input, manages game state, and renders the UI to the terminal.

### 5. Scoring Module (`src/scoring/`)
**Purpose**: Calculates and tracks user performance. It is divided into sub-modules for real-time scoring during a typing session (`calculator`) and for tracking statistics across stages and sessions (`tracker`).

### 6. Storage Module (`src/storage/`)
**Purpose**: Manages data persistence using SQLite. It uses a repository pattern (`repositories`) and DAOs (`daos`) to handle the storage and retrieval of session history, user statistics, and repository metadata. It also includes database migrations.

### 7. Sharing Module (`src/sharing.rs`)
**Purpose**: Provides functionality to share or export user results and session data.

---

## External Dependencies

### Core Dependencies

| Dependency | Purpose | Usage |
|------------|---------|-------|
| `tree-sitter` | Code parsing | Extract functions, classes from source files |
| `ratatui` | Terminal UI | Render game interface |
| `crossterm` | Terminal control | Handle input, colors, cursor positioning |
| `clap` | CLI parsing | Command-line argument handling |
| `rusqlite` | Database | Store session history and statistics |
| `rayon` | Parallelism | Parallel file processing |

### Language-Specific

| Language | Tree-sitter Grammar | Status |
|----------|-------------------|---------|
| Rust | `tree-sitter-rust` | ✅ Full support |
| TypeScript | `tree-sitter-typescript` | ✅ Full support |
| Python | `tree-sitter-python` | ✅ Full support |
| Go | `tree-sitter-go` | ✅ Full support |
| Ruby | `tree-sitter-ruby` | ✅ Full support (includes class methods, singleton methods, attr_accessor) |
| Swift | `tree-sitter-swift` | ✅ Full support |

---

## Design Decisions

### 1. Tree-sitter for Code Parsing

**Why**: Provides accurate, language-aware parsing that understands code structure rather than just text patterns.

**Benefits**:
- Consistent extraction across languages
- Accurate function/class boundaries
- Syntax highlighting support
- Extensible to new languages

### 2. Terminal UI with Ratatui

**Why**: Provides rich, responsive terminal interface without requiring GUI dependencies.

**Benefits**:
- Cross-platform compatibility
- Low resource usage
- Professional appearance
- Real-time updates

### 3. SQLite for Storage

**Why**: Local, file-based database that requires no setup.

**Benefits**:
- No external database server needed
- ACID transactions
- SQL query capabilities
- Portable data files

### 4. Modular Architecture

**Why**: Separation of concerns makes the codebase maintainable and testable.

**Benefits**:
- Easy to add new features
- Component-level testing
- Clear interfaces
- Parallel development

---

## Performance Considerations

### 1. Parallel Processing

```rust
// Parallel file processing with rayon
files.par_iter()
    .map(|file| extract_chunks(file))
    .collect()
```

**Benefits**:
- Faster repository scanning
- Efficient multi-core usage
- Better user experience for large codebases

### 2. Lazy Loading

- Code chunks are processed on-demand
- UI screens are rendered only when needed
- Database connections are managed efficiently

### 3. Memory Management

- Streaming file processing for large repositories
- Efficient string handling for code content
- Bounded memory usage regardless of repository size

### 4. Caching Strategy

- Parsed code chunks can be cached between sessions
- Git repository metadata is cached
- Display rendering optimizations

---

## Extension Points

### Adding New Languages

1. Add tree-sitter grammar dependency
2. Implement language-specific queries
3. Update `Language` enum
4. Add language detection logic
5. Include tests and documentation

### Adding New Features

1. **Game Modes**: Extend `GameMode` enum and `StageBuilder`
2. **Scoring Systems**: Add new metrics to `ScoringEngine`
3. **Display Options**: Implement new `Display` trait variants
4. **Export Formats**: Extend `sharing` module

### Adding New Screens

1. Implement `Screen` trait
2. Add to `screens` module
3. Update `StageManager` flow
4. Add navigation logic

---

## Testing Strategy

### Unit Tests
- Each module has comprehensive unit tests
- Mock dependencies for isolated testing
- Property-based testing for parsers

### Integration Tests
- End-to-end CLI testing
- Database integration tests
- Multi-language extraction tests

### Performance Tests
- Benchmarks for parsing large codebases
- Memory usage profiling
- UI responsiveness testing

---

## Future Architecture Considerations

### Planned Improvements

1. **Plugin System**: Allow external language support
2. **Remote Storage**: Cloud-based session synchronization
3. **Multi-Player**: Real-time competitive typing
4. **Analytics**: Advanced performance insights
5. **Web Interface**: Browser-based version

### Scalability

- Modular design supports horizontal feature additions
- Clear interfaces enable component replacement
- Performance optimizations can be added incrementally

---

This architecture provides a solid foundation for GitType's current features while maintaining flexibility for future enhancements.