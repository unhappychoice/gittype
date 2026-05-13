use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::javascript::JavaScriptExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

#[test]
fn create_parser_succeeds() {
    let result = JavaScriptExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = JavaScriptExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = JavaScriptExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = JavaScriptExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = JavaScriptExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_maps_javascript_constructs() {
    let extractor = JavaScriptExtractor;

    assert_eq!(
        extractor.capture_name_to_chunk_type("function"),
        Some(ChunkType::Function)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("generator_function"),
        Some(ChunkType::Function)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("generator_function_expression"),
        Some(ChunkType::Function)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("method"),
        Some(ChunkType::Method)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("class"),
        Some(ChunkType::Class)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("arrow_function"),
        Some(ChunkType::Function)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("function_expression"),
        Some(ChunkType::Function)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("jsx_element"),
        Some(ChunkType::Component)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("jsx_self_closing_element"),
        Some(ChunkType::Component)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("variable"),
        Some(ChunkType::Variable)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("name"),
        Some(ChunkType::CodeBlock)
    );
    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_maps_javascript_blocks() {
    let extractor = JavaScriptExtractor;

    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("for_in_loop"),
        Some(ChunkType::Loop)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("do_while_loop"),
        Some(ChunkType::Loop)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("try_block"),
        Some(ChunkType::ErrorHandling)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("switch_block"),
        Some(ChunkType::Conditional)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("function_expr"),
        Some(ChunkType::Lambda)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("arrow_lambda"),
        Some(ChunkType::Lambda)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("function_call"),
        Some(ChunkType::FunctionCall)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn extract_name_reads_variable_function_names() {
    let extractor = JavaScriptExtractor;
    let source = "const build = () => true; const create = function () {};";
    let tree = JavaScriptExtractor::create_parser()
        .unwrap()
        .parse(source, None)
        .unwrap();
    let declarations = find_nodes(tree.root_node(), "variable_declarator");

    assert_eq!(
        extractor.extract_name(declarations[0], source, "arrow_function"),
        Some("build".to_string())
    );
    assert_eq!(
        extractor.extract_name(declarations[1], source, "function_expression"),
        Some("create".to_string())
    );
}

#[test]
fn extract_name_returns_none_for_member_assignment_method_name() {
    let extractor = JavaScriptExtractor;
    let source = "service.save = function () {};";
    let tree = JavaScriptExtractor::create_parser()
        .unwrap()
        .parse(source, None)
        .unwrap();
    let assignment = find_node(tree.root_node(), "assignment_expression").unwrap();

    assert_eq!(extractor.extract_name(assignment, source, "method"), None);
}

#[test]
fn extract_name_reads_self_closing_jsx_component_name() {
    let extractor = JavaScriptExtractor;
    let source = "const view = <Dashboard />;";
    let tree = JavaScriptExtractor::create_parser()
        .unwrap()
        .parse(source, None)
        .unwrap();
    let element = find_node(tree.root_node(), "jsx_self_closing_element").unwrap();

    assert_eq!(
        extractor.extract_name(element, source, "jsx_self_closing_element"),
        Some("Dashboard".to_string())
    );
}

#[test]
fn extract_name_returns_none_when_variable_declarator_uses_destructuring() {
    let extractor = JavaScriptExtractor;
    let source = "const { value } = source;";
    let tree = JavaScriptExtractor::create_parser()
        .unwrap()
        .parse(source, None)
        .unwrap();
    let declarator = find_node(tree.root_node(), "variable_declarator").unwrap();

    assert_eq!(
        extractor.extract_name(declarator, source, "arrow_function"),
        None
    );
}

#[test]
fn extract_name_returns_none_for_jsx_capture_when_node_has_no_identifier_child() {
    let extractor = JavaScriptExtractor;
    let source = "const value = 1;";
    let tree = JavaScriptExtractor::create_parser()
        .unwrap()
        .parse(source, None)
        .unwrap();

    assert_eq!(
        extractor.extract_name(tree.root_node(), source, "jsx_element"),
        None
    );
}

fn find_node<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    (node.kind() == kind).then_some(node).or_else(|| {
        let mut cursor = node.walk();
        let result = node
            .children(&mut cursor)
            .find_map(|child| find_node(child, kind));
        result
    })
}

fn find_nodes<'a>(node: Node<'a>, kind: &str) -> Vec<Node<'a>> {
    let mut nodes = (node.kind() == kind)
        .then_some(vec![node])
        .unwrap_or_default();
    let mut cursor = node.walk();
    nodes.extend(
        node.children(&mut cursor)
            .flat_map(|child| find_nodes(child, kind)),
    );
    nodes
}
