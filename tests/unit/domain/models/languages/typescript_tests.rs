use gittype::domain::models::languages::typescript::TypeScript;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = TypeScript;
    assert_eq!(lang.name(), "typescript");
}

#[test]
fn test_extensions() {
    let lang = TypeScript;
    let exts = lang.extensions();
    assert!(exts.contains(&"ts"));
    assert!(exts.contains(&"tsx"));
}

#[test]
fn test_display_name() {
    let lang = TypeScript;
    assert_eq!(lang.display_name(), "TypeScript");
}

#[test]
fn test_color() {
    let lang = TypeScript;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = TypeScript;
    let code = "// comment\nfunction test() { }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
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
    let lang = TypeScript;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = TypeScript;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("TypeScript"));
}
