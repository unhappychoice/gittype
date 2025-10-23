use gittype::domain::models::languages::rust::Rust;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Rust;
    assert_eq!(lang.name(), "rust");
}

#[test]
fn test_extensions() {
    let lang = Rust;
    assert_eq!(lang.extensions(), vec!["rs"]);
}

#[test]
fn test_display_name() {
    let lang = Rust;
    assert_eq!(lang.display_name(), "Rust");
}

#[test]
fn test_color() {
    let lang = Rust;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Rust;
    let code = "// comment\nfn main() { }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
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
    let lang = Rust;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Rust;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Rust"));
}
