use crate::typing_core_test_with_parser;
use gittype::domain::models::typing::ProcessingOptions;

typing_core_test_with_parser!(
    clojure_function_with_comment,
    "clojure",
    r#"(defn greet [name]
  ; This is a comment
  (str "Hello, " name))"#
);

typing_core_test_with_parser!(
    clojure_macro_with_line_comments,
    "clojure",
    r#"(defmacro unless [pred a b]
  ; Guard macro for inverted conditions
  `(if (not ~pred) ~a ~b))"#
);

typing_core_test_with_parser!(
    clojure_function_with_block_comment,
    "clojure",
    r#"(defn calculate [x]
  #_{ This is a block comment
      spanning multiple lines }
  (+ x (* x 2)))"#
);

typing_core_test_with_parser!(
    clojure_protocol_with_comments,
    "clojure",
    r#"(defprotocol Drawable
  ; Protocol for drawable objects
  (draw [this])
  ; Clear method
  (clear [this]))"#
);

typing_core_test_with_parser!(
    clojure_type_with_comments,
    "clojure",
    r#"(deftype Point [x y]
  ; Coordinate point
  Object
  (toString [this]
    ; Override toString
    (str "(" x "," y ")")))"#
);

typing_core_test_with_parser!(
    clojure_record_with_comments,
    "clojure",
    r#"(defrecord User [name email]
  ; User record with profile
  Object
  (toString [_]
    ; String representation
    (str name " <" email ">")))"#
);

typing_core_test_with_parser!(
    clojure_empty_line_preservation_enabled,
    "clojure",
    r#"(defn process [x]
  (let [y 1]

    (+ x y)))

  (println "done")"#,
    ProcessingOptions {
        preserve_empty_lines: true,
        ..ProcessingOptions::default()
    }
);

typing_core_test_with_parser!(
    clojure_empty_line_preservation_disabled,
    "clojure",
    r#"(defn process [x]
  (let [y 1]

    (+ x y)))

  (println "done")"#,
    ProcessingOptions {
        preserve_empty_lines: false,
        ..ProcessingOptions::default()
    }
);

typing_core_test_with_parser!(
    clojure_nested_structure_with_comments,
    "clojure",
    r#"(defn complex-function []
  ; Main function
  (let [result
        ; Initialize with value
        (map
          ; Transform function
          #(* % 2)
          ; Input data
          [1 2 3])]
    ; Return result
    result))"#
);
