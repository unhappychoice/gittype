# Contributing to GitType

Thank you for your interest in contributing to GitType! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Coding Standards](#coding-standards)
- [Adding Language Support](#adding-language-support)

---

## Code of Conduct

This project follows a standard code of conduct. Please be respectful and constructive in all interactions.

---

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- Basic familiarity with Rust and CLI development

> When developing in a [Nix](https://nixos.org/) environment: `nix develop` will create a development shell with all dependencies.

### Development Setup

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/gittype.git
   cd gittype
   ```

2. **Set up the development environment:**
   ```bash
   # Install dependencies and build
   cargo build
   
   # Run tests to ensure everything works
   cargo test
   
   # Try running the application
   cargo run -- --help
   ```

3. **Create a development branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

---

## Project Structure

```
gittype/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── extractor/           # Code extraction logic
│   │   ├── mod.rs           # Main extractor interface
│   │   ├── repository.rs    # Repository loading
│   │   └── languages/       # Language-specific parsers
│   ├── game/                # Game logic and UI
│   │   ├── mod.rs           # Game orchestration
│   │   ├── stage_manager.rs # Stage management
│   │   ├── scoring/         # Scoring system
│   │   └── screens/         # UI screens
│   └── database/            # Session storage
├── tests/                   # Integration tests
├── Cargo.toml              # Project configuration
└── README.md               # Project documentation
```

### Key Components

- **Extractor**: Handles parsing source code using tree-sitter
- **Game**: Manages typing challenges and user interface
- **Database**: Stores session history and statistics
- **Scoring**: Calculates accuracy, speed, and other metrics

---

## Making Changes

### Types of Contributions

1. **Bug Fixes**: Fix issues in existing functionality
2. **Features**: Add new functionality or improve existing features
3. **Performance**: Optimize code performance
4. **Documentation**: Improve or add documentation
5. **Language Support**: Add support for new programming languages

### Before You Start

1. Check existing issues and PRs to avoid duplication
2. For major features, consider opening an issue first to discuss
3. Ensure your changes align with the project goals

---

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration_tests
```

### Generating Test Coverage

To generate a test coverage report, you'll need `cargo-llvm-cov`.

1. **Install `cargo-llvm-cov`:**
   ```bash
   cargo install cargo-llvm-cov
   ```

2. **Generate the report:**
   ```bash
   cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
   ```
   This will create an `lcov.info` file which can be used by coverage visualization tools.

### Test Coverage

- Unit tests for core logic components
- Integration tests for CLI functionality
- Test files in various languages for extractor testing

### Writing Tests

- Add unit tests for new functions
- Include integration tests for CLI features
- Test edge cases and error conditions
- Use descriptive test names

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rust_functions() {
        let code = r#"
            fn example_function() {
                println!("Hello, world!");
            }
        "#;
        
        let extractor = RustExtractor::new();
        let chunks = extractor.extract(code).unwrap();
        
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].function_name(), "example_function");
    }
}
```

---

## Submitting Changes

### Pull Request Process

1. **Ensure your branch is up to date:**
   ```bash
   git checkout main
   git pull upstream main
   git checkout your-branch
   git rebase main
   ```

2. **Run the full test suite and checks:**
   ```bash
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   ```

3. **Commit your changes:**
   ```bash
   git add .
   git commit -m "type: description of changes"
   ```

4. **Push to your fork:**
   ```bash
   git push origin your-branch
   ```

5. **Create a pull request:**
   - Use a descriptive title
   - Explain what changes were made and why
   - Reference any related issues
   - Include screenshots for UI changes

### Commit Message Format

Use conventional commit format:

```
type(scope): description

body (optional)

footer (optional)
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
- `feat: add support for Go language extraction`
- `fix: handle empty files in extractor`
- `docs: update installation instructions`

---

## Coding Standards

### Rust Style

- Follow the [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Write descriptive variable and function names
- Add documentation comments for public APIs

### Code Quality

- Keep functions focused and reasonably sized
- Use meaningful error types and messages
- Handle all error cases appropriately
- Avoid unwrap() in production code - use proper error handling
- Write self-documenting code with good naming

### Example Code Style

```rust
/// Extracts code chunks from a source file
pub fn extract_chunks(
    file_path: &Path,
    language: Language,
) -> Result<Vec<CodeChunk>, ExtractionError> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| ExtractionError::FileRead(e))?;
    
    let parser = create_parser(language)?;
    let tree = parser.parse(&content, None)
        .ok_or(ExtractionError::ParseFailed)?;
    
    extract_from_tree(&tree, &content)
}
```

---

## Adding Language Support

### Steps to Add a New Language

1. **Add the tree-sitter dependency:**
   ```toml
   # In Cargo.toml
   tree-sitter-newlang = "0.20"
   ```

2. **Create language definition:**
   ```rust
   // src/domain/models/languages/newlang.rs
   use crate::domain::models::Language;
   use crate::presentation::ui::Colors;
   use std::hash::Hash;

   #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
   pub struct NewLang;

   impl Language for NewLang {
       fn name(&self) -> &'static str {
           "newlang"
       }
       fn extensions(&self) -> Vec<&'static str> {
           vec!["nl"]
       }
       fn color(&self) -> ratatui::style::Color {
           Colors::lang_newlang()
       }
       fn display_name(&self) -> &'static str {
           "NewLang"
       }
       fn is_valid_comment_node(&self, node: tree_sitter::Node) -> bool {
           node.kind() == "comment"
       }
   }
   ```

3. **Create language-specific extractor:**
   ```rust
   // src/domain/services/source_code_parser/parsers/newlang.rs
   use super::LanguageExtractor;
   use crate::domain::models::ChunkType;
   use crate::{GitTypeError, Result};
   use tree_sitter::{Node, Parser};

   pub struct NewLangExtractor;

   impl LanguageExtractor for NewLangExtractor {
       fn tree_sitter_language(&self) -> tree_sitter::Language {
           tree_sitter_newlang::LANGUAGE.into()
       }

       fn query_patterns(&self) -> &str {
           "(function_declaration name: (identifier) @name) @function"
       }

       fn comment_query(&self) -> &str {
           "(comment) @comment"
       }

       fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
           match capture_name {
               "function" => Some(ChunkType::Function),
               _ => None,
           }
       }

       fn extract_name(&self, node: Node, source_code: &str, _capture_name: &str) -> Option<String> {
           // Extract identifier name from AST node
           None
       }

       fn middle_implementation_query(&self) -> &str {
           "" // Optional: for extracting code blocks within functions
       }

       fn middle_capture_name_to_chunk_type(&self, _capture_name: &str) -> Option<ChunkType> {
           None
       }
   }

   impl NewLangExtractor {
       pub fn create_parser() -> Result<Parser> {
           let mut parser = Parser::new();
           parser.set_language(&tree_sitter_newlang::LANGUAGE.into())
               .map_err(|e| GitTypeError::ExtractionFailed(format!("Failed to set NewLang language: {}", e)))?;
           Ok(parser)
       }
   }
   ```

4. **Register the language:**
   ```rust
   // In src/domain/models/languages/mod.rs
   pub mod newlang;
   pub use newlang::NewLang;

   // In src/domain/models/language.rs
   pub fn all_languages() -> Vec<Box<dyn Language>> {
       vec![
           // ... other languages
           Box::new(NewLang),
       ]
   }

   // In src/domain/services/source_code_parser/parsers/mod.rs
   pub mod newlang;
   register_language!(NewLang, newlang, NewLangExtractor);
   ```

5. **Add color scheme support:**
   ```rust
   // In src/domain/models/color_scheme.rs - add field:
   pub lang_newlang: SerializableColor,

   // In src/presentation/ui/colors.rs - add method:
   pub fn lang_newlang() -> Color {
       Self::get_color_scheme().lang_newlang.into()
   }

   // Add to theme JSON files:
   // assets/languages/lang_dark.json
   // assets/languages/lang_light.json
   // assets/languages/lang_ascii.json
   "lang_newlang": {"r": 123, "g": 45, "b": 67}
   ```

6. **Add tests:**
   ```rust
   // tests/integration/languages/newlang/extractor.rs
   use crate::integration::languages::extractor::test_language_extractor;

   test_language_extractor! {
       name: test_newlang_function_extraction,
       language: "newlang",
       extension: "nl",
       source: r#"
   function hello() {
       print("Hello")
   }
   "#,
       total_chunks: 2,
       chunk_counts: {
           File: 1,
           Function: 1,
       }
   }
   ```

7. **Update documentation:**
   - Add to `README.md` supported languages list
   - Add to `docs/supported-languages.md` with extraction features
   - Update CLI help in `src/presentation/cli/args.rs`
   - Add example code if needed

### Query Writing Guidelines

- Focus on extracting meaningful code units (functions, classes, methods)
- Ensure extracted chunks are self-contained
- Test queries with various code samples
- Consider edge cases and complex syntax

---

## Getting Help

- **Issues**: Browse existing issues or create a new one
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: Check the README and code comments

## Recognition

Contributors will be acknowledged in:
- CONTRIBUTORS.md file
- Release notes for significant contributions
- Repository insights and statistics

Thank you for contributing to GitType! 🚀