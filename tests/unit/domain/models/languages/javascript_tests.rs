use gittype::domain::models::languages::javascript::JavaScript;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = JavaScript;
    assert_eq!(lang.name(), "javascript");
}

#[test]
fn test_extensions() {
    let lang = JavaScript;
    let exts = lang.extensions();
    assert!(exts.contains(&"js"));
    assert!(exts.contains(&"mjs"));
}

#[test]
fn test_display_name() {
    let lang = JavaScript;
    assert_eq!(lang.display_name(), "JavaScript");
}

#[test]
fn test_color() {
    let lang = JavaScript;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = JavaScript;
    let code = "// comment\nfunction test() { }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(code, None).unwrap();
    let root = tree.root_node();

    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "comment" {
            assert!(lang.is_valid_comment_node(child));
        }
    }
}

#[test]
fn test_clone() {
    let lang = JavaScript;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = JavaScript;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("JavaScript"));
}
