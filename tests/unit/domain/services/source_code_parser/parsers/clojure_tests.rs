use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::clojure::ClojureExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_clojure(source: &str) -> tree_sitter::Tree {
    let mut parser = ClojureExtractor::create_parser().unwrap();
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
    let result = ClojureExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = ClojureExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = ClojureExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = ClojureExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn query_patterns_matches_defn() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("defn"));
}

#[test]
fn query_patterns_matches_defmacro() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("defmacro"));
}

#[test]
fn query_patterns_matches_defn_dash() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("defn-"));
}

#[test]
fn query_patterns_matches_deftype() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("deftype"));
}

#[test]
fn query_patterns_matches_defprotocol() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("defprotocol"));
}

#[test]
fn query_patterns_matches_defrecord() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("defrecord"));
}

#[test]
fn query_patterns_matches_def() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("def"));
}

#[test]
fn query_patterns_matches_ns() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("ns"));
}

#[test]
fn query_uses_match_predicates() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    let match_count = patterns.matches("#match?").count();
    assert_eq!(
        match_count, 5,
        "Should use five #match? predicates for different definition types"
    );
}

#[test]
fn query_does_not_use_multiple_eq_predicates() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    let eq_count = patterns.matches("#eq?").count();
    assert_eq!(eq_count, 0, "Should not use #eq? predicates");
}

// ---------------------------------------------------------------------------
// capture_name_to_chunk_type — exhaustive enumeration of mapped capture names
// ---------------------------------------------------------------------------

#[test]
fn capture_name_to_chunk_type_function_def() {
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("function_def"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_variable_def() {
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("variable_def"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_namespace_def() {
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("namespace_def"),
        Some(ChunkType::Namespace)
    );
}

#[test]
fn capture_name_to_chunk_type_class_def() {
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("class_def"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_interface_def() {
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("interface_def"),
        Some(ChunkType::Interface)
    );
}

#[test]
fn capture_name_to_chunk_type_func_name_returns_code_block() {
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("func_name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_var_name_returns_code_block() {
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("var_name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_ns_name_returns_code_block() {
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("ns_name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_class_name_returns_code_block() {
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("class_name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_interface_name_returns_code_block() {
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("interface_name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(ClojureExtractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn capture_name_to_chunk_type_keyword_captures_return_none() {
    // The *_keyword capture names are emitted in queries but not mapped — they
    // should fall through the wildcard arm to None.
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("func_keyword"),
        None
    );
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("var_keyword"),
        None
    );
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("ns_keyword"),
        None
    );
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("class_keyword"),
        None
    );
    assert_eq!(
        ClojureExtractor.capture_name_to_chunk_type("interface_keyword"),
        None
    );
}

// ---------------------------------------------------------------------------
// middle_capture_name_to_chunk_type
// ---------------------------------------------------------------------------

#[test]
fn middle_capture_name_to_chunk_type_expr() {
    assert_eq!(
        ClojureExtractor.middle_capture_name_to_chunk_type("expr"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_function_call() {
    assert_eq!(
        ClojureExtractor.middle_capture_name_to_chunk_type("function_call"),
        Some(ChunkType::FunctionCall)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(
        ClojureExtractor.middle_capture_name_to_chunk_type("unknown"),
        None
    );
}

// ---------------------------------------------------------------------------
// extract_name — exercises the recursive walk that finds the 2nd sym_lit
// ---------------------------------------------------------------------------

#[test]
fn extract_name_for_defn_returns_function_name() {
    let source = "(defn hello [x] x)\n";
    let tree = parse_clojure(source);
    let list = find_node(tree.root_node(), "list_lit").expect("expected a list_lit node");

    let name = ClojureExtractor.extract_name(list, source, "function_def");

    assert_eq!(name.as_deref(), Some("hello"));
}

#[test]
fn extract_name_for_def_returns_variable_name() {
    let source = "(def answer 42)\n";
    let tree = parse_clojure(source);
    let list = find_node(tree.root_node(), "list_lit").expect("expected a list_lit node");

    let name = ClojureExtractor.extract_name(list, source, "variable_def");

    assert_eq!(name.as_deref(), Some("answer"));
}

#[test]
fn extract_name_for_ns_returns_namespace_name() {
    let source = "(ns my.app.core)\n";
    let tree = parse_clojure(source);
    let list = find_node(tree.root_node(), "list_lit").expect("expected a list_lit node");

    let name = ClojureExtractor.extract_name(list, source, "namespace_def");

    assert_eq!(name.as_deref(), Some("my.app.core"));
}

#[test]
fn extract_name_for_deftype_returns_class_name() {
    let source = "(deftype Point [x y])\n";
    let tree = parse_clojure(source);
    let list = find_node(tree.root_node(), "list_lit").expect("expected a list_lit node");

    let name = ClojureExtractor.extract_name(list, source, "class_def");

    assert_eq!(name.as_deref(), Some("Point"));
}

#[test]
fn extract_name_for_defprotocol_returns_interface_name() {
    let source = "(defprotocol Greeter (greet [this]))\n";
    let tree = parse_clojure(source);
    let list = find_node(tree.root_node(), "list_lit").expect("expected a list_lit node");

    let name = ClojureExtractor.extract_name(list, source, "interface_def");

    assert_eq!(name.as_deref(), Some("Greeter"));
}

#[test]
fn extract_name_capture_name_is_ignored() {
    // extract_clojure_name walks structure regardless of capture_name — pass an
    // arbitrary string and the result should still be the 2nd sym_lit.
    let source = "(defn hello [x] x)\n";
    let tree = parse_clojure(source);
    let list = find_node(tree.root_node(), "list_lit").expect("expected a list_lit node");

    let name = ClojureExtractor.extract_name(list, source, "totally-bogus-capture");

    assert_eq!(name.as_deref(), Some("hello"));
}

#[test]
fn extract_name_returns_none_for_list_with_single_sym() {
    // A list containing only one symbol has no second sym_lit, so the walk
    // exits without returning a name.
    let source = "(only-one-symbol)\n";
    let tree = parse_clojure(source);
    let list = find_node(tree.root_node(), "list_lit").expect("expected a list_lit node");

    let name = ClojureExtractor.extract_name(list, source, "function_def");

    assert_eq!(name, None);
}

#[test]
fn extract_name_returns_none_for_leaf_without_children() {
    // A bare numeric literal has no child symbols, so goto_first_child returns
    // false and the function returns None directly.
    let source = "42\n";
    let tree = parse_clojure(source);
    let leaf = find_node(tree.root_node(), "num_lit")
        .or_else(|| find_node(tree.root_node(), "number_literal"))
        .expect("expected a numeric leaf node");

    let name = ClojureExtractor.extract_name(leaf, source, "function_def");

    assert_eq!(name, None);
}
