# GitType Architecture

This document describes the overall architecture and design decisions of GitType, a CLI typing game that uses source code as practice material.

## Table of Contents

- [Overview](#overview)
- [Architecture Patterns](#architecture-patterns)
- [Core Modules](#core-modules)
- [Data Flow](#data-flow)
- [Key Components](#key-components)
- [External Dependencies](#external-dependencies)
- [Design Decisions](#design-decisions)
- [Performance Considerations](#performance-considerations)

---

## Overview

GitType follows a modular architecture with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────────────┐
│                             CLI                                 │
├─────────────────────────────────────────────────────────────────┤
│                         Game Engine                             │
├─────────────────────────────────────────────────────────────────┤
│     Extractor    │    Scoring     │   Storage   │   Sharing    │
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
- `extractor`: Code extraction and parsing
- `game`: Game mechanics and UI
- `scoring`: Performance metrics and calculation
- `storage`: Data persistence and history
- `cli`: Command-line interface and configuration

### Repository Pattern
The `RepositoryLoader` abstracts file system access, making it easy to:
- Test with mock data
- Support different source types (local files, git repos)
- Add filtering and preprocessing

### Strategy Pattern
Language-specific extraction is handled through the `Language` enum and associated parsing strategies, allowing easy extension for new programming languages.

---

## Core Modules

### 1. CLI Module (`src/cli/`)

**Purpose**: Command-line interface and configuration management

```rust
pub struct Config {
    pub repo_path: PathBuf,
    pub languages: Vec<Language>,
    pub max_lines: usize,
    pub stages: usize,
    // ...
}
```

**Key Components**:
- `config.rs`: Configuration parsing and validation
- Command-line argument parsing with `clap`
- Configuration file support

### 2. Extractor Module (`src/extractor/`)

**Purpose**: Extract code chunks from source repositories

**Architecture**:
```
RepositoryLoader -> Parser -> CodeChunk -> ChallengeConverter
```

**Key Components**:
- `repository_loader.rs`: File discovery and filtering
- `parser.rs`: Tree-sitter based code parsing
- `chunk.rs`: Code chunk representation and metadata
- `language.rs`: Language detection and configuration
- `challenge_converter.rs`: Convert chunks to typing challenges

**Data Flow**:
1. `RepositoryLoader` discovers and filters files
2. `CodeExtractor` parses files using tree-sitter
3. `CodeChunk` objects are created with metadata
4. `ChallengeConverter` transforms chunks into typing challenges

### 3. Game Module (`src/game/`)

**Purpose**: Game mechanics, UI, and user interaction

**Architecture**:
```
StageManager -> Screen -> Display -> User Input
     ↓
SessionTracker -> Scoring -> Storage
```

**Key Components**:
- `stage_manager.rs`: Orchestrates game flow and state
- `screens/`: Different UI screens (title, typing, results, etc.)
- `display.rs`: Terminal UI rendering with ratatui
- `challenge.rs`: Individual typing challenge logic
- `session_tracker.rs`: Track user progress through sessions

**Screen Flow**:
```
TitleScreen -> LoadingScreen -> CountdownScreen -> TypingScreen -> ResultScreen
                                        ↑__________________|
```

### 4. Scoring Module (`src/scoring/`)

**Purpose**: Calculate performance metrics

```rust
pub struct TypingMetrics {
    pub accuracy: f64,
    pub words_per_minute: f64,
    pub characters_per_minute: f64,
    pub mistakes: usize,
    pub total_time: Duration,
}
```

**Key Components**:
- `engine.rs`: Real-time scoring calculation
- `metrics.rs`: Performance metric definitions
- `ranking_title.rs`: Rank calculation and titles

### 5. Storage Module (`src/storage/`)

**Purpose**: Data persistence and session history

**Components**:
- `database.rs`: SQLite database management
- `history.rs`: Session history and statistics
- Local storage for user progress and analytics

### 6. Sharing Module (`src/sharing.rs`)

**Purpose**: Share results and statistics

**Features**:
- Export session data
- Generate shareable statistics
- Integration with external services (planned)

---

## Data Flow

### 1. Initialization Flow

```
CLI Args -> Config -> RepositoryLoader -> CodeExtractor
                ↓
            FileDiscovery -> TreeSitter -> CodeChunks
                ↓
            ChallengeConverter -> GameChallenges
```

### 2. Game Loop Flow

```
User Input -> TypingScreen -> ScoringEngine -> RealTimeMetrics
     ↓              ↓              ↓
StageManager -> SessionTracker -> Database
     ↓
ResultScreen -> NextChallenge/GameEnd
```

### 3. Storage Flow

```
SessionData -> Database -> History -> Analytics
                ↓
            Export -> JSON/CSV
```

---

## Key Components

### CodeChunk

Represents a piece of extracted code with metadata:

```rust
pub struct CodeChunk {
    pub content: String,
    pub chunk_type: ChunkType,
    pub file_path: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
    pub language: Language,
}

pub enum ChunkType {
    Function,
    Class,
    Method,
    Struct,
    // ...
}
```

### Challenge

Represents a typing challenge created from a code chunk:

```rust
pub struct Challenge {
    pub id: String,
    pub content: String,
    pub source_info: SourceInfo,
    pub difficulty: DifficultyLevel,
}
```

### StageManager

Orchestrates the game flow and manages state transitions:

```rust
pub struct StageManager {
    pub current_stage: usize,
    pub challenges: Vec<Challenge>,
    pub session_tracker: SessionTracker,
    pub stage_config: StageConfig,
}
```

### Display System

Multi-layered rendering system:
- `display.rs`: Abstract display interface
- `display_ratatui.rs`: Terminal UI implementation
- `display_optimized.rs`: Performance-optimized rendering

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
| Ruby | `tree-sitter-ruby` | ✅ Full support |

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