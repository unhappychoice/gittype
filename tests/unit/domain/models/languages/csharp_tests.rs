use gittype::domain::models::languages::csharp::CSharp;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = CSharp;
    assert_eq!(lang.name(), "csharp");
}

#[test]
fn test_extensions() {
    let lang = CSharp;
    let exts = lang.extensions();
    assert!(exts.contains(&"cs"));
    assert!(exts.contains(&"csx"));
}

#[test]
fn test_display_name() {
    let lang = CSharp;
    assert_eq!(lang.display_name(), "C#");
}

#[test]
fn test_color() {
    let lang = CSharp;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = CSharp;
    let code = "// comment\nclass Test { }";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_c_sharp::LANGUAGE.into())
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
    let lang = CSharp;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = CSharp;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("CSharp"));
}
