# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-09-02

### ✨ Features

- feat: implement C# code extraction with tree-sitter (5573c2e)
- feat: add C# language support to core models (acce492)
- feat: add tree-sitter-c-sharp dependency for C# language support (ae20c9e)
- feat: add JSX component extraction support to TypeScript parser (077f87b)
- feat: add JSX component extraction support to JavaScript parser (8f56853)
- feat: add JSX file extension support to JavaScript language detection (6f9b9f4)
- feat: add comprehensive JSX/TSX support for React components (9c914f3)
- feat: implement JavaScript language extractor (0cc70ce)
- feat: add JavaScript language support to Language enum (2246c15)
- feat: add tree-sitter-javascript dependency (d15e81c)
- feat: add PHP language support (140572d)
- feat: add Java language support (9b2c16e)
- feat: standardize language test formats and add Swift struct/enum support (debd888)
- feat: add Kotlin language support (e15de99)
- feat: Add missing Ruby language constructs support (6c91c27)
- feat: add missing Go language constructs support (ac97d73)
- feat: add TypeScript interface, type alias, enum, and namespace support (ccbac16)
- feat: add comprehensive Rust language constructs support (df427e8)
- feat: Comprehensive Swift language support validation (245e1a1)
- feat: Add complete Swift extension block extraction (8e45442)
- feat: Add Swift extension method extraction support (8f62748)
- feat: Add Swift protocol extraction support (db4dab5)
- feat: Add Swift language support (a022f9f)
- feat: improve key guide color scheme for better UX (fea01b3)

### 🐛 Bug Fixes

- fix: update homebrew formula update process in release workflow (6105030)
- fix: add JavaScript comment support to extractor core (f6ef381)
- fix: resolve clippy len_zero warning in PHP tests (0da8aac)
- fix: resolve clippy warning about length comparison (bb2710e)
- fix(sharing): prevent URL truncation in fallback display (bc4e472)

### 📝 Other Changes

- chore: bump version to v0.2.0 (9a9631c)
- docs: update documentation for C# language support (28a4842)
- test: add comprehensive C# language extraction tests (1272c83)
- test: enhance TSX/JSX tests to verify component extraction (f3733e6)
- docs: update documentation for JavaScript language support (a1ea4ed)
- test: update Language::from_extension tests for JavaScript (9e0648b)
- test: add comprehensive JavaScript language tests (42ed3d8)
- docs: update documentation for PHP and Java language support (bb150c8)
- style: format PHP implementation with cargo fmt (fbab604)
- style: apply rustfmt formatting to Java implementation (4bc1e90)
- test: add tests module declaration (048e418)
- test: clean up existing test files (d79fdb3)
- test: reorganize language extraction tests (f52f55c)
- refactor: remove obsolete extractor files (8f26db3)
- refactor: update extractor module integration (ece7d7d)
- refactor: add modular language-specific parsers (3840f61)
- refactor: reorganize models into dedicated module (772c2f8)
- refactor: add core extractor module (183d530)
- docs: update documentation for Go language enhancements (4ee79d6)
- style: run cargo fmt (ca6b163)
- revert: remove unnecessary README.md changes (95b7618)
- docs: update documentation for enhanced Rust language support (c3d1417)
- style: run cargo fmt (148840d)
- docs: Add Swift language support to documentation (476a9dc)
- style: run cargo fmt (533670c)
- style: fix clippy warnings and run cargo fmt (0714d76)
- docs(README): update features section with ranking system and game modes (8bbab5d)


## [0.1.3] - 2025-08-31

### 🐛 Bug Fixes

- fix(sharing): use game total score instead of single challenge score (a19a504)

### 📝 Other Changes

- chore: bump version to v0.1.3 (af203ac)
- style: run cargo fmt (aa05d09)
- ci(release): pass tag commit SHA to Homebrew bump action as revision to fix --revision requirement (16788c9)


## [0.1.2] - 2025-08-31

### ✨ Features

- feat: add GitHub repository link to exit summary screen (1237651)
- feat: improve stage result screen navigation (bf00265)
- feat: change back to title key from Enter to T (9d9e93c)

### 🐛 Bug Fixes

- fix(terminal): always pop keyboard enhancement flags on cleanup to avoid iTerm2 stuck state (8725df9)
- fix: resolve clippy unreachable code warning (2f45ae7)
- fix: resolve clippy warnings (e3e244f)
- fix: ensure complete terminal cleanup on all exit paths (511f914)
- fix: apply cargo fmt formatting fixes (fe2409f)
- fix: restore enter symbol (↵) display in ratatui interface (526caef)
- fix: improve terminal cleanup to prevent utf character output in iTerm2 (36f0caf)
- fix: restore retry option in session summary screen (2e3d29a)
- fix: resolve clippy warnings and formatting issues (50f3df2)
- fix: resolve clippy warnings and formatting issues (3159237)
- fix: improve cursor highlight visibility in Mac terminal (3ba6f92)
- fix: remove REPORT_EVENT_TYPES flag to prevent double input in iTerm (031ff4c)
- fix: resolve clippy warnings and formatting issues (4eb02c9)
- fix: remove debug output from exit summary screen (300c0fe)
- fix: ensure raw mode is properly disabled on application exit (8dd8772)
- fix: improve keyboard input handling for macOS iTerm (d9cd8f1)
- fix: add revision parameter to homebrew bump formula action (20316d7)

### 📝 Other Changes

- chore: bump version to v0.1.2 (3afc3de)
- docs: remove install commands from banner images (361155c)
- style: apply rustfmt formatting (49b759b)
- refactor: remove unused Exit variant from ResultAction (2455d5c)
- style: apply rustfmt formatting (f558041)


## [0.1.1] - 2025-08-31

### ✨ Features

- feat: use current directory as default repository path (4aef881)
- feat: update title screen logo to oh-my-logo purple style (90f553e)
- feat: add package distribution infrastructure (ee286d9)
- feat: add tier and overall ranking display to result screen (1e57303)
- feat: remove --max-lines option completely (3ef85d8)
- feat: add Go language support (1baa6fc)
- feat: add Ruby language support (aa293da)
- feat: improve typing screen UI/UX (f89e6e8)
- feat: add info dialog with GitHub and Twitter links (9d3a407)
- feat: enhance exit summary screen with session-based sharing (86fced3)
- feat: add comprehensive SNS sharing functionality (5f110b1)
- feat: improve progress reporting for parallel AST parsing (614b0ac)
- feat: implement parallel AST parsing with rayon (48e754e)
- feat: enhance game display with skip functionality and pause support (0ccf826)
- feat: display total effort including partial attempts in session summary (53b82c6)
- feat: separate completed and partial effort tracking in SessionSummary (e583623)
- feat: improve loading screen with detailed progress and checkmarks (f4d2cc7)
- feat: add session summary screen with comprehensive statistics (0cf2a44)
- feat: add typing animation with colored messages and skip functionality (ec3e571)
- feat: add retry functionality to result screen (c3ed536)
- feat: refactor result display and add ASCII rank titles (a6e1bd1)
- feat: add large ASCII score display to result screens (19679ce)
- feat: add Wild difficulty level and refactor character limits (a76f935)
- feat: implement comprehensive scoring and metrics system (a7b887f)
- feat: add TypeScript arrow function support and improve challenge system (6c3527f)
- feat: implement startup loading screen with progress visualization (48dbabd)
- feat: Enhance typing game engine with advanced features (aeb8267)
- feat: Implement AST-based code extraction with gitignore support (423d865)
- feat: Add dependencies for code extraction and game engine (7125ed2)
- feat: Update main.rs to use new modular architecture (a3b6295)
- feat: Add StageManager for multi-stage gameplay (261dd24)
- feat: Add modular screen system (2d1b27d)
- feat: Add Challenge structure for code typing tasks (3dbaa70)
- feat: Add core game modules for text processing and display (0539259)
- feat: Set up project structure and tech stack (7f40ba7)

### 🐛 Bug Fixes

- fix: add contents write permission to release workflow (901942f)
- fix: adjust total_content_height calculation for proper layout (ecbbc98)
- fix: format code with cargo fmt (e52e4d7)
- fix: format code with cargo fmt (0ab8bee)
- fix: format code with cargo fmt (88987cc)
- fix: format code with cargo fmt (0f2f259)
- fix: resolve remaining clippy warnings for CI compliance (d94acd6)
- fix: apply code formatting and clippy auto-fixes (0cb638a)
- fix: resolve compilation errors to enable parallel AST parsing (cc16b41)
- fix: improve Ctrl+C handling to show session summary (8e78a75)
- fix: improve git repository path recognition (c723f8c)
- fix: remove debug code from main.rs (31e1f8c)
- fix: resolve forced termination after loading completion (c56ceef)
- fix: Improve AST comment detection and position mapping (40964df)
- fix: Prevent input processing during countdown screen (06e128d)

### 📝 Other Changes

- chore: bump version to v0.1.1 (63b5358)
- docs: update documentation for Go and Ruby language support (26a79de)
- ci: split CI jobs into format, clippy, and test (9ac35e5)
- chore: remove coverage files and add them to .gitignore (b79bb20)
- ci: add Codecov integration for code coverage tracking (2224def)
- ci: add GitHub Actions workflow for automated testing (34fa071)
- ux: improve keyboard operation consistency (ee18587)
- docs: create comprehensive documentation and banner (2744285)
- refactor: restructure UI organization and rename loading components (fc0b9c0)
- refactor: remove unused LoadingProgress struct (b6996e6)
- refactor: remove unused loading components (5353ca3)
- test: Restructure tests into separate unit and integration files (be2c013)
- refactor: Remove legacy engine.rs (4d86a89)
- refactor: Update module structure and exports (3ba422b)
- deps: Add ctrlc dependency for signal handling (7fef3b0)
- chore: Add .gitignore file (12fe0fb)
- chore: First commit (f7ec3ca)


