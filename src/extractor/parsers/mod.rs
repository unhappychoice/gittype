use crate::extractor::models::{
    language::{
        CSharp, Cpp, Dart, Go, Haskell, Java, JavaScript, Kotlin, Language, Php, Python, Ruby,
        Rust, Swift, TypeScript, C,
    },
};
use crate::models::ChunkType;
use crate::{GitTypeError, Result};
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::collections::HashMap;
use tree_sitter::{Node, Parser, Query, Tree};

pub mod c;
pub mod cpp;
pub mod csharp;
pub mod dart;
pub mod go;
pub mod haskell;
pub mod java;
pub mod javascript;
pub mod kotlin;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod swift;
pub mod typescript;

pub trait LanguageExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language;
    fn query_patterns(&self) -> &str;
    fn comment_query(&self) -> &str;
    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType>;
    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String>;
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

        // Register all supported languages
        registry.register(
            Rust.name().to_string(),
            rust::RustExtractor::create_parser,
            || Box::new(rust::RustExtractor),
        );

        registry.register(
            TypeScript.name().to_string(),
            typescript::TypeScriptExtractor::create_parser,
            || Box::new(typescript::TypeScriptExtractor),
        );

        registry.register(
            JavaScript.name().to_string(),
            javascript::JavaScriptExtractor::create_parser,
            || Box::new(javascript::JavaScriptExtractor),
        );

        registry.register(
            Python.name().to_string(),
            python::PythonExtractor::create_parser,
            || Box::new(python::PythonExtractor),
        );

        registry.register(
            Ruby.name().to_string(),
            ruby::RubyExtractor::create_parser,
            || Box::new(ruby::RubyExtractor),
        );

        registry.register(
            Go.name().to_string(),
            go::GoExtractor::create_parser,
            || Box::new(go::GoExtractor),
        );

        registry.register(
            Swift.name().to_string(),
            swift::SwiftExtractor::create_parser,
            || Box::new(swift::SwiftExtractor),
        );

        registry.register(
            Kotlin.name().to_string(),
            kotlin::KotlinExtractor::create_parser,
            || Box::new(kotlin::KotlinExtractor),
        );

        registry.register(
            Java.name().to_string(),
            java::JavaExtractor::create_parser,
            || Box::new(java::JavaExtractor),
        );

        registry.register(
            Php.name().to_string(),
            php::PhpExtractor::create_parser,
            || Box::new(php::PhpExtractor),
        );

        registry.register(
            CSharp.name().to_string(),
            csharp::CSharpExtractor::create_parser,
            || Box::new(csharp::CSharpExtractor),
        );

        registry.register(C.name().to_string(), c::CExtractor::create_parser, || {
            Box::new(c::CExtractor)
        });

        registry.register(
            Cpp.name().to_string(),
            cpp::CppExtractor::create_parser,
            || Box::new(cpp::CppExtractor),
        );

        registry.register(
            Haskell.name().to_string(),
            haskell::HaskellExtractor::create_parser,
            || Box::new(haskell::HaskellExtractor),
        );

        registry.register(
            Dart.name().to_string(),
            dart::DartExtractor::create_parser,
            || Box::new(dart::DartExtractor),
        );

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

    pub fn supported_languages(&self) -> Vec<String> {
        self.parsers.keys().cloned().collect()
    }
}

static REGISTRY: Lazy<ParserRegistry> = Lazy::new(ParserRegistry::new);

pub fn get_parser_registry() -> &'static ParserRegistry {
    &REGISTRY
}

thread_local! {
    static TL_PARSERS: RefCell<std::collections::HashMap<String, Parser>> = RefCell::new(std::collections::HashMap::new());
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
