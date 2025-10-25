use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_clojure_defn_extraction,
    language: "clojure",
    extension: "clj",
    source: r#"
(defn hello-world []
  42)

(defn add [a b]
  (+ a b))
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        Function: 2,
        CodeBlock: 1,
    }
}

test_language_extractor! {
    name: test_clojure_defmacro_extraction,
    language: "clojure",
    extension: "clj",
    source: r#"
(defmacro unless [pred a b]
  `(if (not ~pred) ~a ~b))

(defmacro my-macro [x]
  x)
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Function: 2,
    }
}

test_language_extractor! {
    name: test_clojure_defn_dash_extraction,
    language: "clojure",
    extension: "clj",
    source: r#"
(defn- private-helper []
  42)

(defn- calculate-internal [x]
  (* x 2))
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Function: 2,
        CodeBlock: 2,
    }
}

test_language_extractor! {
    name: test_clojure_deftype_extraction,
    language: "clojure",
    extension: "clj",
    source: r#"
(deftype Point [x y]
   Object
   (toString [this]
     "Point"))

(deftype Counter [^:volatile-mutable count]
   Incrementable
   (increment [_]
     nil))
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Class: 2,
        CodeBlock: 1,
        Conditional: 1,
    }
}

test_language_extractor! {
    name: test_clojure_defprotocol_extraction,
    language: "clojure",
    extension: "clj",
    source: r#"
(defprotocol Animal
  (speak [this]))

(defprotocol Drawable
  (draw [this x y])
  (clear [this]))
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Interface: 2,
    }
}

test_language_extractor! {
    name: test_clojure_defrecord_extraction,
    language: "clojure",
    extension: "clj",
    source: r#"
(defrecord Person [name age]
   Object
   (toString [_]
     "Person"))

(defrecord Address [street city zip]
   Validateable
   (validate [_]
     true))
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        Class: 2,
        CodeBlock: 1,
    }
}

test_language_extractor! {
    name: test_clojure_mixed_definition_forms,
    language: "clojure",
    extension: "clj",
    source: r#"
(defn public-function []
  1)

(defmacro my-macro [x]
  x)

(defn- private-function []
  2)

(defprotocol MyProtocol
  (my-method [this]))

(deftype MyType [value]
  Object
  (toString [_] "T"))

(defrecord MyRecord [field1 field2]
  Comparable
  (compareTo [this other]
    0))
"#,
    total_chunks: 12,
    chunk_counts: {
        File: 1,
        Function: 3,
        Interface: 1,
        Class: 2,
        CodeBlock: 4,
        Conditional: 1,
    }
}

test_language_extractor! {
    name: test_clojure_def_extraction,
    language: "clojure",
    extension: "clj",
    source: r#"
(def my-config
  {:host "localhost"
   :port 3000})

(def max-retries 5)

(def api-key "secret-key")
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        Variable: 3,
    }
}

test_language_extractor! {
    name: test_clojure_ns_extraction,
    language: "clojure",
    extension: "clj",
    source: r#"
(ns myapp.core
  (:require [clojure.string :as str]))

(ns myapp.utils)
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        Namespace: 2,
        CodeBlock: 1,
    }
}

test_language_extractor! {
    name: test_clojure_def_and_ns_mixed,
    language: "clojure",
    extension: "clj",
    source: r#"
(ns myapp.core)

(def config {:port 3000})

(defn start-server []
  (println "Starting..."))

(def version "1.0.0")
"#,
    total_chunks: 6,
    chunk_counts: {
        File: 1,
        Namespace: 1,
        Variable: 2,
        Function: 1,
        CodeBlock: 1,
    }
}
