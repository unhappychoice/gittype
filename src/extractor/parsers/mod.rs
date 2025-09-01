use crate::extractor::models::{ChunkType, Language};
use crate::{GitTypeError, Result};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tree_sitter::{Node, Parser, Query};

pub mod go;
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
    fn language(&self) -> Language;
    fn file_extensions(&self) -> &[&str];
    fn tree_sitter_language(&self) -> tree_sitter::Language;
    fn query_patterns(&self) -> &str;
    fn comment_query(&self) -> &str;
    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType>;
    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String>;
}

type ParserFactory = fn() -> Result<Parser>;
type ExtractorFactory = fn() -> Box<dyn LanguageExtractor>;

pub struct ParserRegistry {
    parsers: HashMap<Language, ParserFactory>,
    extractors: HashMap<Language, ExtractorFactory>,
}

impl ParserRegistry {
    fn new() -> Self {
        let mut registry = Self {
            parsers: HashMap::new(),
            extractors: HashMap::new(),
        };

        // Register all supported languages
        registry.register(Language::Rust, rust::RustExtractor::create_parser, || {
            Box::new(rust::RustExtractor)
        });

        registry.register(
            Language::TypeScript,
            typescript::TypeScriptExtractor::create_parser,
            || Box::new(typescript::TypeScriptExtractor),
        );

        registry.register(
            Language::JavaScript,
            javascript::JavaScriptExtractor::create_parser,
            || Box::new(javascript::JavaScriptExtractor),
        );

        registry.register(
            Language::Python,
            python::PythonExtractor::create_parser,
            || Box::new(python::PythonExtractor),
        );

        registry.register(Language::Ruby, ruby::RubyExtractor::create_parser, || {
            Box::new(ruby::RubyExtractor)
        });

        registry.register(Language::Go, go::GoExtractor::create_parser, || {
            Box::new(go::GoExtractor)
        });

        registry.register(
            Language::Swift,
            swift::SwiftExtractor::create_parser,
            || Box::new(swift::SwiftExtractor),
        );

        registry.register(
            Language::Kotlin,
            kotlin::KotlinExtractor::create_parser,
            || Box::new(kotlin::KotlinExtractor),
        );

        registry.register(Language::Java, java::JavaExtractor::create_parser, || {
            Box::new(java::JavaExtractor)
        });

        registry.register(Language::Php, php::PhpExtractor::create_parser, || {
            Box::new(php::PhpExtractor)
        });

        registry
    }

    fn register(
        &mut self,
        language: Language,
        parser_factory: ParserFactory,
        extractor_factory: ExtractorFactory,
    ) {
        self.parsers.insert(language, parser_factory);
        self.extractors.insert(language, extractor_factory);
    }

    pub fn create_parser(&self, language: Language) -> Result<Parser> {
        self.parsers
            .get(&language)
            .ok_or_else(|| {
                GitTypeError::ExtractionFailed(format!("Unsupported language: {:?}", language))
            })
            .and_then(|factory| factory())
    }

    pub fn get_extractor(&self, language: Language) -> Result<Box<dyn LanguageExtractor>> {
        self.extractors
            .get(&language)
            .ok_or_else(|| {
                GitTypeError::ExtractionFailed(format!("Unsupported language: {:?}", language))
            })
            .map(|factory| factory())
    }

    pub fn create_query(&self, language: Language) -> Result<Query> {
        let extractor = self.get_extractor(language)?;
        let tree_sitter_lang = extractor.tree_sitter_language();
        let query_str = extractor.query_patterns();

        Query::new(tree_sitter_lang, query_str).map_err(|e| {
            GitTypeError::ExtractionFailed(format!(
                "Failed to create query for {:?}: {}",
                language, e
            ))
        })
    }

    pub fn create_comment_query(&self, language: Language) -> Result<Query> {
        let extractor = self.get_extractor(language)?;
        let tree_sitter_lang = extractor.tree_sitter_language();
        let query_str = extractor.comment_query();

        Query::new(tree_sitter_lang, query_str).map_err(|e| {
            GitTypeError::ExtractionFailed(format!(
                "Failed to create comment query for {:?}: {}",
                language, e
            ))
        })
    }

    pub fn supported_languages(&self) -> Vec<Language> {
        self.parsers.keys().copied().collect()
    }
}

static REGISTRY: Lazy<ParserRegistry> = Lazy::new(ParserRegistry::new);

pub fn get_parser_registry() -> &'static ParserRegistry {
    &REGISTRY
}
