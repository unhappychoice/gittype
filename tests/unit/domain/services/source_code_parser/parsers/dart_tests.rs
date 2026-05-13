use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::dart::DartExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_dart(source: &str) -> tree_sitter::Tree {
    let mut parser = DartExtractor::create_parser().unwrap();
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
    let result = DartExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = DartExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = DartExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = DartExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = DartExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_function() {
    assert_eq!(
        DartExtractor.capture_name_to_chunk_type("function"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_method() {
    assert_eq!(
        DartExtractor.capture_name_to_chunk_type("method"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_class() {
    assert_eq!(
        DartExtractor.capture_name_to_chunk_type("class"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_enum() {
    assert_eq!(
        DartExtractor.capture_name_to_chunk_type("enum"),
        Some(ChunkType::Enum)
    );
}

#[test]
fn capture_name_to_chunk_type_mixin_returns_class() {
    assert_eq!(
        DartExtractor.capture_name_to_chunk_type("mixin"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_extension_returns_class() {
    assert_eq!(
        DartExtractor.capture_name_to_chunk_type("extension"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_variable() {
    assert_eq!(
        DartExtractor.capture_name_to_chunk_type("variable"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_name_returns_code_block() {
    assert_eq!(
        DartExtractor.capture_name_to_chunk_type("name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(DartExtractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_for_loop_returns_loop() {
    assert_eq!(
        DartExtractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_while_loop_returns_loop() {
    assert_eq!(
        DartExtractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_if_block_returns_conditional() {
    assert_eq!(
        DartExtractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_switch_block_returns_conditional() {
    assert_eq!(
        DartExtractor.middle_capture_name_to_chunk_type("switch_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_try_block_returns_error_handling() {
    assert_eq!(
        DartExtractor.middle_capture_name_to_chunk_type("try_block"),
        Some(ChunkType::ErrorHandling)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_code_block() {
    assert_eq!(
        DartExtractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(
        DartExtractor.middle_capture_name_to_chunk_type("unknown"),
        None
    );
}

#[test]
fn extract_name_for_class_declaration_returns_identifier() {
    let source = "class Foo {}\n";
    let tree = parse_dart(source);
    let class =
        find_node(tree.root_node(), "class_declaration").expect("expected a class_declaration");

    let name = DartExtractor.extract_name(class, source, "class");

    assert_eq!(name.as_deref(), Some("Foo"));
}

#[test]
fn extract_name_for_function_declaration_recurses_into_signature() {
    let source = "void greet() {}\n";
    let tree = parse_dart(source);
    let func = find_node(tree.root_node(), "function_signature")
        .expect("expected a function_signature node");

    let name = DartExtractor.extract_name(func, source, "function");

    assert_eq!(name.as_deref(), Some("greet"));
}

#[test]
fn extract_name_returns_none_for_leaf_node() {
    let source = "1\n";
    let tree = parse_dart(source);

    fn first_leaf<'tree>(node: Node<'tree>) -> Node<'tree> {
        if node.child_count() == 0 {
            return node;
        }
        first_leaf(node.child(0).unwrap())
    }

    let leaf = first_leaf(tree.root_node());
    let name = DartExtractor.extract_name(leaf, source, "function");

    assert_eq!(name, None);
}

#[test]
fn extract_name_recurses_into_function_signature_child() {
    let source = "void greet() {}\n";
    let tree = parse_dart(source);
    let func_decl = find_node(tree.root_node(), "function_declaration")
        .expect("expected a function_declaration node");
    assert!(func_decl
        .child(0)
        .map(|child| child.kind() != "identifier")
        .unwrap_or(false));

    let name = DartExtractor.extract_name(func_decl, source, "function");

    assert_eq!(name.as_deref(), Some("greet"));
}

#[test]
fn extract_name_recurses_into_initialized_variable_definition_child() {
    let source = "void main() { int counter = 5; }\n";
    let tree = parse_dart(source);
    let local_decl = find_node(tree.root_node(), "local_variable_declaration")
        .or_else(|| find_node(tree.root_node(), "initialized_variable_definition"))
        .expect("expected a variable declaration node");

    let name = DartExtractor.extract_name(local_decl, source, "variable");

    assert_eq!(name.as_deref(), Some("counter"));
}
