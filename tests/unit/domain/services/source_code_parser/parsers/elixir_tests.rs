use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::elixir::ElixirExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_elixir(source: &str) -> tree_sitter::Tree {
    let mut parser = ElixirExtractor::create_parser().unwrap();
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
    let result = ElixirExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = ElixirExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = ElixirExtractor;
    assert!(!extractor.query_patterns().is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = ElixirExtractor;
    assert!(!extractor.comment_query().is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = ElixirExtractor;
    assert!(!extractor.middle_implementation_query().is_empty());
}

#[test]
fn capture_name_to_chunk_type_function() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("function"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_module() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("module"),
        Some(ChunkType::Module)
    );
}

#[test]
fn capture_name_to_chunk_type_macro_def() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("macro_def"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_protocol() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("protocol"),
        Some(ChunkType::Interface)
    );
}

#[test]
fn capture_name_to_chunk_type_impl_def() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("impl_def"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_struct_def() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("struct_def"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_guard_def() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("guard_def"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_func_name_returns_code_block() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("func_name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_module_name_returns_code_block() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("module_name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_macro_name_returns_code_block() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("macro_name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_protocol_name_returns_code_block() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("protocol_name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_guard_name_returns_code_block() {
    assert_eq!(
        ElixirExtractor.capture_name_to_chunk_type("guard_name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(ElixirExtractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_control_flow() {
    assert_eq!(
        ElixirExtractor.middle_capture_name_to_chunk_type("control_flow"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_function_call() {
    assert_eq!(
        ElixirExtractor.middle_capture_name_to_chunk_type("function_call"),
        Some(ChunkType::FunctionCall)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_anonymous_fn() {
    assert_eq!(
        ElixirExtractor.middle_capture_name_to_chunk_type("anonymous_fn"),
        Some(ChunkType::Lambda)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(
        ElixirExtractor.middle_capture_name_to_chunk_type("unknown"),
        None
    );
}

#[test]
fn extract_name_for_call_returns_target_identifier() {
    let source = "defmodule Foo do\nend\n";
    let tree = parse_elixir(source);
    let call = find_node(tree.root_node(), "call").expect("expected a call node");

    let name = ElixirExtractor.extract_name(call, source, "module");

    assert_eq!(name.as_deref(), Some("defmodule"));
}

#[test]
fn extract_name_for_arguments_with_alias_returns_alias_text() {
    let source = "defmodule Foo do\nend\n";
    let tree = parse_elixir(source);
    let arguments = find_node(tree.root_node(), "arguments").expect("expected an arguments node");

    let name = ElixirExtractor.extract_name(arguments, source, "module_name");

    assert_eq!(name.as_deref(), Some("Foo"));
}

#[test]
fn extract_name_walks_siblings_when_no_identifier_or_alias_present() {
    let source = "def hello(name), do: name\n";
    let tree = parse_elixir(source);
    let arguments = find_node(tree.root_node(), "arguments").expect("expected an arguments node");

    let name = ElixirExtractor.extract_name(arguments, source, "func_name");

    assert_eq!(name, None);
}

#[test]
fn extract_name_returns_none_for_leaf_node_without_children() {
    let source = "1\n";
    let tree = parse_elixir(source);
    let leaf = find_node(tree.root_node(), "integer")
        .or_else(|| find_node(tree.root_node(), "integer_literal"))
        .or_else(|| find_node(tree.root_node(), "number"))
        .expect("expected a numeric leaf node");

    let name = ElixirExtractor.extract_name(leaf, source, "function");

    assert_eq!(name, None);
}

#[test]
fn extract_name_for_dot_target_call_returns_first_argument_identifier() {
    let source = "Foo.bar(x)\n";
    let tree = parse_elixir(source);
    let call = find_node(tree.root_node(), "call").expect("expected a call node");

    let name = ElixirExtractor.extract_name(call, source, "function_call");

    assert_eq!(name.as_deref(), Some("x"));
}

#[test]
fn extract_name_for_dot_target_call_with_no_identifier_args_returns_none() {
    let source = "Foo.bar(:atom)\n";
    let tree = parse_elixir(source);
    let call = find_node(tree.root_node(), "call").expect("expected a call node");

    let name = ElixirExtractor.extract_name(call, source, "function_call");

    assert_eq!(name, None);
}
