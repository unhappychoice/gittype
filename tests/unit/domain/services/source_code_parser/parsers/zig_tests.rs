use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::zig::ZigExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_zig(source: &str) -> tree_sitter::Tree {
    let mut parser = ZigExtractor::create_parser().unwrap();
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
    let result = ZigExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = ZigExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = ZigExtractor;
    assert!(!extractor.query_patterns().is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = ZigExtractor;
    assert!(!extractor.comment_query().is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = ZigExtractor;
    assert!(!extractor.middle_implementation_query().is_empty());
}

#[test]
fn capture_name_to_chunk_type_function() {
    assert_eq!(
        ZigExtractor.capture_name_to_chunk_type("function"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_struct() {
    assert_eq!(
        ZigExtractor.capture_name_to_chunk_type("struct"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_enum() {
    assert_eq!(
        ZigExtractor.capture_name_to_chunk_type("enum"),
        Some(ChunkType::Enum)
    );
}

#[test]
fn capture_name_to_chunk_type_union_returns_struct() {
    assert_eq!(
        ZigExtractor.capture_name_to_chunk_type("union"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_name_returns_code_block() {
    assert_eq!(
        ZigExtractor.capture_name_to_chunk_type("name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(ZigExtractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_for_loop_returns_loop() {
    assert_eq!(
        ZigExtractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_while_loop_returns_loop() {
    assert_eq!(
        ZigExtractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_if_block_returns_conditional() {
    assert_eq!(
        ZigExtractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_switch_expr_returns_conditional() {
    assert_eq!(
        ZigExtractor.middle_capture_name_to_chunk_type("switch_expr"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_code_block() {
    assert_eq!(
        ZigExtractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(
        ZigExtractor.middle_capture_name_to_chunk_type("unknown"),
        None
    );
}

#[test]
fn extract_name_for_function_declaration_returns_identifier() {
    let source = "fn greet() void {}\n";
    let tree = parse_zig(source);
    let function = find_node(tree.root_node(), "function_declaration")
        .expect("expected a function_declaration node");

    let name = ZigExtractor.extract_name(function, source, "function");

    assert_eq!(name.as_deref(), Some("greet"));
}

#[test]
fn extract_name_returns_none_for_leaf_identifier() {
    let source = "x\n";
    let tree = parse_zig(source);

    fn first_leaf<'tree>(node: Node<'tree>) -> Node<'tree> {
        if node.child_count() == 0 {
            return node;
        }
        first_leaf(node.child(0).unwrap())
    }

    let leaf = first_leaf(tree.root_node());
    let name = ZigExtractor.extract_name(leaf, source, "function");

    assert_eq!(name, None);
}
