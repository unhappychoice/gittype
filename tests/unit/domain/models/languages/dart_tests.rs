use gittype::domain::models::languages::dart::Dart;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Dart;
    assert_eq!(lang.name(), "dart");
}

#[test]
fn test_extensions() {
    let lang = Dart;
    assert_eq!(lang.extensions(), vec!["dart"]);
}

#[test]
fn test_display_name() {
    let lang = Dart;
    assert_eq!(lang.display_name(), "Dart");
}

#[test]
fn test_color() {
    let lang = Dart;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Dart;
    let code = "// comment\n/// doc comment\nvoid main() { }";
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_dart::language()).unwrap();
    let tree = parser.parse(code, None).unwrap();
    let root = tree.root_node();

    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "comment" || child.kind() == "documentation_comment" {
            assert!(lang.is_valid_comment_node(child));
        }
    }
}

#[test]
fn test_clone() {
    let lang = Dart;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Dart;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Dart"));
}
