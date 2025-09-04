# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### ✨ Features

- feat(typing): add context lines display and preserve indentation ([4fc40e3](https://github.com/unhappychoice/gittype/commit/4fc40e3))
- feat(typing): improve color scheme and UI elements ([1c64bbc](https://github.com/unhappychoice/gittype/commit/1c64bbc))
- feat(countdown): enhance visual experience with ASCII art and improved colors ([7a8c0f8](https://github.com/unhappychoice/gittype/commit/7a8c0f8))
- feat: add CODEOWNERS file with @unhappychoice as global owner ([f5d4b33](https://github.com/unhappychoice/gittype/commit/f5d4b33))
- feat: add dependabot configuration for daily Cargo updates ([06687e4](https://github.com/unhappychoice/gittype/commit/06687e4))
- feat(stage-manager): integrate TotalTracker for comprehensive session tracking ([749f6a7](https://github.com/unhappychoice/gittype/commit/749f6a7))
- feat(tracking): add TotalTracker for game-wide statistics ([94f6f2a](https://github.com/unhappychoice/gittype/commit/94f6f2a))
- feat(ui): implement comment highlighting in stage renderer ([ae03906](https://github.com/unhappychoice/gittype/commit/ae03906))
- feat(typing-core): add display_comment_ranges method ([929b5e4](https://github.com/unhappychoice/gittype/commit/929b5e4))
- feat(tests): unify typing core tests with parser-based comment detection ([58a1a10](https://github.com/unhappychoice/gittype/commit/58a1a10))
- feat(extractor): make extract_comment_ranges public ([c92c3a8](https://github.com/unhappychoice/gittype/commit/c92c3a8))
- feat(display): improve newline symbol display and positioning ([e6afae6](https://github.com/unhappychoice/gittype/commit/e6afae6))
- feat(typing): preserve empty lines in all challenges ([452874f](https://github.com/unhappychoice/gittype/commit/452874f))
- feat(text-processing): add empty line preservation option ([db73ac3](https://github.com/unhappychoice/gittype/commit/db73ac3))
- feat: include Zen challenges in main challenge loading ([cacf6c0](https://github.com/unhappychoice/gittype/commit/cacf6c0))
- feat(ui): improve typing screen layout with blue theme and line numbers ([5a345e7](https://github.com/unhappychoice/gittype/commit/5a345e7))
- feat(ui): add real-time metrics updates during typing ([9ba9561](https://github.com/unhappychoice/gittype/commit/9ba9561))
- feat(models): create unified model structure ([f971864](https://github.com/unhappychoice/gittype/commit/f971864))
- feat: rename RepoManager to RepositoryManager for consistency ([7fc3958](https://github.com/unhappychoice/gittype/commit/7fc3958))
- feat: complete SessionSummary to SessionResult unification with full implementation ([1cb5d45](https://github.com/unhappychoice/gittype/commit/1cb5d45))
- feat: rename GameDisplayRatatui to StageRenderer for consistency ([f121860](https://github.com/unhappychoice/gittype/commit/f121860))
- feat: rename GameState to SessionState for consistency ([558563b](https://github.com/unhappychoice/gittype/commit/558563b))
- feat: complete terminology unification across entire codebase ([7fb0b5f](https://github.com/unhappychoice/gittype/commit/7fb0b5f))
- feat: fix semantic accuracy of all field and function names ([845aeda](https://github.com/unhappychoice/gittype/commit/845aeda))
- feat: complete method and variable name unification ([04fc71e](https://github.com/unhappychoice/gittype/commit/04fc71e))
- feat: fix Rank/RankingTitle naming inconsistency ([64263f3](https://github.com/unhappychoice/gittype/commit/64263f3))
- feat: complete UI text terminology updates ([76cf9e3](https://github.com/unhappychoice/gittype/commit/76cf9e3))
- feat: complete model terminology unification ([4afec8c](https://github.com/unhappychoice/gittype/commit/4afec8c))
- feat: add new unified model structure ([ee464f5](https://github.com/unhappychoice/gittype/commit/ee464f5))
- feat(options): improve ExtractionOptions with dynamic language patterns ([646900c](https://github.com/unhappychoice/gittype/commit/646900c))
- feat(language): split Language implementations into individual files ([24528ec](https://github.com/unhappychoice/gittype/commit/24528ec))
- feat: implement comprehensive Dart language extractor ([731e083](https://github.com/unhappychoice/gittype/commit/731e083))
- feat: add Dart language detection and file pattern support ([6fee1f4](https://github.com/unhappychoice/gittype/commit/6fee1f4))
- feat: add tree-sitter-dart dependency for Dart language support ([0356278](https://github.com/unhappychoice/gittype/commit/0356278))
- feat: register Haskell language in core system ([97c558f](https://github.com/unhappychoice/gittype/commit/97c558f))
- feat: add Haskell language support ([6839932](https://github.com/unhappychoice/gittype/commit/6839932))
- feat: add C++ file patterns to default extraction options ([eb1f4c0](https://github.com/unhappychoice/gittype/commit/eb1f4c0))
- feat: implement C++ code parser with comprehensive extraction ([ad882a1](https://github.com/unhappychoice/gittype/commit/ad882a1))
- feat: add C++ language detection and validation ([f883f92](https://github.com/unhappychoice/gittype/commit/f883f92))
- feat: add tree-sitter-cpp dependency for C++ language support ([502f0c3](https://github.com/unhappychoice/gittype/commit/502f0c3))
- feat: add C comment detection support ([5a7c55d](https://github.com/unhappychoice/gittype/commit/5a7c55d))
- feat: add TreeSitterLanguageError support ([3a090ef](https://github.com/unhappychoice/gittype/commit/3a090ef))
- feat: add C file patterns to extraction options ([a85eb4f](https://github.com/unhappychoice/gittype/commit/a85eb4f))
- feat: implement C language parser ([ad0dbae](https://github.com/unhappychoice/gittype/commit/ad0dbae))
- feat: add C language enum and detection ([0eb169c](https://github.com/unhappychoice/gittype/commit/0eb169c))
- feat: add tree-sitter-c dependency ([9a988f4](https://github.com/unhappychoice/gittype/commit/9a988f4))
- feat: integrate retry functionality into stage manager ([a75e807](https://github.com/unhappychoice/gittype/commit/a75e807))
- feat: implement retry option on failure and cancellation screens ([33accf7](https://github.com/unhappychoice/gittype/commit/33accf7))
- feat: align result file name display with colons ([a7405fd](https://github.com/unhappychoice/gittype/commit/a7405fd))
- feat: display countdown numbers as ASCII art ([931d433](https://github.com/unhappychoice/gittype/commit/931d433))
- feat: improve social sharing text with repository context ([ed1c2e7](https://github.com/unhappychoice/gittype/commit/ed1c2e7))
- feat: add repository information to result and session screens ([914f248](https://github.com/unhappychoice/gittype/commit/914f248))
- feat: enhance loading screen repository display ([26c7790](https://github.com/unhappychoice/gittype/commit/26c7790))
- feat: improve file path display and add repository context to challenges ([acf709e](https://github.com/unhappychoice/gittype/commit/acf709e))
- feat: implement dynamic score color-coding based on ranking tier ([7237046](https://github.com/unhappychoice/gittype/commit/7237046))
- feat: add terminal color methods to RankingTier and RankingTitle ([615bb2e](https://github.com/unhappychoice/gittype/commit/615bb2e))
- feat: shorten countdown duration for improved UX ([812c5e7](https://github.com/unhappychoice/gittype/commit/812c5e7))

### 🐛 Bug Fixes

- fix(clippy): remove unnecessary mut in context_loader tests ([91bebf5](https://github.com/unhappychoice/gittype/commit/91bebf5))
- fix(clippy): resolve clippy warnings ([e66487a](https://github.com/unhappychoice/gittype/commit/e66487a))
- fix(exit-screen): replace session terminology with total and remove conditions ([ca18640](https://github.com/unhappychoice/gittype/commit/ca18640))
- fix: reset skip count to 3 when retrying failed/cancelled stages ([9375d78](https://github.com/unhappychoice/gittype/commit/9375d78))
- fix(clippy): use $crate instead of crate in macro definitions ([90aed49](https://github.com/unhappychoice/gittype/commit/90aed49))
- fix: update tests to work with refactored typing_core ([12b070e](https://github.com/unhappychoice/gittype/commit/12b070e))
- fix: record keystroke before typing core input processing ([5110a64](https://github.com/unhappychoice/gittype/commit/5110a64))
- fix: use correct display position for current_mistake_position ([67ccd84](https://github.com/unhappychoice/gittype/commit/67ccd84))
- fix(clippy): resolve field_reassign_with_default and needless_range_loop warnings ([0a325c5](https://github.com/unhappychoice/gittype/commit/0a325c5))
- fix(clippy): resolve needless_range_loop warning ([497be0a](https://github.com/unhappychoice/gittype/commit/497be0a))
- fix(scoring): correct real-time CPM calculation during pauses ([c620a64](https://github.com/unhappychoice/gittype/commit/c620a64))
- fix: correct remaining semantic naming issues beyond rank-related terms ([122b178](https://github.com/unhappychoice/gittype/commit/122b178))
- fix(tests): fix test failures by excluding tmp/** pattern in temp directories ([3b89dee](https://github.com/unhappychoice/gittype/commit/3b89dee))
- fix: resolve clippy warning in Dart extractor ([63cb5b1](https://github.com/unhappychoice/gittype/commit/63cb5b1))
- fix: resolve clippy warnings in Haskell extractor ([caadf07](https://github.com/unhappychoice/gittype/commit/caadf07))
- fix: apply cargo fmt to fix spacing in comments ([ff78420](https://github.com/unhappychoice/gittype/commit/ff78420))
- fix: ensure panic error messages are visible to users ([4ff714c](https://github.com/unhappychoice/gittype/commit/4ff714c))
- fix: resolve clippy warning for unused variable ([db42a84](https://github.com/unhappychoice/gittype/commit/db42a84))
- fix(ci): improve Homebrew formula update process with awk ([7c0b626](https://github.com/unhappychoice/gittype/commit/7c0b626))

### 📝 Other Changes

- chore: Update Cargo.lock ([9e0a8e2](https://github.com/unhappychoice/gittype/commit/9e0a8e2))
- style: fix cargo fmt formatting issues ([f36dc09](https://github.com/unhappychoice/gittype/commit/f36dc09))
- chore(deps): bump rusqlite from 0.29.0 to 0.37.0 ([afb84ca](https://github.com/unhappychoice/gittype/commit/afb84ca))
- style: apply cargo fmt formatting changes ([c3131b9](https://github.com/unhappychoice/gittype/commit/c3131b9))
- chore(deps): bump crossterm from 0.27.0 to 0.29.0 ([9d225b0](https://github.com/unhappychoice/gittype/commit/9d225b0))
- chore(deps): bump tree-sitter from 0.24.7 to 0.25.8 ([bfc3da2](https://github.com/unhappychoice/gittype/commit/bfc3da2))
- chore(deps): bump thiserror from 1.0.69 to 2.0.16 ([20b5145](https://github.com/unhappychoice/gittype/commit/20b5145))
- style: fix code formatting with cargo fmt ([e1e4992](https://github.com/unhappychoice/gittype/commit/e1e4992))
- refactor: remove unused code and fields from typing_core ([de29b0a](https://github.com/unhappychoice/gittype/commit/de29b0a))
- refactor: move mistake tracking to typing_core ([ebfff51](https://github.com/unhappychoice/gittype/commit/ebfff51))
- refactor: delegate input processing responsibility to typing_core ([a965527](https://github.com/unhappychoice/gittype/commit/a965527))
- refactor: remove unnecessary was_failed method ([1ed5560](https://github.com/unhappychoice/gittype/commit/1ed5560))
- refactor(typing_screen): improve code organization and reduce duplication ([cc79204](https://github.com/unhappychoice/gittype/commit/cc79204))
- refactor(mod): update module exports ([4f61144](https://github.com/unhappychoice/gittype/commit/4f61144))
- test(snapshots): update test snapshots for comment highlighting ([2565726](https://github.com/unhappychoice/gittype/commit/2565726))
- refactor(ui): simplify display_challenge_with_info method signature ([141dbba](https://github.com/unhappychoice/gittype/commit/141dbba))
- refactor: unify SessionSummary/SessionResult and fix semantic naming ([aaa51d7](https://github.com/unhappychoice/gittype/commit/aaa51d7))
- refactor: rename components for terminology consistency ([8a870a7](https://github.com/unhappychoice/gittype/commit/8a870a7))
- refactor(cli): remove unused config.rs and update CLI structure ([9891198](https://github.com/unhappychoice/gittype/commit/9891198))
- refactor(extractor): update extractors to use polymorphic Language trait ([dcaa065](https://github.com/unhappychoice/gittype/commit/dcaa065))
- refactor(language): refactor Language trait to polymorphic design ([98cbb45](https://github.com/unhappychoice/gittype/commit/98cbb45))
- docs: update documentation for Dart language support ([79ad937](https://github.com/unhappychoice/gittype/commit/79ad937))
- test: add comprehensive Dart language extraction tests ([3034a20](https://github.com/unhappychoice/gittype/commit/3034a20))
- refactor: remove unnecessary delegation and suffix in Haskell extractor ([743fd24](https://github.com/unhappychoice/gittype/commit/743fd24))
- style: format code according to rustfmt standards ([a224121](https://github.com/unhappychoice/gittype/commit/a224121))
- docs: add Haskell to supported languages documentation ([ba2bdf5](https://github.com/unhappychoice/gittype/commit/ba2bdf5))
- refactor: update all language parsers for tree-sitter v0.24 ([e1e0755](https://github.com/unhappychoice/gittype/commit/e1e0755))
- refactor: migrate to tree-sitter v0.24 API ([89f6945](https://github.com/unhappychoice/gittype/commit/89f6945))
- chore(deps): update all tree-sitter dependencies to latest versions ([47d0899](https://github.com/unhappychoice/gittype/commit/47d0899))
- docs: add C and C++ language support to documentation ([ff40f6b](https://github.com/unhappychoice/gittype/commit/ff40f6b))
- test: add comprehensive C++ language integration tests ([1b58863](https://github.com/unhappychoice/gittype/commit/1b58863))
- test: add comprehensive C language tests ([f0238c9](https://github.com/unhappychoice/gittype/commit/f0238c9))
- refactor: remove old result_screen.rs after successful refactoring ([3d1c16c](https://github.com/unhappychoice/gittype/commit/3d1c16c))
- refactor: update screen module exports for new structure ([f1f794f](https://github.com/unhappychoice/gittype/commit/f1f794f))
- refactor: split result_screen.rs into focused screen modules ([e630e27](https://github.com/unhappychoice/gittype/commit/e630e27))
- refactor: redesign countdown screen repository and source info layout ([c796d22](https://github.com/unhappychoice/gittype/commit/c796d22))
- chore: Update Twitter references to X platform ([1c6fdb2](https://github.com/unhappychoice/gittype/commit/1c6fdb2))
- docs(readme): update --repo examples to use relevant Rust libraries ([791996b](https://github.com/unhappychoice/gittype/commit/791996b))
- docs(readme): add --repo option examples ([68e6815](https://github.com/unhappychoice/gittype/commit/68e6815))
- refactor(main): simplify main.rs using new CLI modules ([238ad8c](https://github.com/unhappychoice/gittype/commit/238ad8c))
- refactor(cli): move CLI components to separate modules ([3d82e20](https://github.com/unhappychoice/gittype/commit/3d82e20))
- chore: Update CHANGELOG.md ([32d6d2c](https://github.com/unhappychoice/gittype/commit/32d6d2c))


## [0.3.0] - 2025-09-02

### ✨ Features

- feat: use vendored OpenSSL for all targets to avoid cross-compilation issues ([2bc3269](https://github.com/unhappychoice/gittype/commit/2bc3269))
- feat: update Homebrew to use prebuilt binaries and add ARM64 support ([96836dd](https://github.com/unhappychoice/gittype/commit/96836dd))
- feat: add repository management with GitHub support ([d08a93b](https://github.com/unhappychoice/gittype/commit/d08a93b))
- feat: add GitHub links to commit hashes in changelog ([5881873](https://github.com/unhappychoice/gittype/commit/5881873))
- feat: add automatic CHANGELOG.md generation ([7a2ae38](https://github.com/unhappychoice/gittype/commit/7a2ae38))

### 🐛 Bug Fixes

- fix(extractor): avoid progress updates in parallel path to satisfy Sync and pass clippy ([b8e433d](https://github.com/unhappychoice/gittype/commit/b8e433d))

### 📝 Other Changes

- chore: bump version to v0.3.0 ([33f0df0](https://github.com/unhappychoice/gittype/commit/33f0df0))
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
- feat: add info dialog with GitHub and Twitter links ([9d3a407](https://github.com/unhappychoice/gittype/commit/9d3a407))
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


