use gittype::domain::models::languages::swift::Swift;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Swift;
    assert_eq!(lang.name(), "swift");
}

#[test]
fn test_extensions() {
    let lang = Swift;
    assert_eq!(lang.extensions(), vec!["swift"]);
}

#[test]
fn test_display_name() {
    let lang = Swift;
    assert_eq!(lang.display_name(), "Swift");
}

#[test]
fn test_color() {
    let lang = Swift;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Swift;
    let code = "// comment\nfunc test() { }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_swift::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(code, None).unwrap();
    let root = tree.root_node();

    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "comment" || child.kind() == "multiline_comment" {
            assert!(lang.is_valid_comment_node(child));
        }
    }
}

#[test]
fn test_clone() {
    let lang = Swift;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Swift;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Swift"));
}
