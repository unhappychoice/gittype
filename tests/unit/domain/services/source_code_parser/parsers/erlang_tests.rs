use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::erlang::ErlangExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_erlang(source: &str) -> tree_sitter::Tree {
    let mut parser = ErlangExtractor::create_parser().unwrap();
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
    let result = ErlangExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = ErlangExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = ErlangExtractor;
    assert!(!extractor.query_patterns().is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = ErlangExtractor;
    assert!(!extractor.comment_query().is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = ErlangExtractor;
    assert!(!extractor.middle_implementation_query().is_empty());
}

#[test]
fn capture_name_to_chunk_type_function() {
    assert_eq!(
        ErlangExtractor.capture_name_to_chunk_type("function"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_module_attr() {
    assert_eq!(
        ErlangExtractor.capture_name_to_chunk_type("module_attr"),
        Some(ChunkType::Module)
    );
}

#[test]
fn capture_name_to_chunk_type_export_attr() {
    assert_eq!(
        ErlangExtractor.capture_name_to_chunk_type("export_attr"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_record_decl() {
    assert_eq!(
        ErlangExtractor.capture_name_to_chunk_type("record_decl"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_type_alias() {
    assert_eq!(
        ErlangExtractor.capture_name_to_chunk_type("type_alias"),
        Some(ChunkType::TypeAlias)
    );
}

#[test]
fn capture_name_to_chunk_type_spec_decl() {
    assert_eq!(
        ErlangExtractor.capture_name_to_chunk_type("spec_decl"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_behaviour_attr() {
    assert_eq!(
        ErlangExtractor.capture_name_to_chunk_type("behaviour_attr"),
        Some(ChunkType::Interface)
    );
}

#[test]
fn capture_name_to_chunk_type_func_name_returns_none() {
    assert_eq!(
        ErlangExtractor.capture_name_to_chunk_type("func_name"),
        None
    );
}

#[test]
fn capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(ErlangExtractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_case_expr() {
    assert_eq!(
        ErlangExtractor.middle_capture_name_to_chunk_type("case_expr"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_if_expr() {
    assert_eq!(
        ErlangExtractor.middle_capture_name_to_chunk_type("if_expr"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_receive_expr() {
    assert_eq!(
        ErlangExtractor.middle_capture_name_to_chunk_type("receive_expr"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_try_expr() {
    assert_eq!(
        ErlangExtractor.middle_capture_name_to_chunk_type("try_expr"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_anonymous_fn() {
    assert_eq!(
        ErlangExtractor.middle_capture_name_to_chunk_type("anonymous_fn"),
        Some(ChunkType::Lambda)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(
        ErlangExtractor.middle_capture_name_to_chunk_type("unknown"),
        None
    );
}

#[test]
fn extract_name_for_function_returns_function_atom() {
    let source = "-module(m).\nhello(Name) -> Name.\n";
    let tree = parse_erlang(source);
    let func = find_node(tree.root_node(), "fun_decl").expect("expected a fun_decl node");

    let name = ErlangExtractor.extract_name(func, source, "function");

    assert_eq!(name.as_deref(), Some("hello"));
}

#[test]
fn extract_name_for_module_attr_returns_module_atom() {
    let source = "-module(greeter).\n";
    let tree = parse_erlang(source);
    let module_attr =
        find_node(tree.root_node(), "module_attribute").expect("expected a module_attribute node");

    let name = ErlangExtractor.extract_name(module_attr, source, "module_attr");

    assert_eq!(name.as_deref(), Some("greeter"));
}

#[test]
fn extract_name_for_record_decl_returns_record_atom() {
    let source = "-module(m).\n-record(user, {name, email}).\n";
    let tree = parse_erlang(source);
    let record = find_node(tree.root_node(), "record_decl").expect("expected a record_decl node");

    let name = ErlangExtractor.extract_name(record, source, "record_decl");

    assert_eq!(name.as_deref(), Some("user"));
}

#[test]
fn extract_name_for_behaviour_attr_returns_behaviour_atom() {
    let source = "-module(m).\n-behaviour(gen_server).\n";
    let tree = parse_erlang(source);
    let behaviour = find_node(tree.root_node(), "behaviour_attribute")
        .expect("expected a behaviour_attribute node");

    let name = ErlangExtractor.extract_name(behaviour, source, "behaviour_attr");

    assert_eq!(name.as_deref(), Some("gen_server"));
}

#[test]
fn extract_name_for_unknown_capture_returns_node_text() {
    let source = "-module(m).\n";
    let tree = parse_erlang(source);
    let module_attr =
        find_node(tree.root_node(), "module_attribute").expect("expected a module_attribute node");

    let name = ErlangExtractor.extract_name(module_attr, source, "unknown_capture");

    assert_eq!(name.as_deref(), Some("-module(m)."));
}

#[test]
fn extract_atom_child_returns_none_when_no_atom_present() {
    let source = "1.\n";
    let tree = parse_erlang(source);
    let root = tree.root_node();

    let name = ErlangExtractor.extract_name(root, source, "module_attr");

    assert!(name.is_none());
}
