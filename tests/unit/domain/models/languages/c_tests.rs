use gittype::domain::models::languages::c::C;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = C;
    assert_eq!(lang.name(), "c");
}

#[test]
fn test_extensions() {
    let lang = C;
    assert_eq!(lang.extensions(), vec!["c", "h"]);
}

#[test]
fn test_display_name() {
    let lang = C;
    assert_eq!(lang.display_name(), "C");
}

#[test]
fn test_color() {
    let lang = C;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = C;
    let code = "// comment\nint main() { return 0; }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
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
    let lang = C;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = C;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("C"));
}

#[test]
fn test_hash() {
    use std::collections::HashSet;
    let lang = C;
    let mut set = HashSet::new();
    set.insert(lang);
    assert!(set.contains(&C));
}
