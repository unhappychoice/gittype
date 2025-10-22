use gittype::domain::models::languages::zig::Zig;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Zig;
    assert_eq!(lang.name(), "zig");
}

#[test]
fn test_extensions() {
    let lang = Zig;
    assert_eq!(lang.extensions(), vec!["zig"]);
}

#[test]
fn test_display_name() {
    let lang = Zig;
    assert_eq!(lang.display_name(), "Zig");
}

#[test]
fn test_color() {
    let lang = Zig;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Zig;
    let code = "// comment\npub fn main() void { }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_zig::LANGUAGE.into())
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
    let lang = Zig;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Zig;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Zig"));
}
