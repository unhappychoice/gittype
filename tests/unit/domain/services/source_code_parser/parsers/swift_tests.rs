use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::swift::SwiftExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_swift(source: &str) -> tree_sitter::Tree {
    let mut parser = SwiftExtractor::create_parser().unwrap();
    parser.parse(source, None).unwrap()
}

fn find_node<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    if node.kind() == kind {
        return Some(node);
    }
    (0..node.child_count())
        .filter_map(|index| node.child(index))
        .find_map(|child| find_node(child, kind))
}

#[test]
fn create_parser_succeeds() {
    let result = SwiftExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = SwiftExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = SwiftExtractor;
    assert!(!extractor.query_patterns().is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = SwiftExtractor;
    assert!(!extractor.comment_query().is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = SwiftExtractor;
    assert!(!extractor.middle_implementation_query().is_empty());
}

#[test]
fn capture_name_to_chunk_type_function() {
    assert_eq!(
        SwiftExtractor.capture_name_to_chunk_type("function"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_class() {
    assert_eq!(
        SwiftExtractor.capture_name_to_chunk_type("class"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_struct() {
    assert_eq!(
        SwiftExtractor.capture_name_to_chunk_type("struct"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_enum() {
    assert_eq!(
        SwiftExtractor.capture_name_to_chunk_type("enum"),
        Some(ChunkType::Enum)
    );
}

#[test]
fn capture_name_to_chunk_type_protocol_returns_interface() {
    assert_eq!(
        SwiftExtractor.capture_name_to_chunk_type("protocol"),
        Some(ChunkType::Interface)
    );
}

#[test]
fn capture_name_to_chunk_type_name_returns_code_block() {
    assert_eq!(
        SwiftExtractor.capture_name_to_chunk_type("name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(SwiftExtractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_for_loop_returns_loop() {
    assert_eq!(
        SwiftExtractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_while_loop_returns_loop() {
    assert_eq!(
        SwiftExtractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_if_block_returns_conditional() {
    assert_eq!(
        SwiftExtractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_switch_block_returns_conditional() {
    assert_eq!(
        SwiftExtractor.middle_capture_name_to_chunk_type("switch_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_do_block_returns_special_block() {
    assert_eq!(
        SwiftExtractor.middle_capture_name_to_chunk_type("do_block"),
        Some(ChunkType::SpecialBlock)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_guard_block_returns_special_block() {
    assert_eq!(
        SwiftExtractor.middle_capture_name_to_chunk_type("guard_block"),
        Some(ChunkType::SpecialBlock)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_function_call_returns_function_call() {
    assert_eq!(
        SwiftExtractor.middle_capture_name_to_chunk_type("function_call"),
        Some(ChunkType::FunctionCall)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_closure_returns_lambda() {
    assert_eq!(
        SwiftExtractor.middle_capture_name_to_chunk_type("closure"),
        Some(ChunkType::Lambda)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_code_block() {
    assert_eq!(
        SwiftExtractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(
        SwiftExtractor.middle_capture_name_to_chunk_type("unknown"),
        None
    );
}

#[test]
fn extract_name_for_function_declaration_returns_identifier() {
    let source = "func greet() {}\n";
    let tree = parse_swift(source);
    let function = find_node(tree.root_node(), "function_declaration")
        .expect("expected a function_declaration node");

    let name = SwiftExtractor.extract_name(function, source, "function");

    assert_eq!(name.as_deref(), Some("greet"));
}

#[test]
fn extract_name_for_class_declaration_returns_type_identifier() {
    let source = "class Foo {}\n";
    let tree = parse_swift(source);
    let class =
        find_node(tree.root_node(), "class_declaration").expect("expected a class_declaration");

    let name = SwiftExtractor.extract_name(class, source, "class");

    assert_eq!(name.as_deref(), Some("Foo"));
}

#[test]
fn extract_name_returns_none_for_leaf_node() {
    let source = "1\n";
    let tree = parse_swift(source);
    let root = tree.root_node();

    fn first_leaf<'tree>(node: Node<'tree>) -> Node<'tree> {
        if node.child_count() == 0 {
            return node;
        }
        first_leaf(node.child(0).unwrap())
    }

    let leaf = first_leaf(root);
    let name = SwiftExtractor.extract_name(leaf, source, "function");

    assert_eq!(name, None);
}
