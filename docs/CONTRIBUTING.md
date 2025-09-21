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

> When developing in a [Nix](https://nixos.org/) environment: `nix develop` will create a dev shell with all dependencies.

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

2. **Create language-specific extractor:**
   ```rust
   // src/extractor/languages/newlang.rs
   use tree_sitter::Language;
   
   extern "C" {
       fn tree_sitter_newlang() -> Language;
   }
   
   pub fn language() -> Language {
       unsafe { tree_sitter_newlang() }
   }
   
   pub const QUERY: &str = r#"
       (function_definition
         name: (identifier) @function.name) @function.definition
   "#;
   ```

3. **Update the main extractor:**
   ```rust
   // src/extractor/mod.rs
   match language {
       Language::NewLang => languages::newlang::create_extractor(),
       // ... other languages
   }
   ```

4. **Add tests:**
   ```rust
   #[test]
   fn test_newlang_extraction() {
       // Test with sample code
   }
   ```

5. **Update documentation:**
   - Add to supported languages list in README.md
   - Include examples if needed

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