// Don't import here since they're used inside the macro
// use these inside the macro expansion

/// Test macro for language extractors with snapshot functionality
macro_rules! test_language_extractor {
    (
        name: $test_name:ident,
        language: $language:literal,
        extension: $extension:literal,
        source: $source:expr,
        total_chunks: $total:expr,
        chunk_counts: {
            $($chunk_type:ident: $count:expr),* $(,)?
        }
    ) => {
        #[test]
        fn $test_name() {
            use crate::integration::{extract_chunks_for_test, test_extraction_options};
            use gittype::extractor::CodeChunkExtractor;
            use gittype::models::ChunkType;
            use std::collections::HashMap;
            use std::fs;
            use tempfile::TempDir;

            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join(concat!("test.", $extension));

            fs::write(&file_path, $source).unwrap();

            let mut extractor = CodeChunkExtractor::new().unwrap();
            let chunks = extract_chunks_for_test(
                &mut extractor,
                temp_dir.path(),
                test_extraction_options()
            ).unwrap();

            // Check total chunk count
            assert_eq!(
                chunks.len(),
                $total,
                "Expected {} total chunks, got {}",
                $total,
                chunks.len()
            );

            // Check chunk counts by type
            let mut chunk_counts: HashMap<ChunkType, usize> = HashMap::new();
            for chunk in &chunks {
                *chunk_counts.entry(chunk.chunk_type.clone()).or_insert(0) += 1;
            }

            $(
                let expected_count = $count;
                let actual_count = chunk_counts.get(&ChunkType::$chunk_type).copied().unwrap_or(0);
                assert_eq!(
                    actual_count,
                    expected_count,
                    "Expected {} {:?} chunks, got {}",
                    expected_count,
                    ChunkType::$chunk_type,
                    actual_count
                );
            )*

            // Verify all chunks have correct language
            for chunk in &chunks {
                assert_eq!(chunk.language, $language.to_string());
            }

            // Always create snapshot
            crate::integration::languages::extractor::create_insta_snapshot(&chunks, $source, stringify!($test_name));
        }
    };
}

/// Create a snapshot using insta
pub fn create_insta_snapshot(
    chunks: &[gittype::models::CodeChunk],
    source_code: &str,
    test_name: &str,
) {
    use insta::assert_snapshot;
    use serde_json::json;

    let snapshot_data = json!({
        "test_name": test_name,
        "source_code": source_code,
        "total_chunks": chunks.len(),
        "chunks": chunks.iter().map(|chunk| {
            json!({
                "chunk_type": format!("{:?}", chunk.chunk_type),
                "name": chunk.name,
                "language": chunk.language,
                "start_line": chunk.start_line,
                "end_line": chunk.end_line,
                "original_indentation": chunk.original_indentation,
                "comment_ranges": chunk.comment_ranges,
                "content": chunk.content
            })
        }).collect::<Vec<_>>()
    });

    assert_snapshot!(
        test_name,
        serde_json::to_string_pretty(&snapshot_data).unwrap()
    );
}

// Export the macro for use in other modules
pub(crate) use test_language_extractor;
