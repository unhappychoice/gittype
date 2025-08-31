# Supported Languages

## Current Support

| Language | Extension | Status | Tree-sitter Grammar |
|----------|-----------|--------|-------------------|
| Rust | `.rs` | ✅ Full support | `tree-sitter-rust` |
| TypeScript | `.ts`, `.tsx` | ✅ Full support | `tree-sitter-typescript` |
| Python | `.py` | ✅ Full support | `tree-sitter-python` |
| Go | `.go` | ✅ Full support | `tree-sitter-go` |
| Ruby | `.rb` | ✅ Full support | `tree-sitter-ruby` |

## Extraction Features

### Rust
- Functions (`fn`)
- Implementations (`impl`)
- Structs (`struct`)
- Enums (`enum`)
- Traits (`trait`)
- Modules (`mod`)

### TypeScript
- Functions (`function`)
- Classes (`class`)
- Interfaces (`interface`)
- Methods
- Arrow functions
- Type definitions (`type`)

### Python
- Functions (`def`)
- Classes (`class`)
- Methods
- Decorators
- Lambda functions

### Go
- Functions (`func`)
- Methods (with receivers)
- Structs (`type ... struct`)
- Interfaces (`type ... interface`)
- Type declarations

### Ruby
- Methods (`def`)
- Classes (`class`)
- Modules (`module`)
- Instance methods
- Class methods

## Planned Support

| Language | Priority | Expected | Notes |
|----------|----------|----------|--------|
| JavaScript | High | Next release | ESM/CommonJS support |
| Swift | High | Q1 2025 | iOS/macOS development |
| Kotlin | High | Q1 2025 | Android/JVM support |
| Java | Medium | Q2 2025 | Spring Boot patterns |
| C++ | Medium | Q3 2025 | Modern C++17/20 |
| C# | Medium | Q3 2025 | .NET 6+ features |
| PHP | Low | Future | Laravel/Symfony |
| Dart | Low | Future | Flutter development |

## Language-Specific Options

### Filtering by Language

```bash
# Single language
gittype --langs rust

# Multiple languages
gittype --langs rust,typescript,python,go,ruby
```

### Configuration File

```toml
[default]
langs = ["rust", "typescript", "python", "go", "ruby"]
```

## Code Extraction Quality

### What Gets Extracted

- **Complete Functions**: Full function definitions with signatures
- **Class Definitions**: Complete class structures
- **Method Bodies**: Individual methods and their implementations
- **Self-Contained Blocks**: Code that makes sense in isolation

### What Gets Filtered Out

- **Incomplete Snippets**: Partial code that lacks context
- **Comments Only**: Blocks with only comments
- **Import Statements**: Standalone import/use declarations
- **Very Short Code**: Code blocks under minimum threshold

## Adding New Language Support

See [CONTRIBUTING.md](CONTRIBUTING.md#adding-language-support) for detailed instructions on adding support for new programming languages.