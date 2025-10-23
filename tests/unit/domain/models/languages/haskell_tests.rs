use gittype::domain::models::languages::haskell::Haskell;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Haskell;
    assert_eq!(lang.name(), "haskell");
}

#[test]
fn test_extensions() {
    let lang = Haskell;
    let exts = lang.extensions();
    assert!(exts.contains(&"hs"));
    assert!(exts.contains(&"lhs"));
}

#[test]
fn test_display_name() {
    let lang = Haskell;
    assert_eq!(lang.display_name(), "Haskell");
}

#[test]
fn test_color() {
    let lang = Haskell;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Haskell;
    let code = "-- comment\nmain = return ()";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_haskell::LANGUAGE.into())
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
    let lang = Haskell;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Haskell;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Haskell"));
}
