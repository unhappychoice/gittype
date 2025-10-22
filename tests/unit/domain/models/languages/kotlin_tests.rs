use gittype::domain::models::languages::kotlin::Kotlin;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Kotlin;
    assert_eq!(lang.name(), "kotlin");
}

#[test]
fn test_extensions() {
    let lang = Kotlin;
    let exts = lang.extensions();
    assert!(exts.contains(&"kt"));
    assert!(exts.contains(&"kts"));
}

#[test]
fn test_display_name() {
    let lang = Kotlin;
    assert_eq!(lang.display_name(), "Kotlin");
}

#[test]
fn test_color() {
    let lang = Kotlin;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Kotlin;
    let code = "// comment\nfun main() { }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_kotlin_ng::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(code, None).unwrap();
    let root = tree.root_node();

    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "line_comment" || child.kind() == "multiline_comment" {
            assert!(lang.is_valid_comment_node(child));
        }
    }
}

#[test]
fn test_clone() {
    let lang = Kotlin;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Kotlin;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Kotlin"));
}
