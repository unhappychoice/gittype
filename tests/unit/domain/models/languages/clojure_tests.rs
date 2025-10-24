use gittype::domain::models::languages::clojure::Clojure;
use gittype::domain::models::Language;

#[test]
fn test_name() {
    let lang = Clojure;
    assert_eq!(lang.name(), "clojure");
}

#[test]
fn test_extensions() {
    let lang = Clojure;
    let exts = lang.extensions();
    assert!(exts.contains(&"clj"));
    assert!(exts.contains(&"cljs"));
    assert!(exts.contains(&"cljc"));
}

#[test]
fn test_display_name() {
    let lang = Clojure;
    assert_eq!(lang.display_name(), "Clojure");
}

#[test]
fn test_color() {
    let lang = Clojure;
    let _ = lang.color();
}

#[test]
fn test_is_valid_comment_node() {
    let lang = Clojure;
    let code = ";; comment\n(defn test [] nil)";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_clojure::LANGUAGE.into())
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
    let lang = Clojure;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_debug() {
    let lang = Clojure;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("Clojure"));
}
