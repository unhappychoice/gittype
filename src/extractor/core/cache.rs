// Parser caching is complex due to tree_sitter objects not being cloneable.
// For now, we'll create fresh instances each time for simplicity.
// This could be optimized later with more complex caching strategies.

pub struct ParserCache;

impl Default for ParserCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserCache {
    pub fn new() -> Self {
        Self
    }

    pub fn create_parser(
        &self,
        language: crate::extractor::models::Language,
    ) -> crate::Result<tree_sitter::Parser> {
        let registry = crate::extractor::parsers::get_parser_registry();
        registry.create_parser(language)
    }

    pub fn create_query(
        &self,
        language: crate::extractor::models::Language,
        query_str: &str,
    ) -> crate::Result<tree_sitter::Query> {
        let registry = crate::extractor::parsers::get_parser_registry();
        let extractor = registry.get_extractor(language)?;
        let tree_sitter_lang = extractor.tree_sitter_language();

        tree_sitter::Query::new(tree_sitter_lang, query_str).map_err(|e| {
            crate::GitTypeError::ExtractionFailed(format!("Failed to create query: {}", e))
        })
    }
}
