use gittype::domain::models::languages::go::Go;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Go;
    assert_eq!(lang.name(), "go");
}

#[test]
fn test_extensions() {
    let lang = Go;
    assert_eq!(lang.extensions(), vec!["go"]);
}

#[test]
fn test_display_name() {
    let lang = Go;
    assert_eq!(lang.display_name(), "Go");
}

#[test]
fn test_color() {
    let lang = Go;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Go;
    let code = "// comment\nfunc main() { }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_go::LANGUAGE.into())
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
    let lang = Go;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Go;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Go"));
}
