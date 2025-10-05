use super::parsers::get_parser_registry;
use super::CacheBuilder;
use crate::domain::models::Language;
use crate::Result;
use streaming_iterator::StreamingIterator;
use tree_sitter::{QueryCursor, Tree};

pub struct CommentProcessor;

impl CommentProcessor {
    pub fn extract_comment_ranges(
        tree: &Tree,
        source_code: &str,
        language: &dyn Language,
        byte_to_char_cache: &[usize],
    ) -> Result<Vec<(usize, usize)>> {
        let mut comment_ranges: Vec<(usize, usize)> =
            Self::query_comment_nodes(tree, source_code, language)?
                .into_iter()
                .map(|node| {
                    (
                        Self::byte_to_char_cached(byte_to_char_cache, node.start_byte()),
                        Self::byte_to_char_cached(byte_to_char_cache, node.end_byte()),
                    )
                })
                .collect();

        comment_ranges.sort_by_key(|&(start, _)| start);
        Ok(comment_ranges)
    }

    fn query_comment_nodes<'a>(
        tree: &'a Tree,
        source_code: &str,
        language: &dyn Language,
    ) -> Result<Vec<tree_sitter::Node<'a>>> {
        let comment_query = get_parser_registry().create_comment_query(language.name())?;
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&comment_query, tree.root_node(), source_code.as_bytes());

        let mut all_captures = Vec::new();
        while let Some(m) = matches.next() {
            for capture in m.captures {
                all_captures.push(capture.node);
            }
        }

        Ok(all_captures
            .into_iter()
            .filter(|node| language.is_valid_comment_node(*node))
            .collect())
    }

    pub fn convert_parent_comment_ranges_to_chunk(
        parent_comment_ranges: &[(usize, usize)],
        parent_byte_to_char_cache: &[usize],
        parent_source_code: &str,
        chunk_content: &str,
    ) -> Vec<(usize, usize)> {
        // Find where the chunk content starts in the parent source code
        let byte_offset = if let Some(pos) = parent_source_code.find(chunk_content) {
            pos
        } else {
            // If we can't find the chunk in parent, return empty ranges
            return Vec::new();
        };

        // Convert byte offset to char offset in parent
        let char_offset_in_parent =
            CacheBuilder::byte_to_char_cached(parent_byte_to_char_cache, byte_offset);
        let chunk_char_end = char_offset_in_parent + chunk_content.chars().count();

        // Filter and convert parent comment ranges that overlap with this chunk
        parent_comment_ranges
            .iter()
            .filter_map(|&(comment_start, comment_end)| {
                // Check if comment overlaps with chunk
                if comment_end <= char_offset_in_parent || comment_start >= chunk_char_end {
                    return None;
                }

                // Calculate intersection
                let intersect_start = comment_start.max(char_offset_in_parent);
                let intersect_end = comment_end.min(chunk_char_end);

                // Convert to chunk-relative coordinates
                let chunk_relative_start = intersect_start - char_offset_in_parent;
                let chunk_relative_end = intersect_end - char_offset_in_parent;

                Some((chunk_relative_start, chunk_relative_end))
            })
            .collect()
    }

    fn byte_to_char_cached(cache: &[usize], byte_pos: usize) -> usize {
        if byte_pos >= cache.len() {
            cache.last().copied().unwrap_or(0)
        } else {
            cache[byte_pos]
        }
    }
}
