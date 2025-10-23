use gittype::domain::models::languages::cpp::Cpp;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Cpp;
    assert_eq!(lang.name(), "cpp");
}

#[test]
fn test_extensions() {
    let lang = Cpp;
    let exts = lang.extensions();
    assert!(exts.contains(&"cpp"));
    assert!(exts.contains(&"cc"));
}

#[test]
fn test_display_name() {
    let lang = Cpp;
    assert_eq!(lang.display_name(), "C++");
}

#[test]
fn test_color() {
    let lang = Cpp;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Cpp;
    let code = "// comment\nint main() { return 0; }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_cpp::LANGUAGE.into())
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
    let lang = Cpp;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Cpp;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Cpp"));
}
