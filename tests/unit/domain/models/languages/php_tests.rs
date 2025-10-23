use gittype::domain::models::languages::php::Php;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Php;
    assert_eq!(lang.name(), "php");
}

#[test]
fn test_extensions() {
    let lang = Php;
    let exts = lang.extensions();
    assert!(exts.contains(&"php"));
    assert!(exts.contains(&"phtml"));
}

#[test]
fn test_display_name() {
    let lang = Php;
    assert_eq!(lang.display_name(), "PHP");
}

#[test]
fn test_color() {
    let lang = Php;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Php;

    // Test regular comment
    let code1 = "<?php\n// comment\nfunction test() { }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_php::LANGUAGE_PHP.into())
        .unwrap();
    let tree1 = parser.parse(code1, None).unwrap();
    let root1 = tree1.root_node();

    let mut cursor1 = root1.walk();
    for child in root1.children(&mut cursor1) {
        if child.kind() == "comment" {
            assert!(lang.is_valid_comment_node(child));
        }
    }

    // Test shell comment
    let code2 = "<?php\n# shell comment\nfunction test() { }";
    let tree2 = parser.parse(code2, None).unwrap();
    let root2 = tree2.root_node();

    let mut cursor2 = root2.walk();
    for child in root2.children(&mut cursor2) {
        if child.kind() == "shell_comment_line" {
            assert!(lang.is_valid_comment_node(child));
        }
    }
}

#[test]
fn test_clone() {
    let lang = Php;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Php;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Php"));
}
