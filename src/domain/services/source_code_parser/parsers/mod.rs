use crate::domain::models::languages::{
    CSharp, Clojure, Cpp, Dart, Elixir, Go, Haskell, Java, JavaScript, Kotlin, Php, Python, Ruby,
    Rust, Scala, Swift, TypeScript, Zig, C,
};
use crate::domain::models::ChunkType;
use crate::domain::models::Language;
use crate::{GitTypeError, Result};
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::collections::HashMap;
use tree_sitter::{Node, Parser, Query, Tree};

pub mod c;
pub mod clojure;
pub mod cpp;
pub mod csharp;
pub mod dart;
pub mod elixir;
pub mod go;
pub mod haskell;
pub mod java;
pub mod javascript;
pub mod kotlin;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod swift;
pub mod typescript;
pub mod zig;

pub trait LanguageExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language;

    fn comment_query(&self) -> &str;

    fn query_patterns(&self) -> &str;
    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType>;
    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String>;

    fn middle_implementation_query(&self) -> &str;
    fn middle_capture_name_to_chunk_type(&self, _capture_name: &str) -> Option<ChunkType>;
}

type ParserFactory = fn() -> Result<Parser>;
type ExtractorFactory = fn() -> Box<dyn LanguageExtractor>;

pub struct ParserRegistry {
    parsers: HashMap<String, ParserFactory>,
    extractors: HashMap<String, ExtractorFactory>,
}

impl ParserRegistry {
    fn new() -> Self {
        let mut registry = Self {
            parsers: HashMap::new(),
            extractors: HashMap::new(),
        };

        // Register all supported languages using a macro to reduce repetition
        macro_rules! register_language {
            ($lang:ident, $module:ident, $extractor:ident) => {
                registry.register(
                    $lang.name().to_string(),
                    $module::$extractor::create_parser,
                    || Box::new($module::$extractor),
                );
            };
        }

        register_language!(C, c, CExtractor);
        register_language!(Clojure, clojure, ClojureExtractor);
        register_language!(Cpp, cpp, CppExtractor);
        register_language!(CSharp, csharp, CSharpExtractor);
        register_language!(Dart, dart, DartExtractor);
        register_language!(Elixir, elixir, ElixirExtractor);
        register_language!(Go, go, GoExtractor);
        register_language!(Haskell, haskell, HaskellExtractor);
        register_language!(Java, java, JavaExtractor);
        register_language!(JavaScript, javascript, JavaScriptExtractor);
        register_language!(Kotlin, kotlin, KotlinExtractor);
        register_language!(Php, php, PhpExtractor);
        register_language!(Python, python, PythonExtractor);
        register_language!(TypeScript, typescript, TypeScriptExtractor);
        register_language!(Ruby, ruby, RubyExtractor);
        register_language!(Rust, rust, RustExtractor);
        register_language!(Scala, scala, ScalaExtractor);
        register_language!(Swift, swift, SwiftExtractor);
        register_language!(Zig, zig, ZigExtractor);

        registry
    }

    fn register(
        &mut self,
        language: String,
        parser_factory: ParserFactory,
        extractor_factory: ExtractorFactory,
    ) {
        self.parsers.insert(language.clone(), parser_factory);
        self.extractors.insert(language, extractor_factory);
    }

    pub fn create_parser(&self, language: &str) -> Result<Parser> {
        self.parsers
            .get(language)
            .ok_or_else(|| {
                GitTypeError::ExtractionFailed(format!("Unsupported language: {}", language))
            })
            .and_then(|factory| factory())
    }

    pub fn get_extractor(&self, language: &str) -> Result<Box<dyn LanguageExtractor>> {
        self.extractors
            .get(language)
            .ok_or_else(|| {
                GitTypeError::ExtractionFailed(format!("Unsupported language: {}", language))
            })
            .map(|factory| factory())
    }

    pub fn create_query(&self, language: &str) -> Result<Query> {
        let extractor = self.get_extractor(language)?;
        let tree_sitter_lang = extractor.tree_sitter_language();
        let query_str = extractor.query_patterns();

        Query::new(&tree_sitter_lang, query_str).map_err(|e| {
            GitTypeError::ExtractionFailed(format!(
                "Failed to create query for {}: {}",
                language, e
            ))
        })
    }

    pub fn create_comment_query(&self, language: &str) -> Result<Query> {
        let extractor = self.get_extractor(language)?;
        let tree_sitter_lang = extractor.tree_sitter_language();
        let query_str = extractor.comment_query();

        Query::new(&tree_sitter_lang, query_str).map_err(|e| {
            GitTypeError::ExtractionFailed(format!(
                "Failed to create comment query for {}: {}",
                language, e
            ))
        })
    }

    pub fn create_middle_implementation_query(&self, language: &str) -> Result<Query> {
        let extractor = self.get_extractor(language)?;
        let tree_sitter_lang = extractor.tree_sitter_language();
        let query_str = extractor.middle_implementation_query();

        // If query is empty, create a dummy query that matches nothing
        let actual_query = if query_str.trim().is_empty() {
            "(ERROR) @dummy" // This will never match anything but is syntactically valid
        } else {
            query_str
        };

        Query::new(&tree_sitter_lang, actual_query).map_err(|e| {
            GitTypeError::ExtractionFailed(format!(
                "Failed to create middle implementation query for {}: {}",
                language, e
            ))
        })
    }

    pub fn supported_languages(&self) -> Vec<String> {
        self.parsers.keys().cloned().collect()
    }
}

static REGISTRY: Lazy<ParserRegistry> = Lazy::new(ParserRegistry::new);

pub fn get_parser_registry() -> &'static ParserRegistry {
    &REGISTRY
}

thread_local! {
    static TL_PARSERS: RefCell<HashMap<String, Parser>> = RefCell::new(HashMap::new());
}

/// Parse source using a thread-local parser per language to avoid re-allocations.
pub fn parse_with_thread_local(language: &str, content: &str) -> Option<Tree> {
    TL_PARSERS.with(|cell| {
        let mut map = cell.borrow_mut();
        let parser = match map.get_mut(language) {
            Some(p) => p,
            None => {
                // Create and insert parser if not exists
                match REGISTRY.create_parser(language) {
                    Ok(p) => {
                        map.insert(language.to_string(), p);
                        map.get_mut(language).unwrap()
                    }
                    Err(_) => return None,
                }
            }
        };
        parser.parse(content, None)
    })
}
