use gittype::domain::models::languages::python::Python;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Python;
    assert_eq!(lang.name(), "python");
}

#[test]
fn test_extensions() {
    let lang = Python;
    assert_eq!(lang.extensions(), vec!["py"]);
}

#[test]
fn test_display_name() {
    let lang = Python;
    assert_eq!(lang.display_name(), "Python");
}

#[test]
fn test_color() {
    let lang = Python;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Python;
    let code = "# comment\ndef test(): pass";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
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
    let lang = Python;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Python;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Python"));
}
