use super::parsers::{get_parser_registry, LanguageExtractor};
use super::{CacheBuilder, CommentProcessor, IndentProcessor};
use crate::domain::models::{ChunkType, CodeChunk, Language};
use crate::Result;
use std::path::{Path, PathBuf};
use streaming_iterator::StreamingIterator;
use tree_sitter::{QueryCursor, Tree};

pub struct ChunkExtractor;

pub struct ParentChunk<'a> {
    pub file_path: &'a PathBuf,
    pub start_line: usize,
    pub content: &'a str,
    pub comment_ranges: &'a [(usize, usize)],
    pub byte_to_char_cache: &'a [usize],
}

pub struct ChunkExtractionContext<'a> {
    pub tree: &'a Tree,
    pub source_code: &'a str,
    pub file_path: &'a Path,
    pub language: &'a dyn Language,
    pub line_cache: &'a [usize],
    pub query: &'a tree_sitter::Query,
    pub extractor: &'a dyn LanguageExtractor,
    pub parent: Option<&'a ParentChunk<'a>>,
}

impl ChunkExtractor {
    pub fn extract_chunks_from_tree(
        tree: &Tree,
        source_code: &str,
        file_path: &Path,
        git_root: &Path,
        language: &dyn Language,
    ) -> Result<Vec<CodeChunk>> {
        let mut chunks = Vec::new();
        let registry = get_parser_registry();
        let query = registry.create_query(language.name())?;
        let middle_query = registry.create_middle_implementation_query(language.name())?;
        let extractor = registry.get_extractor(language.name())?;
        let mut parser = registry.create_parser(language.name())?;

        let line_cache = CacheBuilder::build_line_cache(source_code);
        let relative_file_path = file_path
            .strip_prefix(git_root)
            .map(|relative| relative.to_path_buf())
            .unwrap_or_else(|_| file_path.to_path_buf());

        // Pre-compute parent comment ranges once for the entire file
        let parent_byte_to_char_cache = CacheBuilder::build_byte_to_char_cache(source_code);
        let parent_comment_ranges = CommentProcessor::extract_comment_ranges(
            tree,
            source_code,
            language,
            &parent_byte_to_char_cache,
        )?;

        // Extract standard function/class chunks
        let standard_chunks = Self::extract_chunks(&ChunkExtractionContext {
            tree,
            source_code,
            file_path: &relative_file_path,
            language,
            line_cache: &line_cache,
            query: &query,
            extractor: extractor.as_ref(),
            parent: None,
        })?;

        // Middle chunk processing
        let middle_chunks: Vec<_> = standard_chunks
            .iter()
            .filter(|chunk| chunk.content.len() >= 10 && chunk.content.lines().count() >= 3)
            .filter_map(|large_chunk| {
                let chunk_tree = parser.parse(&large_chunk.content, None)?;

                let parent = ParentChunk {
                    file_path: &large_chunk.file_path,
                    start_line: large_chunk.start_line,
                    content: &large_chunk.content,
                    comment_ranges: &parent_comment_ranges,
                    byte_to_char_cache: &parent_byte_to_char_cache,
                };

                let chunks_from_large = Self::extract_chunks(&ChunkExtractionContext {
                    tree: &chunk_tree,
                    source_code,
                    file_path: large_chunk.file_path.as_path(),
                    language,
                    line_cache: &line_cache,
                    query: &middle_query,
                    extractor: extractor.as_ref(),
                    parent: Some(&parent),
                })
                .ok()?;

                Some(chunks_from_large)
            })
            .flatten()
            .collect();

        // Zen chunk creation
        let zen_chunk = Self::build_zen_chunk(tree, source_code, &relative_file_path, language);

        // Add all chunks to final result
        chunks.extend(standard_chunks);
        chunks.extend(middle_chunks);
        chunks.push(zen_chunk);

        // Sorting and deduplication
        chunks.sort_by(|a, b| {
            let pos_cmp = (a.start_line, a.end_line).cmp(&(b.start_line, b.end_line));
            if pos_cmp == std::cmp::Ordering::Equal {
                let a_priority = match a.chunk_type {
                    ChunkType::Function => 0,
                    ChunkType::Class => 0,
                    ChunkType::Method => 0,
                    ChunkType::CodeBlock => 10,
                    ChunkType::File => 20,
                    _ => 5,
                };
                let b_priority = match b.chunk_type {
                    ChunkType::Function => 0,
                    ChunkType::Class => 0,
                    ChunkType::Method => 0,
                    ChunkType::CodeBlock => 10,
                    ChunkType::File => 20,
                    _ => 5,
                };
                a_priority.cmp(&b_priority)
            } else {
                pos_cmp
            }
        });

        chunks.dedup_by(|a, b| {
            a.start_line == b.start_line
                && a.end_line == b.end_line
                && a.chunk_type != ChunkType::File
                && b.chunk_type != ChunkType::File
        });

        Ok(chunks)
    }

    pub fn extract_chunks(ctx: &ChunkExtractionContext) -> Result<Vec<CodeChunk>> {
        let content = ctx.parent.map(|p| p.content).unwrap_or(ctx.source_code);

        let (byte_to_char_cache, comment_ranges) = match ctx.parent {
            Some(p) => {
                // For middle chunks, reuse parent comment ranges with coordinate conversion
                let chunk_byte_to_char_cache = CacheBuilder::build_byte_to_char_cache(content);
                let converted_comment_ranges =
                    CommentProcessor::convert_parent_comment_ranges_to_chunk(
                        p.comment_ranges,
                        p.byte_to_char_cache,
                        ctx.source_code,
                        content,
                    );
                (chunk_byte_to_char_cache, converted_comment_ranges)
            }
            None => {
                // For standard chunks, compute comment ranges normally
                let cache = CacheBuilder::build_byte_to_char_cache(content);
                let ranges = CommentProcessor::extract_comment_ranges(
                    ctx.tree,
                    content,
                    ctx.language,
                    &cache,
                )?;
                (cache, ranges)
            }
        };

        let chunks: Vec<_> = Self::extract_all_captures(ctx.query, ctx.tree.root_node(), content)
            .into_iter()
            .filter_map(|(node, capture_index)| {
                let capture_name = &ctx.query.capture_names()[capture_index];
                Self::build_chunk(
                    node,
                    ctx.source_code,
                    ctx.file_path,
                    ctx.language.name(),
                    capture_name,
                    ctx.extractor,
                    &comment_ranges,
                    &byte_to_char_cache,
                    ctx.line_cache,
                    ctx.parent,
                )
            })
            .collect();

        Ok(chunks)
    }

    #[allow(clippy::too_many_arguments)]
    fn build_chunk(
        node: tree_sitter::Node,
        source_code: &str,
        file_path: &Path,
        language: &str,
        capture_name: &str,
        extractor: &dyn LanguageExtractor,
        comment_ranges: &[(usize, usize)],
        byte_to_char_cache: &[usize],
        line_cache: &[usize],
        parent: Option<&ParentChunk>,
    ) -> Option<CodeChunk> {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let (start_char, end_char) =
            Self::calculate_char_positions(byte_to_char_cache, start_byte, end_byte);

        let content = match parent {
            None => &source_code[start_byte..end_byte],
            Some(p) => &p.content[start_byte..end_byte],
        };

        // Different validation for standard vs middle chunks
        match parent {
            None => {
                // Standard chunk validation
                let trimmed_content = content.trim();
                if trimmed_content.is_empty() || content.len() < 10 {
                    return None;
                }
            }
            Some(_) => {
                // Middle chunk validation
                let line_count = content.lines().count();
                if line_count < 2 || content.len() < 30 || content.len() > 2000 {
                    return None;
                }
            }
        }

        let (start_line, end_line) = match parent {
            None => {
                // Standard chunk: use node position directly
                (node.start_position().row + 1, node.end_position().row + 1)
            }
            Some(parent_chunk) => {
                // Middle chunk: calculate absolute line numbers
                let relative_start_line = node.start_position().row + 1;
                let relative_end_line = node.end_position().row + 1;
                let absolute_start_line = parent_chunk.start_line + relative_start_line - 1;
                let absolute_end_line = parent_chunk.start_line + relative_end_line - 1;
                (absolute_start_line, absolute_end_line)
            }
        };

        let line_row = match parent {
            None => node.start_position().row,
            Some(_) => start_line - 1,
        };

        let chunk_type = match parent {
            None => {
                // Standard chunk: use hard-coded mapping for now
                // TODO: Use extractor.capture_name_to_chunk_type() instead of hard-coding
                match capture_name {
                    "function" => ChunkType::Function,
                    "class" => ChunkType::Class,
                    "method" => ChunkType::Method,
                    _ => ChunkType::CodeBlock,
                }
            }
            Some(_) => {
                // Middle chunk: use extractor method
                extractor.middle_capture_name_to_chunk_type(capture_name)?
            }
        };

        let final_file_path = match parent {
            None => file_path.to_path_buf(),
            Some(parent_chunk) => parent_chunk.file_path.clone(),
        };

        let (normalized_content, indent_offset_chars) =
            Self::process_indentation_and_content(content, source_code, node, line_cache, line_row);

        let chunk_comment_ranges = Self::adjust_comment_ranges_to_chunk(
            comment_ranges,
            start_char,
            end_char,
            indent_offset_chars,
        );

        Some(CodeChunk {
            name: capture_name.to_owned(),
            content: normalized_content,
            chunk_type,
            language: language.to_owned(),
            file_path: final_file_path,
            start_line,
            end_line,
            comment_ranges: chunk_comment_ranges,
            original_indentation: indent_offset_chars,
        })
    }

    pub fn build_zen_chunk(
        tree: &Tree,
        source_code: &str,
        relative_file_path: &Path,
        language: &dyn Language,
    ) -> CodeChunk {
        let byte_to_char_cache = CacheBuilder::build_byte_to_char_cache(source_code);
        let comment_ranges = CommentProcessor::extract_comment_ranges(
            tree,
            source_code,
            language,
            &byte_to_char_cache,
        )
        .unwrap_or_default();

        CodeChunk {
            content: source_code.to_string(),
            file_path: relative_file_path.to_owned(),
            start_line: 1,
            end_line: source_code.lines().count(),
            language: language.name().to_owned(),
            chunk_type: ChunkType::File,
            name: "entire_file".to_string(),
            comment_ranges,
            original_indentation: 0,
        }
    }

    fn extract_all_captures<'a>(
        query: &tree_sitter::Query,
        node: tree_sitter::Node<'a>,
        source_code: &str,
    ) -> Vec<(tree_sitter::Node<'a>, usize)> {
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(query, node, source_code.as_bytes());

        let mut all_captures = Vec::new();
        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let node = capture.node;
                let capture_index = capture.index as usize;
                all_captures.push((node, capture_index));
            }
        }
        all_captures
    }

    fn calculate_char_positions(
        byte_to_char_cache: &[usize],
        start_byte: usize,
        end_byte: usize,
    ) -> (usize, usize) {
        let start_char = CacheBuilder::byte_to_char_cached(byte_to_char_cache, start_byte);
        let end_char = CacheBuilder::byte_to_char_cached(byte_to_char_cache, end_byte);
        (start_char, end_char)
    }

    fn process_indentation_and_content(
        content: &str,
        source_code: &str,
        node: tree_sitter::Node,
        line_cache: &[usize],
        line_row: usize,
    ) -> (String, usize) {
        let (normalized_content, original_indent_chars) =
            IndentProcessor::extract_and_normalize_indentation(
                content,
                source_code,
                line_row,
                node.start_position().column,
                line_cache,
            );
        let indent_offset_chars = original_indent_chars.chars().count();
        (normalized_content, indent_offset_chars)
    }

    fn adjust_comment_ranges_to_chunk(
        comment_ranges: &[(usize, usize)],
        start_char: usize,
        end_char: usize,
        indent_offset_chars: usize,
    ) -> Vec<(usize, usize)> {
        comment_ranges
            .iter()
            .filter_map(|&(comment_raw_start, comment_raw_end)| {
                if comment_raw_start >= start_char && comment_raw_end <= end_char {
                    Some((
                        comment_raw_start - start_char + indent_offset_chars,
                        comment_raw_end - start_char + indent_offset_chars,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }
}
