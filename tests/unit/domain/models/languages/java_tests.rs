use gittype::domain::models::languages::java::Java;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Java;
    assert_eq!(lang.name(), "java");
}

#[test]
fn test_extensions() {
    let lang = Java;
    assert_eq!(lang.extensions(), vec!["java"]);
}

#[test]
fn test_display_name() {
    let lang = Java;
    assert_eq!(lang.display_name(), "Java");
}

#[test]
fn test_color() {
    let lang = Java;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Java;
    let code = "// comment\nclass Test { }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_java::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(code, None).unwrap();
    let root = tree.root_node();

    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "line_comment" || child.kind() == "block_comment" {
            assert!(lang.is_valid_comment_node(child));
        }
    }
}

#[test]
fn test_clone() {
    let lang = Java;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Java;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Java"));
}
