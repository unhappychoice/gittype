# Supported Languages

## Current Support

| Language | Extension | Status | Tree-sitter Grammar |
|----------|-----------|--------|-------------------|
| Rust | `.rs` | ✅ Full support | `tree-sitter-rust` |
| TypeScript | `.ts`, `.tsx` | ✅ Full support | `tree-sitter-typescript` |
| Python | `.py` | ✅ Full support | `tree-sitter-python` |
| Go | `.go` | ✅ Full support | `tree-sitter-go` |
| Ruby | `.rb` | ✅ Full support | `tree-sitter-ruby` |
| Swift | `.swift` | ✅ Full support | `tree-sitter-swift` |
| Kotlin | `.kt`, `.kts` | ✅ Full support | `tree-sitter-kotlin` |

## Extraction Features

### Rust
- Functions (`fn`)
- Implementations (`impl`)
- Structs (`struct`)
- Enums (`enum`) - complete enum definitions with variants and methods
- Traits (`trait`) - trait definitions with associated types and functions
- Modules (`mod`) - module blocks and their contents
- Type aliases (`type`) - type alias declarations

### TypeScript
- Functions (`function`)
- Classes (`class`)
- Interfaces (`interface`) - interface declarations with properties and methods
- Methods
- Arrow functions
- Function expressions
- Type definitions (`type`) - type alias declarations
- Enums (`enum`) - enum declarations with values and computed properties
- Namespaces (`namespace`) - namespace declarations with exported members

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
- Constant declarations (`const` blocks and single constants)
- Variable declarations (`var` blocks and single variables) 
- Type aliases (`type UserID int64`, `type Handler func(...)`)
- Function types
- Pointer types
- Slice/array types
- Map types
- Channel types

### Ruby
- Instance methods (`def method_name`)
- Class methods (`def self.method_name`)
- Singleton methods (methods defined on specific objects)
- Classes (`class`)
- Modules (`module`)
- Attribute accessors (`attr_accessor`, `attr_reader`, `attr_writer`)
- Method aliases (`alias`)

### Swift
- Functions (`func`)
- Classes (`class`)
- Structs (`struct`) - complete struct definitions with properties and methods
- Enums (`enum`) - complete enum definitions with cases and methods
- Protocols (`protocol`)
- Extensions (`extension`) - complete extension blocks
- Initializers (`init`)
- Deinitializers (`deinit`)
- Methods (instance and static)
- Computed properties

### Kotlin
- Functions (`fun`)
- Classes (`class`, `data class`, `sealed class`)
- Object declarations (`object`)
- Companion objects (`companion object`)
- Properties (`val`, `var`)
- Enum entries
- Extension functions
- Lambda expressions
- Interface implementations

## Planned Support

| Language | Priority | Expected | Notes |
|----------|----------|----------|--------|
| JavaScript | High | Next release | ESM/CommonJS support |
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
gittype --langs rust,typescript,python,go,ruby,swift,kotlin
```

### Configuration File

```toml
[default]
langs = ["rust", "typescript", "python", "go", "ruby", "swift", "kotlin"]
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