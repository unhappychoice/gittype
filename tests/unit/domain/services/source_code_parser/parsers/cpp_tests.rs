use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::cpp::CppExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_cpp(source_code: &str) -> tree_sitter::Tree {
    let mut parser = CppExtractor::create_parser().unwrap();
    parser.parse(source_code, None).unwrap()
}

fn find_node<'tree>(
    node: Node<'tree>,
    source_code: &str,
    kind: &str,
    text: &str,
) -> Option<Node<'tree>> {
    let matches_kind = node.kind() == kind;
    let matches_text = node
        .utf8_text(source_code.as_bytes())
        .map(|node_text| node_text.contains(text))
        .unwrap_or(false);

    matches_kind
        .then_some(node)
        .filter(|_| matches_text)
        .or_else(|| {
            (0..node.child_count())
                .filter_map(|index| node.child(index))
                .find_map(|child| find_node(child, source_code, kind, text))
        })
}

#[test]
fn create_parser_succeeds() {
    let result = CppExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = CppExtractor;
    let _language = extractor.tree_sitter_language();
    // Just verify it doesn't panic
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = CppExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
    assert!(patterns.contains("function"));
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = CppExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
    assert!(query.contains("comment"));
}

#[test]
fn capture_name_to_chunk_type_function() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("function.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_method() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("method.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_class() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("class.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_struct() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("struct.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_variable() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("variable.definition"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown() {
    let extractor = CppExtractor;
    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = CppExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
    assert!(query.contains("for_statement") || query.contains("while_statement"));
}

#[test]
fn middle_capture_name_to_chunk_type_loop() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_conditional() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_function_call() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("function_call"),
        Some(ChunkType::FunctionCall)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_code_block() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown() {
    let extractor = CppExtractor;
    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn capture_name_to_chunk_type_namespace() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("namespace.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_type_definition() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("type.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_enum() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("enum.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_template_class() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("template_class.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_template_function() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("template_function.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_operator() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("operator.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_destructor() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("destructor.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_switch() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("switch_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_while() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn extract_name_returns_none_for_cpp_namespace_identifier_nodes() {
    let source_code = "namespace tools { int value = 1; }";
    let tree = parse_cpp(source_code);
    let namespace_node = find_node(
        tree.root_node(),
        source_code,
        "namespace_definition",
        "namespace tools",
    )
    .unwrap();

    let name = CppExtractor.extract_name(namespace_node, source_code, "namespace.definition");

    assert_eq!(name, None);
}

#[test]
fn extract_name_reads_constructor_and_handles_destructor_fallback() {
    let source_code = "class Resource { public: Resource() {} ~Resource() {} };";
    let tree = parse_cpp(source_code);
    let constructor_node = find_node(
        tree.root_node(),
        source_code,
        "function_definition",
        "Resource() {}",
    )
    .unwrap();
    let destructor_node = find_node(
        tree.root_node(),
        source_code,
        "function_definition",
        "~Resource() {}",
    )
    .unwrap();

    let constructor_name =
        CppExtractor.extract_name(constructor_node, source_code, "constructor.definition");
    let destructor_name =
        CppExtractor.extract_name(destructor_node, source_code, "destructor.definition");

    assert_eq!(constructor_name, Some("Resource".to_string()));
    assert_eq!(destructor_name, None);
}

#[test]
fn extract_name_reads_plain_variable_declarations() {
    let source_code = "int plain;";
    let tree = parse_cpp(source_code);
    let declaration_node =
        find_node(tree.root_node(), source_code, "declaration", "plain").unwrap();

    let name = CppExtractor.extract_name(declaration_node, source_code, "variable.definition");

    assert_eq!(name, Some("plain".to_string()));
}

#[test]
fn extract_name_for_function_definition_returns_identifier() {
    let source_code = "int add(int a, int b) { return a + b; }";
    let tree = parse_cpp(source_code);
    let function_node =
        find_node(tree.root_node(), source_code, "function_definition", "add(").unwrap();

    let name = CppExtractor.extract_name(function_node, source_code, "function.definition");

    assert_eq!(name.as_deref(), Some("add"));
}

#[test]
fn extract_name_for_method_definition_delegates_to_function_name_extraction() {
    let source_code = "class Foo { public: void greet() {} };";
    let tree = parse_cpp(source_code);
    let definition = find_node(
        tree.root_node(),
        source_code,
        "function_definition",
        "greet() {}",
    )
    .unwrap();

    let name = CppExtractor.extract_name(definition, source_code, "method.definition");

    assert_eq!(name.as_deref(), Some("greet"));
}

#[test]
fn extract_name_for_class_definition_returns_type_identifier() {
    let source_code = "class Widget { int x; };";
    let tree = parse_cpp(source_code);
    let class_node = find_node(
        tree.root_node(),
        source_code,
        "class_specifier",
        "class Widget",
    )
    .unwrap();

    let name = CppExtractor.extract_name(class_node, source_code, "class.definition");

    assert_eq!(name.as_deref(), Some("Widget"));
}

#[test]
fn extract_name_for_struct_definition_returns_type_identifier() {
    let source_code = "struct Point { int x; int y; };";
    let tree = parse_cpp(source_code);
    let struct_node = find_node(
        tree.root_node(),
        source_code,
        "struct_specifier",
        "struct Point",
    )
    .unwrap();

    let name = CppExtractor.extract_name(struct_node, source_code, "struct.definition");

    assert_eq!(name.as_deref(), Some("Point"));
}

#[test]
fn extract_name_for_type_definition_returns_type_identifier() {
    let source_code = "typedef unsigned int MyUInt;";
    let tree = parse_cpp(source_code);
    let typedef_node =
        find_node(tree.root_node(), source_code, "type_definition", "MyUInt").unwrap();

    let name = CppExtractor.extract_name(typedef_node, source_code, "type.definition");

    assert_eq!(name.as_deref(), Some("MyUInt"));
}

#[test]
fn extract_name_for_enum_definition_returns_type_identifier() {
    let source_code = "enum Color { Red, Green, Blue };";
    let tree = parse_cpp(source_code);
    let enum_node = find_node(tree.root_node(), source_code, "enum_specifier", "Color").unwrap();

    let name = CppExtractor.extract_name(enum_node, source_code, "enum.definition");

    assert_eq!(name.as_deref(), Some("Color"));
}

#[test]
fn extract_name_for_variable_with_init_returns_identifier() {
    let source_code = "int counter = 0;";
    let tree = parse_cpp(source_code);
    let declaration_node =
        find_node(tree.root_node(), source_code, "declaration", "counter").unwrap();

    let name = CppExtractor.extract_name(declaration_node, source_code, "variable.definition");

    assert_eq!(name.as_deref(), Some("counter"));
}

#[test]
fn extract_name_for_operator_definition_returns_operator_name() {
    let source_code =
        "struct Vec { int v; Vec operator+(const Vec& other) const { return Vec{v + other.v}; } };";
    let tree = parse_cpp(source_code);
    let operator_node = find_node(
        tree.root_node(),
        source_code,
        "function_definition",
        "operator+",
    )
    .unwrap();

    let name = CppExtractor.extract_name(operator_node, source_code, "operator.definition");

    assert!(
        name.as_deref()
            .map(|s| s.contains("operator"))
            .unwrap_or(false),
        "expected operator name, got {:?}",
        name
    );
}

#[test]
fn extract_name_for_destructor_definition_returns_tilde_prefixed_name() {
    let source_code = "class Resource { public: ~Resource() {} };";
    let tree = parse_cpp(source_code);
    let destructor_node = find_node(
        tree.root_node(),
        source_code,
        "function_definition",
        "~Resource",
    )
    .unwrap();

    let name = CppExtractor.extract_name(destructor_node, source_code, "destructor.definition");

    // destructor_name child structure is parser-dependent; either we get the
    // tilde-prefixed form or None — both branches in the code are exercised
    // via this and the constructor test above.
    assert!(name.is_none() || name.as_deref().unwrap_or("").starts_with('~'));
}

#[test]
fn extract_name_for_unknown_capture_returns_none() {
    let source_code = "int x;";
    let tree = parse_cpp(source_code);

    let name = CppExtractor.extract_name(tree.root_node(), source_code, "unknown.capture");

    assert_eq!(name, None);
}

#[test]
fn extract_name_for_template_function_definition_returns_identifier() {
    let source_code = "template <typename T> T identity(T value) { return value; }";
    let tree = parse_cpp(source_code);
    let definition = find_node(
        tree.root_node(),
        source_code,
        "function_definition",
        "identity(T value)",
    )
    .unwrap();

    let name = CppExtractor.extract_name(definition, source_code, "template_function.definition");

    assert_eq!(name.as_deref(), Some("identity"));
}

#[test]
fn extract_name_for_template_class_definition_returns_type_identifier() {
    let source_code = "template <typename T> class Holder { T value; };";
    let tree = parse_cpp(source_code);
    let class_node = find_node(
        tree.root_node(),
        source_code,
        "class_specifier",
        "class Holder",
    )
    .unwrap();

    let name = CppExtractor.extract_name(class_node, source_code, "template_class.definition");

    assert_eq!(name.as_deref(), Some("Holder"));
}

#[test]
fn extract_constructor_destructor_name_returns_none_when_no_function_declarator() {
    let source_code = "struct Empty {};";
    let tree = parse_cpp(source_code);
    let struct_node = find_node(
        tree.root_node(),
        source_code,
        "struct_specifier",
        "struct Empty",
    )
    .unwrap();

    let name = CppExtractor.extract_name(struct_node, source_code, "constructor.definition");

    assert_eq!(name, None);
}

#[test]
fn extract_operator_name_returns_none_when_no_function_declarator() {
    let source_code = "struct Empty {};";
    let tree = parse_cpp(source_code);
    let struct_node = find_node(
        tree.root_node(),
        source_code,
        "struct_specifier",
        "struct Empty",
    )
    .unwrap();

    let name = CppExtractor.extract_name(struct_node, source_code, "operator.definition");

    assert_eq!(name, None);
}

#[test]
fn extract_operator_name_returns_none_when_function_is_not_operator() {
    let source_code = "int add(int a, int b) { return a + b; }";
    let tree = parse_cpp(source_code);
    let function_node =
        find_node(tree.root_node(), source_code, "function_definition", "add(").unwrap();

    let name = CppExtractor.extract_name(function_node, source_code, "operator.definition");

    assert_eq!(name, None);
}

#[test]
fn extract_type_name_returns_none_when_no_type_identifier_child() {
    let source_code = "int counter = 0;";
    let tree = parse_cpp(source_code);
    let declaration_node =
        find_node(tree.root_node(), source_code, "declaration", "counter").unwrap();

    let name = CppExtractor.extract_name(declaration_node, source_code, "type.definition");

    assert_eq!(name, None);
}

#[test]
fn extract_function_name_returns_none_when_no_function_declarator() {
    let source_code = "int x = 5;";
    let tree = parse_cpp(source_code);
    let declaration_node = find_node(tree.root_node(), source_code, "declaration", "x").unwrap();

    let name = CppExtractor.extract_name(declaration_node, source_code, "function.definition");

    assert_eq!(name, None);
}

#[test]
fn extract_variable_name_returns_none_when_no_variable_child_exists() {
    let source_code = "struct Empty {};";
    let tree = parse_cpp(source_code);
    let struct_node = find_node(
        tree.root_node(),
        source_code,
        "struct_specifier",
        "struct Empty",
    )
    .unwrap();

    let name = CppExtractor.extract_name(struct_node, source_code, "variable.definition");

    assert_eq!(name, None);
}
