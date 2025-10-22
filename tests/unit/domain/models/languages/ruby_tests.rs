use gittype::domain::models::languages::ruby::Ruby;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Ruby;
    assert_eq!(lang.name(), "ruby");
}

#[test]
fn test_extensions() {
    let lang = Ruby;
    assert_eq!(lang.extensions(), vec!["rb"]);
}

#[test]
fn test_display_name() {
    let lang = Ruby;
    assert_eq!(lang.display_name(), "Ruby");
}

#[test]
fn test_color() {
    let lang = Ruby;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Ruby;
    let code = "# comment\ndef test; end";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_ruby::LANGUAGE.into())
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
    let lang = Ruby;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Ruby;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Ruby"));
}
