# Supported Languages

## Current Support

| Language | Extensions | Aliases | Tree-sitter Grammar |
|----------|------------|---------|-------------------|
| C | `.c`, `.h` | - | `tree_sitter_c` |
| C# | `.cs`, `.csx` | `cs`, `c#` | `tree_sitter_c_sharp` |
| C++ | `.cpp`, `.cc`, `.cxx`, `.hpp` | `c++` | `tree_sitter_cpp` |
| Clojure | `.clj`, `.cljs`, `.cljc` | - | `tree_sitter_clojure` |
| Dart | `.dart` | - | `tree_sitter_dart` |
| Elixir | `.ex`, `.exs` | `ex`, `exs` | `tree_sitter_elixir` |
| Go | `.go` | - | `tree_sitter_go` |
| Haskell | `.hs`, `.lhs` | `hs` | `tree_sitter_haskell` |
| Java | `.java` | - | `tree_sitter_java` |
| JavaScript | `.js`, `.jsx`, `.mjs`, `.cjs` | `js` | `tree_sitter_javascript` |
| Kotlin | `.kt`, `.kts` | `kt` | `tree_sitter_kotlin_ng` |
| PHP | `.php`, `.phtml`, `.php3`, `.php4`, `.php5` | - | `tree_sitter_php` |
| Python | `.py` | `py` | `tree_sitter_python` |
| Ruby | `.rb` | `rb` | `tree_sitter_ruby` |
| Rust | `.rs` | `rs` | `tree_sitter_rust` |
| Scala | `.sc`, `.scala` | `sc` | `tree_sitter_scala` |
| Swift | `.swift` | - | `tree_sitter_swift` |
| TypeScript | `.ts`, `.tsx` | `ts` | `tree_sitter_typescript` (TSX) |
| Zig | `.zig` | - | `tree_sitter_zig` |

## Extraction Features

### C
- **Functions** (`function_definition`) - Function definitions
- **Structs** (`struct_specifier`) - Struct definitions
- **Unions** (`union_specifier`) - Union definitions
- **Enums** (`enum_specifier`) - Enum definitions
- **Type Definitions** (`typedef`) - Type alias definitions

### C#
- **Classes** (`class_declaration`) - Class definitions
- **Structs** (`struct_declaration`) - Struct definitions
- **Interfaces** (`interface_declaration`) - Interface definitions
- **Enums** (`enum_declaration`) - Enum definitions
- **Records** (`record_declaration`) - Record definitions
- **Methods** (`method_declaration`) - Method definitions
- **Constructors** (`constructor_declaration`) - Constructor definitions
- **Destructors** (`destructor_declaration`) - Destructor definitions
- **Properties** (`property_declaration`) - Property definitions
- **Events** (`event_declaration`) - Event definitions
- **Delegates** (`delegate_declaration`) - Delegate definitions
- **Namespaces** (`namespace_declaration`) - Namespace definitions

### C++
- **Functions** (`function_definition`) - Function definitions
- **Methods** (`function_definition`) - Method definitions
- **Classes** (`class_specifier`) - Class definitions
- **Structs** (`struct_specifier`) - Struct definitions
- **Templates** (`template_declaration`) - Template definitions
- **Namespaces** (`namespace_definition`) - Namespace definitions

### Clojure
- **Functions** (`defn`, `defmacro`, `defn-`) - Function and macro definitions
- **Variables** (`def`) - Variable definitions
- **Namespaces** (`ns`) - Namespace declarations
- **Classes** (`deftype`, `defrecord`) - Type and record definitions
- **Interfaces** (`defprotocol`) - Protocol definitions

### Dart
- **Functions** (`function_signature`) - Function definitions
- **Methods** (`method_signature`) - Method definitions
- **Classes** (`class_definition`) - Class definitions
- **Enums** (`enum_declaration`) - Enum declarations
- **Extensions** (`extension_declaration`) - Extension definitions
- **Mixins** (`mixin_declaration`) - Mixin definitions

### Elixir
- **Functions** (`def`, `defp`) - Public and private function definitions
- **Modules** (`defmodule`) - Module definitions
- **Macros** (`defmacro`, `defmacrop`) - Macro definitions
- **Protocols** (`defprotocol`) - Protocol definitions
- **Implementations** (`defimpl`) - Protocol implementation blocks
- **Structs** (`defstruct`) - Struct definitions
- **Guards** (`defguard`, `defguardp`) - Guard definitions

### Go
- **Functions** (`function_declaration`) - Function definitions
- **Methods** (`method_declaration`) - Method definitions with receivers
- **Structs** (`type_spec` with `struct_type`) - Struct type definitions
- **Interfaces** (`type_spec` with `interface_type`) - Interface type definitions
- **Constants** (`const_declaration`) - Constant declarations
- **Variables** (`var_declaration`) - Variable declarations
- **Type Aliases** (`type_spec`) - Type alias definitions

### Haskell
- **Functions** (`function_declaration`) - Function definitions
- **Type Signatures** (`type_signature`) - Type signature declarations
- **Data Types** (`data_type`) - Data type definitions
- **Type Classes** (`class_declaration`) - Type class definitions
- **Instances** (`instance_declaration`) - Instance definitions

### Java
- **Classes** (`class_declaration`) - Class definitions
- **Interfaces** (`interface_declaration`) - Interface definitions
- **Methods** (`method_declaration`) - Method definitions
- **Constructors** (`constructor_declaration`) - Constructor definitions
- **Enums** (`enum_declaration`) - Enum definitions
- **Records** (`record_declaration`) - Record definitions (Java 14+)
- **Annotation Types** (`annotation_type_declaration`) - Annotation type definitions
- **Fields** (`field_declaration`) - Field declarations

### JavaScript
- **Functions** (`function_declaration`) - Function declarations
- **Methods** (`method_definition`) - Object and class methods
- **Classes** (`class_declaration`) - ES6+ class definitions
- **Arrow Functions** (`arrow_function`) - Arrow function expressions
- **Function Expressions** (`function_expression`) - Function expression assignments
- **JSX Elements** (`jsx_element`, `jsx_self_closing_element`) - React components

### Kotlin
- **Functions** (`function_declaration`) - Function definitions
- **Classes** (`class_declaration`) - Class definitions
- **Objects** (`object_declaration`) - Object declarations
- **Properties** (`property_declaration`) - Property definitions
- **Enums** (`enum_class_body`) - Enum class definitions

### PHP
- **Functions** (`function_definition`) - Function definitions
- **Methods** (`method_declaration`) - Method definitions
- **Classes** (`class_declaration`) - Class definitions
- **Interfaces** (`interface_declaration`) - Interface definitions
- **Traits** (`trait_declaration`) - Trait definitions
- **Namespaces** (`namespace_definition`) - Namespace definitions

### Python
- **Functions** (`function_definition`) - Function definitions with decorators
- **Classes** (`class_definition`) - Class definitions with methods and inheritance

### Ruby
- **Instance Methods** (`method`) - Instance method definitions
- **Class Methods** (`singleton_method`) - Class method definitions
- **Classes** (`class`) - Class definitions
- **Modules** (`module`) - Module definitions

### Rust
- **Functions** (`function_item`) - Function definitions with parameters and body
- **Implementations** (`impl_item`) - Implementation blocks for structs/traits
- **Structs** (`struct_item`) - Struct definitions with fields
- **Enums** (`enum_item`) - Enum definitions with variants
- **Traits** (`trait_item`) - Trait definitions with associated functions
- **Modules** (`mod_item`) - Module declarations and definitions
- **Type Aliases** (`type_item`) - Type alias declarations

### Scala
- **Functions** (`function_definition`) - Function definitions
- **Classes** (`class_definition`) - Class definitions
- **Objects** (`object_definition`) - Object definitions
- **Traits** (`trait_definition`) - Trait definitions
- **Type Definitions** (`type_definition`) - Type definitions

### Swift
- **Functions** (`function_declaration`) - Function definitions
- **Classes** (`class_declaration`) - Class definitions
- **Structs** (`struct_declaration`) - Struct definitions
- **Enums** (`enum_declaration`) - Enum definitions
- **Protocols** (`protocol_declaration`) - Protocol definitions
- **Extensions** (`extension_declaration`) - Extension definitions
- **Initializers** (`init_declaration`) - Initializer definitions
- **Methods** (`function_declaration`) - Method definitions

### TypeScript
- **Functions** (`function_declaration`) - Function declarations
- **Methods** (`method_definition`) - Class and object methods
- **Classes** (`class_declaration`) - Class definitions with constructors and methods
- **Arrow Functions** (`arrow_function`) - Arrow function expressions
- **Function Expressions** (`function_expression`) - Function expression assignments
- **Interfaces** (`interface_declaration`) - Interface definitions
- **Type Aliases** (`type_alias_declaration`) - Type alias definitions
- **Enums** (`enum_declaration`) - Enum declarations
- **Namespaces** (`internal_module`) - Namespace declarations
- **JSX Elements** (`jsx_element`, `jsx_self_closing_element`) - React components

### Zig
- **Functions** (`function_declaration`) - Function definitions
- **Structs** (`variable_declaration` with `struct_declaration`) - Struct type definitions
- **Enums** (`variable_declaration` with `enum_declaration`) - Enum type definitions
- **Unions** (`variable_declaration` with `union_declaration`) - Union type definitions

## Language-Specific Options

### Filtering by Language

```bash
# Single language
gittype --langs rust

# Multiple languages
gittype --langs rust,typescript,javascript,python
```

### Configuration File

```toml
[default]
langs = ["rust", "typescript", "javascript", "python", "go", "ruby", "swift", "kotlin", "java", "php", "csharp", "c", "cpp", "haskell", "dart", "scala", "zig", "elixir"]
```

## Code Extraction Quality

### What Gets Extracted

- **Complete Definitions**: Full function/class/method definitions with signatures and bodies
- **Self-Contained Blocks**: Code that makes sense in isolation for typing practice
- **Real-World Constructs**: Actual code patterns from repository codebases

### What Gets Filtered Out

- **Incomplete Snippets**: Partial code lacking context
- **Comments Only**: Blocks containing only comments
- **Import Statements**: Standalone import/use declarations
- **Very Short Code**: Code blocks under minimum threshold for meaningful practice

## Adding New Language Support

See [CONTRIBUTING.md](CONTRIBUTING.md#adding-language-support) for detailed instructions on adding support for new programming languages.