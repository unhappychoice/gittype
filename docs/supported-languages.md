# Supported Languages

## Current Support

| Language | Extension | Status | Tree-sitter Grammar |
|----------|-----------|--------|-------------------|
| Rust | `.rs` | ✅ Full support | `tree-sitter-rust` |
| TypeScript | `.ts`, `.tsx` | ✅ Full support | `tree-sitter-typescript` |
| JavaScript | `.js`, `.jsx`, `.mjs`, `.cjs` | ✅ Full support | `tree-sitter-javascript` |
| Python | `.py` | ✅ Full support | `tree-sitter-python` |
| Go | `.go` | ✅ Full support | `tree-sitter-go` |
| Ruby | `.rb` | ✅ Full support | `tree-sitter-ruby` |
| Swift | `.swift` | ✅ Full support | `tree-sitter-swift` |
| Kotlin | `.kt`, `.kts` | ✅ Full support | `tree-sitter-kotlin` |
| Java | `.java` | ✅ Full support | `tree-sitter-java` |
| PHP | `.php`, `.phtml`, `.php3`, `.php4`, `.php5` | ✅ Full support | `tree-sitter-php` |
| C# | `.cs`, `.csx` | ✅ Full support | `tree-sitter-c-sharp` |
| C | `.c`, `.h` | ✅ Full support | `tree-sitter-c` |
| C++ | `.cpp`, `.cc`, `.cxx`, `.hpp` | ✅ Full support | `tree-sitter-cpp` |
| Haskell | `.hs`, `.lhs` | ✅ Full support | `tree-sitter-haskell` |
| Dart | `.dart` | ✅ Full support | `tree-sitter-dart` |
| Scala | `.scala`, `.sc` | ✅ Full support | `tree-sitter-scala` |

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

### JavaScript
- Function declarations (`function`)
- Arrow functions (`() => {}`)
- Function expressions
- Classes (`class`) - ES6+ class syntax
- Methods (class and object methods)
- Variable declarations with functions (`const fn = () => {}`)
- Async functions (`async function`)
- Export/import statements (ES modules)
- Object literal methods
- **JSX syntax** - React component definitions (`.jsx` files)

### TypeScript/TSX
- All JavaScript features plus:
- **JSX/TSX syntax** - React components with TypeScript (`.tsx` files)
- Full TSX parser support for mixed TypeScript and JSX content

> **Note**: JSX/TSX components are extracted as function and class definitions. Individual JSX elements (`<Component />`) are not extracted as separate typing challenges, but the component definitions that contain JSX are fully supported.

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

### Java
- Methods (`public/private/protected methods`)
- Classes (`class`)
- Interfaces (`interface`)
- Enums (`enum`)
- Constructors
- Static methods and fields
- Abstract classes and methods
- Nested classes and interfaces

### PHP
- Functions (`function`)
- Methods (`public/private/protected methods`)
- Classes (`class`)
- Interfaces (`interface`)
- Traits (`trait`)
- Namespaces (`namespace`)
- Magic methods (`__construct`, `__toString`, etc.)
- Static methods and properties
- Anonymous functions and closures
- Exception handling (`try/catch/finally`)

### C#
- Methods (`public/private/protected methods`)
- Classes (`class`)
- Structs (`struct`)
- Interfaces (`interface`)
- Enums (`enum`)
- Records (`record`)
- Constructors and destructors
- Properties (auto-properties and with getters/setters)
- Events (`event`)
- Delegates (`delegate`)
- Namespaces (`namespace`)
- Extension methods
- Async/await methods
- LINQ expressions
- Attributes
- Access modifiers (public, private, protected, internal)

### C
- Functions (`int main()`, `void function()`)
- Structs (`struct`)
- Type definitions (`typedef`)
- Enum definitions (`enum`)
- Global variables
- Macro definitions (`#define`)
- Static functions and variables
- Function pointers
- Union definitions (`union`)

### Haskell
- Functions (`function_name :: Type -> Type`, `function_name arg = ...`)
- Type signatures (`function_name :: Integer -> Integer`)
- Module declarations (`module ModuleName where`)
- Import declarations (`import Module`)
- Data type declarations (`data Maybe a = Nothing | Just a`)
- Type class definitions (`class Eq a where`)
- Instance declarations (`instance Eq Int where`)
- Type aliases (`type Name = String`)
- Pattern matching functions

### C++
- Functions (`int main()`, `void function()`)
- Methods (class and struct member functions)
- Classes (`class`)
- Structs (`struct`)
- Template classes (`template<typename T> class`)
- Template functions (`template<typename T> T function()`)
- Constructors and destructors
- Operator overloading
- Global variables
- Type definitions (`typedef`, `using`)
- Enum definitions (`enum`, `enum class`)
- Namespace functions (functions within namespaces)

### Dart
- Functions (`String greet(String name) { ... }`)
- Methods (instance and static methods within classes)
- Classes (`class`) - complete class definitions with constructors, methods, and properties
- Enums (`enum`) - enum declarations with values and methods
- Mixins (`mixin`) - mixin declarations with methods and properties
- Extensions (`extension`) - extension methods on existing types
- Variables (`final`, `var`, `const`) - global and local variable declarations
- Constructors (default, named, and factory constructors)
- Getters and setters (`get`, `set`)
- Async functions (`Future<T>`, `async`/`await` patterns)
- Abstract classes and methods
- Static members (methods and properties)
- Type definitions (`typedef`)

### Scala
- Functions (`def`) - method and function definitions
- Classes (`class`, `case class`) - class definitions with constructors and methods
- Object declarations (`object`) - singleton objects and companion objects
- Traits (`trait`) - trait definitions with methods and implementations
- Enums (`enum`) - Scala 3 enum definitions with cases
- Type definitions (`type`) - type aliases and abstract types
- Package objects (`package object`) - utility functions and implicit conversions
- Given definitions (`given`) - Scala 3 contextual abstractions
- Extension methods (`extension`) - Scala 3 extension method definitions

## Planned Support

| Language | Priority | Expected | Notes |
|----------|----------|----------|--------|
| Zig | Low | Future | Systems programming |

## Language-Specific Options

### Filtering by Language

```bash
# Single language
gittype --langs rust

# Multiple languages
gittype --langs rust,typescript,javascript,python,go,ruby,swift,kotlin,java,php,csharp,c,cpp,haskell,dart,scala
```

### Configuration File

```toml
[default]
langs = ["rust", "typescript", "javascript", "python", "go", "ruby", "swift", "kotlin", "java", "php", "csharp", "c", "cpp", "haskell", "dart", "scala"]
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
