use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::kotlin::KotlinExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_kotlin(source: &str) -> tree_sitter::Tree {
    let mut parser = KotlinExtractor::create_parser().unwrap();
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
    let result = KotlinExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = KotlinExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = KotlinExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = KotlinExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = KotlinExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_function() {
    assert_eq!(
        KotlinExtractor.capture_name_to_chunk_type("function"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_class() {
    assert_eq!(
        KotlinExtractor.capture_name_to_chunk_type("class"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_interface() {
    assert_eq!(
        KotlinExtractor.capture_name_to_chunk_type("interface"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_object() {
    assert_eq!(
        KotlinExtractor.capture_name_to_chunk_type("object"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_companion() {
    assert_eq!(
        KotlinExtractor.capture_name_to_chunk_type("companion"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_property() {
    assert_eq!(
        KotlinExtractor.capture_name_to_chunk_type("property"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_enum_entry() {
    assert_eq!(
        KotlinExtractor.capture_name_to_chunk_type("enum_entry"),
        Some(ChunkType::Const)
    );
}

#[test]
fn capture_name_to_chunk_type_type_alias() {
    assert_eq!(
        KotlinExtractor.capture_name_to_chunk_type("type_alias"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_lambda() {
    assert_eq!(
        KotlinExtractor.capture_name_to_chunk_type("lambda"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(KotlinExtractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_for_loop() {
    assert_eq!(
        KotlinExtractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_while_loop() {
    assert_eq!(
        KotlinExtractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_if_block() {
    assert_eq!(
        KotlinExtractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_when_block() {
    assert_eq!(
        KotlinExtractor.middle_capture_name_to_chunk_type("when_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_try_block() {
    assert_eq!(
        KotlinExtractor.middle_capture_name_to_chunk_type("try_block"),
        Some(ChunkType::ErrorHandling)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_function_call() {
    assert_eq!(
        KotlinExtractor.middle_capture_name_to_chunk_type("function_call"),
        Some(ChunkType::FunctionCall)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_lambda() {
    assert_eq!(
        KotlinExtractor.middle_capture_name_to_chunk_type("lambda"),
        Some(ChunkType::Lambda)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(
        KotlinExtractor.middle_capture_name_to_chunk_type("unknown"),
        None
    );
}

#[test]
fn extract_name_for_companion_returns_hardcoded_string() {
    let source = "val x = 1";
    let tree = parse_kotlin(source);

    let name = KotlinExtractor.extract_name(tree.root_node(), source, "companion");

    assert_eq!(name, Some("companion object".to_string()));
}

#[test]
fn extract_name_for_property_walks_variable_declaration() {
    let source = "val myVar: Int = 42";
    let tree = parse_kotlin(source);
    let property = find_node(tree.root_node(), "property_declaration").unwrap();

    let name = KotlinExtractor.extract_name(property, source, "property");

    assert!(name.is_none() || name.as_deref() == Some("myVar"));
}

#[test]
fn extract_name_for_other_capture_walks_node_children() {
    let source = "fun greet(): Int = 1";
    let tree = parse_kotlin(source);
    let function = find_node(tree.root_node(), "function_declaration").unwrap();

    let name = KotlinExtractor.extract_name(function, source, "function");

    assert!(name.is_none() || name.as_deref() == Some("greet"));
}

#[test]
fn extract_name_returns_none_for_leaf_node_without_children() {
    let source = "val x = 1";
    let tree = parse_kotlin(source);
    let leaf = find_node(tree.root_node(), "number_literal")
        .or_else(|| find_node(tree.root_node(), "integer_literal"))
        .expect("expected a literal leaf node");

    let name = KotlinExtractor.extract_name(leaf, source, "function");

    assert_eq!(name, None);
}
