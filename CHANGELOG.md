# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### ✨ Features

- feat: use vendored OpenSSL for all targets to avoid cross-compilation issues ([2bc3269](https://github.com/unhappychoice/gittype/commit/2bc3269))

### 🐛 Bug Fixes

- fix(ci): improve Homebrew formula update process with awk ([7c0b626](https://github.com/unhappychoice/gittype/commit/7c0b626))

### 📝 Other Changes

- chore: bump version to v0.3.0 ([33f0df0](https://github.com/unhappychoice/gittype/commit/33f0df0))


## [0.3.0] - 2025-09-02

### ✨ Features

- feat: use vendored OpenSSL for all targets to avoid cross-compilation issues ([420f877](https://github.com/unhappychoice/gittype/commit/420f877))
- feat: update Homebrew to use prebuilt binaries and add ARM64 support ([96836dd](https://github.com/unhappychoice/gittype/commit/96836dd))
- feat: add repository management with GitHub support ([d08a93b](https://github.com/unhappychoice/gittype/commit/d08a93b))
- feat: add GitHub links to commit hashes in changelog ([5881873](https://github.com/unhappychoice/gittype/commit/5881873))
- feat: add automatic CHANGELOG.md generation ([7a2ae38](https://github.com/unhappychoice/gittype/commit/7a2ae38))

### 🐛 Bug Fixes

- fix(extractor): avoid progress updates in parallel path to satisfy Sync and pass clippy ([b8e433d](https://github.com/unhappychoice/gittype/commit/b8e433d))

### 📝 Other Changes

- chore: bump version to v0.3.0 ([806a529](https://github.com/unhappychoice/gittype/commit/806a529))
- perf(extractor): reuse thread-local tree-sitter parser and precompile glob patterns in scanner ([b5f84e0](https://github.com/unhappychoice/gittype/commit/b5f84e0))
- perf(extractor): parallelize challenge conversion and zen file processing ([55a2ac0](https://github.com/unhappychoice/gittype/commit/55a2ac0))
- chore: Fix cargo clippy warnings ([4cfedac](https://github.com/unhappychoice/gittype/commit/4cfedac))
- refactor: cleanup and module organization ([c2e14c0](https://github.com/unhappychoice/gittype/commit/c2e14c0))
- refactor: update extractor system for new step architecture ([8c46a02](https://github.com/unhappychoice/gittype/commit/8c46a02))
- refactor: improve error handling in main with Result chaining ([73b023d](https://github.com/unhappychoice/gittype/commit/73b023d))
- refactor: improve LoadingScreen with polymorphic step system ([c553e3a](https://github.com/unhappychoice/gittype/commit/c553e3a))


## [0.2.0] - 2025-09-02

### ✨ Features

- feat: implement C# code extraction with tree-sitter ([5573c2e](https://github.com/unhappychoice/gittype/commit/5573c2e))
- feat: add C# language support to core models ([acce492](https://github.com/unhappychoice/gittype/commit/acce492))
- feat: add tree-sitter-c-sharp dependency for C# language support ([ae20c9e](https://github.com/unhappychoice/gittype/commit/ae20c9e))
- feat: add JSX component extraction support to TypeScript parser ([077f87b](https://github.com/unhappychoice/gittype/commit/077f87b))
- feat: add JSX component extraction support to JavaScript parser ([8f56853](https://github.com/unhappychoice/gittype/commit/8f56853))
- feat: add JSX file extension support to JavaScript language detection ([6f9b9f4](https://github.com/unhappychoice/gittype/commit/6f9b9f4))
- feat: add comprehensive JSX/TSX support for React components ([9c914f3](https://github.com/unhappychoice/gittype/commit/9c914f3))
- feat: implement JavaScript language extractor ([0cc70ce](https://github.com/unhappychoice/gittype/commit/0cc70ce))
- feat: add JavaScript language support to Language enum ([2246c15](https://github.com/unhappychoice/gittype/commit/2246c15))
- feat: add tree-sitter-javascript dependency ([d15e81c](https://github.com/unhappychoice/gittype/commit/d15e81c))
- feat: add PHP language support ([140572d](https://github.com/unhappychoice/gittype/commit/140572d))
- feat: add Java language support ([9b2c16e](https://github.com/unhappychoice/gittype/commit/9b2c16e))
- feat: standardize language test formats and add Swift struct/enum support ([debd888](https://github.com/unhappychoice/gittype/commit/debd888))
- feat: add Kotlin language support ([e15de99](https://github.com/unhappychoice/gittype/commit/e15de99))
- feat: Add missing Ruby language constructs support ([6c91c27](https://github.com/unhappychoice/gittype/commit/6c91c27))
- feat: add missing Go language constructs support ([ac97d73](https://github.com/unhappychoice/gittype/commit/ac97d73))
- feat: add TypeScript interface, type alias, enum, and namespace support ([ccbac16](https://github.com/unhappychoice/gittype/commit/ccbac16))
- feat: add comprehensive Rust language constructs support ([df427e8](https://github.com/unhappychoice/gittype/commit/df427e8))
- feat: Comprehensive Swift language support validation ([245e1a1](https://github.com/unhappychoice/gittype/commit/245e1a1))
- feat: Add complete Swift extension block extraction ([8e45442](https://github.com/unhappychoice/gittype/commit/8e45442))
- feat: Add Swift extension method extraction support ([8f62748](https://github.com/unhappychoice/gittype/commit/8f62748))
- feat: Add Swift protocol extraction support ([db4dab5](https://github.com/unhappychoice/gittype/commit/db4dab5))
- feat: Add Swift language support ([a022f9f](https://github.com/unhappychoice/gittype/commit/a022f9f))
- feat: improve key guide color scheme for better UX ([fea01b3](https://github.com/unhappychoice/gittype/commit/fea01b3))

### 🐛 Bug Fixes

- fix: update homebrew formula update process in release workflow ([6105030](https://github.com/unhappychoice/gittype/commit/6105030))
- fix: add JavaScript comment support to extractor core ([f6ef381](https://github.com/unhappychoice/gittype/commit/f6ef381))
- fix: resolve clippy len_zero warning in PHP tests ([0da8aac](https://github.com/unhappychoice/gittype/commit/0da8aac))
- fix: resolve clippy warning about length comparison ([bb2710e](https://github.com/unhappychoice/gittype/commit/bb2710e))
- fix(sharing): prevent URL truncation in fallback display ([bc4e472](https://github.com/unhappychoice/gittype/commit/bc4e472))

### 📝 Other Changes

- chore: bump version to v0.2.0 ([9a9631c](https://github.com/unhappychoice/gittype/commit/9a9631c))
- docs: update documentation for C# language support ([28a4842](https://github.com/unhappychoice/gittype/commit/28a4842))
- test: add comprehensive C# language extraction tests ([1272c83](https://github.com/unhappychoice/gittype/commit/1272c83))
- test: enhance TSX/JSX tests to verify component extraction ([f3733e6](https://github.com/unhappychoice/gittype/commit/f3733e6))
- docs: update documentation for JavaScript language support ([a1ea4ed](https://github.com/unhappychoice/gittype/commit/a1ea4ed))
- test: update Language::from_extension tests for JavaScript ([9e0648b](https://github.com/unhappychoice/gittype/commit/9e0648b))
- test: add comprehensive JavaScript language tests ([42ed3d8](https://github.com/unhappychoice/gittype/commit/42ed3d8))
- docs: update documentation for PHP and Java language support ([bb150c8](https://github.com/unhappychoice/gittype/commit/bb150c8))
- style: format PHP implementation with cargo fmt ([fbab604](https://github.com/unhappychoice/gittype/commit/fbab604))
- style: apply rustfmt formatting to Java implementation ([4bc1e90](https://github.com/unhappychoice/gittype/commit/4bc1e90))
- test: add tests module declaration ([048e418](https://github.com/unhappychoice/gittype/commit/048e418))
- test: clean up existing test files ([d79fdb3](https://github.com/unhappychoice/gittype/commit/d79fdb3))
- test: reorganize language extraction tests ([f52f55c](https://github.com/unhappychoice/gittype/commit/f52f55c))
- refactor: remove obsolete extractor files ([8f26db3](https://github.com/unhappychoice/gittype/commit/8f26db3))
- refactor: update extractor module integration ([ece7d7d](https://github.com/unhappychoice/gittype/commit/ece7d7d))
- refactor: add modular language-specific parsers ([3840f61](https://github.com/unhappychoice/gittype/commit/3840f61))
- refactor: reorganize models into dedicated module ([772c2f8](https://github.com/unhappychoice/gittype/commit/772c2f8))
- refactor: add core extractor module ([183d530](https://github.com/unhappychoice/gittype/commit/183d530))
- docs: update documentation for Go language enhancements ([4ee79d6](https://github.com/unhappychoice/gittype/commit/4ee79d6))
- style: run cargo fmt ([ca6b163](https://github.com/unhappychoice/gittype/commit/ca6b163))
- revert: remove unnecessary README.md changes ([95b7618](https://github.com/unhappychoice/gittype/commit/95b7618))
- docs: update documentation for enhanced Rust language support ([c3d1417](https://github.com/unhappychoice/gittype/commit/c3d1417))
- style: run cargo fmt ([148840d](https://github.com/unhappychoice/gittype/commit/148840d))
- docs: Add Swift language support to documentation ([476a9dc](https://github.com/unhappychoice/gittype/commit/476a9dc))
- style: run cargo fmt ([533670c](https://github.com/unhappychoice/gittype/commit/533670c))
- style: fix clippy warnings and run cargo fmt ([0714d76](https://github.com/unhappychoice/gittype/commit/0714d76))
- docs(README): update features section with ranking system and game modes ([8bbab5d](https://github.com/unhappychoice/gittype/commit/8bbab5d))


## [0.1.3] - 2025-08-31

### 🐛 Bug Fixes

- fix(sharing): use game total score instead of single challenge score ([a19a504](https://github.com/unhappychoice/gittype/commit/a19a504))

### 📝 Other Changes

- chore: bump version to v0.1.3 ([af203ac](https://github.com/unhappychoice/gittype/commit/af203ac))
- style: run cargo fmt ([aa05d09](https://github.com/unhappychoice/gittype/commit/aa05d09))
- ci(release): pass tag commit SHA to Homebrew bump action as revision to fix --revision requirement ([16788c9](https://github.com/unhappychoice/gittype/commit/16788c9))


## [0.1.2] - 2025-08-31

### ✨ Features

- feat: add GitHub repository link to exit summary screen ([1237651](https://github.com/unhappychoice/gittype/commit/1237651))
- feat: improve stage result screen navigation ([bf00265](https://github.com/unhappychoice/gittype/commit/bf00265))
- feat: change back to title key from Enter to T ([9d9e93c](https://github.com/unhappychoice/gittype/commit/9d9e93c))

### 🐛 Bug Fixes

- fix(terminal): always pop keyboard enhancement flags on cleanup to avoid iTerm2 stuck state ([8725df9](https://github.com/unhappychoice/gittype/commit/8725df9))
- fix: resolve clippy unreachable code warning ([2f45ae7](https://github.com/unhappychoice/gittype/commit/2f45ae7))
- fix: resolve clippy warnings ([e3e244f](https://github.com/unhappychoice/gittype/commit/e3e244f))
- fix: ensure complete terminal cleanup on all exit paths ([511f914](https://github.com/unhappychoice/gittype/commit/511f914))
- fix: apply cargo fmt formatting fixes ([fe2409f](https://github.com/unhappychoice/gittype/commit/fe2409f))
- fix: restore enter symbol (↵) display in ratatui interface ([526caef](https://github.com/unhappychoice/gittype/commit/526caef))
- fix: improve terminal cleanup to prevent utf character output in iTerm2 ([36f0caf](https://github.com/unhappychoice/gittype/commit/36f0caf))
- fix: restore retry option in session summary screen ([2e3d29a](https://github.com/unhappychoice/gittype/commit/2e3d29a))
- fix: resolve clippy warnings and formatting issues ([50f3df2](https://github.com/unhappychoice/gittype/commit/50f3df2))
- fix: resolve clippy warnings and formatting issues ([3159237](https://github.com/unhappychoice/gittype/commit/3159237))
- fix: improve cursor highlight visibility in Mac terminal ([3ba6f92](https://github.com/unhappychoice/gittype/commit/3ba6f92))
- fix: remove REPORT_EVENT_TYPES flag to prevent double input in iTerm ([031ff4c](https://github.com/unhappychoice/gittype/commit/031ff4c))
- fix: resolve clippy warnings and formatting issues ([4eb02c9](https://github.com/unhappychoice/gittype/commit/4eb02c9))
- fix: remove debug output from exit summary screen ([300c0fe](https://github.com/unhappychoice/gittype/commit/300c0fe))
- fix: ensure raw mode is properly disabled on application exit ([8dd8772](https://github.com/unhappychoice/gittype/commit/8dd8772))
- fix: improve keyboard input handling for macOS iTerm ([d9cd8f1](https://github.com/unhappychoice/gittype/commit/d9cd8f1))
- fix: add revision parameter to homebrew bump formula action ([20316d7](https://github.com/unhappychoice/gittype/commit/20316d7))

### 📝 Other Changes

- chore: bump version to v0.1.2 ([3afc3de](https://github.com/unhappychoice/gittype/commit/3afc3de))
- docs: remove install commands from banner images ([361155c](https://github.com/unhappychoice/gittype/commit/361155c))
- style: apply rustfmt formatting ([49b759b](https://github.com/unhappychoice/gittype/commit/49b759b))
- refactor: remove unused Exit variant from ResultAction ([2455d5c](https://github.com/unhappychoice/gittype/commit/2455d5c))
- style: apply rustfmt formatting ([f558041](https://github.com/unhappychoice/gittype/commit/f558041))


## [0.1.1] - 2025-08-31

### ✨ Features

- feat: use current directory as default repository path ([4aef881](https://github.com/unhappychoice/gittype/commit/4aef881))
- feat: update title screen logo to oh-my-logo purple style ([90f553e](https://github.com/unhappychoice/gittype/commit/90f553e))
- feat: add package distribution infrastructure ([ee286d9](https://github.com/unhappychoice/gittype/commit/ee286d9))
- feat: add tier and overall ranking display to result screen ([1e57303](https://github.com/unhappychoice/gittype/commit/1e57303))
- feat: remove --max-lines option completely ([3ef85d8](https://github.com/unhappychoice/gittype/commit/3ef85d8))
- feat: add Go language support ([1baa6fc](https://github.com/unhappychoice/gittype/commit/1baa6fc))
- feat: add Ruby language support ([aa293da](https://github.com/unhappychoice/gittype/commit/aa293da))
- feat: improve typing screen UI/UX ([f89e6e8](https://github.com/unhappychoice/gittype/commit/f89e6e8))
- feat: add info dialog with GitHub and X links ([9d3a407](https://github.com/unhappychoice/gittype/commit/9d3a407))
- feat: enhance exit summary screen with session-based sharing ([86fced3](https://github.com/unhappychoice/gittype/commit/86fced3))
- feat: add comprehensive SNS sharing functionality ([5f110b1](https://github.com/unhappychoice/gittype/commit/5f110b1))
- feat: improve progress reporting for parallel AST parsing ([614b0ac](https://github.com/unhappychoice/gittype/commit/614b0ac))
- feat: implement parallel AST parsing with rayon ([48e754e](https://github.com/unhappychoice/gittype/commit/48e754e))
- feat: enhance game display with skip functionality and pause support ([0ccf826](https://github.com/unhappychoice/gittype/commit/0ccf826))
- feat: display total effort including partial attempts in session summary ([53b82c6](https://github.com/unhappychoice/gittype/commit/53b82c6))
- feat: separate completed and partial effort tracking in SessionSummary ([e583623](https://github.com/unhappychoice/gittype/commit/e583623))
- feat: improve loading screen with detailed progress and checkmarks ([f4d2cc7](https://github.com/unhappychoice/gittype/commit/f4d2cc7))
- feat: add session summary screen with comprehensive statistics ([0cf2a44](https://github.com/unhappychoice/gittype/commit/0cf2a44))
- feat: add typing animation with colored messages and skip functionality ([ec3e571](https://github.com/unhappychoice/gittype/commit/ec3e571))
- feat: add retry functionality to result screen ([c3ed536](https://github.com/unhappychoice/gittype/commit/c3ed536))
- feat: refactor result display and add ASCII rank titles ([a6e1bd1](https://github.com/unhappychoice/gittype/commit/a6e1bd1))
- feat: add large ASCII score display to result screens ([19679ce](https://github.com/unhappychoice/gittype/commit/19679ce))
- feat: add Wild difficulty level and refactor character limits ([a76f935](https://github.com/unhappychoice/gittype/commit/a76f935))
- feat: implement comprehensive scoring and metrics system ([a7b887f](https://github.com/unhappychoice/gittype/commit/a7b887f))
- feat: add TypeScript arrow function support and improve challenge system ([6c3527f](https://github.com/unhappychoice/gittype/commit/6c3527f))
- feat: implement startup loading screen with progress visualization ([48dbabd](https://github.com/unhappychoice/gittype/commit/48dbabd))
- feat: Enhance typing game engine with advanced features ([aeb8267](https://github.com/unhappychoice/gittype/commit/aeb8267))
- feat: Implement AST-based code extraction with gitignore support ([423d865](https://github.com/unhappychoice/gittype/commit/423d865))
- feat: Add dependencies for code extraction and game engine ([7125ed2](https://github.com/unhappychoice/gittype/commit/7125ed2))
- feat: Update main.rs to use new modular architecture ([a3b6295](https://github.com/unhappychoice/gittype/commit/a3b6295))
- feat: Add StageManager for multi-stage gameplay ([261dd24](https://github.com/unhappychoice/gittype/commit/261dd24))
- feat: Add modular screen system ([2d1b27d](https://github.com/unhappychoice/gittype/commit/2d1b27d))
- feat: Add Challenge structure for code typing tasks ([3dbaa70](https://github.com/unhappychoice/gittype/commit/3dbaa70))
- feat: Add core game modules for text processing and display ([0539259](https://github.com/unhappychoice/gittype/commit/0539259))
- feat: Set up project structure and tech stack ([7f40ba7](https://github.com/unhappychoice/gittype/commit/7f40ba7))

### 🐛 Bug Fixes

- fix: add contents write permission to release workflow ([901942f](https://github.com/unhappychoice/gittype/commit/901942f))
- fix: adjust total_content_height calculation for proper layout ([ecbbc98](https://github.com/unhappychoice/gittype/commit/ecbbc98))
- fix: format code with cargo fmt ([e52e4d7](https://github.com/unhappychoice/gittype/commit/e52e4d7))
- fix: format code with cargo fmt ([0ab8bee](https://github.com/unhappychoice/gittype/commit/0ab8bee))
- fix: format code with cargo fmt ([88987cc](https://github.com/unhappychoice/gittype/commit/88987cc))
- fix: format code with cargo fmt ([0f2f259](https://github.com/unhappychoice/gittype/commit/0f2f259))
- fix: resolve remaining clippy warnings for CI compliance ([d94acd6](https://github.com/unhappychoice/gittype/commit/d94acd6))
- fix: apply code formatting and clippy auto-fixes ([0cb638a](https://github.com/unhappychoice/gittype/commit/0cb638a))
- fix: resolve compilation errors to enable parallel AST parsing ([cc16b41](https://github.com/unhappychoice/gittype/commit/cc16b41))
- fix: improve Ctrl+C handling to show session summary ([8e78a75](https://github.com/unhappychoice/gittype/commit/8e78a75))
- fix: improve git repository path recognition ([c723f8c](https://github.com/unhappychoice/gittype/commit/c723f8c))
- fix: remove debug code from main.rs ([31e1f8c](https://github.com/unhappychoice/gittype/commit/31e1f8c))
- fix: resolve forced termination after loading completion ([c56ceef](https://github.com/unhappychoice/gittype/commit/c56ceef))
- fix: Improve AST comment detection and position mapping ([40964df](https://github.com/unhappychoice/gittype/commit/40964df))
- fix: Prevent input processing during countdown screen ([06e128d](https://github.com/unhappychoice/gittype/commit/06e128d))

### 📝 Other Changes

- chore: bump version to v0.1.1 ([63b5358](https://github.com/unhappychoice/gittype/commit/63b5358))
- docs: update documentation for Go and Ruby language support ([26a79de](https://github.com/unhappychoice/gittype/commit/26a79de))
- ci: split CI jobs into format, clippy, and test ([9ac35e5](https://github.com/unhappychoice/gittype/commit/9ac35e5))
- chore: remove coverage files and add them to .gitignore ([b79bb20](https://github.com/unhappychoice/gittype/commit/b79bb20))
- ci: add Codecov integration for code coverage tracking ([2224def](https://github.com/unhappychoice/gittype/commit/2224def))
- ci: add GitHub Actions workflow for automated testing ([34fa071](https://github.com/unhappychoice/gittype/commit/34fa071))
- ux: improve keyboard operation consistency ([ee18587](https://github.com/unhappychoice/gittype/commit/ee18587))
- docs: create comprehensive documentation and banner ([2744285](https://github.com/unhappychoice/gittype/commit/2744285))
- refactor: restructure UI organization and rename loading components ([fc0b9c0](https://github.com/unhappychoice/gittype/commit/fc0b9c0))
- refactor: remove unused LoadingProgress struct ([b6996e6](https://github.com/unhappychoice/gittype/commit/b6996e6))
- refactor: remove unused loading components ([5353ca3](https://github.com/unhappychoice/gittype/commit/5353ca3))
- test: Restructure tests into separate unit and integration files ([be2c013](https://github.com/unhappychoice/gittype/commit/be2c013))
- refactor: Remove legacy engine.rs ([4d86a89](https://github.com/unhappychoice/gittype/commit/4d86a89))
- refactor: Update module structure and exports ([3ba422b](https://github.com/unhappychoice/gittype/commit/3ba422b))
- deps: Add ctrlc dependency for signal handling ([7fef3b0](https://github.com/unhappychoice/gittype/commit/7fef3b0))
- chore: Add .gitignore file ([12fe0fb](https://github.com/unhappychoice/gittype/commit/12fe0fb))
- chore: First commit ([f7ec3ca](https://github.com/unhappychoice/gittype/commit/f7ec3ca))


